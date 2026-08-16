import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ListResourcesRequestSchema,
  ListToolsRequestSchema,
  ReadResourceRequestSchema,
} from "@modelcontextprotocol/sdk/types.js";
import * as fs from "node:fs";
import * as path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const VERSION = "0.1.0";

type Lang = { code: string; name: string };
type Mod = { id: string; desc: string };
type Cli = { id: string; name: string; agentFile: string; dir: string };

const LANGUAGES: Lang[] = [
  { code: "zh-CN", name: "中文" },
  { code: "en-US", name: "English" },
  { code: "ja-JP", name: "日本語" },
  { code: "ko-KR", name: "한국어" },
  { code: "es-ES", name: "Español" },
  { code: "fr-FR", name: "Français" },
  { code: "de-DE", name: "Deutsch" },
  { code: "pt-BR", name: "Português" },
  { code: "ru-RU", name: "Русский" },
];

const MODULES: Mod[] = [
  { id: "agent", desc: "导师角色（默认必选）" },
  { id: "security", desc: "安全规范" },
  { id: "style", desc: "交互风格" },
  { id: "workflow", desc: "开发工作流" },
  { id: "complete", desc: "完整版合并提示词" },
];

const CLIS: Cli[] = [
  { id: "opencode", name: "opencode", agentFile: "AGENTS.md", dir: "" },
  { id: "claude-code", name: "Claude Code", agentFile: "CLAUDE.md", dir: "" },
  { id: "codex", name: "OpenAI Codex", agentFile: "AGENTS.md", dir: "" },
  { id: "cursor", name: "Cursor", agentFile: "AGENTS.md", dir: ".cursor/rules" },
  { id: "other", name: "其他（自定义）", agentFile: "AGENTS.md", dir: "" },
];

const KNOWN_FILES = new Set(["AGENTS.md", "security.md", "style.md", "workflow.md"]);

function resolvePromptsRoot(): string {
  const env = process.env.MENTOR_PROMPTS_DIR;
  if (env && fs.existsSync(path.join(env, "zh-CN", "prompts", "AGENTS.md"))) {
    return env;
  }
  const bundled = path.join(__dirname, "..", "prompts");
  if (fs.existsSync(path.join(bundled, "zh-CN", "prompts", "AGENTS.md"))) {
    return bundled;
  }
  let dir = process.cwd();
  for (;;) {
    if (fs.existsSync(path.join(dir, "zh-CN", "prompts", "AGENTS.md"))) {
      return dir;
    }
    const parent = path.dirname(dir);
    if (parent === dir) break;
    dir = parent;
  }
  throw new Error(
    "无法定位导师提示词目录：请设置 MENTOR_PROMPTS_DIR 指向包含 <lang>/prompts 的目录"
  );
}

function moduleFileName(lang: string, module: string): string {
  const dir = path.join(resolvePromptsRoot(), lang, "prompts");
  if (module === "agent") return "AGENTS.md";
  if (module === "complete") {
    const files = fs
      .readdirSync(dir)
      .filter((f) => f.endsWith(".md") && !KNOWN_FILES.has(f));
    if (files.length === 0) {
      throw new Error(`语言 ${lang} 缺少完整版提示词文件`);
    }
    return files[0];
  }
  return `${module}.md`;
}

function readPrompt(lang: string, module: string): string {
  const dir = path.join(resolvePromptsRoot(), lang, "prompts");
  const name = moduleFileName(lang, module);
  const p = path.join(dir, name);
  if (!fs.existsSync(p)) {
    throw new Error(`提示词不存在: ${lang}/${name}`);
  }
  return fs.readFileSync(p, "utf-8");
}

function targetFileName(module: string, cli: Cli): string {
  if (module === "agent") return cli.agentFile;
  if (module === "complete") return "complete-mentor-prompt.md";
  return `${module}.md`;
}

function detectTool(dir: string): Cli | null {
  const probe = (p: string) => fs.existsSync(path.join(dir, p));
  if (probe(".opencode")) return CLIS.find((c) => c.id === "opencode")!;
  if (probe("opencode.json")) return CLIS.find((c) => c.id === "opencode")!;
  if (probe("opencode.jsonc")) return CLIS.find((c) => c.id === "opencode")!;
  if (probe("CLAUDE.md")) return CLIS.find((c) => c.id === "claude-code")!;
  if (probe(".cursor")) return CLIS.find((c) => c.id === "cursor")!;
  if (probe(".codex")) return CLIS.find((c) => c.id === "codex")!;
  if (probe("AGENTS.md")) return CLIS.find((c) => c.id === "codex")!;
  return null;
}

const RESOURCE_ESTIMATE_ZH = `# 项目资源预估表（阶段0）

| 项 | 内容 |
|----|------|
| 项目名称 | {name} |
| 预估总代码量（行） | （< 500 行可启用轻量模式） |
| 峰值并发用户数 | |

## 三档资源预估
| 资源维度 | 最低配置 | 推荐配置 | 高可用配置 |
|----------|---------|---------|-----------|
| 内存 | | | |
| 磁盘 | | | |
| CPU 核数 | | | |
| 数据库 | SQLite | MySQL/PostgreSQL | 集群 + 读写分离 |

## 性能与资源预案
- [ ] 列表查询默认分页，禁止全表扫描
- [ ] 数据库设计同步输出索引方案
- [ ] 大文件/大数据量操作采用流式处理
- [ ] 大内存操作有明确释放机制
- [ ] 外部请求设置超时与重试
`;

const RESOURCE_ESTIMATE_EN = `# Project Resource Estimate (Phase 0)

| Item | Value |
|------|-------|
| Project name | {name} |
| Estimated LOC | (< 500 lines enables lightweight mode) |
| Peak concurrent users | |

## Three-tier Resource Estimate
| Dimension | Minimum | Recommended | High-availability |
|-----------|---------|-------------|-------------------|
| Memory | | | |
| Disk | | | |
| CPU cores | | | |
| Database | SQLite | MySQL/PostgreSQL | Cluster + read-write split |

## Performance & Resource Plan
- [ ] List endpoints paginate by default; no full-table scans
- [ ] Database design includes an index plan
- [ ] Large-data operations use streaming
- [ ] Large memory operations have a release mechanism
- [ ] External requests set timeout & retry
`;

function toolResult(text: string, isError = false) {
  return { content: [{ type: "text" as const, text }], isError };
}

function toolsList() {
  return {
    tools: [
      {
        name: "install",
        description:
          "把导师提示词安装到目标项目（按目标 AI 工具写入正确的文件名与位置）。",
        inputSchema: {
          type: "object",
          properties: {
            lang: { type: "string", description: "语言代码，如 zh-CN / en-US" },
            modules: {
              type: "array",
              items: { type: "string" },
              description: "模块列表，默认 ['agent']，可选 security/style/workflow/complete",
            },
            cli: {
              type: "string",
              description: "目标工具：opencode / claude-code / codex / cursor / other",
            },
            dir: { type: "string", description: "安装目录，默认当前目录" },
          },
          required: ["lang"],
        },
        annotations: { readOnlyHint: false, destructiveHint: false, idempotentHint: true },
      },
      {
        name: "detect_tool",
        description: "检测项目目录使用的 AI 编码工具。",
        inputSchema: {
          type: "object",
          properties: {
            dir: { type: "string", description: "要检测的目录，默认当前目录" },
          },
        },
        annotations: { readOnlyHint: true, idempotentHint: true },
      },
      {
        name: "list_languages",
        description: "列出可用的提示词语言。",
        inputSchema: { type: "object", properties: {} },
        annotations: { readOnlyHint: true, idempotentHint: true },
      },
      {
        name: "list_modules",
        description: "列出可用的导师模块（agent/security/style/workflow/complete）。",
        inputSchema: { type: "object", properties: {} },
        annotations: { readOnlyHint: true, idempotentHint: true },
      },
      {
        name: "generate_resource_estimate",
        description: "生成《项目资源预估表》模板（阶段0），可填入项目名称。",
        inputSchema: {
          type: "object",
          properties: {
            name: { type: "string", description: "项目名称" },
            lang: { type: "string", description: "语言：zh-CN（默认）/ en-US" },
          },
        },
        annotations: { readOnlyHint: true, idempotentHint: true },
      },
    ],
  };
}

async function handleTool(name: string, args: unknown): Promise<ReturnType<typeof toolResult>> {
  const a = (args ?? {}) as Record<string, unknown>;
  switch (name) {
    case "list_languages": {
      const lines = LANGUAGES.map((l) => `${l.code} — ${l.name}`);
      return toolResult(lines.join("\n"));
    }
    case "list_modules": {
      const lines = MODULES.map((m) => `${m.id} — ${m.desc}`);
      return toolResult(lines.join("\n"));
    }
    case "detect_tool": {
      const dir = typeof a.dir === "string" ? a.dir : ".";
      const hit = detectTool(dir);
      return toolResult(hit ? `检测到: ${hit.name} (${hit.id})` : "未检测到已知工具");
    }
    case "generate_resource_estimate": {
      const lang = a.lang === "en-US" ? "en-US" : "zh-CN";
      const name = typeof a.name === "string" ? a.name : "（待填）";
      const tpl = lang === "en-US" ? RESOURCE_ESTIMATE_EN : RESOURCE_ESTIMATE_ZH;
      return toolResult(tpl.replace("{name}", name));
    }
    case "install": {
      const lang = typeof a.lang === "string" ? a.lang : "zh-CN";
      const langOk = LANGUAGES.some((l) => l.code === lang);
      if (!langOk) return toolResult(`未知语言: ${lang}`, true);
      let mods: string[];
      if (Array.isArray(a.modules) && a.modules.length > 0) {
        mods = a.modules as string[];
      } else {
        mods = ["agent"];
      }
      const cliId = typeof a.cli === "string" ? a.cli : null;
      let cli = cliId ? CLIS.find((c) => c.id === cliId) : null;
      if (!cli) cli = detectTool(typeof a.dir === "string" ? a.dir : ".") ?? CLIS[4];
      if (!cli) return toolResult("无法确定目标工具", true);
      const dir = typeof a.dir === "string" ? a.dir : ".";
      const written: string[] = [];
      for (const m of mods) {
        const mod = MODULES.find((x) => x.id === m);
        if (!mod) return toolResult(`未知模块: ${m}`, true);
        const content = readPrompt(lang, mod.id);
        const name = targetFileName(mod.id, cli);
        const target = path.join(dir, cli.dir, name);
        fs.mkdirSync(path.dirname(target), { recursive: true });
        fs.writeFileSync(target, content, "utf-8");
        written.push(target);
      }
      return toolResult(`已安装 ${written.length} 个模块到 ${cli.name}:\n` + written.map((p) => `- ${p}`).join("\n"));
    }
    default:
      return toolResult(`未知工具: ${name}`, true);
  }
}

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
  }
  return { resources };
}

function parsePromptUri(uri: string): { lang: string; module: string } | null {
  const m = uri.match(/^mentor:\/\/prompts\/([^/]+)\/([^/]+)$/);
  if (!m) return null;
  return { lang: decodeURIComponent(m[1]), module: decodeURIComponent(m[2]) };
}

async function main() {
  const server = new Server(
    { name: "mentor-mcp", version: VERSION },
    { capabilities: { tools: {}, resources: {} } }
  );

  server.setRequestHandler(ListToolsRequestSchema, async () => toolsList());

  server.setRequestHandler(CallToolRequestSchema, async (request) => {
    const { name, arguments: args } = request.params;
    return handleTool(name, args);
  });

  server.setRequestHandler(ListResourcesRequestSchema, async () => listAllResources());

  server.setRequestHandler(ReadResourceRequestSchema, async (request) => {
    const parsed = parsePromptUri(request.params.uri);
    if (!parsed) {
      throw new Error(`无效的资源 URI: ${request.params.uri}`);
    }
    const { lang, module } = parsed;
    const text = readPrompt(lang, module);
    return {
      contents: [
        {
          uri: request.params.uri,
          mimeType: "text/markdown",
          text,
        },
      ],
    };
  });

  const transport = new StdioServerTransport();
  await server.connect(transport);
}

main().catch((err) => {
  console.error("mentor-mcp 启动失败:", err);
  process.exit(1);
});
