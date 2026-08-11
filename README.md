# AI Model Mentor ｜ AI 模型导师

> **Turn your AI coding assistant into a cautious 10-year full-stack mentor — pure prompts, zero dependencies.**
> 在项目初期加载这套 AI 模型导师框架，让你的 AI 更加谨慎地开发、代码更加方便维护。纯提示词版本，零依赖。

[English Version](./README_EN.md) ｜ 中文版本（当前页）

---

## 这是什么？

一套**纯提示词（Prompt）框架**，把 AI 编码助手塑造成一位**拥有 10 年经验的全栈架构师兼开发导师**，核心服务对象是零基础代码小白。

它会强制 AI 遵守一套"铁律"，把"安全兜底、逻辑透明、文档先行、Token 高效、分步落地"变成默认行为——让 AI 从"能写出代码"升级为"写出安全、可维护、有文档的代码"。

> ⚠️ 当前支持平台：**小米 MIMO CLI**。需要其他产品（如 Claude Code、Cursor 等）的优化版，请到仓库留言。

## 核心特性

| 模块 | 文件 | 作用 |
|------|------|------|
| 🧑‍🏫 角色定义 | `AGENTS.md` | 全栈架构师导师人设 + 6 大铁律 + 安全自检清单 |
| 🛡️ 安全规范 | `security.md` | 8 大安全领域规范：密钥管理 / 输入校验 / 数据库 / XSS / 文件系统 / 外部请求 / 异常处理 / 性能 |
| 🎨 交互风格 | `style.md` | 生活化类比、阶段标签、先确认后执行、渐进式复杂度 |
| 📋 开发工作流 | `workflow.md` | 文档体系 / 前端定位协议 / 部署灾备 / 测试自检闭环 / 版本锚点 |

### 6 大铁律（角色模块核心）

1. **代码即文档** — 所有代码带中文注释，解释"为什么这样做"
2. **安全前置** — 禁止硬编码密钥、输入严格校验、参数化查询、防 XSS
3. **零破坏性变更** — 修改前分析依赖，标注【必选修改】/【可选优化】
4. **分步执行** — 单次输出不超过 300 行，每步等确认
5. **模块化隔离** — 单文件不超过 500 行，预留扩展接口
6. **Token 高效** — 每次对话生成【上下文摘要】与【续传暗号】

## 快速上手（3 步）

```bash
# 1. 把导师角色复制进你的项目（重命名为 AGENTS.md）
cp AGENTS_ZH.md AGENTS.md

# 2.（推荐）安全/风格/工作流规范一并加入项目
cp security_ZH.md security.md
cp style_ZH.md style.md
cp workflow_ZH.md workflow.md
```

3. 启动小米 MIMO，直接说：

> "我是零基础小白，这是我的【项目需求说明书】：项目名称 ____、核心目标 ____、用户角色 ____、核心操作流程 ____、必须存储的数据 ____。请从阶段 0：环境准备与技术栈选型开始，逐步带我开发。"

AI 将按"设计 → 核心逻辑 → 界面 → 测试"分步推进，每步等你确认。

## 文件结构

```
AI_Model_Development_Mentor/
├── README.md            # 本文件（双语门面）
├── README_ZH.md         # 中文完整版入口
├── README_EN.md         # English full version
├── AGENTS_ZH.md         # 导师角色定义（中文）★ 核心，必用
├── AGENTS_EN.md         # Mentor role definition (EN)
├── security_ZH.md       # 安全规范详细手册（中文）
├── security_EN.md       # Security spec (EN)
├── style_ZH.md          # 交互风格与输出规范（中文）
├── style_EN.md          # Interaction style (EN)
├── workflow_ZH.md       # 开发工作流规范（中文）
├── workflow_EN.md       # Dev workflow (EN)
└── LICENSE              # MIT License
```

## 常见问题

**Q：一定要用全部 4 个模块吗？**
A：不是。`AGENTS.md` 是核心必须，其余 3 个按需加载——想要更强安全就加 `security.md`，想要更舒适的对话体验就加 `style.md`。

**Q：支持其他 AI 产品吗？**
A：目前只适配了小米 MIMO。其他产品的优化版正在计划中，欢迎留言告诉我们你的需求。

## 许可

[MIT License](./LICENSE) © 2026 guapimm
