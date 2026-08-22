import { spawnSync } from "node:child_process";
import * as fs from "node:fs";
import * as path from "node:path";

const DENY_BASENAMES = new Set([".env"]);
const COMMAND_BLACKLIST = [
  /\brm\s+(-[a-zA-Z]*f[a-zA-Z]*\s+)?\/\s*$/i,
  /\bformat\s+[a-z]:/i,
  /\bmkfs\b/i,
  /\bshutdown\b/i,
  /\bdel\s+\/s\b/i,
  /\bdd\s+if=/i,
  /:\(\)\s*\{\s*:\s*\|\s*:\s*&\s*\}\s*;/,
  /\breg\s+delete\b/i,
  /\bcipher\s+\/w/i,
];

export class SandboxError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "SandboxError";
  }
}

export function resolveRoot(dir: string): string {
  const abs = path.resolve(dir);
  if (!fs.existsSync(abs)) {
    throw new SandboxError(`工作区不存在: ${abs}`);
  }
  return fs.realpathSync.native(abs);
}

function existingParent(p: string): string {
  let cur = p;
  while (!fs.existsSync(cur)) {
    const parent = path.dirname(cur);
    if (parent === cur) return cur;
    cur = parent;
  }
  return fs.realpathSync.native(cur);
}

/** Resolve rel against workspace root; throw if it escapes. */
export function resolveInRoot(root: string, rel: string): string {
  const rootReal = resolveRoot(root);
  const abs = path.resolve(rootReal, rel);
  const parentReal = existingParent(abs);
  const rest = path.relative(parentReal, abs);
  const candidate = rest ? path.resolve(parentReal, rest) : parentReal;
  const relToRoot = path.relative(rootReal, candidate);
  if (relToRoot.startsWith("..") || path.isAbsolute(relToRoot)) {
    throw new SandboxError(`路径越出工作区: ${rel}`);
  }
  const segs = relToRoot.split(/[/\\]/);
  if (segs.includes(".git")) {
    throw new SandboxError("禁止访问 .git/");
  }
  return candidate;
}

export function assertWritable(root: string, absPath: string): void {
  const base = path.basename(absPath);
  if (DENY_BASENAMES.has(base)) {
    throw new SandboxError("禁止写入真实 .env（请写 .env.example，密钥走环境变量）");
  }
  const rel = path.relative(resolveRoot(root), absPath);
  if (rel.split(/[/\\]/).includes(".git")) {
    throw new SandboxError("禁止写入 .git/");
  }
}

export function toPosixRel(root: string, absPath: string): string {
  return path.relative(resolveRoot(root), absPath).split(path.sep).join("/");
}

function sanitizeEnv(): NodeJS.ProcessEnv {
  const keep = [
    "PATH",
    "Path",
    "PATHEXT",
    "SYSTEMROOT",
    "SystemRoot",
    "COMSPEC",
    "ComSpec",
    "HOME",
    "USERPROFILE",
    "HOMEDRIVE",
    "HOMEPATH",
    "LANG",
    "LC_ALL",
    "TERM",
    "TMP",
    "TEMP",
    "TMPDIR",
  ];
  const env: NodeJS.ProcessEnv = {};
  for (const k of keep) {
    if (process.env[k] !== undefined) env[k] = process.env[k];
  }
  return env;
}

export function assertCommandAllowed(command: string): void {
  const c = command.trim();
  if (!c) throw new SandboxError("命令为空");
  for (const re of COMMAND_BLACKLIST) {
    if (re.test(c)) throw new SandboxError(`命令被沙箱拒绝: ${c}`);
  }
}

export type RunResult = {
  stdout: string;
  stderr: string;
  status: number | null;
  sandbox: "jail" | "docker" | "docker-fallback-jail";
};

function runLocal(root: string, command: string, timeoutMs: number): RunResult {
  const cwd = resolveRoot(root);
  const isWin = process.platform === "win32";
  const result = spawnSync(isWin ? "cmd.exe" : "sh", isWin ? ["/c", command] : ["-c", command], {
    cwd,
    env: sanitizeEnv(),
    encoding: "utf-8",
    timeout: timeoutMs,
    maxBuffer: 2 * 1024 * 1024,
    windowsHide: true,
  });
  if (result.error && (result.error as NodeJS.ErrnoException).code === "ETIMEDOUT") {
    throw new SandboxError(`命令超时（${timeoutMs}ms）`);
  }
  return {
    stdout: result.stdout ?? "",
    stderr: result.stderr ?? "",
    status: result.status,
    sandbox: "jail",
  };
}

function runDocker(root: string, command: string, timeoutMs: number): RunResult | null {
  const cwd = resolveRoot(root);
  const image = process.env.MENTOR_DOCKER_IMAGE || "node:22-alpine";
  const args = [
    "run",
    "--rm",
    "--network",
    "none",
    "--memory",
    "512m",
    "--cpus",
    "1",
    "-v",
    `${cwd}:/workspace`,
    "-w",
    "/workspace",
    image,
    "sh",
    "-lc",
    command,
  ];
  const result = spawnSync("docker", args, {
    encoding: "utf-8",
    timeout: timeoutMs + 5000,
    maxBuffer: 2 * 1024 * 1024,
    windowsHide: true,
  });
  if (result.error) return null;
  if (result.status === 127 || /not found|cannot find/i.test(result.stderr ?? "")) return null;
  return {
    stdout: result.stdout ?? "",
    stderr: result.stderr ?? "",
    status: result.status,
    sandbox: "docker",
  };
}

export function runCommand(
  root: string,
  command: string,
  opts?: { timeoutMs?: number; sandbox?: "jail" | "docker" }
): RunResult {
  assertCommandAllowed(command);
  const timeoutMs = opts?.timeoutMs ?? 30_000;
  const mode = opts?.sandbox ?? (process.env.MENTOR_SANDBOX === "docker" ? "docker" : "jail");
  if (mode === "docker") {
    const hit = runDocker(root, command, timeoutMs);
    if (hit) return hit;
    const fallback = runLocal(root, command, timeoutMs);
    return { ...fallback, sandbox: "docker-fallback-jail" };
  }
  return runLocal(root, command, timeoutMs);
}
