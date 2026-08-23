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

### 两段式：先估算再让用户选强度（用户未指定强度时）

1. `analyze <路径> --detail brief` → 秒级获得全项目 Token 估算与核心文件清单
2. 用提问能力向用户展示估算并让其选择，选项模板：

   > Token 估算结果：全部代码约 **X tokens**，精读核心 Top15 约 **Y tokens**。请选择理解强度：
   > ⚡ 简要（本次报告 ~750 tokens）
   > ⚖️ 标准（报告 ~5K tokens）
   > 🔬 详尽（报告 ~12.5K tokens，含全量依赖边）

3. 按用户选择再次 `analyze` 对应强度

用户已明确指定强度时跳过询问，直接分析。

### 标准精读流程

1. `analyze <路径> --detail standard` → 掌握技术栈与核心模块
2. 对核心模块中的关键文件 `symbols` → 定位函数与行号
3. 用文件读取工具按行号精读

### 导出思维导图（两步出图法）

导出的架构图包含总览/入口点/核心模块/警告/目录树五分支；目录树节点自带「语言·行数·被依赖数·符号构成」标注，且每个代码文件的备注自动附带静态【符号大纲】。要让导图进一步说明「每个文件在做什么」：

1. 理解各文件职责后撰写摘要：为**尽可能多的代码文件**提供条目（至少覆盖核心 Top15 与各主要目录的代表文件），重要文件鼓励填写 details 深入解析
2. MCP 方式：调用 `analyze` 时同时传 `xmind_out` 与 `file_summaries`；CLI 方式无此参数（仅 MCP 支持）

## 通用注意事项

- 路径传项目根目录的绝对路径；symbols 的文件参数用 `/` 分隔
- 符号解析支持 Rust/Python/TS/JS/Go/Java/C/C++/C#/PHP/Ruby；其他语言返回空大纲
- 输出被截断时改用 `--detail detailed` 或缩小分析范围，不要原样重试

## 交互约定

- **只读约束**：仅做只读阅读与分析；除导出 .xmind 外不得创建、修改或删除任何文件
- 用户未指定强度时，先用 brief 快扫获取 Token 估算，再带成本信息弹窗让用户选择（见推荐工作流）
- 导出 xmind 前必须准备文件职责摘要经 file_summaries 传入（见两步出图法）

## 大型项目分治工作流（opencode 多 Agent）

**触发条件**：brief 分析显示 Token 估算 >100K，或代码文件 >200 个。

### 步骤

1. **快扫定分片**：`analyze <路径> --detail brief` → 看 Token 估算、语言占比与顶层目录结构
2. **切分模块**：按顶层目录/主要模块切成 N 片（每片 ≤50K tokens 为宜，一般 N=2~6）
3. **派发子代理**：用 Task 工具并行派出 explore 子代理，每个子代理的提示词模板：

   ```
   你负责理解项目 <根路径> 的 <子目录> 部分。
   1. 运行 code-superman analyze <根路径>/<子目录> --detail standard
   2. 对其中入度最高或行数最多的关键文件运行 code-superman symbols 并精读
   3. 只返回一个 JSON 对象（不要其他内容），键为相对项目根的文件路径，值为：
      {"summary": "该文件一句话职责", "details": "实现要点/数据流/依赖关系等深入解析"}
   只做只读分析，不创建或修改任何文件。
   ```

4. **合并结果**：主模型将各子代理返回的 JSON 合并为一个 map（键冲突时保留更详细的一方）
5. **统一出图**：调用 `analyze`（path=项目根）传 `xmind_out` 与合并后的 `file_summaries`

### 注意事项

- 子代理返回必须是合法 JSON，解析失败则要求其重试
- 合并后条目过多时可截断到最重要的 ~200 个文件
- 超大项目单次 MCP 调用可能超时，可在 opencode 配置 `experimental.mcp_timeout`（毫秒）调大

## MCP Server 模式

若宿主支持 MCP，可直接注册（tools 与 CLI 参数一致）：

```json
{
  "mcpServers": {
    "code-superman": { "command": "code-superman", "args": ["serve"] }
  }
}
```

可用 tools：`analyze`（path / strength / xmind_out / file_summaries）、`get_file_symbols`（path / file）、`export_xmind`（path / out / file_summaries）。file_summaries 的值支持 `"一句话"` 或 `{"summary": "...", "details": "..."}` 双字段。
