# Code Superman 使用教程

> 当前版本 v0.2.0 · 3 个工具 / 6 个参数 · 完全离线免费 · 无需任何 API Key

---

## 一、它是什么

Code Superman 把任意代码库变成大模型可理解的"白盒"：扫描目录、识别技术栈、定位入口点、统计代码度量、解析 import 依赖关系、生成符号大纲。它**本身不做 AI 解读**——调用它的大模型拿到结构化报告后自己完成深度理解。

三种使用形态：

| 形态 | 适合场景 |
|---|---|
| **MCP Server** | opencode / Claude Code 等 MCP 客户端，模型自主调用（推荐） |
| **CLI** | 手动在终端快速查看，或非 MCP 环境的 agent |
| **Agent Skill** | 给 agent 一份"使用说明书"，教它何时如何调 CLI |

---

## 二、安装与更新

需要 Rust 工具链（`rustup` 安装）：

```bash
cd D:\Users\username\Desktop\code_superman
cargo install --path crates/cli     # 安装/更新到 cargo bin
code-superman --version             # 应输出 0.2.0
```

⚠️ **如果报"拒绝访问 (os error 5)"**：说明旧版进程还在运行（MCP server 被客户端拉起）。先重启客户端或执行 `Stop-Process -Name code-superman -Force` 再装。

---

## 三、MCP 方式（推荐）

### 注册配置

在 MCP 客户端配置中注册一次。opencode 的全局配置位于 `~/.config/opencode/opencode.jsonc`：

```jsonc
{
  "mcp": {
    "code-superman": {
      "type": "local",
      "command": ["<cargo bin 路径>/code-superman.exe", "serve"],
      "enabled": true
    }
  }
}
```

Claude Code / Cursor 等客户端同理，把 command 换成二进制的绝对路径。

### 生效条件

**配置只在启动时加载**。改完配置或更新二进制后，必须退出并重启客户端。

### 验证

新会话中直接对模型说：

```
用 code-superman 分析一下当前项目
```

模型会自动调用 `analyze` 工具。也可以明确指定强度：

```
详细分析 D:\some\project，用 detailed 强度
```

---

## 四、三个工具详解

### 1️⃣ `analyze` —— 项目全貌分析（核心工具）

一次调用完成：目录扫描 → 语言占比 → 技术栈识别 → 入口点定位 → **Token 用量估算** → 代码度量 → 依赖图核心模块排序，可选同时导出思维导图。

| 参数 | 类型 | 说明 |
|---|---|---|
| `path` | 必填 | 项目根目录绝对路径 |
| `strength` | 可选 | `brief` / `standard`(默认) / `detailed` |
| `xmind_out` | 可选 | 传入路径则同时导出架构 .xmind |
| `file_summaries` | 可选 | `{文件路径: 职责摘要}`，写入导图节点备注 |

**三档强度的内容递进：**

| 强度 | 输出内容 | 上限 |
|---|---|---|
| ⚡ `brief` | 语言占比、技术栈、入口点、Token 估算 | 3K 字符 |
| ⚖️ `standard` | + 目录树(深度2)、最大文件 Top20、核心文件 Top15（按入度=被依赖次数）、超大文件警告 | 20K |
| 🔬 `detailed` | + 全量依赖边列表、全量度量 | 50K |

**选档经验：**

- 大型陌生仓库第一次看 → `brief` 快速摸底，参考 Token 估算判断精读成本
- 日常了解项目结构 → `standard`
- 要重构/画架构图/找所有依赖关系 → `detailed`

> 💡 **交互约定**：MCP 模式下，若你没有指定强度或没说要不要导图，模型应先弹选项询问你，再调用工具。

### 2️⃣ `get_file_symbols` —— 单文件大纲

| 参数 | 说明 |
|---|---|
| `path` | 项目根目录 |
| `file` | 相对路径，用 `/` 分隔，如 `src/main.rs` |

返回该文件的函数/类/结构体清单（含行号范围和签名）+ import 列表。支持 Rust / Python / TS / TSX / JS / Go / Java / C / C++ / C# / PHP / Ruby 共 12 种语言，其他语言返回空大纲。

### 3️⃣ `export_xmind` —— 架构思维导图

导出的不是单纯文件树，而是**五分支架构图**：

```
🦸 项目名
├── 📊 总览      技术栈、语言占比、统计、Token 估算
├── 🚪 入口点    入口文件及识别原因
├── ⭐ 核心模块   Top15 按被依赖次数排序（如「scanner.rs（被 3 个文件依赖）」）
├── ⚠️ 警告      超大文件等
└── 📁 目录结构   完整文件树，节点标注「语言·行数·TODO·被依赖数·符号构成」
```

| 参数 | 说明 |
|---|---|
| `path` | 项目根目录 |
| `out` | 可选，默认 `<path>/architecture.xmind` |
| `file_summaries` | 可选，`{文件路径: 一句话职责}`，写入对应节点备注 |

> 💡 **两步出图法**（获得带"每个文件在做什么"的导图）：先 `analyze` 理解项目 → 为核心文件各写一句职责 → 调用时通过 `file_summaries` 传入。不传摘要时导图只有静态标注（语言/行数/符号构成等）。

---

## 五、典型工作流

### 场景 A：接手一个完全陌生的项目

```
第1步  analyze <项目路径> --detail standard
       ↓ 模型得知：什么技术栈、多大、入口在哪、哪些是核心文件
第2步  对入度最高的 1-2 个文件：
       get_file_symbols(<路径>, <核心文件>)
       ↓ 得到函数清单和行号范围
第3步  模型用普通读文件能力精读关键函数
       ↓ 输出完整的项目理解报告
```

### 场景 B：重构前评估

```
analyze <路径> --detail detailed
→ 全量依赖边暴露耦合点；"警告"段落直接列出超 2000 行建议拆分的文件
```

### 场景 C：给领导/同事出架构图

```
analyze <路径> --detail brief --xmind D:\out\架构图.xmind
→ 秒级得到可双击打开的 .xmind 文件
```

### 场景 D：CI/脚本中手动跑（CLI）

```bash
code-superman analyze . --detail brief                    # 提交前快览
code-superman analyze D:\proj --detail detailed           # 全量报告
code-superman symbols D:\proj src/main.rs                 # 单文件大纲
code-superman xmind D:\proj -o D:\out\arch.xmind          # 单独导出
```

---

## 六、Skill 形态（可选）

把 `skill/code-superman/SKILL.md` 复制到技能目录：

- opencode：`.opencode/skills/code-superman/SKILL.md`（项目级）或 `~/.config/opencode/skills/`（全局）
- Claude Code：`~/.claude/skills/code-superman/SKILL.md`

适合**不支持 MCP 但能执行 shell 命令**的 agent 环境。SKILL.md 内置了推荐工作流，模型读到后知道何时、以什么顺序调用哪个命令。

---

## 七、故障排查

| 现象 | 原因与解决 |
|---|---|
| 会话里看不到 code-superman 工具 | 配置启动时加载——**重启 opencode** |
| 更新二进制后行为没变 | 同上，且旧进程锁文件需先结束再 install |
| 输出末尾有"输出已截断" | 改用更高强度，或缩小 path 范围（分析子目录） |
| symbols 返回空大纲 | 语言不在 12 种支持列表内，或不支持 tree-sitter 解析 |
| 依赖边个别连错 | 启发式解析的已知限制（如跨目录同名文件），不影响大局判断 |
| "无法从扩展名识别语言" | file 参数缺扩展名或非常规后缀 |

---

## 八、一句话总结

**重启客户端后，对模型说"帮我理解 XX 项目"就够了**——模型自己会选择合适的工具和强度；需要更细控制时，用 `strength` 三档调节，需要导图就带上 `xmind_out`。
