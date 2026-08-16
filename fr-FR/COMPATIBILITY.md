# Guide de chargement des outils IA (guide de compatibilité)

Le contenu des prompts du répertoire `prompts/` est **indépendant** de l'outil IA : n'importe quel outil de codage basé sur un grand modèle de langage peut les utiliser. La différence ne porte que sur le **mode de chargement** : nom du fichier principal, emplacement de stockage et commande de chargement. Ce document n'est donc rien d'autre qu'« un fichier de chargement » — pour ajouter un nouvel outil, il suffit d'y ajouter une ligne.

> Astuce : tous les outils peuvent être installés en une seule commande via le CLI `mentor` (qui écrit automatiquement les fichiers au bon endroit pour chaque outil), voir en fin de document.

## Tableau comparatif rapide

| Outil | Fichier principal (rôle d'agent) | Emplacement | Mode de chargement | Autres modules (security/style/workflow) |
|------|----------------------------------|-------------|--------------------|------------------------------------------|
| Xiaomi MIMO | `AGENTS.md` | Racine du projet | Manuel : `/skill AGENTS.md` | À charger un par un avec `/skill security.md`, etc. |
| Claude Code | `CLAUDE.md` ou `AGENTS.md` | Racine du projet | Chargement automatique | Référencé via `@security.md` dans le fichier principal, ou placé dans un sous-répertoire et chargé à la demande |
| OpenAI Codex | `AGENTS.md` | Racine du projet | Chargement automatique | Référencé via `@security.md` dans le fichier principal |
| Cursor | `AGENTS.md` | `.cursor/rules/` | Chargement automatique (les rules peuvent porter un glob pour limiter la portée) | Placer les fichiers homonymes dans le même répertoire |
| Gemini CLI | `GEMINI.md` | Racine du projet | Chargement automatique | À renommer puis placer dans le même répertoire, ou référencer avec `@` |
| Google Jules | `JULES.md` | Racine du projet | Chargement automatique | Idem |
| Aider | `CONVENTIONS.md` | Racine du projet | Chargement automatique | Fusionner le contenu ou référencer via des fichiers séparés |
| Windsurf | `.windsurfrules` | Racine du projet | Chargement automatique | Idem |
| GitHub Copilot Agent | `AGENTS.md` | Racine du projet | Chargement automatique | Référencé via `@security.md` |
| Tout client MCP | via `mentor-mcp` | stdio (`node mcp/dist/index.js`) | Automatique (resources + tools) | Tous les modules exposés comme ressources MCP `mentor://prompts/{lang}/{module}` |

## Détails par outil

### Xiaomi MIMO
1. Copiez `prompts/AGENTS.md` à la racine du projet
2. Dans une session MIMO, tapez `/skill AGENTS.md` pour charger le rôle de mentor
3. Si vous avez besoin des normes de sécurité / style / workflow, chargez-les à la demande : `/skill security.md`, `/skill style.md`, `/skill workflow.md`
4. Projets à long terme : utilisez `/dream` pour consolider les règles dans MEMORY.md ; en cas de déconnexion, reprenez la session avec `mimo --continue`

### Claude Code
1. Copiez `prompts/AGENTS.md` → renommez-le en `CLAUDE.md` (ou conservez `AGENTS.md`, les nouvelles versions le reconnaissent automatiquement)
2. Placez-le à la racine du projet : il se charge automatiquement à chaque session
3. Référencez les autres modules dans `CLAUDE.md` avec `@security.md`, ou fusionnez-les directement à la suite du fichier
4. Un `CLAUDE.md` placé dans un sous-répertoire est chargé à la demande lorsque vous entrez dans ce répertoire

### OpenAI Codex
1. Copiez `prompts/AGENTS.md` à la racine du projet (Codex charge automatiquement le `AGENTS.md` de la racine)
2. Référencez les autres modules dans `AGENTS.md` avec `@security.md`
3. Reprise après déconnexion : `codex --resume` (ou `codex exec --resume`)

### Cursor
1. Copiez `prompts/AGENTS.md` dans le répertoire `.cursor/rules/` (l'Agent charge automatiquement les rules)
2. Pour limiter l'application à certains fichiers, convertissez le fichier au format `.mdc` et ajoutez un frontmatter `globs` de correspondance
3. Placez les fichiers homonymes des autres modules dans le même répertoire `.cursor/rules/`

### Gemini CLI
1. Copiez `prompts/AGENTS.md` → renommez-le en `GEMINI.md`, placez-le à la racine du projet : chargement automatique
2. Les autres modules peuvent être fusionnés dans `GEMINI.md` ou référencés à la demande avec `@`

### Google Jules
1. Copiez `prompts/AGENTS.md` → renommez-le en `JULES.md`, placez-le à la racine du projet : chargement automatique

### Aider
1. Copiez `prompts/AGENTS.md` → renommez-le en `CONVENTIONS.md`, placez-le à la racine du projet : il se charge automatiquement dans les sessions d'édition

### Windsurf
1. Copiez `prompts/AGENTS.md` → renommez-le en `.windsurfrules`, placez-le à la racine du projet : chargement automatique

### GitHub Copilot Agent
1. Copiez `prompts/AGENTS.md` à la racine du projet : chargement automatique ; référencez les autres modules avec `@security.md`

### MCP (Model Context Protocol)
1. Compilez le serveur : `cd mcp && npm install && npm run build`
2. Pointez votre client MCP vers `node <repo>/mcp/dist/index.js`
3. Le serveur expose les prompts comme ressources (`mentor://prompts/{lang}/{module}`) et outils (`install`, `detect_tool`, `list_languages`, `list_modules`, `generate_resource_estimate`)
4. Voir `mcp/README.md` pour plus de détails

## Installation en une commande avec le CLI mentor

```bash
mentor install          # Mode interactif : choisir la langue → choisir les modules (agent par défaut) → détecter/sélectionner l'outil automatiquement
mentor install --lang zh-CN --modules agent,security --cli claude-code
mentor add workflow     # Ajouter un module
mentor list             # Afficher les modules déjà installés
```

`mentor` écrit automatiquement les fichiers au nom et à l'emplacement requis par chaque outil selon le tableau ci-dessus (Claude Code → `CLAUDE.md`, Cursor → `.cursor/rules/`, les autres → `AGENTS.md`, etc.).

## Version complète du prompt

Si vous n'avez pas besoin de découper les modules, utilisez directement `prompts/Prompt-Complet-du-Mentor.md` (les quatre modules fusionnés, à charger en une seule fois).
