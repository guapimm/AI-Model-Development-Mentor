import * as fs from "node:fs";
import * as path from "node:path";
import { CODE_EXT, FRONTEND_EXT, type Phase } from "./constants.js";
import { loadFragment } from "./policy.js";
import { resolveRoot } from "./sandbox.js";
import type { MentorState } from "./state.js";

export const MAX_WRITE_LINES = 300;
export const MAX_FILE_LINES = 500;

const SECRET_ASSIGN = /(API_KEY|SECRET|PASSWORD|TOKEN|AKIA)[A-Z0-9_]*\s*=\s*['"][^'"]{8,}['"]/i;

export type GateFail = {
  code: string;
  message: string;
  fragmentIds: string[];
};

export function countLines(text: string): number {
  if (!text) return 0;
  return text.replace(/\r\n/g, "\n").split("\n").length;
}

export function isDocRel(rel: string): boolean {
  const n = rel.replace(/\\/g, "/").replace(/^\.\//, "");
  if (n.startsWith("docs/")) return true;
  if (n.startsWith(".mentor/")) return true;
  const base = n.split("/").pop() ?? n;
  return (
    n === "REQUIREMENTS.md" ||
    n === "README.md" ||
    n === ".gitignore" ||
    n === ".env.example" ||
    base === "SNAPSHOT.md"
  );
}

export function checkWrite(opts: {
  root: string;
  rel: string;
  content: string;
  state: MentorState;
}): GateFail | null {
  const { root, rel, content, state } = opts;
  const lines = countLines(content);
  if (lines > MAX_WRITE_LINES) {
    return {
      code: "line-300",
      message: `单次写入 ${lines} 行，超过 ${MAX_WRITE_LINES} 行上限。请拆成多步，每步等确认。`,
      fragmentIds: ["agent.rules"],
    };
  }
  const abs = path.join(resolveRoot(root), rel);
  let existing = 0;
  if (fs.existsSync(abs)) {
    existing = countLines(fs.readFileSync(abs, "utf-8"));
  }
  // resulting file ≈ new content (overwrite model)
  if (lines > MAX_FILE_LINES) {
    return {
      code: "line-500",
      message: `目标文件将有 ${lines} 行（现有 ${existing}），超过 ${MAX_FILE_LINES} 行/文件。请拆分模块。`,
      fragmentIds: ["agent.rules"],
    };
  }
  if (SECRET_ASSIGN.test(content) && !rel.replace(/\\/g, "/").endsWith(".env.example")) {
    return {
      code: "secrets-in-body",
      message: "检测到疑似硬编码密钥。请改用环境变量，并只在 .env.example 中列出变量名。",
      fragmentIds: ["security.secrets"],
    };
  }
  const ext = path.extname(rel).toLowerCase();
  const doc = isDocRel(rel);
  const codingPhases: Phase[] = ["logic", "ui", "test", "enhance"];
  if (!doc && CODE_EXT.has(ext) && !codingPhases.includes(state.phase)) {
    return {
      code: "phase0-before-code",
      message: `当前阶段是 ${state.phase}，还不能写业务代码。请先完成需求与阶段0资源预估，再 session_advance。`,
      fragmentIds: ["workflow.phase0", "agent.startup"],
    };
  }
  if (FRONTEND_EXT.has(ext) && !state.uiMapped && !doc) {
    return {
      code: "ui-mapping-before-fe",
      message: "写前端文件前请先在 docs/ui_mapping.md 输出页面布局 + UI/事件映射表。",
      fragmentIds: ["workflow.ui"],
    };
  }
  return null;
}

export function checkExec(command: string, root: string, state: MentorState): GateFail | null {
  const c = command.toLowerCase();
  const deployish = /\b(deploy|docker-compose\s+up|kubectl\s+apply|helm\s+install|pm2\s+(restart|start))\b/.test(
    c
  );
  if (deployish) {
    const backup = path.join(resolveRoot(root), "local_backup");
    if (!fs.existsSync(backup) && !state.resourceEstimateDone) {
      // backup dir is the real gate; resourceEstimate is extra hint
    }
    if (!fs.existsSync(backup)) {
      return {
        code: "backup-before-deploy",
        message: "部署前必须存在 ./local_backup/。请先备份代码+配置+数据库。",
        fragmentIds: ["workflow.deploy"],
      };
    }
  }
  return null;
}

export function checkAdvance(state: MentorState, root: string): GateFail | null {
  const r = resolveRoot(root);
  if (state.phase === "need_requirements") {
    const req = path.join(r, "REQUIREMENTS.md");
    if (!fs.existsSync(req)) {
      return {
        code: "need-requirements",
        message: "请先写入 REQUIREMENTS.md（项目名称、目标、角色、流程、数据）。",
        fragmentIds: ["agent.startup"],
      };
    }
  }
  if (state.phase === "phase0" && !state.resourceEstimateDone) {
    return {
      code: "need-estimate",
      message: "进入下一阶段前必须有 docs/resource_estimate.md。请调用 generate_resource_estimate 或自行写入。",
      fragmentIds: ["workflow.phase0"],
    };
  }
  if (state.phase === "ui" && !state.uiMapped) {
    return {
      code: "need-ui-map",
      message: "进入测试前建议完成 docs/ui_mapping.md。若尚未写前端可先 session_advance 到 test。",
      fragmentIds: ["workflow.ui"],
    };
  }
  return null;
}

export function formatGateError(fail: GateFail, lang: string, bodies: { id: string; body: string }[]): string {
  const parts = [`[gate:${fail.code}] ${fail.message}`, ""];
  for (const b of bodies) {
    parts.push(`## policy:${b.id}`, b.body, "");
  }
  if (bodies.length === 0 && fail.fragmentIds.length) {
    for (const id of fail.fragmentIds) {
      try {
        parts.push(`## policy:${id}`, loadFragment(id, lang), "");
      } catch {
        parts.push(`## policy:${id} (load failed)`);
      }
    }
  }
  return parts.join("\n").trim();
}
