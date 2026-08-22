# 各 AI 工具加载说明（兼容性指南）

`prompts/` 目录下的提示词内容与具体 AI 工具**无关**，任何基于大模型的编码工具都能使用。差异只在于**加载方式**：主文件名、存放位置、加载命令。本文件就是"一个加载说明文件"——新增工具只需在这里补一行。

> 提示：所有工具均可通过 `mentor` CLI 一键安装（自动按工具写入正确位置），见文末。

## 快速对照表

| 工具 | 主文件（agent 角色） | 存放位置 | 加载方式 | 其他模块（security/style/workflow） |
|------|---------------------|---------|---------|-----------------------------------|
| opencode | `AGENTS.md` | 项目根目录 | 自动加载 | 同样用 `@security.md` 等逐个引用 |
| Claude Code | `CLAUDE.md` 或 `AGENTS.md` | 项目根目录 | 自动加载 | 主文件里用 `@security.md` 引用，或放入子目录按需加载 |
| OpenAI Codex | `AGENTS.md` | 项目根目录 | 自动加载 | 主文件里用 `@security.md` 引用 |
| Cursor | `AGENTS.md` | `.cursor/rules/` | 自动加载（rules 可带 glob 匹配范围） | 同名文件放入同一目录 |
| Gemini CLI | `GEMINI.md` | 项目根目录 | 自动加载 | 重命名后一并放入，或用 `@` 引用 |
| Google Jules | `JULES.md` | 项目根目录 | 自动加载 | 同上 |
| Aider | `CONVENTIONS.md` | 项目根目录 | 自动加载 | 合并内容或分文件引用 |
| Windsurf | `.windsurfrules` | 项目根目录 | 自动加载 | 同上 |
| GitHub Copilot Agent | `AGENTS.md` | 项目根目录 | 自动加载 | 用 `@security.md` 引用 |
| 任意 MCP 客户端 | 通过 `mentor-mcp` | stdio（`node mcp/dist/index.js`） | 自动（resources + tools） | 所有模块以 MCP 资源 `mentor://prompts/{lang}/{module}` 暴露 |

## 各工具详细说明

### opencode
1. 复制 `prompts/AGENTS.md` 到项目根目录
2. opencode 每次会话自动加载 AGENTS.md，无需手动操作
3. 需要安全/风格/工作流时，`@security.md`、`@style.md`、`@workflow.md` 按需加载
4. 长周期项目：规则沉淀在 AGENTS.md 中；断线用 `opencode --continue` 恢复

### Claude Code
1. 复制 `prompts/AGENTS.md` → 重命名为 `CLAUDE.md`（或保留 `AGENTS.md`，新版自动识别）
2. 放入项目根目录，每次会话自动加载
3. 其他模块在 `CLAUDE.md` 中用 `@security.md` 引用，或直接追加合并
4. 子目录内的 `CLAUDE.md` 会在进入该目录时按需加载

### OpenAI Codex
1. 复制 `prompts/AGENTS.md` 到项目根目录（Codex 自动加载根目录 `AGENTS.md`）
2. 其他模块在 `AGENTS.md` 中用 `@security.md` 引用
3. 断线恢复用 `codex --resume`（或 `codex exec --resume`）

### Cursor
1. 复制 `prompts/AGENTS.md` 到 `.cursor/rules/` 目录（Agent 自动加载 rules）
2. 如需按文件范围生效，可转为 `.mdc` 格式并加 frontmatter 的 `globs` 匹配
3. 其他模块同名文件一并放入 `.cursor/rules/`

### Gemini CLI
1. 复制 `prompts/AGENTS.md` → 重命名为 `GEMINI.md`，放入项目根目录，自动加载
2. 其他模块可合并进 `GEMINI.md` 或按需 `@` 引用

### Google Jules
1. 复制 `prompts/AGENTS.md` → 重命名为 `JULES.md`，放入项目根目录，自动加载

### Aider
1. 复制 `prompts/AGENTS.md` → 重命名为 `CONVENTIONS.md`，放入项目根目录，编辑会话自动加载

### Windsurf
1. 复制 `prompts/AGENTS.md` → 重命名为 `.windsurfrules`，放入项目根目录，自动加载

### GitHub Copilot Agent
1. 复制 `prompts/AGENTS.md` 到项目根目录，自动加载；其他模块用 `@security.md` 引用

### MCP（Model Context Protocol）
1. 构建服务：`cd mcp && npm install && npm run build`
2. 将 MCP 客户端指向 `node <repo>/mcp/dist/index.js`（样例见 `mcp/examples/mcp.json`）
3. **按需加载**：先 `session_start` 拿到 L0 目录，需要某条规则再 `policy_load`；不要每轮塞入完整 security/style/workflow
4. 读写项目文件请走 `fs_write` / `run_command`（路径监狱；可选 Docker）
5. 资源：`mentor://prompts/{lang}/{module}`（全文）与 `mentor://policy/{lang}/{fragmentId}`（切片）
6. 详见 `mcp/README.md`

## 用 mentor CLI 一键安装

```bash
mentor install          # 交互式：选语言 → 选模块（默认 agent）→ 自动识别/选择工具
mentor install --lang zh-CN --modules agent,security --cli claude-code
mentor add workflow     # 追加模块
mentor list             # 查看已安装模块
```

`mentor` 会自动按上表规则把文件写入各工具要求的文件名和位置（Claude Code → `CLAUDE.md`、Cursor → `.cursor/rules/`、其余 → `AGENTS.md` 等）。

## 完整版提示词

不需要按模块拆分时，可直接使用 `prompts/开发者导师提示词完整版.md`（四模块合并版，一次性加载）。
