# mentor-mcp

> AI Model Mentor 的 MCP 服务：把导师框架暴露为 **按需政策片段 + 沙箱工具**，供 Cursor / Claude Code / VS Code / Windsurf 等 IDE 接入。
>
> **本服务不跑 LLM。** IDE 才是 Agent；mentor-mcp 负责选片、闸门和路径监狱。
>
> **未发布到 npmjs。** `"private": true`，没有 `npx @guapimm/mentor-mcp`。正式用法是从 **GitHub Release** 下载自包含 `.tgz`（内含 `dist` + 提示词，不用自己 `tsc`）。开发者仍可 clone 后本地构建。

## 使用教程（IDE）

### 0. 前置

- Node.js ≥ 18（只要 MCP 这条路径；纯提示词用户不需要）
- **推荐：** 从 [Releases](https://github.com/guapimm/AI-Model-Development-Mentor/releases) 下载 `guapimm-mentor-mcp-*.tgz`
- **或者** clone 本仓库后在 `mcp/` 里本地编译

### 1a. 用 Release 包（不用自己 build）

```bash
# 下载 Release 里的 guapimm-mentor-mcp-0.1.0.tgz 后：
npx -y file:./guapimm-mentor-mcp-0.1.0.tgz
```

IDE 配置见 `examples/mcp.release.json`（把路径改成 tgz 的绝对路径）。

### 1b. 从源码构建

```bash
git clone https://github.com/guapimm/AI-Model-Development-Mentor.git
cd AI-Model-Development-Mentor/mcp
npm install          # 本地依赖，不是发包
npm run build
node dist/index.js
```

可选自检（模拟 IDE 的 stdio 握手）：

```bash
node scripts/verify-stdio.mjs
```

### 2. 写入 IDE 配置

复制 `examples/mcp.json`，把路径改成**你机器上的绝对路径**（Windows 可用 `E:/...` 正斜杠）。

| IDE | 配置文件 |
|-----|----------|
| Cursor | `.cursor/mcp.json` 或 Cursor Settings → MCP |
| Claude Code | 项目根 `.mcp.json` |
| VS Code | `.vscode/mcp.json`（有的插件字段叫 `servers` 而不是 `mcpServers`） |
| Windsurf | 与 Cursor 类似的 MCP 设置 |

```json
{
  "mcpServers": {
    "mentor": {
      "command": "node",
      "args": ["/absolute/path/to/AI-Model-Development-Mentor/mcp/dist/index.js"],
      "env": {
        "MENTOR_PROMPTS_DIR": "/absolute/path/to/AI-Model-Development-Mentor"
      }
    }
  }
}
```

改完后重启 MCP / 重载窗口，确认列表里出现 `mentor` 以及 `session_start` 等工具。

### 3. 推荐对话流程

1. 对 Agent 说：先调用 `session_start`（可带 `lang: "zh-CN"`）。
2. 用 `init_project` 或自己准备 `REQUIREMENTS.md`。
3. `generate_resource_estimate` 写出阶段 0 预估表，再 `session_advance`。
4. 写文件走 `fs_write`，跑命令走 `run_command`（不要绕过沙箱用 IDE 自带 Write/Bash，否则闸门不生效）。
5. 需要某条规则时 `policy_search` / `policy_load`，不要整文件 `@security.md`。

## 上下文分层（不要整文件塞规则）

| 层 | 何时出现 | 内容 |
|----|----------|------|
| L0 | `session_start` 一次 | 人设 + 6 条铁律一行版 + **片段目录** + 当前阶段 |
| L1 | 进入某阶段 | 该阶段 1–2 个 fragment |
| L2 | `fs_write` / `run_command` 返回值 | 与本次操作相关的安全/工作流片段 |
| L3 | `policy_load` / `policy_search` | 模型主动取一条当前未驻留的规则 |

同一会话已加载过的 id 只回一行提醒。需要全文时 `policy_load` 并设 `force: true`。

模型「忘了」规则时的兜底：**服务端闸门**（越界、`.env`、300/500 行、阶段 0 未完成就写业务代码、前端无 UI 映射、部署无 `local_backup/`）——失败响应里会附上对应 fragment。

## 工具

| 工具 | 作用 |
|------|------|
| `session_start` | 建/恢复 `.mentor/state.json`，返回 L0 |
| `session_advance` | 阶段前进一步 |
| `policy_load` / `policy_search` | 按需取规则 |
| `fs_read` / `fs_write` / `fs_list` | 工作区读写（路径监狱） |
| `run_command` | 沙箱命令；`MENTOR_SANDBOX=docker` 时进容器，否则路径监狱 |
| `snapshot_update` / `init_project` / `generate_resource_estimate` | 快照、骨架、阶段 0 预估表（写盘） |
| `install` / `detect_tool` / `list_languages` / `list_modules` | 提示词安装与探测 |

MCP Prompts：`mentor-start` / `mentor-phase0` / `mentor-coding` / `mentor-security`。

Resources：`mentor://prompts/{lang}/{module}`（全文，调试用）以及 `mentor://policy/{lang}/{fragmentId}`（切片）。

提示词定位顺序：`MENTOR_PROMPTS_DIR` → 内置 `mcp/prompts/` → 向上查找仓库根。

可选环境变量：

- `MENTOR_SANDBOX=docker` — `run_command` 走 Docker（`--network none`）；没有 Docker 时回退路径监狱并在结果里说明
- `MENTOR_DOCKER_IMAGE` — 默认 `node:22-alpine`

## 使用规则

1. **先 `session_start`，再写代码。** L0 只有目录，不是四份模块全文。
2. **按需 `policy_load`。** 登录/密钥 → `security.secrets`；SQL → `security.db`；前端 → `security.xss` + `workflow.ui`。不要每轮 `@security.md` `@workflow.md`。
3. **项目文件走沙箱工具。** `fs_write` / `run_command` 才有 300/500 行、`.env`、阶段闸门。IDE 自带 Write/Bash 会绕过。
4. **阶段 0 完成前不写业务代码。** 先有 `REQUIREMENTS.md` 和 `docs/resource_estimate.md`，再 `session_advance`。
5. **写前端前先有 `docs/ui_mapping.md`。**
6. **不要把 `complete` 完整版当源。** 它缺「资源前置」等较新铁律；源是拆开的四模块。
7. **密钥只出现在环境变量和 `.env.example`（仅变量名）。** 真实 `.env` 禁止写入。

## 注意事项

- **没有 npmjs 包。** 不要 `npm install -g @guapimm/mentor-mcp`。用户从 GitHub Release 下 `.tgz`；开发者 `git pull` 后重新 `npm install && npm run build`。
- **Docker（可选）：** 在已 `sync` + `build` 的 `mcp/` 目录执行 `docker build -t mentor-mcp .`，然后 `docker run -i --rm mentor-mcp`（stdio）。镜像默认不推仓库。
- **配置必须用绝对路径。** 相对路径在 IDE 里经常解析失败。改完配置需重载 MCP。
- **沙箱是路径监狱，不是虚拟机。** 默认只锁工作区目录、禁 `.env` / `.git`、限制危险命令。Docker 是可选的 `run_command` 后端。
- **闸门管不到 IDE 原生工具。** 请在对话里明确要求 Agent 用 mentor 的 `fs_*` / `run_command`。
- **`.mentor/state.json` 是本机会话状态**，不要提交密钥进去；可按需加入项目 `.gitignore`。
- **Windows：** `args` 里建议正斜杠 `E:/path/to/...`；需本机已安装 Node 且 `node` 在 PATH 中。
- **只想用提示词、不接 MCP：** 用 GitHub Releases 的 `mentor` 二进制或手动复制 `prompts/`，不需要 Node。
