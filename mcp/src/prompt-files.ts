import * as fs from "node:fs";
import * as path from "node:path";
import { fileURLToPath } from "node:url";
import { KNOWN_FILES } from "./constants.js";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

/** mcp/ directory (parent of dist/ at runtime). */
export const MCP_ROOT = path.resolve(__dirname, "..");

export function resolvePromptsRoot(): string {
  const env = process.env.MENTOR_PROMPTS_DIR;
  if (env && fs.existsSync(path.join(env, "zh-CN", "prompts", "AGENTS.md"))) {
    return env;
  }
  const bundled = path.join(MCP_ROOT, "prompts");
  if (fs.existsSync(path.join(bundled, "zh-CN", "prompts", "AGENTS.md"))) {
    return bundled;
  }
  let dir = process.cwd();
  for (;;) {
    if (fs.existsSync(path.join(dir, "zh-CN", "prompts", "AGENTS.md"))) {
      return dir;
    }
    const parent = path.dirname(dir);
    if (parent === dir) break;
    dir = parent;
  }
  throw new Error(
    "无法定位导师提示词目录：请设置 MENTOR_PROMPTS_DIR 指向包含 <lang>/prompts 的目录"
  );
}

export function moduleFileName(lang: string, module: string): string {
  const dir = path.join(resolvePromptsRoot(), lang, "prompts");
  if (module === "agent") return "AGENTS.md";
  if (module === "complete") {
    const files = fs
      .readdirSync(dir)
      .filter((f) => f.endsWith(".md") && !KNOWN_FILES.has(f));
    if (files.length === 0) {
      throw new Error(`语言 ${lang} 缺少完整版提示词文件`);
    }
    return files[0];
  }
  return `${module}.md`;
}

export function readPrompt(lang: string, module: string): string {
  const dir = path.join(resolvePromptsRoot(), lang, "prompts");
  const name = moduleFileName(lang, module);
  const p = path.join(dir, name);
  if (!fs.existsSync(p)) {
    throw new Error(`提示词不存在: ${lang}/${name}`);
  }
  return fs.readFileSync(p, "utf-8");
}

export function readTemplate(lang: string, name: string): string | null {
  const root = resolvePromptsRoot();
  const p = path.join(root, "templates", lang, name);
  if (fs.existsSync(p)) return fs.readFileSync(p, "utf-8");
  const fallback = path.join(root, "templates", "en-US", name);
  if (fs.existsSync(fallback)) return fs.readFileSync(fallback, "utf-8");
  return null;
}
