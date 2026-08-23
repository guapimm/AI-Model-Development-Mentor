import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import {
  ProjectAnalysis,
  ScanResult,
  Settings,
  SummarizeProgress,
  FileNode,
} from "./types";
import { aiExplainFile, aiSummarizeProject, exportXmind } from "./api";
import FileTree from "./components/FileTree";
import LanguageStats from "./components/LanguageStats";
import SettingsModal from "./components/SettingsModal";
import { formatBytes } from "./utils";

const DEFAULTS = { baseUrl: "https://api.deepseek.com/v1", apiKey: "", model: "deepseek-chat" };

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

  const [fileExplanation, setFileExplanation] = useState<string | null>(null);
  const [explaining, setExplaining] = useState(false);
  const [toast, setToast] = useState<string | null>(null);
  const selectedRef = useRef<FileNode | null>(null);
  selectedRef.current = selected;

  function showToast(msg: string) {
    setToast(msg);
    window.setTimeout(() => setToast(null), 3000);
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

  useEffect(() => {
    invoke<Settings>("get_settings").then(setSettings).catch(() => setSettings(null));
  }, []);

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
      setFileExplanation(null);
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
      const res = await aiSummarizeProject(rootPath, 0, (p) => setProgress(p));
      setAnalysis(res);
    } catch (e) {
      setError(String(e));
    } finally {
      setAnalyzing(false);
      setProgress(null);
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
        text = await aiExplainFile(rootPath, node.relativePath);
      }
      // Ignore result if user switched files meanwhile.
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

  function handleSelect(node: FileNode) {
    setSelected(node);
    setFileExplanation(null);
  }

  return (
    <div className="app">
      <header className="topbar">
        <div className="logo">🦸 Code Superman</div>
        <button className="primary-btn" onClick={handleOpenFolder} disabled={loading}>
          {loading ? "扫描中..." : "打开项目文件夹"}
        </button>
        {result && (
          <>
            <button
              className="primary-btn"
              onClick={handleAnalyzeProject}
              disabled={analyzing || loading}
              title={settings?.apiKey ? "" : "请先配置 AI 服务"}
            >
              {analyzing ? "AI 理解中..." : "🤖 AI 理解项目"}
            </button>
            <button
              className="ghost-btn"
              onClick={handleExportXmind}
              disabled={loading}
              title="将项目架构导出为 XMind 思维导图（含 AI 摘要备注）"
            >
              📤 导出 xmind
            </button>
            <span className="summary">
              {result.rootName} · {result.totalFiles} 个文件 · {formatBytes(result.totalSize)}
              {result.truncated && <em className="warn">（项目过大，结果已截断）</em>}
            </span>
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
        <main className="layout">
          <aside className="sidebar">
            <FileTree root={result.tree} onSelect={handleSelect} selectedPath={selected?.relativePath ?? null} />
          </aside>
          <section className="detail">
            {selected ? (
              <>
                <h2>{selected.isDir ? "📁" : "📄"} {selected.name}</h2>
                <p className="path">{selected.relativePath}</p>
                <dl className="props">
                  {selected.language && (<><dt>语言</dt><dd>{selected.language}</dd></>)}
                  <dt>{selected.isDir ? "内容大小" : "文件大小"}</dt>
                  <dd>{formatBytes(selected.size)}</dd>
                  {!selected.isDir && (<><dt>直接子项</dt><dd>{selected.children.length}</dd></>)}
                </dl>

                {!selected.isDir && (
                  <div className="ai-section">
                    <div className="ai-section-head">
                      <h3>🤖 AI 解读</h3>
                      <button
                        className="primary-btn"
                        onClick={() => handleExplainFile(selected)}
                        disabled={explaining}
                      >
                        {explaining ? "分析中..." : fileExplanation ? "重新解读" : "解读此文件"}
                      </button>
                    </div>
                    {explaining && <div className="thinking">正在阅读代码并生成解读...</div>}
                    {fileExplanation && (
                      <pre className="ai-text">{fileExplanation}</pre>
                    )}
                  </div>
                )}
              </>
            ) : analyzing && progress ? (
              <div className="progress-panel">
                <h3>{progress.phase}</h3>
                <div className="progress-bar">
                  <div
                    className="progress-fill"
                    style={{ width: `${progress.total ? ((progress.done / progress.total) * 100).toFixed(0) : 0}%` }}
                  />
                </div>
                <p className="progress-current">{progress.current}</p>
              </div>
            ) : analysis ? (
              <>
                <h2>📋 项目架构总览</h2>
                <pre className="ai-text overview">{analysis.overview}</pre>
              </>
            ) : null}

            {!selected && !analysis && !analyzing && (
              <LanguageStats languages={result.languages} />
            )}
          </section>
        </main>
      )}

      {showSettings && (
        <SettingsModal
          initial={
            settings ?? { ...DEFAULTS }
          }
          onClose={() => setShowSettings(false)}
          onSaved={(s) => setSettings(s)}
        />
      )}
    </div>
  );
}
