import * as fs from "node:fs";
import * as path from "node:path";
import { CLIS, LANGUAGES, MODULES, PHASES, type Cli } from "./constants.js";
import { attachPolicies, toolResult } from "./envelope.js";
import {
  checkAdvance,
  checkExec,
  checkWrite,
  formatGateError,
  isDocRel,
} from "./gates.js";
import { catalogText, l0AlwaysOn, loadFragment, searchFragments } from "./policy.js";
import { readPrompt, readTemplate } from "./prompt-files.js";
import {
  resolveInRoot,
  resolveRoot,
  runCommand,
  SandboxError,
  assertWritable,
  toPosixRel,
} from "./sandbox.js";
import { phaseEnterIds, selectFragmentIds } from "./select.js";
import {
  loadOrCreateState,
  nextPhase,
  readState,
  writeState,
  type MentorState,
} from "./state.js";

function argStr(a: Record<string, unknown>, key: string, fallback?: string): string {
  const v = a[key];
  return typeof v === "string" && v.length ? v : fallback ?? "";
}

function workspace(a: Record<string, unknown>): string {
  const d = argStr(a, "dir", ".");
  return path.resolve(d);
}

function langOf(a: Record<string, unknown>, state?: MentorState | null): string {
  return argStr(a, "lang", state?.lang || "zh-CN");
}

function detectTool(dir: string): Cli | null {
  const probe = (p: string) => fs.existsSync(path.join(dir, p));
  if (probe(".opencode") || probe("opencode.json") || probe("opencode.jsonc")) {
    return CLIS.find((c) => c.id === "opencode")!;
  }
  if (probe("CLAUDE.md")) return CLIS.find((c) => c.id === "claude-code")!;
  if (probe(".cursor")) return CLIS.find((c) => c.id === "cursor")!;
  if (probe(".codex")) return CLIS.find((c) => c.id === "codex")!;
  if (probe("AGENTS.md")) return CLIS.find((c) => c.id === "codex")!;
  return null;
}

function targetFileName(module: string, cli: Cli): string {
  if (module === "agent") return cli.agentFile;
  if (module === "complete") return "complete-mentor-prompt.md";
  return `${module}.md`;
}

const SNAPSHOT_TEMPLATE = `# 项目快照（断点续传）

> 由 AI 导师在每次对话结束时更新，用于断点续传与上下文恢复。控制在 200 行以内。

## 技术栈版本

| 技术 | 版本 |
|------|------|
| （待填） | |

## 数据库表清单

| 表名 | 用途 |
|------|------|
| | |

## 已完成的 API 接口

| 方法 | 路径 | 说明 |
|------|------|------|
| | | |

## 当前进度与待办

- 当前阶段：
- 下一步：
- 待确认事项：

## 续传暗号

（此处填入本次对话结束时的续传暗号）
`;

function ensureGitignoreEnv(root: string): void {
  const p = path.join(root, ".gitignore");
  let cur = fs.existsSync(p) ? fs.readFileSync(p, "utf-8") : "";
  if (!/(^|\/)\.env(\s|$)/m.test(cur)) {
    if (cur && !cur.endsWith("\n")) cur += "\n";
    cur += ".env\n.mentor/\n";
    fs.writeFileSync(p, cur, "utf-8");
  }
}

function patchSnapshotPhase(root: string, phase: string): void {
  const p = path.join(root, "docs", "SNAPSHOT.md");
  fs.mkdirSync(path.dirname(p), { recursive: true });
  let body = fs.existsSync(p) ? fs.readFileSync(p, "utf-8") : SNAPSHOT_TEMPLATE;
  if (/当前阶段：.*/.test(body)) {
    body = body.replace(/当前阶段：.*/, `当前阶段：${phase}`);
  } else {
    body += `\n\n- 当前阶段：${phase}\n`;
  }
  const lines = body.split(/\r?\n/);
  if (lines.length > 200) body = lines.slice(0, 200).join("\n") + "\n";
  fs.writeFileSync(p, body, "utf-8");
}

export function toolsList() {
  const dir = { type: "string", description: "工作区根目录，默认当前目录" };
  const lang = { type: "string", description: "语言代码，如 zh-CN / en-US" };
  return {
    tools: [
      {
        name: "session_start",
        description:
          "启动/恢复导师会话：写入 .mentor/state.json，返回 L0（人设+铁律一行版+片段目录+当前阶段）。开始任何开发前先调用。",
        inputSchema: {
          type: "object",
          properties: {
            dir,
            lang,
            sandbox: { type: "string", description: "jail（默认）或 docker" },
          },
        },
        annotations: { readOnlyHint: false, idempotentHint: true },
      },
      {
        name: "session_advance",
        description: "状态机前进一步（need_requirements→phase0→design→logic→ui→test→enhance）。闸门不通过会拒绝并注入相关规则。",
        inputSchema: { type: "object", properties: { dir } },
        annotations: { readOnlyHint: false },
      },
      {
        name: "policy_load",
        description: "按需加载一条政策片段（id 见 session_start 的 catalog）。不要整文件加载 security.md。",
        inputSchema: {
          type: "object",
          properties: {
            id: { type: "string", description: "片段 id，如 security.db" },
            dir,
            force: { type: "boolean", description: "true 则即使已加载也展开全文" },
          },
          required: ["id"],
        },
        annotations: { readOnlyHint: true, idempotentHint: true },
      },
      {
        name: "policy_search",
        description: "检索政策片段（当模型需要一条当前未驻留的规则）。返回 id + 预览，再用 policy_load 展开。",
        inputSchema: {
          type: "object",
          properties: { query: { type: "string" }, dir, lang },
          required: ["query"],
        },
        annotations: { readOnlyHint: true, idempotentHint: true },
      },
      {
        name: "fs_read",
        description: "在工作区沙箱内读文件（路径监狱，禁止 .. 与越界）。",
        inputSchema: {
          type: "object",
          properties: { path: { type: "string" }, dir },
          required: ["path"],
        },
        annotations: { readOnlyHint: true },
      },
      {
        name: "fs_write",
        description:
          "在工作区沙箱内写文件。服务端强制：≤300行/次、≤500行/文件、禁止 .env、阶段闸门、前端需 UI 映射。相关政策按事件注入。",
        inputSchema: {
          type: "object",
          properties: {
            path: { type: "string" },
            content: { type: "string" },
            dir,
            intent: { type: "string", description: "本次写入意图，用于选片，如 login / sql" },
          },
          required: ["path", "content"],
        },
        annotations: { readOnlyHint: false, destructiveHint: false },
      },
      {
        name: "fs_list",
        description: "列出工作区某目录（默认根，非递归）。",
        inputSchema: {
          type: "object",
          properties: { path: { type: "string" }, dir },
        },
        annotations: { readOnlyHint: true },
      },
      {
        name: "run_command",
        description:
          "在工作区沙箱执行命令（cwd 锁定；可选 Docker）。部署类命令要求存在 ./local_backup/。",
        inputSchema: {
          type: "object",
          properties: {
            command: { type: "string" },
            dir,
            timeout_ms: { type: "number" },
          },
          required: ["command"],
        },
        annotations: { readOnlyHint: false, destructiveHint: true },
      },
      {
        name: "snapshot_update",
        description: "更新 docs/SNAPSHOT.md（≤200 行）。可传入 markdown 正文；省略则只同步当前阶段。",
        inputSchema: {
          type: "object",
          properties: { content: { type: "string" }, dir },
        },
        annotations: { readOnlyHint: false },
      },
      {
        name: "init_project",
        description: "初始化项目骨架：REQUIREMENTS.md、.env.example、docs/、.gitignore（含 .env）。",
        inputSchema: {
          type: "object",
          properties: {
            name: { type: "string" },
            goal: { type: "string" },
            dir,
            lang,
          },
          required: ["name"],
        },
        annotations: { readOnlyHint: false },
      },
      {
        name: "install",
        description: "把导师提示词安装到目标项目（按目标 AI 工具写入正确的文件名与位置）。",
        inputSchema: {
          type: "object",
          properties: {
            lang: { type: "string" },
            modules: { type: "array", items: { type: "string" } },
            cli: { type: "string" },
            dir: { type: "string" },
          },
          required: ["lang"],
        },
        annotations: { readOnlyHint: false, idempotentHint: true },
      },
      {
        name: "detect_tool",
        description: "检测项目目录使用的 AI 编码工具。",
        inputSchema: { type: "object", properties: { dir: { type: "string" } } },
        annotations: { readOnlyHint: true, idempotentHint: true },
      },
      {
        name: "list_languages",
        description: "列出可用的提示词语言。",
        inputSchema: { type: "object", properties: {} },
        annotations: { readOnlyHint: true, idempotentHint: true },
      },
      {
        name: "list_modules",
        description: "列出可用的导师模块。",
        inputSchema: { type: "object", properties: {} },
        annotations: { readOnlyHint: true, idempotentHint: true },
      },
      {
        name: "generate_resource_estimate",
        description: "生成并写入 docs/resource_estimate.md（阶段0）。完成后可 session_advance。",
        inputSchema: {
          type: "object",
          properties: {
            name: { type: "string" },
            lang: { type: "string" },
            dir: { type: "string" },
          },
        },
        annotations: { readOnlyHint: false, idempotentHint: true },
      },
    ],
  };
}

export async function handleTool(name: string, args: unknown) {
  const a = (args ?? {}) as Record<string, unknown>;
  try {
    switch (name) {
      case "list_languages":
        return toolResult(LANGUAGES.map((l) => `${l.code} — ${l.name}`).join("\n"));
      case "list_modules":
        return toolResult(MODULES.map((m) => `${m.id} — ${m.desc}`).join("\n"));
      case "detect_tool": {
        const dir = argStr(a, "dir", ".");
        const hit = detectTool(dir);
        return toolResult(hit ? `检测到: ${hit.name} (${hit.id})` : "未检测到已知工具");
      }
      case "install":
        return cmdInstall(a);
      case "session_start":
        return cmdSessionStart(a);
      case "session_advance":
        return cmdSessionAdvance(a);
      case "policy_load":
        return cmdPolicyLoad(a);
      case "policy_search":
        return cmdPolicySearch(a);
      case "fs_read":
        return cmdFsRead(a);
      case "fs_write":
        return cmdFsWrite(a);
      case "fs_list":
        return cmdFsList(a);
      case "run_command":
        return cmdRun(a);
      case "snapshot_update":
        return cmdSnapshot(a);
      case "init_project":
        return cmdInit(a);
      case "generate_resource_estimate":
        return cmdEstimate(a);
      default:
        return toolResult(`未知工具: ${name}`, true);
    }
  } catch (e) {
    const msg = e instanceof Error ? e.message : String(e);
    return toolResult(msg, true);
  }
}

function cmdSessionStart(a: Record<string, unknown>) {
  const dir = workspace(a);
  fs.mkdirSync(dir, { recursive: true });
  const lang = langOf(a);
  const sandbox = argStr(a, "sandbox") === "docker" ? "docker" : "jail";
  const state = loadOrCreateState(dir, lang, sandbox);
  writeState(dir, state);
  patchSnapshotPhase(dir, state.phase);
  const l0 = l0AlwaysOn(state.lang, state.phase);
  const enter = phaseEnterIds(state.phase);
  const extra = attachPolicies(state, enter);
  writeState(dir, state);
  return toolResult(
    `${l0}${extra}\n\n工作区: ${dir}\nstate: ${path.join(dir, ".mentor", "state.json")}\n沙箱: ${state.sandbox}`
  );
}

function cmdSessionAdvance(a: Record<string, unknown>) {
  const dir = workspace(a);
  const state = readState(dir);
  if (!state) return toolResult("请先 session_start", true);
  const fail = checkAdvance(state, dir);
  if (fail && (state.phase === "need_requirements" || state.phase === "phase0")) {
    return toolResult(formatGateError(fail, state.lang, []), true);
  }
  const nxt = nextPhase(state.phase);
  if (!nxt) return toolResult(`已在最后阶段 ${state.phase}`);
  state.phase = nxt;
  writeState(dir, state);
  patchSnapshotPhase(dir, state.phase);
  const extra = attachPolicies(state, phaseEnterIds(state.phase));
  writeState(dir, state);
  const tags = attachPolicies(state, selectFragmentIds(state, { tool: "session_advance" }));
  writeState(dir, state);
  return toolResult(`已进入阶段: ${state.phase}${extra}${tags}`);
}

function cmdPolicyLoad(a: Record<string, unknown>) {
  const id = argStr(a, "id");
  if (!id) return toolResult("缺少 id", true);
  const dir = workspace(a);
  let state = readState(dir);
  if (!state) state = loadOrCreateState(dir, "zh-CN");
  const force = a.force === true;
  const extra = attachPolicies(state, [id], { force });
  writeState(dir, state);
  if (force) {
    return toolResult(`## policy:${id}\n${loadFragment(id, state.lang)}`);
  }
  return toolResult(extra.trim() || loadFragment(id, state.lang));
}

function cmdPolicySearch(a: Record<string, unknown>) {
  const q = argStr(a, "query");
  const dir = workspace(a);
  const state = readState(dir);
  const lang = langOf(a, state);
  const hits = searchFragments(q, lang);
  if (!hits.length) {
    return toolResult(`无命中。可用 catalog:\n${catalogText(lang)}`);
  }
  const lines = hits.map((h) => `- ${h.id}\n  ${h.preview}`);
  return toolResult(
    `命中 ${hits.length} 条。需要全文请 policy_load(id)。\n\n${lines.join("\n\n")}`
  );
}

function cmdFsRead(a: Record<string, unknown>) {
  const dir = workspace(a);
  const rel = argStr(a, "path");
  const abs = resolveInRoot(dir, rel);
  if (!fs.existsSync(abs) || !fs.statSync(abs).isFile()) {
    return toolResult(`不是文件: ${rel}`, true);
  }
  return toolResult(fs.readFileSync(abs, "utf-8"));
}

function cmdFsWrite(a: Record<string, unknown>) {
  const dir = workspace(a);
  const rel = argStr(a, "path");
  const content = typeof a.content === "string" ? a.content : "";
  const state = readState(dir) ?? loadOrCreateState(dir, "zh-CN");
  let abs: string;
  try {
    abs = resolveInRoot(dir, rel);
    assertWritable(dir, abs);
  } catch (e) {
    const msg = e instanceof Error ? e.message : String(e);
    const extra = attachPolicies(state, ["security.fs", "security.secrets"]);
    writeState(dir, state);
    return toolResult(`${msg}${extra}`, true);
  }
  const posix = toPosixRel(dir, abs);
  const fail = checkWrite({ root: dir, rel: posix, content, state });
  if (fail) {
    return toolResult(formatGateError(fail, state.lang, []), true);
  }
  fs.mkdirSync(path.dirname(abs), { recursive: true });
  fs.writeFileSync(abs, content, "utf-8");
  if (posix === "docs/resource_estimate.md" || posix.endsWith("/resource_estimate.md")) {
    state.resourceEstimateDone = true;
  }
  if (posix === "docs/ui_mapping.md" || posix.endsWith("/ui_mapping.md")) {
    state.uiMapped = true;
  }
  const ids = selectFragmentIds(state, {
    tool: "fs_write",
    relPath: posix,
    intent: argStr(a, "intent"),
  });
  if (!isDocRel(posix)) ids.unshift("agent.checklist");
  const extra = attachPolicies(state, ids);
  writeState(dir, state);
  return toolResult(`已写入 ${posix}（${content.split(/\r?\n/).length} 行）${extra}`);
}

function cmdFsList(a: Record<string, unknown>) {
  const dir = workspace(a);
  const rel = argStr(a, "path", ".");
  const abs = resolveInRoot(dir, rel);
  if (!fs.existsSync(abs) || !fs.statSync(abs).isDirectory()) {
    return toolResult(`不是目录: ${rel}`, true);
  }
  const names = fs.readdirSync(abs).map((n) => {
    const st = fs.statSync(path.join(abs, n));
    return `${st.isDirectory() ? "d" : "f"} ${n}`;
  });
  return toolResult(names.join("\n") || "（空目录）");
}

function cmdRun(a: Record<string, unknown>) {
  const dir = workspace(a);
  const command = argStr(a, "command");
  const state = readState(dir) ?? loadOrCreateState(dir, "zh-CN");
  const fail = checkExec(command, dir, state);
  if (fail) return toolResult(formatGateError(fail, state.lang, []), true);
  const timeout = typeof a.timeout_ms === "number" ? a.timeout_ms : 30_000;
  let result;
  try {
    result = runCommand(dir, command, { timeoutMs: timeout, sandbox: state.sandbox });
  } catch (e) {
    if (e instanceof SandboxError) return toolResult(e.message, true);
    throw e;
  }
  const extra = attachPolicies(state, selectFragmentIds(state, { tool: "run_command", command }));
  writeState(dir, state);
  const note =
    result.sandbox === "docker-fallback-jail"
      ? "（Docker 不可用，已回退路径监狱）\n"
      : `sandbox=${result.sandbox}\n`;
  return toolResult(
    `${note}exit ${result.status}\n--- stdout ---\n${result.stdout}\n--- stderr ---\n${result.stderr}${extra}`
  );
}

function cmdSnapshot(a: Record<string, unknown>) {
  const dir = workspace(a);
  const state = readState(dir) ?? loadOrCreateState(dir, "zh-CN");
  const docs = path.join(resolveRoot(dir), "docs");
  fs.mkdirSync(docs, { recursive: true });
  const p = path.join(docs, "SNAPSHOT.md");
  let body = typeof a.content === "string" && a.content.trim() ? a.content : "";
  if (!body) {
    body = fs.existsSync(p) ? fs.readFileSync(p, "utf-8") : SNAPSHOT_TEMPLATE;
  }
  const lines = body.split(/\r?\n/);
  if (lines.length > 200) {
    return toolResult("SNAPSHOT.md 超过 200 行，请压缩后再写入。", true);
  }
  if (!/当前阶段：/.test(body)) body += `\n\n- 当前阶段：${state.phase}\n`;
  else body = body.replace(/当前阶段：.*/, `当前阶段：${state.phase}`);
  fs.writeFileSync(p, body, "utf-8");
  const extra = attachPolicies(state, ["agent.token"]);
  writeState(dir, state);
  return toolResult(`已更新 docs/SNAPSHOT.md${extra}`);
}

function cmdInit(a: Record<string, unknown>) {
  const dir = workspace(a);
  fs.mkdirSync(dir, { recursive: true });
  const name = argStr(a, "name", "未命名项目");
  const goal = argStr(a, "goal", "（待补充）");
  const lang = langOf(a);
  const req = `# 项目需求说明书：${name}\n\n## 核心目标\n${goal}\n\n## 用户角色\n（待补充，请与导师确认）\n\n## 核心操作流程\n（待补充）\n\n## 必须存储的数据\n（待补充）\n`;
  fs.writeFileSync(path.join(dir, "REQUIREMENTS.md"), req, "utf-8");
  const env =
    "# 环境变量示例：复制为 .env 并填入真实值，切勿把 .env 提交到仓库\n# 数据库连接串\nDATABASE_URL=\n# 密钥（示例）\nSECRET_KEY=\n";
  fs.writeFileSync(path.join(dir, ".env.example"), env, "utf-8");
  fs.mkdirSync(path.join(dir, "docs"), { recursive: true });
  const snap = path.join(dir, "docs", "SNAPSHOT.md");
  if (!fs.existsSync(snap)) fs.writeFileSync(snap, SNAPSHOT_TEMPLATE, "utf-8");
  ensureGitignoreEnv(dir);
  const state = loadOrCreateState(dir, lang);
  writeState(dir, state);
  return toolResult(
    `已初始化 ${dir}\n- REQUIREMENTS.md\n- .env.example\n- docs/SNAPSHOT.md\n- .gitignore\n下一步: session_start → 阶段0 资源预估`
  );
}

function cmdEstimate(a: Record<string, unknown>) {
  const dir = workspace(a);
  const state = readState(dir) ?? loadOrCreateState(dir, langOf(a));
  const lang = langOf(a, state);
  const name = argStr(a, "name", "（待填）");
  let tpl = readTemplate(lang, "resource_estimate_template.md");
  if (!tpl) {
    tpl = `# 项目资源预估表（阶段0）\n\n| 项 | 内容 |\n|----|------|\n| 项目名称 | ${name} |\n`;
  }
  tpl = tpl.replace(/\{name\}/g, name);
  if (/\| 项目名称 \|.*\|/.test(tpl)) {
    tpl = tpl.replace(/\| 项目名称 \|[^|]*/ , `| 项目名称 | ${name} `);
  }
  const docs = path.join(resolveRoot(dir), "docs");
  fs.mkdirSync(docs, { recursive: true });
  fs.writeFileSync(path.join(docs, "resource_estimate.md"), tpl, "utf-8");
  state.resourceEstimateDone = true;
  const extra = attachPolicies(state, ["workflow.phase0"]);
  writeState(dir, state);
  return toolResult(`已写入 docs/resource_estimate.md${extra}`);
}

function cmdInstall(a: Record<string, unknown>) {
  const lang = argStr(a, "lang", "zh-CN");
  if (!LANGUAGES.some((l) => l.code === lang)) return toolResult(`未知语言: ${lang}`, true);
  let mods: string[];
  if (Array.isArray(a.modules) && a.modules.length > 0) {
    mods = a.modules as string[];
  } else {
    mods = ["agent"];
  }
  const cliId = argStr(a, "cli") || null;
  let cli = cliId ? CLIS.find((c) => c.id === cliId) : null;
  const dir = argStr(a, "dir", ".");
  if (!cli) cli = detectTool(dir) ?? CLIS[4];
  const written: string[] = [];
  for (const m of mods) {
    const mod = MODULES.find((x) => x.id === m);
    if (!mod) return toolResult(`未知模块: ${m}`, true);
    const content = readPrompt(lang, mod.id);
    const name = targetFileName(mod.id, cli);
    const target = path.join(dir, cli.dir, name);
    fs.mkdirSync(path.dirname(target), { recursive: true });
    fs.writeFileSync(target, content, "utf-8");
    written.push(target);
  }
  return toolResult(
    `已安装 ${written.length} 个模块到 ${cli.name}:\n` + written.map((p) => `- ${p}`).join("\n")
  );
}
