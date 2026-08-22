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

- [COMPATIBILITY.md](./COMPATIBILITY.md) — per-tool loading guide (opencode / Claude Code / Codex / Cursor, etc.)
- [Complete-Mentor-Prompt.md](./prompts/Complete-Mentor-Prompt.md) — one-shot consolidated mentor prompt (all modules merged)

## ⬇️ Install & Use the mentor CLI

**Option A: Go binary (recommended — zero dependencies, cross-platform)**

Download the `mentor` executable for your platform from GitHub Releases (v0.1.0, supports Windows / Linux / macOS), put it on your PATH, then:

```bash
mentor install                        # Interactive wizard: pick a language → pick modules (agent by default) → auto-detect the tool
mentor install --lang zh-CN --modules agent,security --cli claude-code --dir ./proj
mentor add workflow                   # Append a module
mentor list                           # List installed modules
mentor detect                         # Detect which AI tool the project uses
mentor pack                           # Generate a compatible skill directory
```

`mentor` automatically writes the correct file name and location for each tool: opencode/Codex → `AGENTS.md`, Claude Code → `CLAUDE.md`, Cursor → `.cursor/rules/`.

**Option B: Manual copy**

Following the instructions in [COMPATIBILITY.md](./COMPATIBILITY.md), copy the files under `prompts/` to the corresponding locations in your project.

> Supported commands: `install` / `add` / `remove` / `list` / `detect` / `pack`; modules: agent (default) / security / style / workflow / complete; tools: opencode / claude-code / codex / cursor / other.

## 🧩 Connect an IDE via MCP (on-demand prompts)

Full tutorial, rules, and caveats: [mcp/README.md](../mcp/README.md). **Not on npm** — clone and build locally.

```bash
git clone https://github.com/guapimm/AI-Model-Development-Mentor.git
cd AI-Model-Development-Mentor/mcp
npm install          # local deps only, not a registry publish
npm run build
```

Copy [mcp/examples/mcp.json](../mcp/examples/mcp.json), replace the placeholder with **your absolute path**, and add it to Cursor / Claude Code / VS Code. First tool call: `session_start`.

## Usage rules

1. **Load the smallest set that fits the task.** Everyday coding: `AGENTS.md` only. Auth/security: add `security.md`. Greenfield: add `workflow.md`. With MCP: `session_start` + `policy_load(id)` — do not paste all four modules every turn.
2. **Phase 0 before application code.** Requirements + resource estimate first; then Design → Logic → UI → Test, confirm at each step.
3. **Prefer MCP sandbox tools when connected.** `fs_write` / `run_command` enforce line limits, `.env` deny, and phase gates. The IDE’s own Write/Bash bypasses them.
4. **Do not treat the complete dump as source of truth.** `Complete-Mentor-Prompt.md` is an older merge and misses later resource-control rules. Use the four split modules.
5. **Secrets live in environment variables.** Only `.env.example` (names, no values) belongs in git.

## Notes / caveats

- **Two install paths:** GitHub Release `mentor` binary (prompts only, no Node) **or** clone + local `mcp/` build (IDE MCP). There is no `npx @guapimm/mentor-mcp`.
- **MCP config needs absolute paths** and a reload after edits. Windows: `E:/path/to/repo` is fine.
- **Sandbox is a path jail, not a VM.** I/O stays in the workspace; `.env` / `.git` writes are blocked. Docker is optional for `run_command` only.
- **Node ≥ 18** is required only for MCP.
- After `git pull`, rebuild: `cd mcp && npm install && npm run build`.

## 📖 Usage Guide (opencode)

### Command Cheat-Sheet

| Scenario | Action |
|------|------|
| Everyday development | Enter the project → opencode auto-loads AGENTS.md → talk normally |
| Long-running project | Keep the rules in AGENTS.md (update with `/init`) |
| Unexpected disconnect | Recover with `opencode --continue`; the rules are still there |
| Starting a new session on purpose | Just launch `opencode` — AGENTS.md is auto-loaded |

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
@security.md
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
@workflow.md
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
@style.md
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
@workflow.md
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
| Asking the AI to translate a piece of text | No module is needed at all |
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

3. Launch opencode and provide your Project Requirement Specification (project name, core goals, user roles, core workflows, data to persist). The AI starts from Phase 0: Environment Setup & Tech Stack Selection and advances step by step, waiting for your confirmation.

> 📦 New tools are supported by adding a row in [COMPATIBILITY.md](./COMPATIBILITY.md) — no per-tool directories needed.
