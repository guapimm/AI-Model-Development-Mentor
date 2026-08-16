# AI Model Mentor

> **Turn your AI coding assistant into a cautious 10-year full-stack mentor — pure prompts, zero dependencies.**
> Load this framework at the early stage of your project to make your AI act more cautiously and produce code that is easier to maintain. Pure prompt implementation, no dependencies.

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](./LICENSE)
[![Release](https://img.shields.io/github/v/release/guapimm/AI-Model-Development-Mentor?include_prereleases)](https://github.com/guapimm/AI-Model-Development-Mentor/releases)
[![Languages](https://img.shields.io/badge/languages-9-green.svg)](#-language--语言)
[![Stars](https://img.shields.io/github/stars/guapimm/AI-Model-Development-Mentor?style=social)](https://github.com/guapimm/AI-Model-Development-Mentor/stargazers)

> **中文一句话介绍**：一个「纯提示词」框架，把你的 AI 编程助手变成一个有 10 年经验、谨慎靠谱的全栈导师，专为编程零基础小白设计。加载后 AI 会默认遵守 6 条铁律（代码即文档、安全前置、零破坏性改动、分步执行、模块化隔离、性能与资源前置），产出安全、可维护、带文档的代码。

## 🌍 Language / 语言

[English](./README.md) · [中文](./zh-CN/README.md) · [日本語](./ja-JP/README.md) · [한국어](./ko-KR/README.md) · [Español](./es-ES/README.md) · [Français](./fr-FR/README.md) · [Deutsch](./de-DE/README.md) · [Português](./pt-BR/README.md) · [Русский](./ru-RU/README.md)

---

## What is this?

A **pure-prompt framework** that shapes your AI coding assistant into a **full-stack architect & development mentor with 10 years of experience**, built for coding beginners with zero foundation.

It forces the AI to follow a set of "iron rules" — making *Security First, Transparent Logic, Documentation First, Token Efficiency, Phased Implementation, and Resource Control* its default behavior. The result: AI that doesn't just *write code*, but writes **safe, maintainable, documented** code.

> ✅ **Tool-agnostic**: works with opencode, Claude Code, Codex, Cursor, Gemini CLI, Google Jules, Aider, Windsurf, GitHub Copilot Agent — only the loading method differs (see the table below).

## Core Modules

| Module | File | Purpose |
|--------|------|---------|
| 🧑‍🏫 Mentor Role | `AGENTS.md` | Architect-mentor persona + 6 iron rules + security & performance self-check checklist |
| 🛡️ Security Spec | `security.md` | 8 security domains: secrets / input validation / database / XSS / file system / external requests / error handling / performance & resource |
| 🎨 Interaction Style | `style.md` | Life analogies, phase tags, confirm-before-act, progressive complexity |
| 📋 Dev Workflow | `workflow.md` | Docs system / resource estimation / database design / frontend mapping protocol / deploy & rollback / test loop / version anchors |

### The 6 Iron Rules

1. **Code as Documentation** — all code carries comments explaining the "why"
2. **Security Upfront** — no hard-coded secrets, strict input validation, parameterized queries, XSS prevention
3. **Zero-Destructive Changes** — analyze dependencies first, tag edits as [Mandatory] / [Optional]
4. **Phased Execution** — never more than 300 lines per output, wait for confirmation at each step
5. **Modular Isolation** — max 500 lines per file, reserve extension interfaces
6. **Performance & Resource Upfront** — output index design with database schema, default pagination for list queries, three-tier resource estimation (memory/disk/CPU) at project start, release mechanisms for large memory operations

## 🛠️ Supported AI Tools

The prompt content is **tool-agnostic** — every tool only differs in *how* it loads files (filename, location, command). See [zh-CN/COMPATIBILITY.md](./zh-CN/COMPATIBILITY.md) for the full per-tool guide (also available in all 9 languages).

| Tool | Main file (agent) | Location | Loading |
|------|------------------|----------|---------|
| opencode | `AGENTS.md` | project root | auto |
| Claude Code | `CLAUDE.md` / `AGENTS.md` | project root | auto |
| OpenAI Codex | `AGENTS.md` | project root | auto |
| Cursor | `AGENTS.md` | `.cursor/rules/` | auto |
| Gemini CLI | `GEMINI.md` | project root | auto |
| Google Jules | `JULES.md` | project root | auto |
| Aider | `CONVENTIONS.md` | project root | auto |
| Windsurf | `.windsurfrules` | project root | auto |
| GitHub Copilot Agent | `AGENTS.md` | project root | auto |
| **Any MCP client** | via `mentor-mcp` | stdio (`node mcp/dist/index.js`) | auto (resources + tools) |

> 💡 **One-click install:** the `mentor` CLI writes files to the correct name/location for any of these tools automatically (see `cli/`).

## 🧩 MCP Server

For tools that speak **MCP (Model Context Protocol)**, a TypeScript server (`mcp/`) exposes the mentor framework as standard resources and tools — no manual file copying:

- **Resources** `mentor://prompts/{lang}/{module}` — read any prompt (9 languages × 5 modules) on demand.
- **Tools** — `install`, `detect_tool`, `list_languages`, `list_modules`, `generate_resource_estimate`.

Run it locally (clone the repo, then):

```bash
cd mcp && npm install && npm run build && node dist/index.js
```

Then point your MCP client at `node <repo>/mcp/dist/index.js`. See [mcp/README.md](./mcp/README.md) for details.

## ⬇️ Install

Two ways, same result:

```bash
# Option A — binary (zero dependencies, cross-platform)
# Go: mentor-*  |  Rust: mentor-rust-*  (both on GitHub Releases), then:
mentor install

# Option B — manual
# copy files from <lang>/prompts/ per your language's COMPATIBILITY.md
cp en-US/prompts/AGENTS.md AGENTS.md
```

Both support: interactive wizard (language → modules → tool), non-interactive flags
(`--lang zh-CN --modules agent,security --cli claude-code --dir ./proj`), and `add` / `remove` / `list` / `detect` / `pack`.

## Quick Start (3 steps)

```bash
# 1. (Optional) Bootstrap a new project scaffold: requirements doc + .env.example + docs/
mentor init --name "my-app" --goal "a personal finance tracker"

# 2. Copy the mentor role into your project (rename it)
cp en-US/prompts/AGENTS.md AGENTS.md

# 3. (Recommended) Add security / style / workflow specs too
cp en-US/prompts/security.md security.md
cp en-US/prompts/style.md style.md
cp en-US/prompts/workflow.md workflow.md
```

4. Launch your AI tool and say:

> "I'm a complete beginner. Here is my Project Requirement Specification: project name ____, core goals ____, user roles ____, core workflows ____, data to persist ____. Start from Phase 0: Environment Setup & Tech Stack Selection + Resource Estimation and guide me step by step."

The AI will advance through "Design → Core Logic → UI → Testing", waiting for your confirmation at every stage.

## 📐 Fill-in Templates

Pre-made tables referenced by the workflow module, ready to copy into your project:

- `templates/{lang}/resource_estimate_template.md` — the Phase 0 《Project Resource Estimate》 table.
- `templates/{lang}/ui_mapping_template.md` — the frontend UI/element + event mapping table.

## File Structure

```
AI_Model_Development_Mentor/
├── README.md            # English landing page + language switcher
├── LICENSE              # Apache-2.0 License
├── cli/                 # mentor CLI (Go)
├── rust/                # mentor CLI (Rust, zero-dependency mirror)
├── mcp/                 # mentor-mcp server (TypeScript, MCP resources + tools)
├── templates/           # fill-in templates (resource estimate / UI mapping)
├── zh-CN/  en-US/  ja-JP/  ko-KR/  es-ES/  fr-FR/  de-DE/  pt-BR/  ru-RU/
└── <lang>/
    ├── README.md        # language entry + usage guide
    ├── COMPATIBILITY.md # per-tool loading instructions (the "one adapter file")
    └── prompts/         # tool-agnostic content (per language)
        ├── AGENTS.md    # mentor role ★ required
        ├── security.md  # security spec
        ├── style.md     # interaction style
        ├── workflow.md  # dev workflow
        └── <full>.md    # consolidated one-shot prompt
```

> 📦 New tools are supported by adding a row in each language's `COMPATIBILITY.md` — no per-tool directories needed.

## 🚀 Releasing (maintainers)

Tag a version to trigger the CI pipeline (`.github/workflows/release.yml`):

```bash
git tag v0.1.0 && git push origin v0.1.0
```

The workflow builds both implementations — Go binaries (windows/linux/darwin × amd64/arm64) and Rust binaries (mentor-rust-*) — and uploads them to the GitHub Release. No secrets or tokens required.

## FAQ

**Q: Do I need all 4 modules?**
A: No. `AGENTS.md` is the only must-have. Add `security.md` for stronger guardrails, `style.md` for a friendlier conversation experience.

**Q: Does this work with other AI products?**
A: Yes. The prompt content is tool-agnostic — every tool just loads it differently. See the table above or `COMPATIBILITY.md` for each tool's loading guide.

**Q: Is this translated by machine?**
A: All language versions are reviewed translations of the same Chinese original. If you spot an issue, feel free to open an issue or PR.

## License

[Apache-2.0](./LICENSE) © 2026 guapimm
