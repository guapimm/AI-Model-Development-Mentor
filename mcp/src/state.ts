import * as fs from "node:fs";
import * as path from "node:path";
import { PHASES, type Phase } from "./constants.js";
import { resolveInRoot, resolveRoot } from "./sandbox.js";

export type SandboxMode = "jail" | "docker";

export type MentorState = {
  version: 1;
  lang: string;
  phase: Phase;
  loaded: string[];
  sandbox: SandboxMode;
  bugFails: Record<string, number>;
  resourceEstimateDone: boolean;
  uiMapped: boolean;
};

const STATE_REL = ".mentor/state.json";

export function defaultState(lang: string, sandbox: SandboxMode = "jail"): MentorState {
  return {
    version: 1,
    lang,
    phase: "need_requirements",
    loaded: [],
    sandbox,
    bugFails: {},
    resourceEstimateDone: false,
    uiMapped: false,
  };
}

export function statePath(root: string): string {
  return resolveInRoot(root, STATE_REL);
}

function refreshFlags(root: string, state: MentorState): MentorState {
  const est = path.join(resolveRoot(root), "docs", "resource_estimate.md");
  const ui = path.join(resolveRoot(root), "docs", "ui_mapping.md");
  const snap = path.join(resolveRoot(root), "docs", "SNAPSHOT.md");
  if (fs.existsSync(est)) state.resourceEstimateDone = true;
  if (fs.existsSync(ui)) state.uiMapped = true;
  if (fs.existsSync(snap) && state.phase === "need_requirements") {
    // keep phase; snapshot presence does not skip requirements
  }
  return state;
}

export function readState(root: string): MentorState | null {
  const p = path.join(resolveRoot(root), ".mentor", "state.json");
  if (!fs.existsSync(p)) return null;
  const raw = JSON.parse(fs.readFileSync(p, "utf-8")) as MentorState;
  if (!PHASES.includes(raw.phase)) raw.phase = "need_requirements";
  if (!Array.isArray(raw.loaded)) raw.loaded = [];
  return refreshFlags(root, raw);
}

export function writeState(root: string, state: MentorState): void {
  const dir = path.join(resolveRoot(root), ".mentor");
  fs.mkdirSync(dir, { recursive: true });
  fs.writeFileSync(path.join(dir, "state.json"), JSON.stringify(state, null, 2), "utf-8");
}

export function loadOrCreateState(root: string, lang: string, sandbox?: SandboxMode): MentorState {
  const existing = readState(root);
  if (existing) {
    if (lang) existing.lang = lang;
    if (sandbox) existing.sandbox = sandbox;
    return refreshFlags(root, existing);
  }
  const mode = sandbox ?? (process.env.MENTOR_SANDBOX === "docker" ? "docker" : "jail");
  const state = defaultState(lang || "zh-CN", mode);
  writeState(root, refreshFlags(root, state));
  return state;
}

export function markLoaded(state: MentorState, ids: string[]): string[] {
  const fresh: string[] = [];
  for (const id of ids) {
    if (!state.loaded.includes(id)) {
      state.loaded.push(id);
      fresh.push(id);
    }
  }
  return fresh;
}

export function nextPhase(phase: Phase): Phase | null {
  const i = PHASES.indexOf(phase);
  if (i < 0 || i >= PHASES.length - 1) return null;
  return PHASES[i + 1];
}
