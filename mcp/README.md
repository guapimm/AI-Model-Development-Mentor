# mentor-mcp

> AI Model Mentor 的 MCP（Model Context Protocol）服务：把导师提示词框架暴露为标准 MCP `resources` 与 `tools`，供支持 MCP 的 AI 编码工具动态加载与调用。

## 是什么

本项目是一个「纯提示词」框架。原方案是手动把 `<lang>/prompts/` 下的文件复制到项目里。`mentor-mcp` 让这一过程标准化：任何支持 MCP 的工具（Claude Code、Cursor、Windsurf、opencode 等）都能通过协议读取导师提示词、一键安装、检测工具。

## 能力一览

### Resources（提示词资源）

URI 格式：`mentor://prompts/{语言}/{模块}`

- 语言：`zh-CN` / `en-US` / `ja-JP` / `ko-KR` / `es-ES` / `fr-FR` / `de-DE` / `pt-BR` / `ru-RU`
- 模块：`agent`（导师角色）/ `security` / `style` / `workflow` / `complete`

### Tools（工具）

| 工具 | 作用 |
|------|------|
| `install` | 把提示词写入目标项目（按工具自动写对文件名与位置） |
| `detect_tool` | 检测项目目录使用的 AI 工具 |
| `list_languages` | 列出可用语言 |
| `list_modules` | 列出可用模块 |
| `generate_resource_estimate` | 生成《项目资源预估表》模板（阶段0） |

## 运行方式（本地构建）

```bash
cd mcp
npm install
npm run build
node dist/index.js          # 以 stdio 模式启动
```

服务启动后按 `MENTOR_PROMPTS_DIR` → 内置 `mcp/prompts/` → 向上查找仓库根目录 的顺序定位提示词，因此克隆仓库即可直接使用，无需额外配置。

> 提示：`npm run sync` 会把仓库根目录的 `<lang>/prompts/` 同步到 `mcp/prompts/`（生成产物，已被 gitignore）。本地开发通常不需要这一步——服务会自动向上查找到仓库根目录。

## 在 AI 工具中接入

不同工具的 MCP 配置写法略不同，通用形如（`mcp/dist/index.js` 替换为本机实际路径）：

```json
{
  "mcpServers": {
    "mentor": {
      "command": "node",
      "args": ["/absolute/path/to/AI-Model-Development-Mentor/mcp/dist/index.js"]
    }
  }
}
```

## 提示词同步

提示词唯一来源是仓库根目录的 `<lang>/prompts/`。需要自包含分发时，用 `npm run sync` 把提示词同步到 `mcp/prompts/`（该目录已被 gitignore，属生成产物）。
