import { loadFragment } from "./policy.js";
import { markLoaded, type MentorState } from "./state.js";

export function attachPolicies(
  state: MentorState,
  ids: string[],
  opts?: { force?: boolean }
): string {
  if (!ids.length) return "";
  const force = opts?.force ?? false;
  const blocks: string[] = [];
  const fresh = force ? ids : markLoaded(state, ids);
  const remind = force ? [] : ids.filter((id) => !fresh.includes(id));
  for (const id of fresh) {
    try {
      const body = loadFragment(id, state.lang);
      blocks.push(`## policy:${id}\n${body}`);
    } catch (e) {
      blocks.push(`## policy:${id}\n(无法加载: ${e instanceof Error ? e.message : e})`);
    }
  }
  for (const id of remind) {
    blocks.push(`(policy ${id} 已在本会话加载；policy_load("${id}", force=true) 可重新展开)`);
  }
  if (!blocks.length) return "";
  return "\n\n---\nmentor.policy\n" + blocks.join("\n\n");
}

export function toolResult(text: string, isError = false) {
  return { content: [{ type: "text" as const, text }], isError };
}
