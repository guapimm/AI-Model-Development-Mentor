# AI Model Mentor (English)

English entry page. The full English version lives at the repo root [README.md](../README.md), which also hosts the 🌍 language switcher. Chinese: [中文](../zh-CN/README.md).

## English Modules (multi-tool compatible)

| Module | File | Purpose |
|--------|------|---------|
| 🧑‍🏫 Mentor Role | [AGENTS.md](./prompts/AGENTS.md) | Architect-mentor persona + 6 iron rules + security & performance self-check checklist ★ core, must-use |
| 🛡️ Security Spec | [security.md](./prompts/security.md) | 8 security domains: secrets / input validation / database / XSS / file system / external requests / error handling / performance & resources |
| 🎨 Interaction Style | [style.md](./prompts/style.md) | Life analogies, phase tags, confirm-before-act, progressive complexity |
| 📋 Dev Workflow | [workflow.md](./prompts/workflow.md) | Docs system / resource estimation / database design / frontend mapping protocol / deploy & rollback / test loop / version anchors |

## 📦 More Docs

- [COMPATIBILITY.md](./COMPATIBILITY.md) — per-tool loading guide (MIMO / Claude Code / Codex / Cursor, etc.)
- [Complete-Mentor-Prompt.md](./prompts/Complete-Mentor-Prompt.md) — one-shot consolidated mentor prompt (all modules merged)

## 📖 Usage Guide (MIMO CLI)

### Command Cheat-Sheet

| Scenario | Action |
|------|------|
| Everyday development | Enter the project → `/skill AGENTS.md` → talk normally |
| Long-running project | After the first load, use `/dream` to distill the rules into MEMORY.md |
| Unexpected disconnect | Recover with `mimo --continue`; the skill rules are still there |
| Starting a new session on purpose | After running `/new`, remember to `/skill AGENTS.md` again |

### Project File Structure

```
📁 my-project/
├── 📄 AGENTS.md          ← Main prompt
├── 📄 security.md        ← Security spec
├── 📄 workflow.md        ← Workflow spec
├── 📄 style.md           ← Interaction style
└── 📁 src/
```

---

### Scenario Walkthroughs

#### Scenario 1: Everyday coding (load only AGENTS.md)

> You: "Help me write an API that fetches the user list"

Needs loaded: AGENTS.md (auto-loaded, no action needed)

AI will automatically:

- Write code with Chinese comments
- Tick the security checklist before outputting
- Execute in steps (≤300 lines)
- Keep single files ≤500 lines

#### Scenario 2: Writing login/registration endpoints (load AGENTS.md + security.md)

> You: "Help me write the user login feature, following the requirements in security.md"

Needs loaded:

```bash
/skill security.md
```

AI will additionally:

- Store passwords hashed with bcrypt
- Set an expiration time on the JWT token
- Prevent brute-force attacks (login failure limits)
- Prevent SQL injection (parameterized queries)

#### Scenario 3: Bootstrapping a project from scratch (load AGENTS.md + workflow.md)

> You: "I want to build a blog system; follow workflow.md to scaffold the project skeleton"

Needs loaded:

```bash
/skill workflow.md
```

AI will additionally:

- Create docs/architecture.md (tech-stack selection + architecture diagram)
- Create docs/dev_log.md (development log template)
- Create docs/api_interface.md (interface contract template)
- Create docs/SNAPSHOT.md (project snapshot)
- Generate backup.sh and rollback.sh scripts

#### Scenario 4: AI explanations are too jargon-heavy (load style.md)

> You: "Following style.md, explain what JWT is using an everyday-life analogy"

Needs loaded:

```bash
/skill style.md
```

AI will additionally:

- Explain JWT with the "restaurant membership card" analogy
- Add a phase tag [📋 Requirement Analysis]
- Give the conclusion first, then the details
- Offer 2-3 alternative options

#### Scenario 5: Deploying to production (load AGENTS.md + workflow.md)

> You: "Following the deployment spec in workflow.md, help me write the Docker deployment config"

Needs loaded:

```bash
/skill workflow.md
```

AI will additionally:

- Separate dev/prod environment configs
- Generate docker-compose.yml
- Generate health_check.sh
- Remind you of the backup and rollback steps

### ⚠️ When NOT to Load?

| Don't-load case | Reason |
|---------------|------|
| A pure technical question (e.g., "how do I use React useEffect") | AGENTS.md is enough; adding workflow only interferes |
| Tweaking a single CSS style | No need for the security spec or the deployment flow |
| Asking the AI to translate a piece of text | No skill is needed at all |
| Simple refactoring of existing code | AGENTS.md's security checklist already covers it |

### 💡 One-Line Summary

> AGENTS.md is the default skin; the other three are special-effect plugins — turn them on only when needed, keep them off the rest of the time: fewer tokens, cleaner experience.

## Quick Start (3 steps)

```bash
# 1. Copy the mentor role into your project (rename it)
cp prompts/AGENTS.md AGENTS.md

# 2. (Recommended) Add security / style / workflow specs too
cp prompts/security.md security.md
cp prompts/style.md style.md
cp prompts/workflow.md workflow.md
```

3. Launch Xiaomi MIMO and provide your Project Requirement Specification (project name, core goals, user roles, core workflows, data to persist). The AI starts from Phase 0: Environment Setup & Tech Stack Selection and advances step by step, waiting for your confirmation.

> 📦 New product builds are added as sibling directories, e.g. `en-US/claude-code/`, `en-US/cursor/`.
