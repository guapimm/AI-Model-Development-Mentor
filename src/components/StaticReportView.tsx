import { useState } from "react";
import { Channel } from "@tauri-apps/api/core";
import { invoke } from "@tauri-apps/api/core";
import {
  EntryPoint,
  FileMetric,
  StaticProgress,
  StaticReport,
  TechStackItem,
} from "../types";
import { langColor } from "../utils";
import DepGraphView from "./DepGraphView";

function TechStack({ items }: { items: TechStackItem[] }) {
  if (items.length === 0) {
    return (
      <p className="dim-note">未识别到已知的技术栈清单文件（如 package.json、Cargo.toml 等）。</p>
    );
  }
  const groups = new Map<string, TechStackItem[]>();
  for (const item of items) {
    const list = groups.get(item.category) ?? [];
    list.push(item);
    groups.set(item.category, list);
  }
  return (
    <div className="stack-groups">
      {[...groups.entries()].map(([cat, list]) => (
        <div key={cat} className="stack-group">
          <span className="stack-cat">{cat}</span>
          <div className="chips">
            {list.map((t) => (
              <span key={t.name} className="chip" title={`来源: ${t.source}`}>
                {t.name}
              </span>
            ))}
          </div>
        </div>
      ))}
    </div>
  );
}

export default function StaticReportView({
  report,
  rootPath,
  onSelectFile,
}: {
  report: StaticReport;
  rootPath: string;
  onSelectFile: (path: string) => void;
}) {
  const topMetrics: FileMetric[] = report.metrics.slice(0, 20);
  const entries: EntryPoint[] = report.entryPoints;

  const [graphData, setGraphData] = useState<import("../types").DepGraphData | null>(null);
  const [graphLoading, setGraphLoading] = useState(false);
  const [graphPercent, setGraphPercent] = useState(0);
  const [graphError, setGraphError] = useState<string | null>(null);

  async function loadGraph() {
    if (graphLoading || !rootPath) return;
    setGraphLoading(true);
    setGraphError(null);
    setGraphPercent(0);
    try {
      const channel = new Channel<StaticProgress>();
      channel.onmessage = (p) => setGraphPercent(p.percent);
      const data = await invoke<import("../types").DepGraphData>("get_dependency_graph", {
        rootPath,
        channel,
      });
      setGraphData(data);
    } catch (e) {
      setGraphError(String(e));
    } finally {
      setGraphLoading(false);
    }
  }

  return (
    <div className="static-report">
      <h2>📊 静态分析报告 · {report.rootName}</h2>

      <div className="stat-row">
        <div className="stat-box">
          <b>{report.totalCodeFiles}</b>
          <span>代码文件</span>
        </div>
        <div className="stat-box">
          <b>{(report.totalLines / 1000).toFixed(1)}K</b>
          <span>总行数</span>
        </div>
        <div className="stat-box">
          <b>{report.techStack.length}</b>
          <span>技术组件</span>
        </div>
        <div className="stat-box">
          <b>{report.totalTodos}</b>
          <span>TODO/FIXME</span>
        </div>
      </div>

      <section className="panel">
        <h3>技术栈</h3>
        <TechStack items={report.techStack} />
      </section>

      {entries.length > 0 && (
        <section className="panel">
          <h3>入口点（建议从这里开始阅读）</h3>
          <ul className="entry-list">
            {entries.map((e) => (
              <li key={e.relativePath}>
                <a
                  className="file-link"
                  onClick={() => onSelectFile(e.relativePath)}
                >
                  {e.relativePath}
                </a>
                <span className="entry-reason">{e.reason}</span>
              </li>
            ))}
          </ul>
        </section>
      )}

      <section className="panel">
        <h3>文件行数 TOP {topMetrics.length}</h3>
        <table className="metrics-table">
          <thead>
            <tr>
              <th>文件</th>
              <th>语言</th>
              <th>行数</th>
              <th>有效行</th>
              <th>TODO</th>
            </tr>
          </thead>
          <tbody>
            {topMetrics.map((m) => (
              <tr key={m.relativePath} onClick={() => onSelectFile(m.relativePath)}>
                <td className="mono" title={m.relativePath}>{m.relativePath}</td>
                <td>
                  <span className="lang-dot" style={{ background: langColor(m.language) }} />{" "}
                  {m.language}
                </td>
                <td>{m.lines}</td>
                <td>{m.codeLines}</td>
                <td>{m.todos > 0 ? m.todos : ""}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </section>

      <section className="panel">
        <h3>🕸 依赖关系图谱</h3>
        {!graphData && !graphLoading && (
          <button className="primary-btn" onClick={loadGraph}>
            生成全项目依赖图谱
          </button>
        )}
        {graphLoading && (
          <div className="progress-panel" style={{ marginTop: 0 }}>
            <div className="progress-percent">{graphPercent}%</div>
            <div className="progress-bar">
              <div className="progress-fill" style={{ width: `${graphPercent}%` }} />
            </div>
          </div>
        )}
        {graphError && <p className="dim-note">生成失败：{graphError}</p>}
        {graphData && (
          <DepGraphView data={graphData} onSelectFile={onSelectFile} />
        )}
      </section>

      {report.warnings.length > 0 && (
        <section className="panel">
          <h3>⚠️ 提示</h3>
          <ul className="warn-list">
            {report.warnings.map((w, i) => (
              <li key={i}>{w}</li>
            ))}
          </ul>
        </section>
      )}
    </div>
  );
}
