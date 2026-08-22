/**
 * Copy <repo>/<lang>/prompts into mcp/prompts for a self-contained package.
 */
import * as fs from "node:fs";
import * as path from "node:path";
import { fileURLToPath } from "node:url";

const mcp = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const root = path.resolve(mcp, "..");
const dest = path.join(mcp, "prompts");
const langs = ["zh-CN", "en-US", "ja-JP", "ko-KR", "es-ES", "fr-FR", "de-DE", "pt-BR", "ru-RU"];

if (fs.existsSync(dest)) fs.rmSync(dest, { recursive: true, force: true });
fs.mkdirSync(dest, { recursive: true });

for (const lang of langs) {
  const src = path.join(root, lang, "prompts");
  if (!fs.existsSync(src)) {
    throw new Error(`missing ${src}`);
  }
  const d = path.join(dest, lang, "prompts");
  fs.mkdirSync(d, { recursive: true });
  for (const name of fs.readdirSync(src)) {
    fs.copyFileSync(path.join(src, name), path.join(d, name));
  }
}
console.log("prompts synced to mcp/prompts");
