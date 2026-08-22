/**
 * Protocol-level check: spawn mentor-mcp over stdio like an IDE would.
 * Usage: node scripts/verify-stdio.mjs
 */
import { spawn } from "node:child_process";
import * as fs from "node:fs";
import * as os from "node:os";
import * as path from "node:path";
import { fileURLToPath } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
const mcpRoot = path.resolve(here, "..");
const repoRoot = path.resolve(mcpRoot, "..");
const serverJs = path.join(mcpRoot, "dist", "index.js");

if (!fs.existsSync(serverJs)) {
  console.error("FAIL: dist/index.js missing. Run: cd mcp && npm run build");
  process.exit(1);
}

const child = spawn(process.execPath, [serverJs], {
  cwd: mcpRoot,
  env: { ...process.env, MENTOR_PROMPTS_DIR: repoRoot },
  stdio: ["pipe", "pipe", "pipe"],
});

let buf = "";
const pending = new Map();
let nextId = 1;

child.stderr.on("data", (d) => process.stderr.write(d));
child.stdout.on("data", (chunk) => {
  buf += chunk.toString("utf8");
  let nl;
  while ((nl = buf.indexOf("\n")) !== -1) {
    const line = buf.slice(0, nl).replace(/\r$/, "");
    buf = buf.slice(nl + 1);
    if (!line.trim()) continue;
    let msg;
    try {
      msg = JSON.parse(line);
    } catch {
      console.error("non-json stdout:", line.slice(0, 200));
      continue;
    }
    if (msg.id != null && pending.has(msg.id)) {
      pending.get(msg.id)(msg);
      pending.delete(msg.id);
    }
  }
});

function send(obj) {
  child.stdin.write(JSON.stringify(obj) + "\n");
}

function request(method, params) {
  const id = nextId++;
  return new Promise((resolve, reject) => {
    const t = setTimeout(() => reject(new Error(`timeout: ${method}`)), 8000);
    pending.set(id, (msg) => {
      clearTimeout(t);
      if (msg.error) reject(new Error(`${method}: ${JSON.stringify(msg.error)}`));
      else resolve(msg.result);
    });
    send({ jsonrpc: "2.0", id, method, params });
  });
}

const tmp = fs.mkdtempSync(path.join(os.tmpdir(), "mentor-ide-"));

try {
  const init = await request("initialize", {
    protocolVersion: "2025-03-26",
    capabilities: {},
    clientInfo: { name: "mentor-verify", version: "0.0.0" },
  });
  send({ jsonrpc: "2.0", method: "notifications/initialized" });

  const tools = await request("tools/list");
  const prompts = await request("prompts/list");
  const resources = await request("resources/list");
  const start = await request("tools/call", {
    name: "session_start",
    arguments: { dir: tmp, lang: "zh-CN" },
  });
  const startText = start.content?.map((c) => c.text).join("") ?? "";

  const names = (tools.tools ?? []).map((t) => t.name);
  const need = [
    "session_start",
    "policy_load",
    "fs_write",
    "run_command",
    "install",
  ];
  const missing = need.filter((n) => !names.includes(n));

  console.log("OK initialize");
  console.log("  server:", init.serverInfo?.name, init.serverInfo?.version);
  console.log("  protocol:", init.protocolVersion);
  console.log("  capabilities:", Object.keys(init.capabilities ?? {}).join(", ") || "(none)");
  console.log("OK tools/list:", names.length, "tools");
  console.log("  ", names.join(", "));
  console.log("OK prompts/list:", (prompts.prompts ?? []).map((p) => p.name).join(", "));
  console.log("OK resources/list:", (resources.resources ?? []).length, "resources");
  console.log("OK session_start in", tmp);
  console.log("  L0 has catalog:", startText.includes("security.db"));
  console.log("  L0 has full SQL rule (should be false):", startText.includes("禁止字符串拼接 SQL"));
  console.log("  state file:", fs.existsSync(path.join(tmp, ".mentor", "state.json")));

  if (missing.length) {
    console.error("FAIL missing tools:", missing.join(", "));
    process.exit(1);
  }
  if (!startText.includes("security.db") || startText.includes("禁止字符串拼接 SQL")) {
    console.error("FAIL L0 catalog contract");
    process.exit(1);
  }
  console.log("\nIDE stdio handshake: PASS");
  console.log("Point Cursor/Claude Code/VS Code at:");
  console.log(`  command: node`);
  console.log(`  args:    ${serverJs.replace(/\\/g, "/")}`);
  console.log(`  env.MENTOR_PROMPTS_DIR: ${repoRoot.replace(/\\/g, "/")}`);
} catch (e) {
  console.error("FAIL", e);
  process.exit(1);
} finally {
  child.kill();
}
