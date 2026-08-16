# Per-AI-Tool Loading Guide (Compatibility)

The prompts under the `prompts/` directory are **tool-agnostic** — any LLM-based coding tool can use them. The only differences are in **how they are loaded**: the main file name, the location, and the load command. This file is a single "loading guide" — adding a new tool only means adding one more line here.

> Tip: every tool can be installed in one step with the `mentor` CLI (it automatically writes the files to the correct location for each tool), see the end of this document.

## Quick Reference Table

| Tool | Main file (agent role) | Location | How to load | Other modules (security/style/workflow) |
|------|---------------------|---------|---------|-----------------------------------|
| opencode | `AGENTS.md` | Project root | Auto | Load them one by one with `@security.md`, etc. |
| Claude Code | `CLAUDE.md` or `AGENTS.md` | Project root | Auto-loaded | Reference with `@security.md` in the main file, or put them in a subdirectory to load on demand |
| OpenAI Codex | `AGENTS.md` | Project root | Auto-loaded | Reference with `@security.md` in the main file |
| Cursor | `AGENTS.md` | `.cursor/rules/` | Auto-loaded (rules can use glob matching to scope which files they apply to) | Put same-name files in the same directory |
| Gemini CLI | `GEMINI.md` | Project root | Auto-loaded | Rename and place them together, or reference them with `@` |
| Google Jules | `JULES.md` | Project root | Auto-loaded | Same as above |
| Aider | `CONVENTIONS.md` | Project root | Auto-loaded | Merge the content, or reference it from separate files |
| Windsurf | `.windsurfrules` | Project root | Auto-loaded | Same as above |
| GitHub Copilot Agent | `AGENTS.md` | Project root | Auto-loaded | Reference with `@security.md` |
| Any MCP client | via `mentor-mcp` | stdio (`node mcp/dist/index.js`) | Auto (resources + tools) | All modules exposed as MCP resources `mentor://prompts/{lang}/{module}` |

## Per-Tool Details

### opencode
1. Copy `prompts/AGENTS.md` to the project root
2. opencode auto-loads AGENTS.md in every session — no manual step needed
3. When you need security/style/workflow, load them on demand with `@security.md`, `@style.md`, `@workflow.md`
4. For long-running projects: keep the rules in AGENTS.md; recover from a disconnect with `opencode --continue`

### Claude Code
1. Copy `prompts/AGENTS.md` → rename it to `CLAUDE.md` (or keep it as `AGENTS.md`; newer versions detect it automatically)
2. Place it in the project root; it is auto-loaded in every session
3. Reference the other modules in `CLAUDE.md` with `@security.md`, or append and merge them directly
4. A `CLAUDE.md` inside a subdirectory is loaded on demand when you enter that directory

### OpenAI Codex
1. Copy `prompts/AGENTS.md` to the project root (Codex auto-loads the root `AGENTS.md`)
2. Reference the other modules in `AGENTS.md` with `@security.md`
3. Resume after a disconnect with `codex --resume` (or `codex exec --resume`)

### Cursor
1. Copy `prompts/AGENTS.md` into the `.cursor/rules/` directory (the Agent auto-loads rules)
2. To scope it to specific files, convert it to `.mdc` format and add `globs` matching in the frontmatter
3. Place the same-name files of the other modules in `.cursor/rules/` as well

### Gemini CLI
1. Copy `prompts/AGENTS.md` → rename it to `GEMINI.md`, place it in the project root; it is auto-loaded
2. Other modules can be merged into `GEMINI.md` or referenced with `@` on demand

### Google Jules
1. Copy `prompts/AGENTS.md` → rename it to `JULES.md`, place it in the project root; it is auto-loaded

### Aider
1. Copy `prompts/AGENTS.md` → rename it to `CONVENTIONS.md`, place it in the project root; it is auto-loaded in editing sessions

### Windsurf
1. Copy `prompts/AGENTS.md` → rename it to `.windsurfrules`, place it in the project root; it is auto-loaded

### GitHub Copilot Agent
1. Copy `prompts/AGENTS.md` to the project root; it is auto-loaded. Reference the other modules with `@security.md`

### MCP (Model Context Protocol)
1. Build the server: `cd mcp && npm install && npm run build`
2. Point your MCP client at `node <repo>/mcp/dist/index.js`
3. The server exposes the prompts as resources (`mentor://prompts/{lang}/{module}`) and tools (`install`, `detect_tool`, `list_languages`, `list_modules`, `generate_resource_estimate`)
4. See `mcp/README.md` for details

## Install with the mentor CLI (one command)

```bash
mentor install          # Interactive: pick language → pick module (default: agent) → auto-detect/select the tool
mentor install --lang zh-CN --modules agent,security --cli claude-code
mentor add workflow     # Append a module
mentor list             # View the installed modules
```

`mentor` automatically writes the files to the filename and location each tool requires, following the rules in the table above (Claude Code → `CLAUDE.md`, Cursor → `.cursor/rules/`, everything else → `AGENTS.md`, etc.).

## Consolidated Prompt

If you don't need to split into modules, you can use `prompts/Complete-Mentor-Prompt.md` directly (the four-module merged version, loaded in one shot).
