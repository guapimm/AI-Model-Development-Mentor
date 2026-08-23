# AGENTS.md — 项目记忆

## 仓库信息（勿推错仓库）

- **GitHub 仓库**：`guapimm/AI-Model-Development-Mentor`
- **远程**：`origin = https://github.com/guapimm/AI-Model-Development-Mentor.git`（唯一 remote）
- 推送目标只能是 `origin/main`；push 前用 `git remote -v` 复核

## 项目概要

Code Superman：代码库理解工具，MCP Server + CLI 双形态供大模型自主调用。
Rust workspace：`crates/core`（纯分析库：scanner / static_analysis / symbols / depgraph / xmind）+ `crates/cli`（二进制：clap + rmcp）。

## 开发约定

- 测试：`cargo test --workspace`
- 构建：`cargo build --release`
- 安装/更新二进制：`cargo install --path crates/cli`
  - ⚠️ 更新前需先结束运行中的进程：`Stop-Process -Name code-superman -Force`（MCP server 被客户端拉起会锁文件，报 os error 5 即此原因）
- opencode 的 MCP 注册位于 `~/.config/opencode/opencode.jsonc`；配置与二进制更新后必须重启 opencode 才生效
- 文档三件套：README.md（简介）/ docs/TUTORIAL.md（详细教程）/ skill/code-superman/SKILL.md（Agent Skill），改接口时三者同步
