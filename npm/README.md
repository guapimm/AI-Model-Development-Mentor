# ai-model-mentor

> Turn your AI coding assistant into a cautious 10-year full-stack mentor — pure prompts, zero dependencies.

Install the AI Model Mentor prompt framework into any AI coding CLI (Xiaomi MIMO, Claude Code, OpenAI Codex, Cursor, Gemini CLI, ...) with one command. Pick your language and modules — `AGENTS.md` (agent) is loaded by default.

## Install

```bash
npm i -g ai-model-mentor
```

## Usage

```bash
ai-mentor install                          # interactive wizard: language → modules → tool
ai-mentor install --lang zh-CN --modules agent,security --cli claude-code --dir ./proj
ai-mentor add workflow --lang en-US        # add a module later
ai-mentor list                             # show installed modules
ai-mentor remove style                     # remove a module
ai-mentor detect                           # detect which AI tool the project uses
ai-mentor pack                             # emit a skill directory (for ~/.claude/skills etc.)
```

The CLI writes each module to the correct filename and location for your tool:
MIMO/Codex → `AGENTS.md`, Claude Code → `CLAUDE.md`, Cursor → `.cursor/rules/`, etc.

## Content

The package embeds all 9 languages (zh-CN, en-US, ja-JP, ko-KR, es-ES, fr-FR, de-DE, pt-BR, ru-RU) × 5 modules (agent, security, style, workflow, complete) under `lib/`. You can also copy files directly from there — see the per-language `COMPATIBILITY.md` in the [source repo](https://github.com/guapimm/AI-Model-Development-Mentor) for manual loading instructions.

## License

MIT © 2026 guapimm
