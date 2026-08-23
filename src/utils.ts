const LANG_COLORS: Record<string, string> = {
  Rust: "#dea584",
  TypeScript: "#3178c6",
  TSX: "#3178c6",
  JavaScript: "#f1e05a",
  JSX: "#f1e05a",
  Python: "#3572A5",
  Go: "#00ADD8",
  Java: "#b07219",
  Kotlin: "#A97BFF",
  Swift: "#F05138",
  C: "#555555",
  "C++": "#f34b7d",
  "C#": "#178600",
  Ruby: "#701516",
  PHP: "#4F5D95",
  Vue: "#41b883",
  HTML: "#e34c26",
  CSS: "#563d7c",
  SCSS: "#c6538c",
  JSON: "#8a8a8a",
  Markdown: "#083fa1",
  YAML: "#cb171e",
  Shell: "#89e051",
};

export function langColor(lang: string | null): string {
  if (!lang) return "#7a7a7a";
  return LANG_COLORS[lang] ?? "#9aa0a6";
}

export function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
  return `${(bytes / 1024 / 1024 / 1024).toFixed(2)} GB`;
}
