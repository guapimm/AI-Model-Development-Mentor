import { useMemo } from "react";
import { LangStat } from "../types";
import { langColor } from "../utils";

export default function LanguageStats({ languages }: { languages: LangStat[] }) {
  const totalBytes = useMemo(
    () => languages.reduce((sum, l) => sum + l.bytes, 0),
    [languages]
  );
  const visible = languages.slice(0, 10);

  if (languages.length === 0) return null;

  return (
    <section className="panel">
      <h3>语言构成</h3>
      <div className="lang-bar">
        {visible.map((l) => (
          <div
            key={l.language}
            className="lang-bar-seg"
            style={{
              width: `${(l.bytes / totalBytes) * 100}%`,
              background: langColor(l.language),
            }}
            title={`${l.language}: ${((l.bytes / totalBytes) * 100).toFixed(1)}%`}
          />
        ))}
      </div>
      <ul className="lang-list">
        {visible.map((l) => (
          <li key={l.language}>
            <span
              className="lang-dot"
              style={{ background: langColor(l.language) }}
            />
            <span className="lang-name">{l.language}</span>
            <span className="lang-meta">
              {((l.bytes / totalBytes) * 100).toFixed(1)}% · {l.files} 个文件 ·{" "}
              {(l.bytes / 1024).toFixed(0)} KB
            </span>
          </li>
        ))}
      </ul>
    </section>
  );
}
