export const VERSION = "0.1.0";

export type Lang = { code: string; name: string };
export type Mod = { id: string; desc: string };
export type Cli = { id: string; name: string; agentFile: string; dir: string };

export const LANGUAGES: Lang[] = [
  { code: "zh-CN", name: "中文" },
  { code: "en-US", name: "English" },
  { code: "ja-JP", name: "日本語" },
  { code: "ko-KR", name: "한국어" },
  { code: "es-ES", name: "Español" },
  { code: "fr-FR", name: "Français" },
  { code: "de-DE", name: "Deutsch" },
  { code: "pt-BR", name: "Português" },
  { code: "ru-RU", name: "Русский" },
];

export const MODULES: Mod[] = [
  { id: "agent", desc: "导师角色（默认必选）" },
  { id: "security", desc: "安全规范" },
  { id: "style", desc: "交互风格" },
  { id: "workflow", desc: "开发工作流" },
  { id: "complete", desc: "完整版合并提示词" },
];

export const CLIS: Cli[] = [
  { id: "opencode", name: "opencode", agentFile: "AGENTS.md", dir: "" },
  { id: "claude-code", name: "Claude Code", agentFile: "CLAUDE.md", dir: "" },
  { id: "codex", name: "OpenAI Codex", agentFile: "AGENTS.md", dir: "" },
  { id: "cursor", name: "Cursor", agentFile: "AGENTS.md", dir: ".cursor/rules" },
  { id: "other", name: "其他（自定义）", agentFile: "AGENTS.md", dir: "" },
];

export const KNOWN_FILES = new Set(["AGENTS.md", "security.md", "style.md", "workflow.md"]);

export const PHASES = [
  "need_requirements",
  "phase0",
  "design",
  "logic",
  "ui",
  "test",
  "enhance",
] as const;

export type Phase = (typeof PHASES)[number];

export const FRONTEND_EXT = new Set([".tsx", ".jsx", ".vue", ".html", ".svelte", ".css"]);
export const CODE_EXT = new Set([
  ".ts",
  ".js",
  ".mjs",
  ".cjs",
  ".tsx",
  ".jsx",
  ".vue",
  ".py",
  ".go",
  ".rs",
  ".java",
  ".kt",
  ".cs",
  ".php",
  ".rb",
  ".swift",
  ".sql",
  ".html",
  ".svelte",
]);
