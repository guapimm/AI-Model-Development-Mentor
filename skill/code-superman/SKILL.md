---
name: code-superman
description: Analyze and understand any codebase — tech stack detection, entry points, symbol outlines via tree-sitter, core-module detection via dependency graph, code metrics, XMind export. Use when the user asks to understand, explore, summarize, or map an unfamiliar project/codebase.
---

# Code Superman — 代码库理解工具

通过 `code-superman` CLI 理解陌生代码库。所有输出为 Markdown，尊重 .gitignore。仅 3 个子命令，`strength` 三档控制详略。

## 前置检查

```bash
code-superman --version
# 若不可用：cargo install --path <repo>/crates/cli
```

## 命令

### 1. analyze — 项目全貌（第一步，通常也是最重要的一步）

```bash
code-superman analyze <项目路径> --detail brief|standard|detailed
```

报告头部含 **Token 估算**（全部代码 / 核心 Top15，按 4 字符/token 粗估），用于判断精读成本。

| 强度 | 内容 | 输出上限 |
|---|---|---|
| brief | 语言占比、技术栈、入口点、Token 估算 | 3000 字符 |
| standard（默认） | + 目录树、度量 Top20、核心文件 Top15（按入度）、警告 | 20000 |
| detailed | + 全量依赖边、全量度量 | 50000 |

可选 `--xmind <路径>` 导出**架构思维导图**（含总览/技术栈/入口点/核心模块/目录树分支）：

```bash
code-superman analyze <项目路径> --xmind <项目路径>/architecture.xmind
```

### 2. symbols — 单文件深入（第二步）

```bash
code-superman symbols <项目路径> <相对文件路径>
```

返回函数/类/结构体大纲及行号范围，配合 Read 工具精读关键函数。

### 3. xmind — 单独导出思维导图

```bash
code-superman xmind <项目路径> -o out.xmind
```

## 推荐工作流

1. `analyze <路径> --detail standard` → 掌握技术栈与核心模块（参考 Token 估算决定后续精读范围）
2. 对核心模块中的关键文件 `symbols` → 定位函数与行号
3. 用文件读取工具按行号精读
4. 用户需要思维导图时：
   - 先理解各关键文件的职责，为每个文件写一句话摘要
   - MCP 方式：调用 `analyze` 时传 `xmind_out` 和 `file_summaries`（摘要会写入对应节点备注）
   - CLI 方式：`analyze <路径> --xmind <路径>/architecture.xmind`

## 注意事项

- 路径传项目根目录的绝对路径；symbols 的文件参数用 `/` 分隔
- 符号解析支持 Rust/Python/TS/JS/Go/Java/C/C++/C#/PHP/Ruby；其他语言返回空大纲
- 输出被截断时改用 `--detail detailed` 或缩小分析范围，不要原样重试

## MCP Server 模式

若宿主支持 MCP，可直接注册（tools 与 CLI 参数一致）：

```json
{
  "mcpServers": {
    "code-superman": { "command": "code-superman", "args": ["serve"] }
  }
}
```

可用 tools：`analyze`（path / strength / xmind_out / file_summaries）、`get_file_symbols`（path / file）、`export_xmind`（path / out / file_summaries）。

**交互约定**：用户未指定强度、未表明是否需要 xmind 时，先用宿主提问能力向用户确认选项，再调用工具。
