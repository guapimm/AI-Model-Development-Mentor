import * as path from "node:path";
import { FRONTEND_EXT } from "./constants.js";
import { allFragments, type FragmentDef } from "./policy.js";
import type { MentorState } from "./state.js";

export type SelectEvent = {
  tool: string;
  relPath?: string;
  intent?: string;
  command?: string;
};

function extOf(rel: string): string {
  return path.extname(rel).toLowerCase();
}

function haystack(ev: SelectEvent): string {
  return [ev.relPath ?? "", ev.intent ?? "", ev.command ?? "", ev.tool]
    .join(" ")
    .toLowerCase();
}

function matchDef(def: FragmentDef, state: MentorState, ev: SelectEvent): boolean {
  const t = def.triggers ?? {};
  if (t.phase?.includes(state.phase)) return true;
  if (t.event?.includes(ev.tool)) return true;
  if (ev.relPath) {
    const ext = extOf(ev.relPath);
    if (t.ext?.some((e) => e.toLowerCase() === ext)) return true;
    const p = ev.relPath.replace(/\\/g, "/").toLowerCase();
    if (t.path?.some((seg) => p.includes(seg.toLowerCase()))) return true;
  }
  const h = haystack(ev);
  if (t.intent?.some((k) => h.includes(k.toLowerCase()))) return true;
  return false;
}

/** Pick fragment ids for this event. Caller handles first-load vs reminder. */
export function selectFragmentIds(state: MentorState, ev: SelectEvent): string[] {
  const ids: string[] = [];
  const seen = new Set<string>();
  const push = (id: string) => {
    if (!seen.has(id)) {
      seen.add(id);
      ids.push(id);
    }
  };

  if (ev.tool === "session_start") {
    if (state.phase === "need_requirements") push("agent.startup");
    if (state.phase === "phase0") push("workflow.phase0");
  }
  if (ev.tool === "session_advance") {
    push("style.tags");
    push("style.rhythm");
  }
  if (ev.tool === "fs_write" && ev.relPath && FRONTEND_EXT.has(extOf(ev.relPath))) {
    push("security.xss");
    push("workflow.ui");
  }
  if (ev.tool === "fs_write") push("style.zerodestroy");

  for (const def of allFragments()) {
    if (matchDef(def, state, ev)) push(def.id);
  }
  return ids;
}

export function phaseEnterIds(phase: string): string[] {
  switch (phase) {
    case "need_requirements":
      return ["agent.startup", "style.confirm"];
    case "phase0":
      return ["workflow.phase0", "workflow.docs"];
    case "design":
      return ["workflow.docs", "workflow.db"];
    case "logic":
      return ["agent.output", "agent.checklist"];
    case "ui":
      return ["workflow.ui", "security.xss"];
    case "test":
      return ["workflow.test"];
    case "enhance":
      return ["workflow.enhance"];
    default:
      return [];
  }
}
