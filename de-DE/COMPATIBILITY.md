# Ladeanleitung für jedes KI-Tool (Kompatibilitätsleitfaden)

Die Prompts im Verzeichnis `prompts/` sind **unabhängig** vom jeweiligen KI-Tool – jedes auf einem Sprachmodell basierende Codierungswerkzeug kann sie verwenden. Der Unterschied liegt nur in der **Ladeweise**: Hauptdateiname, Ablageort, Ladebefehl. Dieses Dokument ist im Grunde „ein Ladeanleitungs-Dokument" – ein neues Tool lässt sich einfach durch eine zusätzliche Zeile hier ergänzen.

> Hinweis: Alle Tools lassen sich über die `mentor` CLI mit einem Klick installieren (schreibt die Dateien automatisch an die richtige Stelle für das jeweilige Tool), siehe Ende des Dokuments.

## Schnellübersicht

| Tool | Hauptdatei (Agent-Rolle) | Ablageort | Ladeweise | Weitere Module (security/style/workflow) |
|------|--------------------------|-----------|-----------|-------------------------------------------|
| opencode | `AGENTS.md` | Projektstammverzeichnis | Automatisch geladen | Ebenso einzeln laden, z. B. `@security.md` |
| Claude Code | `CLAUDE.md` oder `AGENTS.md` | Projektstammverzeichnis | Automatisch geladen | Im Haupt-Prompt mit `@security.md` referenzieren oder in einem Unterverzeichnis ablegen und bei Bedarf laden |
| OpenAI Codex | `AGENTS.md` | Projektstammverzeichnis | Automatisch geladen | Im Haupt-Prompt mit `@security.md` referenzieren |
| Cursor | `AGENTS.md` | `.cursor/rules/` | Automatisch geladen (Regeln können per glob-Muster einen Dateibereich abdecken) | Gleichnamige Dateien in dasselbe Verzeichnis legen |
| Gemini CLI | `GEMINI.md` | Projektstammverzeichnis | Automatisch geladen | Umbenennen und gemeinsam ablegen oder mit `@` referenzieren |
| Google Jules | `JULES.md` | Projektstammverzeichnis | Automatisch geladen | Wie oben |
| Aider | `CONVENTIONS.md` | Projektstammverzeichnis | Automatisch geladen | Inhalte zusammenführen oder in Einzeldateien referenzieren |
| Windsurf | `.windsurfrules` | Projektstammverzeichnis | Automatisch geladen | Wie oben |
| GitHub Copilot Agent | `AGENTS.md` | Projektstammverzeichnis | Automatisch geladen | Mit `@security.md` referenzieren |
| Jeder MCP-Client | über `mentor-mcp` | stdio (`node mcp/dist/index.js`) | Automatisch (resources + tools) | Alle Module werden als MCP-Ressourcen `mentor://prompts/{lang}/{module}` bereitgestellt |

## Ausführliche Anleitungen je Tool

### opencode
1. `prompts/AGENTS.md` in das Projektstammverzeichnis kopieren
2. opencode lädt AGENTS.md in jeder Sitzung automatisch — kein manueller Schritt nötig
3. Bei Bedarf Sicherheits-/Stil-/Workflow-Vorgaben laden: `@security.md`, `@style.md`, `@workflow.md`
4. Bei langlaufenden Projekten: Regeln in AGENTS.md verankern; nach einer Unterbrechung mit `opencode --continue` fortfahren

### Claude Code
1. `prompts/AGENTS.md` kopieren → in `CLAUDE.md` umbenennen (oder als `AGENTS.md` belassen, neuere Versionen erkennen es automatisch)
2. Ins Projektstammverzeichnis legen; wird in jeder Sitzung automatisch geladen
3. Weitere Module in `CLAUDE.md` mit `@security.md` referenzieren oder direkt zusammenführen
4. Eine `CLAUDE.md` in einem Unterverzeichnis wird beim Betreten des jeweiligen Verzeichnisses bei Bedarf geladen

### OpenAI Codex
1. `prompts/AGENTS.md` in das Projektstammverzeichnis kopieren (Codex lädt die `AGENTS.md` im Stammverzeichnis automatisch)
2. Weitere Module in `AGENTS.md` mit `@security.md` referenzieren
3. Nach einer Unterbrechung mit `codex --resume` (bzw. `codex exec --resume`) fortfahren

### Cursor
1. `prompts/AGENTS.md` in das Verzeichnis `.cursor/rules/` kopieren (der Agent lädt die Regeln automatisch)
2. Soll die Regel nur für bestimmte Dateibereiche gelten, kann sie ins `.mdc`-Format umgewandelt und im Frontmatter per `globs` abgeglichen werden
3. Die übrigen Module als gleichnamige Dateien ebenfalls in `.cursor/rules/` ablegen

### Gemini CLI
1. `prompts/AGENTS.md` kopieren → in `GEMINI.md` umbenennen, ins Projektstammverzeichnis legen; wird automatisch geladen
2. Weitere Module in `GEMINI.md` zusammenführen oder bei Bedarf mit `@` referenzieren

### Google Jules
1. `prompts/AGENTS.md` kopieren → in `JULES.md` umbenennen, ins Projektstammverzeichnis legen; wird automatisch geladen

### Aider
1. `prompts/AGENTS.md` kopieren → in `CONVENTIONS.md` umbenennen, ins Projektstammverzeichnis legen; wird in Bearbeitungssitzungen automatisch geladen

### Windsurf
1. `prompts/AGENTS.md` kopieren → in `.windsurfrules` umbenennen, ins Projektstammverzeichnis legen; wird automatisch geladen

### GitHub Copilot Agent
1. `prompts/AGENTS.md` in das Projektstammverzeichnis kopieren; wird automatisch geladen; weitere Module mit `@security.md` referenzieren

### MCP (Model Context Protocol)
1. Server bauen: `cd mcp && npm install && npm run build`
2. MCP-Client auf `node <repo>/mcp/dist/index.js` verweisen
3. Der Server stellt die Prompts als Ressourcen (`mentor://prompts/{lang}/{module}`) und Tools (`install`, `detect_tool`, `list_languages`, `list_modules`, `generate_resource_estimate`) bereit
4. Details siehe `mcp/README.md`

## Mit der mentor CLI in einem Schritt installieren

```bash
mentor install          # Interaktiv: Sprache wählen → Modul wählen (Standard: agent) → Tool automatisch erkennen/auswählen
mentor install --lang de-DE --modules agent,security --cli claude-code
mentor add workflow     # Modul ergänzen
mentor list             # Installierte Module anzeigen
```

`mentor` schreibt die Dateien gemäß den obigen Tabellenregeln automatisch unter dem vom jeweiligen Tool verlangten Dateinamen und an die richtige Stelle (Claude Code → `CLAUDE.md`, Cursor → `.cursor/rules/`, übrige → `AGENTS.md` usw.).

## Vollständiger Prompt

Wenn keine Aufteilung in Module nötig ist, kann direkt `prompts/Vollständiger-Mentor-Prompt.md` verwendet werden (zusammengeführter Komplett-Prompt aus allen vier Modulen, einmaliges Laden).
