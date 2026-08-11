# AI Model Mentor (English)

> **Turn your AI coding assistant into a cautious 10-year full-stack mentor — pure prompts, zero dependencies.**

[中文版本](../README.md) ｜ English (current page)

---

## What is this?

A **pure-prompt framework** that shapes your AI coding assistant into a **full-stack architect & development mentor with 10 years of experience**, built for coding beginners with zero foundation.

It forces the AI to follow a set of "iron rules" — making *Security First, Transparent Logic, Documentation First, Token Efficiency, and Phased Implementation* its default behavior. The result: AI that doesn't just *write code*, but writes **safe, maintainable, documented** code.

> ⚠️ Currently supports: **Xiaomi MIMO CLI**. Optimized builds for other products (Claude Code, Cursor, etc.) are planned — leave a comment if you need one.

## Core Modules (Xiaomi MIMO build)

| Module | File | Purpose |
|--------|------|---------|
| 🧑‍🏫 Mentor Role | [AGENTS.md](./xiaomi-mimo/AGENTS.md) | Architect-mentor persona + 6 iron rules + security self-check checklist ★ core, must-use |
| 🛡️ Security Spec | [security.md](./xiaomi-mimo/security.md) | 8 security domains: secrets / input validation / database / XSS / file system / external requests / error handling / performance |
| 🎨 Interaction Style | [style.md](./xiaomi-mimo/style.md) | Life analogies, phase tags, confirm-before-act, progressive complexity |
| 📋 Dev Workflow | [workflow.md](./xiaomi-mimo/workflow.md) | Docs system / frontend mapping protocol / deploy & rollback / test loop / version anchors |

### The 6 Iron Rules

1. **Code as Documentation** — all code carries comments explaining the "why"
2. **Security Upfront** — no hard-coded secrets, strict input validation, parameterized queries, XSS prevention
3. **Zero-Destructive Changes** — analyze dependencies first, tag edits as [Mandatory] / [Optional]
4. **Phased Execution** — never more than 300 lines per output, wait for confirmation at each step
5. **Modular Isolation** — max 500 lines per file, reserve extension interfaces
6. **Token Efficiency** — generate a context summary + resume token after every conversation

## Quick Start (3 steps)

```bash
# 1. Copy the mentor role into your project (rename it)
cp xiaomi-mimo/AGENTS.md AGENTS.md

# 2. (Recommended) Add security / style / workflow specs too
cp xiaomi-mimo/security.md security.md
cp xiaomi-mimo/style.md style.md
cp xiaomi-mimo/workflow.md workflow.md
```

3. Launch Xiaomi MIMO and say:

> "I'm a complete beginner. Here is my Project Requirement Specification: project name ____, core goals ____, user roles ____, core workflows ____, data to persist ____. Start from Phase 0: Environment Setup & Tech Stack Selection and guide me step by step."

The AI will advance through "Design → Core Logic → UI → Testing", waiting for your confirmation at every stage.

## File Structure

```
AI_Model_Development_Mentor/
├── README.md            # Bilingual landing page
├── LICENSE              # MIT License
├── zh-CN/               # Chinese
│   ├── README.md        # Chinese entry
│   └── xiaomi-mimo/     # Xiaomi MIMO build
│       ├── AGENTS.md    # Mentor role (ZH)
│       ├── security.md  # Security spec (ZH)
│       ├── style.md     # Interaction style (ZH)
│       └── workflow.md  # Dev workflow (ZH)
└── en-US/               # English
    ├── README.md        # English entry (this file)
    └── xiaomi-mimo/     # Xiaomi MIMO build
        ├── AGENTS.md    # Mentor role (EN)
        ├── security.md  # Security spec (EN)
        ├── style.md     # Interaction style (EN)
        └── workflow.md  # Dev workflow (EN)
```

> 📦 New product builds are added as sibling directories under each language dir, e.g. `zh-CN/claude-code/`, `en-US/cursor/`.

## FAQ

**Q: Do I need all 4 modules?**
A: No. `AGENTS.md` is the only must-have. Add `security.md` for stronger guardrails, `style.md` for a friendlier conversation experience.

**Q: Does this work with other AI products?**
A: Only Xiaomi MIMO is supported for now. Optimized versions for other products are in progress — leave a comment to tell us what you need.

## License

[MIT License](../LICENSE) © 2026 guapimm
