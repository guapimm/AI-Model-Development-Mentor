# Guía de carga de cada herramienta de IA (guía de compatibilidad)

El contenido de los prompts de `prompts/` es **independiente de la herramienta de IA**: cualquier herramienta de codificación basada en modelos de lenguaje puede usarlo. La diferencia está únicamente en la **forma de carga**: el nombre del archivo principal, la ubicación y el comando de carga. Este documento es, literalmente, "la guía de carga": cuando añadas una herramienta nueva, solo tienes que añadir una fila aquí.

> Consejo: todas las herramientas se pueden instalar con un solo comando del CLI `mentor` (que escribe automáticamente cada archivo en la ubicación correcta según la herramienta). Ver el final de este documento.

## Tabla de referencia rápida

| Herramienta | Archivo principal (rol de agente) | Ubicación | Forma de carga | Otros módulos (security/style/workflow) |
|------|---------------------|---------|---------|-----------------------------------|
| opencode | `AGENTS.md` | Raíz del proyecto | Carga automática | Igual: cárgalos uno a uno con `@security.md`, etc. |
| Claude Code | `CLAUDE.md` o `AGENTS.md` | Raíz del proyecto | Carga automática | Referéncialos con `@security.md` dentro del archivo principal, o ponlos en un subdirectorio y cárgalos según necesidad |
| OpenAI Codex | `AGENTS.md` | Raíz del proyecto | Carga automática | Referéncialos con `@security.md` dentro del archivo principal |
| Cursor | `AGENTS.md` | `.cursor/rules/` | Carga automática (las rules admiten coincidencia por rango con glob) | Pon los archivos con el mismo nombre en el mismo directorio |
| Gemini CLI | `GEMINI.md` | Raíz del proyecto | Carga automática | Ponlos juntos tras renombrarlos, o referéncialos con `@` |
| Google Jules | `JULES.md` | Raíz del proyecto | Carga automática | Igual que arriba |
| Aider | `CONVENTIONS.md` | Raíz del proyecto | Carga automática | Fusiona el contenido o referéncialos en archivos separados |
| Windsurf | `.windsurfrules` | Raíz del proyecto | Carga automática | Igual que arriba |
| GitHub Copilot Agent | `AGENTS.md` | Raíz del proyecto | Carga automática | Referéncialos con `@security.md` |
| Cualquier cliente MCP | vía `mentor-mcp` | stdio (`node mcp/dist/index.js`) | Automático (resources + tools) | Todos los módulos se exponen como recursos MCP `mentor://prompts/{lang}/{module}` |

## Explicación detallada de cada herramienta

### opencode

1. Copia `prompts/AGENTS.md` a la raíz del proyecto.
2. opencode carga AGENTS.md automáticamente en cada sesión; no se necesita ningún paso manual.
3. Cuando necesites seguridad/estilo/workflow, carga `@security.md`, `@style.md` y `@workflow.md` según necesites.
4. Proyectos de ciclo largo: mantén las reglas en AGENTS.md; si se corta la conexión, retoma con `opencode --continue`.

### Claude Code

1. Copia `prompts/AGENTS.md` → renómbralo como `CLAUDE.md` (o conserva `AGENTS.md`; las versiones recientes lo detectan automáticamente).
2. Ponlo en la raíz del proyecto: se carga automáticamente en cada sesión.
3. Los demás módulos se referencian con `@security.md` dentro de `CLAUDE.md`, o se añaden y fusionan directamente.
4. Un `CLAUDE.md` dentro de un subdirectorio se carga bajo demanda al entrar en ese directorio.

### OpenAI Codex

1. Copia `prompts/AGENTS.md` a la raíz del proyecto (Codex carga automáticamente el `AGENTS.md` de la raíz).
2. Los demás módulos se referencian con `@security.md` dentro de `AGENTS.md`.
3. Para retomar una sesión interrumpida usa `codex --resume` (o `codex exec --resume`).

### Cursor

1. Copia `prompts/AGENTS.md` al directorio `.cursor/rules/` (el Agent carga automáticamente las rules).
2. Si quieres que se aplique a un ámbito de archivos concreto, puedes convertirlo al formato `.mdc` y añadir la coincidencia `globs` en el frontmatter.
3. Pon también los demás módulos (con el mismo nombre) en `.cursor/rules/`.

### Gemini CLI

1. Copia `prompts/AGENTS.md` → renómbralo como `GEMINI.md`, ponlo en la raíz del proyecto: se carga automáticamente.
2. Los demás módulos se pueden fusionar dentro de `GEMINI.md` o referenciarse con `@` según necesidad.

### Google Jules

1. Copia `prompts/AGENTS.md` → renómbralo como `JULES.md`, ponlo en la raíz del proyecto: se carga automáticamente.

### Aider

1. Copia `prompts/AGENTS.md` → renómbralo como `CONVENTIONS.md`, ponlo en la raíz del proyecto: las sesiones de edición lo cargan automáticamente.

### Windsurf

1. Copia `prompts/AGENTS.md` → renómbralo como `.windsurfrules`, ponlo en la raíz del proyecto: se carga automáticamente.

### GitHub Copilot Agent

1. Copia `prompts/AGENTS.md` a la raíz del proyecto: se carga automáticamente; los demás módulos se referencian con `@security.md`.

### MCP (Model Context Protocol)
1. Compila el servidor: `cd mcp && npm install && npm run build`
2. Apunta tu cliente MCP a `node <repo>/mcp/dist/index.js`
3. El servidor expone los prompts como recursos (`mentor://prompts/{lang}/{module}`) y herramientas (`install`, `detect_tool`, `list_languages`, `list_modules`, `generate_resource_estimate`)
4. Consulta `mcp/README.md` para más detalles

## Instalación con un solo comando mediante el CLI `mentor`

```bash
mentor install          # Interactivo: elige idioma → elige módulo (por defecto agent) → detecta/selecciona la herramienta automáticamente
mentor install --lang zh-CN --modules agent,security --cli claude-code
mentor add workflow     # Añade un módulo
mentor list             # Consulta los módulos instalados
```

`mentor` escribe automáticamente los archivos con el nombre y en la ubicación que exige cada herramienta según las reglas de la tabla anterior (Claude Code → `CLAUDE.md`, Cursor → `.cursor/rules/`, el resto → `AGENTS.md`, etc.).

## Prompt completo

Si no necesitas dividirlo por módulos, puedes usar directamente `prompts/Prompt-Completo-del-Mentor.md` (versión fusionada de los cuatro módulos, lista para cargar de una sola vez).
