#!/usr/bin/env node
// ai-model-mentor CLI - install the mentor prompt framework into any AI coding tool.
// Mirror of the Go binary (cli/); same commands, zero dependencies.
import { readFile, writeFile, mkdir, readdir, stat, rm } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
import readline from "node:readline";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const LIB = path.join(__dirname, "..", "lib");
const pkg = JSON.parse(await readFile(path.join(__dirname, "..", "package.json"), "utf8"));

const languages = [
  ["zh-CN", "中文"], ["en-US", "English"], ["ja-JP", "日本語"], ["ko-KR", "한국어"],
  ["es-ES", "Español"], ["fr-FR", "Français"], ["de-DE", "Deutsch"],
  ["pt-BR", "Português"], ["ru-RU", "Русский"],
];
const modules = [
  ["agent", "agent.md", "导师角色（默认必选）"],
  ["security", "security.md", "安全规范"],
  ["style", "style.md", "交互风格"],
  ["workflow", "workflow.md", "开发工作流"],
  ["complete", "complete.md", "完整版合并提示词"],
];
const clis = [
  ["mimo", "小米 MIMO", "AGENTS.md", ""],
  ["claude-code", "Claude Code", "CLAUDE.md", ""],
  ["codex", "OpenAI Codex", "AGENTS.md", ""],
  ["cursor", "Cursor", "AGENTS.md", ".cursor/rules"],
  ["other", "其他（自定义）", "AGENTS.md", ""],
];

const rl = readline.createInterface({ input: process.stdin, output: process.stdout });
const question = (q) => new Promise((res) => rl.question(q, res));

function readArgs(argv) {
  const out = { lang: "", modules: "", cli: "", dir: ".", positional: [] };
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    if (a === "--lang") out.lang = argv[++i] ?? "";
    else if (a === "--modules") out.modules = argv[++i] ?? "";
    else if (a === "--cli") out.cli = argv[++i] ?? "";
    else if (a === "--dir") out.dir = argv[++i] ?? ".";
    else out.positional.push(a);
  }
  return out;
}

async function pickLang(flag) {
  if (flag) {
    const hit = languages.find(([code]) => code === flag);
    if (!hit) throw new Error(`未知语言: ${flag}`);
    return hit;
  }
  console.log("🌍 选择语言:");
  languages.forEach(([code, name], i) => console.log(`  ${i + 1}. ${name} (${code})`));
  for (;;) {
    const n = Number(await question("> "));
    if (Number.isInteger(n) && n >= 1 && n <= languages.length) return languages[n - 1];
    console.log(`请输入 1-${languages.length} 之间的数字`);
  }
}

async function pickModules(flag) {
  let ids;
  if (flag) {
    ids = flag.split(",").map((s) => s.trim());
  } else {
    console.log("\n📦 选择模块（多选逗号分隔，如 1,2,4；回车默认只装 agent）:");
    modules.forEach(([id, , desc], i) => console.log(`  ${i + 1}. ${id} — ${desc}`));
    const in0 = (await question("> ")).trim();
    ids = in0 === "" ? ["agent"] : in0.split(",").map((s) => s.trim());
  }
  const out = [];
  for (const id of ids) {
    const n = Number(id);
    const hit = Number.isInteger(n) && n >= 1 && n <= modules.length ? modules[n - 1] : modules.find(([m]) => m === id);
    if (!hit) throw new Error(`未知模块: ${id}`);
    if (!out.some(([m]) => m === hit[0])) out.push(hit);
  }
  return out;
}

async function detectCLI(dir) {
  const probe = async (p) => (await stat(p).catch(() => null)) !== null;
  if (await probe(path.join(dir, ".mimocode"))) return "mimo";
  if (await probe(path.join(dir, "CLAUDE.md"))) return "claude-code";
  if (await probe(path.join(dir, ".cursor"))) return "cursor";
  if (await probe(path.join(dir, ".codex"))) return "codex";
  if (await probe(path.join(dir, "AGENTS.md"))) return "codex";
  return "";
}

async function pickCLI(flag, dir) {
  if (flag) {
    const hit = clis.find(([id]) => id === flag);
    if (!hit) throw new Error(`未知工具: ${flag}`);
    return hit;
  }
  const d = await detectCLI(dir);
  if (d) {
    const hit = clis.find(([id]) => id === d);
    if (hit) { console.log(`\n🖥️ 检测到工具: ${hit[1]}`); return hit; }
  }
  console.log("\n🖥️ 选择目标工具:");
  clis.forEach(([, name], i) => console.log(`  ${i + 1}. ${name}`));
  for (;;) {
    const n = Number(await question("> "));
    if (Number.isInteger(n) && n >= 1 && n <= clis.length) return clis[n - 1];
    console.log(`请输入 1-${clis.length} 之间的数字`);
  }
}

function fileBase(id) {
  return id === "complete" ? "complete-mentor-prompt.md" : `${id}.md`;
}

async function installFiles(lang, mods, cli, dir) {
  const [code] = lang;
  const [, , agentFile, subdir] = cli;
  for (const [id, file] of mods) {
    const data = await readFile(path.join(LIB, code, file), "utf8");
    const name = id === "agent" ? agentFile : fileBase(id);
    const target = path.join(dir, subdir, name);
    await mkdir(path.dirname(target), { recursive: true });
    await writeFile(target, data, "utf8");
    console.log(`✓ ${target}  (${code} ${id})`);
  }
  console.log("完成。按 COMPATIBILITY.md 的说明启动对应工具即可。");
}

async function cmdInstall(argv) {
  const a = readArgs(argv);
  const lang = await pickLang(a.lang);
  const mods = await pickModules(a.modules);
  const cli = await pickCLI(a.cli, a.dir);
  await installFiles(lang, mods, cli, a.dir);
}

async function cmdAdd(argv) {
  const a = readArgs(argv);
  if (a.positional.length === 0) throw new Error("用法: mentor add <模块>... [--lang zh-CN] [--dir .]");
  const lang = await pickLang(a.lang || "zh-CN");
  const mods = [];
  for (const id of a.positional) {
    const hit = modules.find(([m]) => m === id);
    if (!hit) throw new Error(`未知模块: ${id}`);
    mods.push(hit);
  }
  const cli = await pickCLI(a.cli, a.dir);
  await installFiles(lang, mods, cli, a.dir);
}

async function cmdRemove(argv) {
  const a = readArgs(argv);
  if (a.positional.length === 0) throw new Error("用法: mentor remove <模块>... [--dir .]");
  for (const id of a.positional) {
    const names = id === "agent" ? ["AGENTS.md", "CLAUDE.md"] : [fileBase(id)];
    for (const n of names) {
      for (const p of [path.join(a.dir, n), path.join(a.dir, ".cursor", "rules", n)]) {
        if (await rm(p, { force: true }).then(() => true).catch(() => false)) console.log(`✗ 已移除 ${p}`);
      }
    }
  }
}

async function cmdList(argv) {
  const a = readArgs(argv);
  const names = ["AGENTS.md", "CLAUDE.md", "security.md", "style.md", "workflow.md", "complete-mentor-prompt.md"];
  let found = false;
  for (const n of names) {
    for (const p of [path.join(a.dir, n), path.join(a.dir, ".cursor", "rules", n)]) {
      if (await stat(p).catch(() => null)) { console.log(`✓ ${p}`); found = true; }
    }
  }
  if (!found) console.log("（未检测到已安装的导师模块）");
}

async function cmdDetect(argv) {
  const a = readArgs(argv);
  const d = await detectCLI(a.dir);
  if (!d) return console.log("未检测到已知工具（可手动指定: mimo / claude-code / codex / cursor / other）");
  const hit = clis.find(([id]) => id === d);
  console.log(`检测到: ${hit ? hit[1] : d}`);
}

async function cmdPack(argv) {
  const a = readArgs(argv);
  const out = a.dir === "." ? "skill" : a.dir;
  await mkdir(out, { recursive: true });
  const skillMD = "# AI Model Mentor Skill\n\n提示词技能包：选择语言与模块后，按各语言 COMPATIBILITY.md 说明加载到你的 AI 工具。\n\n语言目录：zh-CN / en-US / ja-JP / ko-KR / es-ES / fr-FR / de-DE / pt-BR / ru-RU\n模块：agent（默认）/ security / style / workflow / complete\n";
  await writeFile(path.join(out, "SKILL.md"), skillMD, "utf8");
  for (const lang of await readdir(LIB)) {
    await mkdir(path.join(out, lang), { recursive: true });
    for (const f of await readdir(path.join(LIB, lang))) {
      await writeFile(path.join(out, lang, f), await readFile(path.join(LIB, lang, f), "utf8"), "utf8");
    }
  }
  console.log(`✓ skill 包已生成到 ${out}/`);
}

const usage = () => console.log(`AI 模型导师 ai-mentor v${pkg.version}

用法:
  ai-mentor install             交互式安装向导（选语言 → 选模块 → 选工具）
  ai-mentor install --lang zh-CN --modules agent,security --cli claude-code --dir ./proj
  ai-mentor add <模块>...        追加模块，如: ai-mentor add security --lang zh-CN
  ai-mentor remove <模块>...     移除模块
  ai-mentor list                列出当前项目已安装的模块
  ai-mentor detect              检测项目使用的 AI 工具
  ai-mentor pack                生成兼容 skill 目录
  ai-mentor version             版本号
  ai-mentor help                帮助

模块: agent(默认), security, style, workflow, complete
语言: zh-CN en-US ja-JP ko-KR es-ES fr-FR de-DE pt-BR ru-RU
工具: mimo claude-code codex cursor other`);

const [cmd, ...rest] = process.argv.slice(2);
try {
  switch (cmd) {
    case "install": await cmdInstall(rest); break;
    case "add": await cmdAdd(rest); break;
    case "remove": await cmdRemove(rest); break;
    case "list": await cmdList(rest); break;
    case "detect": await cmdDetect(rest); break;
    case "pack": await cmdPack(rest); break;
    case "version": case "-v": case "--version": console.log(`ai-mentor v${pkg.version}`); break;
    case "help": case "-h": case "--help": usage(); break;
    case undefined: usage(); break;
    default:
      console.error(`未知命令: ${cmd}\n`);
      usage();
      process.exitCode = 1;
  }
} catch (e) {
  console.error("错误:", e.message);
  process.exitCode = 1;
} finally {
  rl.close();
}
