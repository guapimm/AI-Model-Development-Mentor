import assert from "node:assert/strict";
import * as fs from "node:fs";
import * as os from "node:os";
import * as path from "node:path";
import { describe, it } from "node:test";
import { checkAdvance, checkWrite, MAX_WRITE_LINES } from "./gates.js";
import { defaultState, writeState } from "./state.js";

function tmp(): string {
  return fs.mkdtempSync(path.join(os.tmpdir(), "mentor-gate-"));
}

describe("gates", () => {
  it("rejects writes over 300 lines", () => {
    const root = tmp();
    const state = defaultState("zh-CN");
    const content = Array.from({ length: MAX_WRITE_LINES + 1 }, (_, i) => `line ${i}`).join("\n");
    const fail = checkWrite({ root, rel: "docs/note.md", content, state });
    assert.ok(fail);
    assert.equal(fail!.code, "line-300");
  });

  it("blocks business code before phase0 done", () => {
    const root = tmp();
    const state = defaultState("zh-CN");
    state.phase = "phase0";
    const fail = checkWrite({ root, rel: "src/app.ts", content: "export {}\n", state });
    assert.ok(fail);
    assert.equal(fail!.code, "phase0-before-code");
    assert.ok(fail!.fragmentIds.includes("workflow.phase0"));
  });

  it("allows docs during phase0", () => {
    const root = tmp();
    const state = defaultState("zh-CN");
    state.phase = "phase0";
    const fail = checkWrite({ root, rel: "docs/architecture.md", content: "# a\n", state });
    assert.equal(fail, null);
  });

  it("blocks frontend without ui mapping", () => {
    const root = tmp();
    const state = defaultState("zh-CN");
    state.phase = "logic";
    const fail = checkWrite({ root, rel: "src/App.tsx", content: "export default function A(){return null}\n", state });
    assert.ok(fail);
    assert.equal(fail!.code, "ui-mapping-before-fe");
  });

  it("advance from phase0 without estimate fails", () => {
    const root = tmp();
    fs.mkdirSync(path.join(root, ".mentor"), { recursive: true });
    const state = defaultState("zh-CN");
    state.phase = "phase0";
    writeState(root, state);
    const fail = checkAdvance(state, root);
    assert.ok(fail);
    assert.equal(fail!.code, "need-estimate");
  });
});
