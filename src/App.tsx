import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import {
  ProjectAnalysis,
  ScanResult,
  Settings,
  StaticProgress,
  StaticReport,
  SummarizeProgress,
  FileNode,
  FileSymbols,
} from "./types";
import {
  aiExplainFile,
  aiSummarizeProject,
  analyzeStatic,
  exportXmind,
  getFileSymbols,
  readFileContent,
  saveFileContent,
  Strength,
} from "./api";
import FileTree from "./components/FileTree";
import LanguageStats from "./components/LanguageStats";
import SettingsModal from "./components/SettingsModal";
import StaticReportView from "./components/StaticReportView";
import CodeEditor from "./components/CodeEditor";
import { formatBytes } from "./utils";

const DEFAULTS: Settings = {
  protocol: "openai",
  baseUrl: "https://api.deepseek.com/v1",
  apiKey: "",
  model: "deepseek-chat",
  azure_deployment: null,
  azure_api_version: null,
  proxy_mode: "system",
  proxy_url: "",
};

type Tab =
  | {
      id: string;
      kind: "file";
      path: string;
      language: string | null;
      content: string;
      dirty: boolean;
      loading: boolean;
      error: string | null;
    }
  | { id: "report"; kind: "report" }
  | { id: "overview"; kind: "overview" };

function findNode(root: FileNode | undefined, path: string): FileNode | null {
  if (!root) return null;
  const walk = (node: FileNode): FileNode | null => {
    if (node.relativePath === path) return node;
    for (const c of node.children) {
      const hit = walk(c);
      if (hit) return hit;
    }
    return null;
  };
  return walk(root);
}

export default function App() {
  const [result, setResult] = useState<ScanResult | null>(null);
  const [rootPath, setRootPath] = useState<string>("");
  const [selected, setSelected] = useState<FileNode | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const [settings, setSettings] = useState<Settings | null>(null);
  const [showSettings, setShowSettings] = useState(false);

  const [analysis, setAnalysis] = useState<ProjectAnalysis | null>(null);
  const [analyzing, setAnalyzing] = useState(false);
  const [progress, setProgress] = useState<SummarizeProgress | null>(null);
  const [strength, setStrength] = useState<Strength>("medium");
  const [fullScope, setFullScope] = useState(false);
  const [unlimitedOutput, setUnlimitedOutput] = useState(false);

  const [staticReport, setStaticReport] = useState<StaticReport | null>(null);
  const [staticRunning, setStaticRunning] = useState(false);
  const [staticProgress, setStaticProgress] = useState<StaticProgress | null>(null);

  // ---- Tabs ----
  const [tabs, setTabs] = useState<Tab[]>([]);
  const [activeId, setActiveId] = useState<string | null>(null);

  const [fileSymbols, setFileSymbols] = useState<FileSymbols | null>(null);
  const [fileExplanation, setFileExplanation] = useState<string | null>(null);
  const [explaining, setExplaining] = useState(false);

  const [toast, setToast] = useState<string | null>(null);
  const selectedRef = useRef<FileNode | null>(null);
  selectedRef.current = selected;

  function showToast(msg: string) {
    setToast(msg);
    window.setTimeout(() => setToast(null), 3000);
  }

  useEffect(() => {
    invoke<Settings>("get_settings").then(setSettings).catch(() => setSettings(null));
  }, []);

  // ---------- Tab management ----------
  function activate(id: string) {
    setActiveId(id);
    setTabs((ts) => {
      const t = ts.find((x) => x.id === id);
      if (t && t.kind === "file") {
        // Sync right-hand detail panel with the activated file.
        const node = findNode(result?.tree, t.path);
        if (node) loadSymbolsFor(node);
      }
      return ts;
    });
  }

  function loadSymbolsFor(node: FileNode) {
    setSelected(node);
    setFileExplanation(null);
    setFileSymbols(null);
    if (!node.isDir && node.language && rootPath) {
      getFileSymbols(rootPath, node.relativePath, node.language)
        .then((fs) => {
          if (selectedRef.current?.relativePath === node.relativePath) {
            setFileSymbols(fs);
          }
        })
        .catch(() => {});
    }
  }

  function openFile(node: FileNode) {
    if (node.isDir) return;
    const id = `file:${node.relativePath}`;
    setTabs((ts) => {
      if (!ts.some((t) => t.id === id)) {
        const tab: Tab = {
          id,
          kind: "file",
          path: node.relativePath,
          language: node.language,
          content: "",
          dirty: false,
          loading: true,
          error: null,
        };
        setActiveId(id);
        return [...ts, tab];
      }
      setActiveId(id);
      return ts;
    });
    loadSymbolsFor(node);

    readFileContent(rootPath, node.relativePath)
      .then((fc) => {
        setTabs((ts) =>
          ts.map((t) =>
            t.id === id && t.kind === "file"
              ? { ...t, content: fc.content, loading: false, error: null }
              : t
          )
        );
      })
      .catch((e) => {
        setTabs((ts) =>
          ts.map((t) =>
            t.id === id && t.kind === "file"
              ? { ...t, loading: false, error: String(e) }
              : t
          )
        );
      });
  }

  function closeTab(id: string) {
    const tab = tabs.find((t) => t.id === id);
    if (tab && tab.kind === "file" && tab.dirty) {
      const ok = window.confirm(`「${tab.path.split("/").pop()}」有未保存的修改，确定关闭？`);
      if (!ok) return;
    }
    setTabs((ts) => {
      const idx = ts.findIndex((t) => t.id === id);
      const next = ts.filter((t) => t.id !== id);
      if (activeId === id) {
        const fallback = next[Math.max(0, idx - 1)];
        setActiveId(fallback ? fallback.id : null);
      }
      return next;
    });
  }

  function updateFileTab(id: string, patch: Partial<Extract<Tab, { kind: "file" }>>) {
    setTabs((ts) => ts.map((t) => (t.id === id && t.kind === "file" ? { ...t, ...patch } : t)));
  }

  function saveActive() {
    const tab = tabs.find((t) => t.id === activeId);
    if (!tab || tab.kind !== "file" || tab.loading || tab.error) return;
    saveFileContent(rootPath, tab.path, tab.content)
      .then(() => {
        updateFileTab(tab.id, { dirty: false });
        showToast(`✅ 已保存 ${tab.path}`);
      })
      .catch((e) => setError(String(e)));
  }

  function ensureTab(id: "report" | "overview") {
    setTabs((ts) => (ts.some((t) => t.id === id) ? ts : [...ts, { id, kind: id } as Tab]));
    setActiveId(id);
  }

  // ---------- Actions ----------
  async function handleOpenFolder() {
    setError(null);
    const dir = await open({ directory: true, multiple: false });
    if (!dir) return;
    setLoading(true);
    try {
      const res = await invoke<ScanResult>("scan_project", { path: dir });
      setResult(res);
      setRootPath(dir);
      setSelected(null);
      setAnalysis(null);
      setStaticReport(null);
      setFileExplanation(null);
      setTabs([]);
      setActiveId(null);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }

  function requireConfigured(): boolean {
    if (!settings || !settings.apiKey) {
      setShowSettings(true);
      return false;
    }
    return true;
  }

  async function handleAnalyzeProject() {
    if (!requireConfigured() || !rootPath) return;
    setAnalyzing(true);
    setError(null);
    setProgress(null);
    try {
      const res = await aiSummarizeProject(
        rootPath,
        strength,
        fullScope,
        unlimitedOutput,
        (p) => setProgress(p)
      );
      setAnalysis(res);
      ensureTab("overview");
    } catch (e) {
      setError(String(e));
    } finally {
      setAnalyzing(false);
      setProgress(null);
    }
  }

  async function handleStaticAnalysis() {
    if (!rootPath || staticRunning) return;
    setStaticRunning(true);
    setError(null);
    setStaticProgress(null);
    try {
      const report = await analyzeStatic(rootPath, (p) => setStaticProgress(p));
      setStaticReport(report);
      ensureTab("report");
    } catch (e) {
      setError(String(e));
    } finally {
      setStaticRunning(false);
      setStaticProgress(null);
    }
  }

  async function handleExportXmind() {
    if (!result || !rootPath) return;
    const out = await save({
      title: "导出 xmind 思维导图",
      defaultPath: `${result.rootName}.xmind`,
      filters: [{ name: "XMind 思维导图", extensions: ["xmind"] }],
    });
    if (!out) return;
    try {
      await exportXmind(rootPath, out, analysis?.fileSummaries);
      showToast(`✅ 已导出: ${out}`);
    } catch (e) {
      setError(String(e));
    }
  }

  async function handleExplainFile(node: FileNode) {
    if (!requireConfigured()) return;
    setExplaining(true);
    setFileExplanation(null);
    try {
      let text: string;
      const cached = analysis?.fileSummaries.find(
        (s) => s.relativePath === node.relativePath
      );
      if (cached && !cached.summary.startsWith("⚠️")) {
        text = cached.summary;
      } else {
        text = await aiExplainFile(rootPath, node.relativePath, strength, unlimitedOutput);
      }
      if (selectedRef.current?.relativePath === node.relativePath) {
        setFileExplanation(text);
      }
    } catch (e) {
      if (selectedRef.current?.relativePath === node.relativePath) {
        setFileExplanation(`⚠️ ${String(e)}`);
      }
    } finally {
      setExplaining(false);
    }
  }

  const activeTab = tabs.find((t) => t.id === activeId) ?? null;

  return (
    <div className="app">
      <header className="topbar">
        <div className="logo">🦸 Code Superman</div>
        <button className="primary-btn" onClick={handleOpenFolder} disabled={loading}>
          {loading ? "扫描中..." : "打开项目文件夹"}
        </button>
        {result && (
          <>
            <button className="ghost-btn" onClick={handleStaticAnalysis} disabled={staticRunning || loading}>
              📊 静态分析{staticReport ? " ✓" : ""}
            </button>
            <select
              className="strength-select"
              value={strength}
              onChange={(e) => setStrength(e.target.value as Strength)}
              title="理解强度：控制 AI 输出内容的详细程度"
              disabled={analyzing}
            >
              <option value="light">⚡ 简要</option>
              <option value="medium">⚖️ 标准</option>
              <option value="deep">🔬 详尽</option>
            </select>
            <label className="toggle" title="开启后尝试分析全部代码文件（上限200个），耗时和费用更高">
              <input type="checkbox" checked={fullScope} onChange={(e) => setFullScope(e.target.checked)} disabled={analyzing} />
              全量分析
            </label>
            <label className="toggle" title="不限制 AI 单次输出的长度（费用可能增加）">
              <input type="checkbox" checked={unlimitedOutput} onChange={(e) => setUnlimitedOutput(e.target.checked)} disabled={analyzing} />
              不限长度
            </label>
            <button
              className="primary-btn"
              onClick={handleAnalyzeProject}
              disabled={analyzing || loading}
              title={settings?.apiKey ? "" : "请先配置 AI 服务"}
            >
              {analyzing ? "AI 理解中..." : "🤖 AI 理解项目"}
            </button>
            <button className="ghost-btn" onClick={handleExportXmind} disabled={loading}>
              📤 导出 xmind
            </button>
          </>
        )}
        <div style={{ marginLeft: "auto" }}>
          <button
            className={`ghost-btn ${settings?.apiKey ? "" : "attention"}`}
            onClick={() => setShowSettings(true)}
          >
            ⚙️ {settings?.apiKey ? `已连接: ${settings.model}` : "配置 AI 服务"}
          </button>
        </div>
      </header>

      {error && <div className="error-bar">{error}</div>}
      {toast && <div className="toast">{toast}</div>}

      {!result ? (
        <div className="welcome">
          <h1>把你的代码黑盒变成白盒</h1>
          <p>打开一个项目文件夹，AI 将为你解析架构、技术栈与每个文件的作用。</p>
          <button className="primary-btn big" onClick={handleOpenFolder} disabled={loading}>
            选择项目文件夹
          </button>
        </div>
      ) : (
        <main className="layout ide-layout">
          <aside className="sidebar">
            <FileTree
              root={result.tree}
              onSelect={(n) => {
                if (n.isDir) {
                  setSelected(n);
                  setFileSymbols(null);
                  setFileExplanation(null);
                } else {
                  openFile(n);
                }
              }}
              selectedPath={selected?.relativePath ?? null}
            />
          </aside>

          <section className="workspace">
            {/* Running indicators */}
            {(staticRunning || analyzing) && (
              <div className="run-strip">
                {staticRunning && staticProgress && (
                  <div className="run-item">
                    <span>{staticProgress.phase}</span>
                    <div className="mini-bar">
                      <div className="mini-bar-fill" style={{ width: `${staticProgress.percent}%` }} />
                    </div>
                    <b>{staticProgress.percent}%</b>
                  </div>
                )}
                {analyzing && progress && (
                  <div className="run-item">
                    <span>{progress.phase}</span>
                    <div className="mini-bar">
                      <div
                        className="mini-bar-fill"
                        style={{
                          width: `${progress.total ? Math.round((progress.done / progress.total) * 100) : 0}%`,
                        }}
                      />
                    </div>
                    <b>{progress.total ? Math.round((progress.done / progress.total) * 100) : 0}%</b>
                  </div>
                )}
              </div>
            )}

            {tabs.length > 0 && (
              <div className="tab-bar">
                {tabs.map((t) => (
                  <div
                    key={t.id}
                    className={`tab ${t.id === activeId ? "active" : ""}`}
                    onClick={() => activate(t.id)}
                  >
                    <span>
                      {t.kind === "file"
                        ? `📄 ${t.path.split("/").pop()}${t.dirty ? " ●" : ""}`
                        : t.kind === "report"
                          ? "📊 静态分析报告"
                          : "🤖 架构总览"}
                    </span>
                    <button
                      className="tab-close"
                      title="关闭（仅点击此处才退出该页面）"
                      onClick={(e) => {
                        e.stopPropagation();
                        closeTab(t.id);
                      }}
                    >
                      ×
                    </button>
                  </div>
                ))}
              </div>
            )}

            <div className="tab-body">
              {!activeTab ? (
                <div className="empty-hint">
                  <p>点击左侧文件打开代码编辑，或运行「📊 静态分析」「🤖 AI 理解项目」生成报告页签。</p>
                  {!staticReport && !analysis && <LanguageStats languages={result.languages} />}
                </div>
              ) : activeTab.kind === "file" ? (
                activeTab.loading ? (
                  <div className="empty-hint">正在加载文件...</div>
                ) : activeTab.error ? (
                  <div className="empty-hint">⚠️ {activeTab.error}</div>
                ) : (
                  <>
                    <div className="editor-toolbar">
                      <span className="mono">{activeTab.path}</span>
                      {activeTab.dirty && <em className="dirty-tag">未保存</em>}
                      <button className="primary-btn small" onClick={saveActive} disabled={!activeTab.dirty}>
                        💾 保存 (Ctrl+S)
                      </button>
                    </div>
                    <CodeEditor
                      key={activeTab.id}
                      content={activeTab.content}
                      language={activeTab.language}
                      readOnly={false}
                      onChange={(v) => updateFileTab(activeTab.id, { content: v, dirty: true })}
                      onSave={saveActive}
                    />
                  </>
                )
              ) : activeTab.kind === "report" && staticReport ? (
                <StaticReportView
                  report={staticReport}
                  rootPath={rootPath}
                  onSelectFile={(path) => {
                    const node = findNode(result.tree, path);
                    if (node) openFile(node);
                  }}
                />
              ) : activeTab.kind === "overview" && analysis ? (
                <div className="overview-wrap">
                  <h2>📋 项目架构总览</h2>
                  <pre className="ai-text">{analysis.overview}</pre>
                </div>
              ) : (
                <div className="empty-hint">数据已失效，请重新生成。</div>
              )}
            </div>
          </section>

          <aside className="right-panel">
            {selected ? (
              <>
                <h2>{selected.isDir ? "📁" : "📄"} {selected.name}</h2>
                <p className="path">{selected.relativePath}</p>
                <dl className="props">
                  {selected.language && (<><dt>语言</dt><dd>{selected.language}</dd></>)}
                  <dt>{selected.isDir ? "内容大小" : "文件大小"}</dt>
                  <dd>{formatBytes(selected.size)}</dd>
                </dl>

                {fileSymbols && fileSymbols.symbols.length > 0 && (
                  <div className="ai-section">
                    <div className="ai-section-head">
                      <h3>🧩 符号大纲（{fileSymbols.symbols.length}）</h3>
                    </div>
                    <ul className="outline-list">
                      {fileSymbols.symbols.map((s) => (
                        <li key={`${s.kind}-${s.name}-${s.startLine}`} className="outline-item">
                          <span className="outline-kind">{s.kind}</span>
                          <span className="outline-name" title={s.signature}>{s.name}</span>
                          <span className="outline-lines">L{s.startLine}{s.endLine > s.startLine ? `-${s.endLine}` : ""}</span>
                        </li>
                      ))}
                    </ul>
                  </div>
                )}

                {!selected.isDir && (
                  <div className="ai-section">
                    <div className="ai-section-head">
                      <h3>🤖 AI 解读</h3>
                      <button
                        className="primary-btn small"
                        onClick={() => handleExplainFile(selected)}
                        disabled={explaining}
                      >
                        {explaining ? "分析中..." : fileExplanation ? "重新解读" : "解读此文件"}
                      </button>
                    </div>
                    {explaining && <div className="thinking">正在阅读代码并生成解读...</div>}
                    {fileExplanation && <pre className="ai-text">{fileExplanation}</pre>}
                  </div>
                )}
              </>
            ) : (
              <>
                <LanguageStats languages={result.languages} />
                <p className="dim-note" style={{ marginTop: 16 }}>
                  点击左侧文件查看详情与符号大纲。
                </p>
              </>
            )}
          </aside>
        </main>
      )}

      {showSettings && (
        <SettingsModal
          initial={settings ?? DEFAULTS}
          onClose={() => setShowSettings(false)}
          onSaved={(s) => setSettings(s)}
        />
      )}
    </div>
  );
}
