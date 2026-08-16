# AI Model Mentor ｜ AI 模型导师（中文）

中文模块入口。🌍 语言切换见仓库首页 [README.md](../README.md)，英文版见 [en-US/README.md](../en-US/README.md)。

## 中文模块（多工具兼容）

| 模块 | 文件 | 作用 |
|------|------|------|
| 🧑‍🏫 角色定义 | [AGENTS.md](./prompts/AGENTS.md) | 全栈架构师导师人设 + 6 大铁律 + 安全与性能自检清单 ★ 核心，必用 |
| 🛡️ 安全规范 | [security.md](./prompts/security.md) | 8 大安全领域规范：密钥管理 / 输入校验 / 数据库 / XSS / 文件系统 / 外部请求 / 异常处理 / 性能与资源 |
| 🎨 交互风格 | [style.md](./prompts/style.md) | 生活化类比、阶段标签、先确认后执行、渐进式复杂度 |
| 📋 开发工作流 | [workflow.md](./prompts/workflow.md) | 文档体系 / 资源预估 / 数据库设计 / 前端定位协议 / 部署灾备 / 测试自检闭环 / 版本锚点 |

## 📦 更多文档

- [COMPATIBILITY.md](./COMPATIBILITY.md) — 各 AI 工具（opencode / Claude Code / Codex / Cursor 等）加载说明
- [开发者导师提示词完整版.md](./prompts/开发者导师提示词完整版.md) — 四模块合并版完整提示词，一次性加载

## ⬇️ 安装与使用 mentor CLI

**方式 A：二进制文件（推荐，零依赖、跨平台）**

从 GitHub Releases 下载对应平台的 `mentor` 可执行文件（v0.1.0，支持 Windows / Linux / macOS；Go 版 `mentor-*` 与 Rust 版 `mentor-rust-*` 功能一致，任选其一），放入 PATH 后：

```bash
mentor install                        # 交互向导：选语言 → 选模块（默认 agent）→ 自动识别工具
mentor install --lang zh-CN --modules agent,security --cli claude-code --dir ./proj
mentor add workflow                   # 追加模块
mentor list                           # 查看已安装模块
mentor detect                         # 检测项目使用的 AI 工具
mentor pack                           # 生成兼容 skill 目录
```

`mentor` 会自动按工具写入正确文件名和位置：opencode/Codex → `AGENTS.md`、Claude Code → `CLAUDE.md`、Cursor → `.cursor/rules/`。

**方式 B：手动复制**

按 [COMPATIBILITY.md](./COMPATIBILITY.md) 的说明，把 `prompts/` 下的文件复制到项目对应位置。

> 支持的命令：`install` / `add` / `remove` / `list` / `detect` / `pack`；模块：agent（默认）/ security / style / workflow / complete；工具：opencode / claude-code / codex / cursor / other。

## 📖 使用指南

### 命令速览

| 场景 | 操作 |
|------|------|
| 日常开发 | 进入项目 → opencode 自动加载 AGENTS.md → 正常对话 |
| 长周期项目 | 规则沉淀在 AGENTS.md 中（用 `/init` 更新） |
| 意外断线 | 用 `opencode --continue` 恢复，规则还在 |
| 主动新开会话 | 直接启动 `opencode`，AGENTS.md 自动加载 |

### 项目文件结构

```
📁 my-project/
├── 📄 AGENTS.md          ← 主提示词
├── 📄 security.md        ← 安全规范
├── 📄 workflow.md        ← 工作流规范
├── 📄 style.md           ← 交互风格
└── 📁 src/
```

### 具体场景演示

#### 场景 1：日常写代码（只加载 AGENTS.md）

> 你："帮我写一个获取用户列表的 API"

需要加载：AGENTS.md（已自动加载，无需操作）

AI 会自动：

- 代码带中文注释
- 输出前勾选安全清单
- 分步执行（≤300行）
- 单文件≤500行

#### 场景 2：写登录/注册接口（加载 AGENTS.md + security.md）

> 你："帮我写用户登录功能，按照 security.md 的要求做"

需要加载：

```bash
@security.md
```

AI 会额外：

- 密码用 bcrypt 哈希存储
- JWT Token 设置过期时间
- 防暴力破解（登录失败限制）
- 防 SQL 注入（参数化查询）

#### 场景 3：从零启动项目（加载 AGENTS.md + workflow.md）

> 你："我要做一个博客系统，参考 workflow.md 帮我搭建项目骨架"

需要加载：

```bash
@workflow.md
```

AI 会额外：

- 创建 docs/architecture.md（技术选型+架构图）
- 创建 docs/dev_log.md（开发日志模板）
- 创建 docs/api_interface.md（接口契约模板）
- 创建 docs/SNAPSHOT.md（项目快照）
- 生成 backup.sh 和 rollback.sh 脚本

#### 场景 4：AI 解释太晦涩（加载 style.md）

> 你："按照 style.md 的方式，用生活化类比给我解释什么是 JWT"

需要加载：

```bash
@style.md
```

AI 会额外：

- 用"餐厅会员卡"解释 JWT
- 加上阶段标签 [📋需求分析]
- 先给结论再给细节
- 提供 2-3 个可选方案

#### 场景 5：部署上线（加载 AGENTS.md + workflow.md）

> 你："按照 workflow.md 的部署规范，帮我写 Docker 部署配置"

需要加载：

```bash
@workflow.md
```

AI 会额外：

- 区分开发/生产环境配置
- 生成 docker-compose.yml
- 生成 health_check.sh
- 提醒备份和回滚步骤

### ⚠️ 什么时候不用加载？

| 不用加载的情况 | 原因 |
|---------------|------|
| 问纯技术问题（如"React useEffect 怎么用"） | AGENTS.md 已足够，加 workflow 反而干扰 |
| 修改一个 CSS 样式 | 不需要安全规范和部署流程 |
| 让 AI 翻译一段文字 | 完全不需要任何模块 |
| 已有代码做简单重构 | AGENTS.md 的安全清单已覆盖 |

### 💡 一句话总结

> AGENTS.md 是默认皮肤，其他三个是特效插件——需要时才开，平时别开，省 Token 又清爽。

## 快速上手（3 步）

```bash
# 1. 把导师角色复制进你的项目（重命名为 AGENTS.md）
cp prompts/AGENTS.md AGENTS.md

# 2.（推荐）安全/风格/工作流规范一并加入项目
cp prompts/security.md security.md
cp prompts/style.md style.md
cp prompts/workflow.md workflow.md
```

3. 启动opencode，提供【项目需求说明书】（项目名称、核心目标、用户角色、核心操作流程、必须存储的数据），从阶段 0 开始逐步开发。

> 📦 新增工具将作为行添加到 [COMPATIBILITY.md](./COMPATIBILITY.md)，无需再按产品创建子目录。
