import assert from "node:assert/strict";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, it } from "node:test";
import { allFragments, extractSection, loadFragment, searchFragments } from "./policy.js";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");
process.env.MENTOR_PROMPTS_DIR = repoRoot;

describe("policy fragments", () => {
  it("zh-CN and en-US every id slices a non-empty section", () => {
    const ids = allFragments().map((f) => f.id);
    assert.ok(ids.length >= 20);
    for (const lang of ["zh-CN", "en-US"]) {
      for (const id of ids) {
        const body = loadFragment(id, lang);
        assert.ok(body.length > 20, `${lang} ${id} too short`);
        assert.ok(/^#{1,6}\s+/.test(body), `${lang} ${id} should start with heading`);
      }
    }
  });

  it("search XSS hits security.xss", () => {
    const hits = searchFragments("XSS", "zh-CN");
    assert.ok(hits.some((h) => h.id === "security.xss"), JSON.stringify(hits));
  });

  it("extractSection stops at next same-level heading", () => {
    const md = "## A\nfoo\n## B\nbar\n";
    assert.equal(extractSection(md, "A"), "## A\nfoo");
    assert.equal(extractSection(md, "B"), "## B\nbar");
  });
});
