# AI Model Mentor

> **Turn your AI coding assistant into a cautious 10-year full-stack mentor — pure prompts, zero dependencies.**
> Load this framework at the early stage of your project to make your AI act more cautiously and produce code that is easier to maintain. Pure prompt implementation, no dependencies.

## 🌍 Language / 语言

[English](./README.md) · [中文](./zh-CN/README.md) · [日本語](./ja-JP/README.md) · [한국어](./ko-KR/README.md) · [Español](./es-ES/README.md) · [Français](./fr-FR/README.md) · [Deutsch](./de-DE/README.md) · [Português](./pt-BR/README.md) · [Русский](./ru-RU/README.md)

---

## What is this?

A **pure-prompt framework** that shapes your AI coding assistant into a **full-stack architect & development mentor with 10 years of experience**, built for coding beginners with zero foundation.

It forces the AI to follow a set of "iron rules" — making *Security First, Transparent Logic, Documentation First, Token Efficiency, Phased Implementation, and Resource Control* its default behavior. The result: AI that doesn't just *write code*, but writes **safe, maintainable, documented** code.

> ⚠️ Currently supports: **Xiaomi MIMO CLI**. Optimized builds for other products (Claude Code, Cursor, etc.) are planned — leave a comment if you need one.

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
| Xiaomi MIMO | `AGENTS.md` | project root | manual `/skill AGENTS.md` |
| Claude Code | `CLAUDE.md` / `AGENTS.md` | project root | auto |
| OpenAI Codex | `AGENTS.md` | project root | auto |
| Cursor | `AGENTS.md` | `.cursor/rules/` | auto |
| Gemini CLI | `GEMINI.md` | project root | auto |
| Google Jules | `JULES.md` | project root | auto |
| Aider | `CONVENTIONS.md` | project root | auto |
| Windsurf | `.windsurfrules` | project root | auto |
| GitHub Copilot Agent | `AGENTS.md` | project root | auto |

> 💡 **One-click install:** the `mentor` CLI writes files to the correct name/location for any of these tools automatically (see `cli/`).

## ⬇️ Install

Three ways, same result:

```bash
# Option A — Go binary (zero dependencies, cross-platform)
# download mentor from GitHub Releases, then:
mentor install

# Option B — npm
npm i -g ai-model-mentor
ai-mentor install

# Option C — manual
# copy files from <lang>/prompts/ per your language's COMPATIBILITY.md
cp en-US/prompts/AGENTS.md AGENTS.md
```

All three support: interactive wizard (language → modules → tool), non-interactive flags
(`--lang zh-CN --modules agent,security --cli claude-code --dir ./proj`), and `add` / `remove` / `list` / `detect` / `pack`.

## Quick Start (3 steps)

```bash
# 1. Copy the mentor role into your project (rename it)
cp en-US/prompts/AGENTS.md AGENTS.md

# 2. (Recommended) Add security / style / workflow specs too
cp en-US/prompts/security.md security.md
cp en-US/prompts/style.md style.md
cp en-US/prompts/workflow.md workflow.md
```

3. Launch Xiaomi MIMO and say:

> "I'm a complete beginner. Here is my Project Requirement Specification: project name ____, core goals ____, user roles ____, core workflows ____, data to persist ____. Start from Phase 0: Environment Setup & Tech Stack Selection + Resource Estimation and guide me step by step."

The AI will advance through "Design → Core Logic → UI → Testing", waiting for your confirmation at every stage.

## File Structure

```
AI_Model_Development_Mentor/
├── README.md            # English landing page + language switcher
├── LICENSE              # MIT License
├── cli/                 # mentor CLI (Go, single binary) — one-click install
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

> 📦 New product builds are added as sibling directories under each language dir, e.g. `zh-CN/claude-code/`, `en-US/cursor/`.

## 🚀 Releasing (maintainers)

Tag a version to trigger the CI pipeline (`.github/workflows/release.yml`):

```bash
git tag v0.1.0 && git push origin v0.1.0
```

The workflow builds the Go binaries (windows/linux/darwin × amd64/arm64), uploads them to the GitHub Release, then publishes the `ai-model-mentor` npm package. Requires the `NPM_TOKEN` secret in repo settings (see [npm access tokens](https://docs.npmjs.com/creating-and-viewing-access-tokens)).

## FAQ

**Q: Do I need all 4 modules?**
A: No. `AGENTS.md` is the only must-have. Add `security.md` for stronger guardrails, `style.md` for a friendlier conversation experience.

**Q: Does this work with other AI products?**
A: Only Xiaomi MIMO is supported for now. Optimized versions for other products are in progress — leave a comment to tell us what you need.

**Q: Is this translated by machine?**
A: All language versions are reviewed translations of the same Chinese original. If you spot an issue, feel free to open an issue or PR.

## License

[MIT License](./LICENSE) © 2026 guapimm
