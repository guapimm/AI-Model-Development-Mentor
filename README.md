# Code Superman 🦸

AI 代码库理解与可视化桌面工具 —— 把 AI 生成的"黑盒"代码变成可导航的白盒。

打开任意项目文件夹，秒级获得技术栈报告、依赖关系图谱、代码度量与符号大纲；可选接入全球主流大模型，进一步生成每个文件的深度解读与项目架构总览，并一键导出 XMind 思维导图。

## 功能总览

### 🆓 静态分析（免费、离线、无需任何配置）
- **目录扫描**：尊重 .gitignore，自动跳过 node_modules 等目录，语言占比统计
- **技术栈识别**：解析 package.json / Cargo.toml / go.mod / requirements.txt / pom.xml / .csproj 等 12 类清单文件，内置 100+ 条框架识别规则
- **入口点定位**：识别 main.* / app.* / index.* 等入口文件
- **符号大纲**：基于 tree-sitter 解析 **10 大语言**（Rust / Python / TypeScript / TSX / JavaScript / Go / Java / C / C++ / C# / PHP / Ruby）的函数、类、结构体、接口清单及 import 依赖
- **依赖关系图谱**：import 关系解析为文件级依赖边，AntV G6 力导向图可视化，节点大小即连接度
- **代码度量**：行数/有效行/TODO-FIXME 统计、超大文件警告
- **导出**：架构树导出为标准 .xmind 文件

### 🤖 AI 增强（用户自带 API Key）
- **单文件解读**：文件职责 + 函数作用 + 模块关系
- **项目架构总览**：项目用途推断、模块职责划分、核心数据流、建议阅读顺序
- **XMind 导出附带 AI 摘要备注**

### ⚖️ 理解强度（控制输出详略程度）
| 强度 | 单次输出上限 | 输出内容 |
|---|---|---|
| ⚡ 简要 | ~500 tokens | 2~3 句话概括 |
| ⚖️ 标准 | ~1500 tokens | 职责 + 主要函数列表 + 模块关系 |
| 🔬 详尽 | ~3000 tokens | 逐函数逻辑 + 算法解释 + 数据流 + 问题风险 |

附加选项：
- ☑️ **全量分析**——尝试分析全部代码文件（上限 200 个），默认智能挑选 30 个
- ☑️ **不限长度**——解除输出 token 上限

## 支持的 AI 服务商

| 协议 | 服务商 |
|---|---|
| OpenAI 兼容 | OpenAI、DeepSeek、Kimi（月之暗面）、通义千问、智谱 GLM、豆包·火山方舟、硅基流动、xAI Grok、Mistral、Groq、OpenRouter、Ollama、LM Studio、vLLM 及一切兼容端点 |
| Anthropic 原生 | Claude 官方 API |
| Gemini 原生 | Google Gemini 官方 API |
| Azure OpenAI | Azure 企业部署（支持自定义 deployment / api-version） |

> 设置界面提供预设一键填充；API Key 仅保存在本机应用数据目录，不上传到任何服务器。

## 技术栈

- **桌面壳**：Tauri 2（Rust 后端）
- **前端**：React 19 + TypeScript + Vite + AntV G6
- **代码解析**：tree-sitter（10 语言 grammar）
- **AI 接入**：四协议适配层（请求体构建/响应解析均有单元测试）

## 开发

```bash
npm install          # 安装前端依赖
npm run tauri dev    # 开发模式（需 Rust 工具链）
npm run build        # 前端构建检查
npm run tauri build  # 打包发布版
```

Rust 测试：

```bash
cd src-tauri && cargo test
```

## 目录结构

```
├── src/                      # React 前端
│   ├── components/
│   │   ├── FileTree.tsx      # 目录树
│   │   ├── LanguageStats.tsx # 语言占比
│   │   ├── StaticReportView.tsx  # 静态分析报告
│   │   ├── DepGraphView.tsx  # G6 依赖图谱
│   │   └── SettingsModal.tsx # AI 服务设置
│   ├── App.tsx               # 主界面
│   └── types.ts              # 与 Rust 对齐的数据结构
└── src-tauri/src/
    ├── scanner.rs            # 目录扫描 + 语言识别
    ├── static_analysis.rs    # 技术栈识别 + 代码度量
    ├── symbols.rs            # tree-sitter 符号提取
    ├── depgraph.rs           # 依赖关系解析建图
    ├── xmind.rs              # .xmind 生成
    ├── llm.rs                # 四协议 LLM 适配层
    ├── settings.rs           # 配置存储与迁移
    └── lib.rs                # Tauri 命令注册
```

## Roadmap

- [ ] AI 问答（Chat with Codebase，基于 RAG）
- [ ] 语义搜索
- [ ] 增量更新（文件变更只重新分析差异部分）
- [ ] 对比模式（两个版本差异的 AI 解读）
- [ ] PDF 项目说明书导出
