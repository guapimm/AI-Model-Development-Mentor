# AI Model Mentor (English)

English entry page. The full English version lives at the repo root [README.md](../README.md), which also hosts the 🌍 language switcher. Chinese: [中文](../zh-CN/README.md).

## English Modules (Xiaomi MIMO build)

| Module | File | Purpose |
|--------|------|---------|
| 🧑‍🏫 Mentor Role | [AGENTS.md](./xiaomi-mimo/AGENTS.md) | Architect-mentor persona + 6 iron rules + security self-check checklist ★ core, must-use |
| 🛡️ Security Spec | [security.md](./xiaomi-mimo/security.md) | 8 security domains: secrets / input validation / database / XSS / file system / external requests / error handling / performance |
| 🎨 Interaction Style | [style.md](./xiaomi-mimo/style.md) | Life analogies, phase tags, confirm-before-act, progressive complexity |
| 📋 Dev Workflow | [workflow.md](./xiaomi-mimo/workflow.md) | Docs system / frontend mapping protocol / deploy & rollback / test loop / version anchors |

## Quick Start (3 steps)

```bash
# 1. Copy the mentor role into your project (rename it)
cp xiaomi-mimo/AGENTS.md AGENTS.md

# 2. (Recommended) Add security / style / workflow specs too
cp xiaomi-mimo/security.md security.md
cp xiaomi-mimo/style.md style.md
cp xiaomi-mimo/workflow.md workflow.md
```

3. Launch Xiaomi MIMO and provide your Project Requirement Specification (project name, core goals, user roles, core workflows, data to persist). The AI starts from Phase 0: Environment Setup & Tech Stack Selection and advances step by step, waiting for your confirmation.

> 📦 New product builds are added as sibling directories, e.g. `en-US/claude-code/`, `en-US/cursor/`.
