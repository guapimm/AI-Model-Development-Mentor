---
name: code-superman
description: Analyze and understand any codebase — tech stack detection, entry points, symbol outlines via tree-sitter, file dependency graphs, code metrics, XMind export. Use when the user asks to understand, explore, summarize, or map an unfamiliar project/codebase.
---

# Code Superman — 代码库理解工具

通过 `code-superman` CLI 快速理解陌生代码库。所有输出为 Markdown，尊重 .gitignore。

## 前置检查

先确认二进制可用：

```bash
code-superman --version
# 若不可用：cargo install --path <repo>/crates/cli
```

## 推荐工作流（按序调用）

1. **总览**：`code-superman analyze <项目路径> --top 10`
   → 技术栈、入口点、最大文件、TODO 统计。这是理解项目的第一步。
2. **核心模块**：`code-superman deps <项目路径> --top 20`
   → 按入度/出度排序的核心文件列表 + import 依赖边。入度高的文件通常是核心。
3. **深入文件**：`code-superman symbols <项目路径> <相对文件路径>`
   → 单文件的函数/类/结构体大纲及行号范围。配合 Read 工具按行号精读关键函数。
4. **目录结构**（可选）：`code-superman scan <项目路径> --depth 2`
   → 语言占比 + 目录树概览。

## 输出控制

- 所有命令支持 `--max-chars N` 截断超长输出（默认 20000）
- `deps`/`analyze` 的 `--top N` 控制表格行数
- 若输出被截断，请用更小的 `--top` 或更具体的路径重试，不要盲目重试相同参数

## 注意事项

- 路径传项目根目录的绝对路径；symbols 的第二个参数是相对根目录的文件路径（用 `/` 分隔）
- 符号解析支持 Rust/Python/TS/JS/Go/Java/C/C++/C#/PHP/Ruby；其他语言返回空大纲
- 依赖图基于启发式 import 解析，个别边可能误连；跨同名文件的误报属已知限制
- 导出思维导图：`code-superman xmind <项目路径> -o out.xmind`

## MCP Server 模式

若宿主环境支持 MCP，可直接以 stdio 方式注册本工具，无需通过 shell 调用：

```json
{
  "mcpServers": {
    "code-superman": { "command": "code-superman", "args": ["serve"] }
  }
}
```

可用 tools：`analyze_static`、`scan_project`、`get_file_symbols`、`get_dependency_graph`、`export_xmind`，参数与 CLI 对齐（path / top / max_chars / depth / file / out）。
