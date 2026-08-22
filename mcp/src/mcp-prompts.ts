import { allFragments, l0AlwaysOn, loadFragment } from "./policy.js";
import { phaseEnterIds } from "./select.js";

export function listMentorPrompts() {
  return {
    prompts: [
      {
        name: "mentor-start",
        title: "Mentor boot (L0)",
        description: "Always-on 人设 + 铁律一行版 + 片段目录。不要整文件加载四模块。",
        arguments: [{ name: "lang", description: "zh-CN / en-US / …", required: false }],
      },
      {
        name: "mentor-phase0",
        title: "Phase 0 resource estimate",
        description: "阶段0：资源预估与文档体系（按需片段，非整份 workflow.md）。",
        arguments: [{ name: "lang", required: false }],
      },
      {
        name: "mentor-coding",
        title: "Coding phase",
        description: "编码阶段：四层输出格式 + 安全清单。",
        arguments: [{ name: "lang", required: false }],
      },
      {
        name: "mentor-security",
        title: "Security catalog",
        description: "只列出 security.* 片段 id，不注入全文。",
        arguments: [{ name: "lang", required: false }],
      },
    ],
  };
}

function userMsg(text: string) {
  return {
    messages: [
      {
        role: "user" as const,
        content: { type: "text" as const, text },
      },
    ],
  };
}

export function getMentorPrompt(name: string, args?: Record<string, string>) {
  const lang = args?.lang || "zh-CN";
  if (name === "mentor-start") {
    return userMsg(l0AlwaysOn(lang, "need_requirements"));
  }
  if (name === "mentor-phase0") {
    const ids = phaseEnterIds("phase0");
    const body = ids.map((id) => `## policy:${id}\n${loadFragment(id, lang)}`).join("\n\n");
    return userMsg(body);
  }
  if (name === "mentor-coding") {
    const ids = ["agent.output", "agent.checklist"];
    const body = ids.map((id) => `## policy:${id}\n${loadFragment(id, lang)}`).join("\n\n");
    return userMsg(body);
  }
  if (name === "mentor-security") {
    const ids = allFragments()
      .filter((f) => f.id.startsWith("security."))
      .map((f) => `- ${f.id} | ${lang === "zh-CN" ? f.hint.zh : f.hint.en}`)
      .join("\n");
    return userMsg(`需要某条时 policy_load(id)：\n${ids}`);
  }
  throw new Error(`未知 prompt: ${name}`);
}
