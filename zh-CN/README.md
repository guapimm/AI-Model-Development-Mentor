# AI Model Mentor ｜ AI 模型导师（中文）

中文模块入口。🌍 语言切换见仓库首页 [README.md](../README.md)，英文版见 [en-US/README.md](../en-US/README.md)。

## 中文模块（小米 MIMO 适配版）

| 模块 | 文件 | 作用 |
|------|------|------|
| 🧑‍🏫 角色定义 | [AGENTS.md](./xiaomi-mimo/AGENTS.md) | 全栈架构师导师人设 + 6 大铁律 + 安全自检清单 ★ 核心，必用 |
| 🛡️ 安全规范 | [security.md](./xiaomi-mimo/security.md) | 8 大安全领域规范：密钥管理 / 输入校验 / 数据库 / XSS / 文件系统 / 外部请求 / 异常处理 / 性能 |
| 🎨 交互风格 | [style.md](./xiaomi-mimo/style.md) | 生活化类比、阶段标签、先确认后执行、渐进式复杂度 |
| 📋 开发工作流 | [workflow.md](./xiaomi-mimo/workflow.md) | 文档体系 / 前端定位协议 / 部署灾备 / 测试自检闭环 / 版本锚点 |

## 快速上手（3 步）

```bash
# 1. 把导师角色复制进你的项目（重命名为 AGENTS.md）
cp xiaomi-mimo/AGENTS.md AGENTS.md

# 2.（推荐）安全/风格/工作流规范一并加入项目
cp xiaomi-mimo/security.md security.md
cp xiaomi-mimo/style.md style.md
cp xiaomi-mimo/workflow.md workflow.md
```

3. 启动小米 MIMO，提供【项目需求说明书】（项目名称、核心目标、用户角色、核心操作流程、必须存储的数据），从阶段 0 开始逐步开发。

> 📦 新增产品适配时，本目录下将平行添加产品目录，如 `zh-CN/claude-code/`、`zh-CN/cursor/`。
