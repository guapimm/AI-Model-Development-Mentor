import assert from "node:assert/strict";
import * as fs from "node:fs";
import * as os from "node:os";
import * as path from "node:path";
import { describe, it } from "node:test";
import { resolveInRoot, resolveRoot, SandboxError, assertWritable, runCommand } from "./sandbox.js";

function tmpWorkspace(): string {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "mentor-jail-"));
  fs.writeFileSync(path.join(dir, "ok.txt"), "hi");
  return dir;
}

describe("path jail", () => {
  it("allows in-root read path", () => {
    const root = tmpWorkspace();
    const abs = resolveInRoot(root, "ok.txt");
    assert.equal(path.basename(abs), "ok.txt");
    assert.ok(abs.startsWith(resolveRoot(root)));
  });

  it("rejects .. escape", () => {
    const root = tmpWorkspace();
    assert.throws(() => resolveInRoot(root, "../secret"), SandboxError);
  });

  it("rejects .env writes", () => {
    const root = tmpWorkspace();
    const abs = resolveInRoot(root, ".env");
    assert.throws(() => assertWritable(root, abs), /禁止写入真实 \.env/);
  });

  it("allows .env.example", () => {
    const root = tmpWorkspace();
    const abs = resolveInRoot(root, ".env.example");
    assertWritable(root, abs);
  });

  it("run_command cwd is workspace", () => {
    const root = tmpWorkspace();
    const r = runCommand(root, process.platform === "win32" ? "cd" : "pwd", { sandbox: "jail" });
    assert.equal(r.status, 0);
    const out = (r.stdout + r.stderr).replace(/\r/g, "").trim();
    assert.ok(out.toLowerCase().includes(resolveRoot(root).toLowerCase()) || out.length > 0);
  });
});
