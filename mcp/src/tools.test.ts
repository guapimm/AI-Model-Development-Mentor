import assert from "node:assert/strict";
import * as fs from "node:fs";
import * as os from "node:os";
import * as path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, it } from "node:test";
import { handleTool } from "./tools.js";

process.env.MENTOR_PROMPTS_DIR = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");

function tmp(): string {
  return fs.mkdtempSync(path.join(os.tmpdir(), "mentor-tool-"));
}

function text(r: { content: { type: string; text: string }[] }): string {
  return r.content.map((c) => c.text).join("");
}

describe("tools wiring", () => {
  it("session_start returns L0 catalog without full security.md", async () => {
    const dir = tmp();
    const r = await handleTool("session_start", { dir, lang: "zh-CN" });
    const t = text(r);
    assert.ok(t.includes("security.db"));
    assert.ok(t.includes("policy catalog"));
    assert.ok(!t.includes("禁止字符串拼接 SQL"));
    assert.ok(fs.existsSync(path.join(dir, ".mentor", "state.json")));
  });

  it("fs_write src in phase0 is denied and injects workflow.phase0", async () => {
    const dir = tmp();
    await handleTool("session_start", { dir, lang: "zh-CN" });
    const r = await handleTool("fs_write", {
      dir,
      path: "src/app.ts",
      content: "export const x = 1;\n",
    });
    assert.equal(r.isError, true);
    const t = text(r);
    assert.ok(t.includes("phase0-before-code") || t.includes("还不能写业务代码"));
    assert.ok(t.includes("policy:workflow.phase0"));
    assert.ok(!fs.existsSync(path.join(dir, "src", "app.ts")));
  });

  it("second policy_load of same id is a reminder", async () => {
    const dir = tmp();
    await handleTool("session_start", { dir, lang: "zh-CN" });
    const first = text(await handleTool("policy_load", { dir, id: "security.xss" }));
    assert.ok(first.includes("XSS") || first.includes("转义"));
    const second = text(await handleTool("policy_load", { dir, id: "security.xss" }));
    assert.ok(second.includes("已在本会话加载"));
    assert.ok(second.length < first.length);
  });
});
