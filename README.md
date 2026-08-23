# Code Superman 🦸

AI 代码库理解与可视化桌面工具 —— 把 AI 生成的"黑盒"代码变成可导航的白盒。

## 功能规划

- [x] 打开项目文件夹，扫描目录树 + 语言占比统计
- [ ] 理解强度分级（轻度静态分析 / 中度文件摘要 / 深度函数解读）
- [ ] LLM 分层摘要（文件 → 模块 → 全局架构）
- [ ] 全景架构视图 + 依赖关系图谱
- [ ] 导出 xmind / Markdown / PDF
- [ ] AI 问答（Chat with Codebase）
- [ ] 技术栈报告、Token 成本预估

## 技术栈

- **桌面壳**：Tauri 2（Rust 后端）
- **前端**：React 19 + TypeScript + Vite
- **扫描器**：`ignore` crate（尊重 .gitignore）+ 自研语言识别
- **AI 接入**：OpenAI 兼容协议（支持 OpenAI / DeepSeek / Kimi / Ollama 等，用户自带 Key）

## 开发

```bash
npm install        # 安装前端依赖
npm run tauri dev  # 启动开发模式（需 Rust 工具链）
npm run build      # 前端构建检查
npm run tauri build  # 打包发布版
```

Rust 单元测试：

```bash
cd src-tauri && cargo test
```

## 目录结构

```
├── src/                # React 前端
│   ├── components/     # FileTree / LanguageStats 等组件
│   ├── App.tsx         # 主界面
│   └── types.ts        # 与 Rust 对齐的数据结构
├── src-tauri/
│   └── src/
│       ├── scanner.rs  # 目录扫描 + 语言识别核心
│       └── lib.rs      # Tauri 命令注册
```
