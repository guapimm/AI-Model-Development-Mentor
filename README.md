# Code Superman 🦸

AI 代码库理解工具 —— 以 **MCP Server + CLI** 形态供大模型自主调用，把"黑盒"代码变成可导航的白盒。

不做 GUI，不需要填 API Key：调用者本身就是大模型。模型通过工具获取结构化的静态分析结果（技术栈、符号大纲、依赖图、度量），由模型自己完成深度解读。

## 两种用法

### 1. MCP Server（推荐）

任何支持 MCP 的客户端（Claude Code、opencode 等）注册一次即可：

```json
{
  "mcpServers": {
    "code-superman": { "command": "code-superman", "args": ["serve"] }
  }
}
```

仅 **3 个工具**，`strength` 三档控制详略：

| 工具 | 参数 | 说明 |
|---|---|---|
| `analyze` | `path`, `strength?`, `xmind_out?` | 语言占比、技术栈、入口点、度量、核心模块；brief/standard/detailed 控制详略 |
| `get_file_symbols` | `path`, `file` | tree-sitter 单文件函数/类大纲 |
| `export_xmind` | `path`, `out?` | 导出架构 .xmind 思维导图 |

### 2. CLI

```bash
code-superman analyze <path> [--detail brief|standard|detailed] [--xmind out.xmind]
code-superman symbols <path> <file>
code-superman xmind <path> [-o out.xmind]
```

强度分档：**brief** = 语言/技术栈/入口点（3K 字符）；**standard**（默认）= 加目录树、度量 Top20、核心文件 Top15、警告（20K）；**detailed** = 加全量依赖边与全量度量（50K）。

## 能力细节

- **静态分析**：完全离线免费。解析 package.json / Cargo.toml / go.mod / requirements.txt / pom.xml / .csproj 等 12 类清单文件，内置 100+ 条框架识别规则；识别 main/app/index 等入口点
- **符号大纲**：tree-sitter 解析 Rust / Python / TypeScript / TSX / JavaScript / Go / Java / C / C++ / C# / PHP / Ruby 共 12 种语言
- **依赖图谱**：import 启发式解析为文件级有向边，入度 = 被依赖次数，快速定位核心模块
- **目录扫描**：尊重 .gitignore，自动跳过 node_modules 等

## 安装

```bash
cargo install --path crates/cli    # 安装 code-superman 到 cargo bin
```

## 技术栈

- Rust workspace：`crates/core`（纯逻辑库）+ `crates/cli`（二进制）
- [rmcp](https://github.com/modelcontextprotocol/rust-sdk)（官方 MCP Rust SDK）+ clap + tree-sitter

## 开发

```bash
cargo test            # 运行单元测试
cargo build --release # 构建发布版二进制
```
