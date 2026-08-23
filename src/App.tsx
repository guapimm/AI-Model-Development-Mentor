import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { ScanResult, FileNode } from "./types";
import FileTree from "./components/FileTree";
import LanguageStats from "./components/LanguageStats";
import { formatBytes } from "./utils";

export default function App() {
  const [result, setResult] = useState<ScanResult | null>(null);
  const [selected, setSelected] = useState<FileNode | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function handleOpenFolder() {
    setError(null);
    const dir = await open({ directory: true, multiple: false });
    if (!dir) return;
    setLoading(true);
    try {
      const res = await invoke<ScanResult>("scan_project", { path: dir });
      setResult(res);
      setSelected(null);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="app">
      <header className="topbar">
        <div className="logo">🦸 Code Superman</div>
        <button className="primary-btn" onClick={handleOpenFolder} disabled={loading}>
          {loading ? "扫描中..." : "打开项目文件夹"}
        </button>
        {result && (
          <span className="summary">
            {result.rootName} · {result.totalFiles} 个文件 ·{" "}
            {formatBytes(result.totalSize)}
            {result.truncated && (
              <em className="warn">（项目过大，结果已截断）</em>
            )}
          </span>
        )}
      </header>

      {error && <div className="error-bar">出错了：{error}</div>}

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
            <FileTree root={result.tree} onSelect={setSelected} selectedPath={selected?.relativePath ?? null} />
          </aside>
          <section className="detail">
            {selected ? (
              <>
                <h2>
                  {selected.isDir ? "📁" : "📄"} {selected.name}
                </h2>
                <p className="path">{selected.relativePath}</p>
                <dl className="props">
                  {selected.language && (
                    <>
                      <dt>语言</dt>
                      <dd>{selected.language}</dd>
                    </>
                  )}
                  <dt>{selected.isDir ? "内容大小" : "文件大小"}</dt>
                  <dd>{formatBytes(selected.size)}</dd>
                  {!selected.isDir && (
                    <>
                      <dt>直接子项</dt>
                      <dd>{selected.children.length}</dd>
                    </>
                  )}
                </dl>
                {!selected.isDir && (
                  <div className="placeholder-note">
                    🤖 AI 文件作用解读将在接入 LLM 后提供
                  </div>
                )}
              </>
            ) : (
              <LanguageStats languages={result.languages} />
            )}
          </section>
        </main>
      )}
    </div>
  );
}
