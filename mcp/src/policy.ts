import * as fs from "node:fs";
import * as path from "node:path";
import { MCP_ROOT, readPrompt } from "./prompt-files.js";

export type FragmentTriggers = {
  ext?: string[];
  path?: string[];
  intent?: string[];
  phase?: string[];
  event?: string[];
};

export type FragmentDef = {
  id: string;
  module: string;
  headings: Record<string, string>;
  hint: { zh: string; en: string };
  triggers: FragmentTriggers;
};

type IndexFile = { version: number; fragments: FragmentDef[] };

let cached: IndexFile | null = null;

export function fragmentsPath(): string {
  return path.join(MCP_ROOT, "policy", "fragments.json");
}

export function loadIndex(): IndexFile {
  if (cached) return cached;
  const raw = fs.readFileSync(fragmentsPath(), "utf-8");
  cached = JSON.parse(raw) as IndexFile;
  return cached;
}

export function allFragments(): FragmentDef[] {
  return loadIndex().fragments;
}

export function getFragmentDef(id: string): FragmentDef | undefined {
  return allFragments().find((f) => f.id === id);
}

/** Extract a markdown section starting at a heading, until next heading of same or higher level. */
export function extractSection(markdown: string, heading: string): string | null {
  const lines = markdown.split(/\r?\n/);
  let start = -1;
  let startLevel = 0;
  const needle = heading.trim();
  for (let i = 0; i < lines.length; i++) {
    const m = lines[i].match(/^(#{1,6})\s+(.*)$/);
    if (!m) continue;
    const text = m[2].trim();
    if (start < 0) {
      if (text === needle || text.endsWith(needle) || needle.endsWith(text)) {
        start = i;
        startLevel = m[1].length;
      }
      continue;
    }
    if (m[1].length <= startLevel) {
      return lines.slice(start, i).join("\n").trim();
    }
  }
  if (start >= 0) return lines.slice(start).join("\n").trim();
  return null;
}

function h2Sections(markdown: string): string[] {
  const lines = markdown.split(/\r?\n/);
  const starts: number[] = [];
  for (let i = 0; i < lines.length; i++) {
    if (/^##\s+/.test(lines[i])) starts.push(i);
  }
  const out: string[] = [];
  for (let i = 0; i < starts.length; i++) {
    const end = i + 1 < starts.length ? starts[i + 1] : lines.length;
    out.push(lines.slice(starts[i], end).join("\n").trim());
  }
  return out;
}

export function loadFragment(id: string, lang: string): string {
  const def = getFragmentDef(id);
  if (!def) throw new Error(`未知片段: ${id}`);
  const md = readPrompt(lang, def.module);
  const heading =
    def.headings[lang] ?? def.headings["en-US"] ?? def.headings["zh-CN"];
  if (heading) {
    const hit = extractSection(md, heading);
    if (hit) return hit;
  }
  for (const h of Object.values(def.headings)) {
    const hit = extractSection(md, h);
    if (hit) return hit;
  }
  const siblings = allFragments().filter((f) => f.module === def.module);
  const idx = siblings.findIndex((f) => f.id === id);
  const sections = h2Sections(md);
  if (idx >= 0 && idx < sections.length) return sections[idx];
  throw new Error(`无法在 ${lang}/${def.module} 中切出片段 ${id}`);
}

export function catalogText(lang: string): string {
  const zh = lang === "zh-CN";
  const lines = allFragments().map((f) => {
    const hint = zh ? f.hint.zh : f.hint.en;
    return `- ${f.id.padEnd(20)} | ${hint}`;
  });
  const header = zh
    ? "policy catalog（不确定时先 policy_load(id)；不要整文件加载 security/style/workflow）:"
    : "policy catalog (call policy_load(id) if unsure; do not load full security/style/workflow files):";
  return [header, ...lines].join("\n");
}

export function searchFragments(query: string, lang: string, limit = 5): { id: string; preview: string }[] {
  const q = query.trim().toLowerCase();
  if (!q) return [];
  const scored: { id: string; score: number; preview: string }[] = [];
  for (const def of allFragments()) {
    let score = 0;
    if (def.id.toLowerCase().includes(q)) score += 5;
    const hint = `${def.hint.zh} ${def.hint.en}`.toLowerCase();
    if (hint.includes(q)) score += 3;
    for (const h of Object.values(def.headings)) {
      if (h.toLowerCase().includes(q)) score += 4;
    }
    const intents = def.triggers.intent ?? [];
    if (intents.some((i) => i.toLowerCase().includes(q) || q.includes(i.toLowerCase()))) score += 4;
    let preview = "";
    try {
      const body = loadFragment(def.id, lang);
      preview = body.slice(0, 200).replace(/\s+/g, " ");
      if (body.toLowerCase().includes(q)) score += 2;
    } catch {
      preview = hint;
    }
    if (score > 0) scored.push({ id: def.id, score, preview });
  }
  scored.sort((a, b) => b.score - a.score);
  return scored.slice(0, limit).map(({ id, preview }) => ({ id, preview }));
}

export function l0AlwaysOn(lang: string, phase: string): string {
  const zh = lang === "zh-CN";
  const persona = zh
    ? "你是一位 10 年经验的全栈架构师兼开发导师，服务零基础小白。安全兜底、逻辑透明、文档先行、分步落地、资源可控。"
    : "You are a 10-year full-stack architect-mentor for coding beginners. Security first, transparent logic, docs first, phased delivery, resource control.";
  const rules = zh
    ? "铁律（一行版）：1 代码即文档  2 安全前置  3 零破坏性变更  4 每步≤300行且需确认  5 单文件≤500行  6 性能与资源前置"
    : "Iron rules (one-liners): 1 code-as-docs  2 security-first  3 zero-destructive  4 ≤300 lines/step + confirm  5 ≤500 lines/file  6 resource-upfront";
  const phaseLine = zh ? `当前阶段: ${phase}` : `Current phase: ${phase}`;
  const how = zh
    ? "需要细则时调用 policy_load(id)。写文件/跑命令必须走本 MCP 的 fs_* / run_command（沙箱）。"
    : "Call policy_load(id) for details. File/command operations MUST use this MCP's fs_* / run_command (sandboxed).";
  return [persona, rules, phaseLine, how, "", catalogText(lang)].join("\n");
}
