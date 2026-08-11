# AI Model Mentor

> **Turn your AI coding assistant into a cautious 10-year full-stack mentor — pure prompts, zero dependencies.**

[中文版本](./README.md) ｜ English (current page)

---

## What is this?

A **pure-prompt framework** that shapes your AI coding assistant into a **full-stack architect & development mentor with 10 years of experience**, built for coding beginners with zero foundation.

It forces the AI to follow a set of "iron rules" — making *Security First, Transparent Logic, Documentation First, Token Efficiency, and Phased Implementation* its default behavior. The result: AI that doesn't just *write code*, but writes **safe, maintainable, documented** code.

> ⚠️ Currently supports: **Xiaomi MIMO CLI**. Optimized builds for other products (Claude Code, Cursor, etc.) are planned — leave a comment if you need one.

## Core Modules

| Module | File | Purpose |
|--------|------|---------|
| 🧑‍🏫 Mentor Role | `AGENTS.md` | Architect-mentor persona + 6 iron rules + security self-check checklist |
| 🛡️ Security Spec | `security.md` | 8 security domains: secrets / input validation / database / XSS / file system / external requests / error handling / performance |
| 🎨 Interaction Style | `style.md` | Life analogies, phase tags, confirm-before-act, progressive complexity |
| 📋 Dev Workflow | `workflow.md` | Docs system / frontend mapping protocol / deploy & rollback / test loop / version anchors |

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
cp AGENTS_EN.md AGENTS.md

# 2. (Recommended) Add security / style / workflow specs too
cp security_EN.md security.md
cp style_EN.md style.md
cp workflow_EN.md workflow.md
```

3. Launch Xiaomi MIMO and say:

> "I'm a complete beginner. Here is my Project Requirement Specification: project name ____, core goals ____, user roles ____, core workflows ____, data to persist ____. Start from Phase 0: Environment Setup & Tech Stack Selection and guide me step by step."

The AI will advance through "Design → Core Logic → UI → Testing", waiting for your confirmation at every stage.

## File Structure

```
AI_Model_Development_Mentor/
├── README.md            # Bilingual landing page
├── README_ZH.md         # Chinese full version entry
├── README_EN.md         # English full version (this file)
├── AGENTS_ZH.md         # Mentor role (ZH) ★ core, must-use
├── AGENTS_EN.md         # Mentor role (EN)
├── security_ZH.md       # Security spec (ZH)
├── security_EN.md       # Security spec (EN)
├── style_ZH.md          # Interaction style (ZH)
├── style_EN.md          # Interaction style (EN)
├── workflow_ZH.md       # Dev workflow (ZH)
├── workflow_EN.md       # Dev workflow (EN)
└── LICENSE              # MIT License
```

## FAQ

**Q: Do I need all 4 modules?**
A: No. `AGENTS.md` is the only must-have. Add `security.md` for stronger guardrails, `style.md` for a friendlier conversation experience.

**Q: Does this work with other AI products?**
A: Only Xiaomi MIMO is supported for now. Optimized versions for other products are in progress — leave a comment to tell us what you need.

## License

[MIT License](./LICENSE) © 2026 guapimm
