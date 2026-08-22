import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  GetPromptRequestSchema,
  ListPromptsRequestSchema,
  ListResourcesRequestSchema,
  ListToolsRequestSchema,
  ReadResourceRequestSchema,
} from "@modelcontextprotocol/sdk/types.js";
import { LANGUAGES, MODULES, VERSION } from "./constants.js";
import { getMentorPrompt, listMentorPrompts } from "./mcp-prompts.js";
import { allFragments, loadFragment } from "./policy.js";
import { readPrompt } from "./prompt-files.js";
import { handleTool, toolsList } from "./tools.js";

function listAllResources() {
  const resources = [];
  for (const lang of LANGUAGES) {
    for (const mod of MODULES) {
      resources.push({
        uri: `mentor://prompts/${lang.code}/${mod.id}`,
        name: `${mod.id} — ${lang.name}`,
        description: `${lang.code} ${mod.desc}`,
        mimeType: "text/markdown",
      });
    }
    for (const frag of allFragments()) {
      resources.push({
        uri: `mentor://policy/${lang.code}/${frag.id}`,
        name: `${frag.id} — ${lang.name}`,
        description: lang.code === "zh-CN" ? frag.hint.zh : frag.hint.en,
        mimeType: "text/markdown",
      });
    }
  }
  return { resources };
}

function parseUri(uri: string): { kind: "prompts" | "policy"; lang: string; id: string } | null {
  const m = uri.match(/^mentor:\/\/(prompts|policy)\/([^/]+)\/(.+)$/);
  if (!m) return null;
  return { kind: m[1] as "prompts" | "policy", lang: decodeURIComponent(m[2]), id: decodeURIComponent(m[3]) };
}

async function main() {
  const server = new Server(
    { name: "mentor-mcp", version: VERSION },
    { capabilities: { tools: {}, resources: {}, prompts: {} } }
  );

  server.setRequestHandler(ListToolsRequestSchema, async () => toolsList());
  server.setRequestHandler(CallToolRequestSchema, async (request) => {
    const { name, arguments: args } = request.params;
    return handleTool(name, args);
  });
  server.setRequestHandler(ListResourcesRequestSchema, async () => listAllResources());
  server.setRequestHandler(ReadResourceRequestSchema, async (request) => {
    const parsed = parseUri(request.params.uri);
    if (!parsed) throw new Error(`无效的资源 URI: ${request.params.uri}`);
    const text =
      parsed.kind === "prompts"
        ? readPrompt(parsed.lang, parsed.id)
        : loadFragment(parsed.id, parsed.lang);
    return {
      contents: [{ uri: request.params.uri, mimeType: "text/markdown", text }],
    };
  });
  server.setRequestHandler(ListPromptsRequestSchema, async () => listMentorPrompts());
  server.setRequestHandler(GetPromptRequestSchema, async (request) => {
    const { name, arguments: args } = request.params;
    return getMentorPrompt(name, args as Record<string, string> | undefined);
  });

  const transport = new StdioServerTransport();
  await server.connect(transport);
}

main().catch((err) => {
  console.error("mentor-mcp 启动失败:", err);
  process.exit(1);
});
