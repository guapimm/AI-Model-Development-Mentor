# Guia de Carregamento para Cada Ferramenta de IA (Compatibilidade)

O conteúdo dos prompts no diretório `prompts/` é **independente da ferramenta de IA** — qualquer ferramenta de codificação baseada em modelo de linguagem pode usá-lo. A diferença está apenas na **forma de carregamento**: o nome do arquivo principal, o local de armazenamento e o comando de carregamento. Este arquivo é um "guia de carregamento" — adicionar uma nova ferramenta é só acrescentar uma linha aqui.

> Dica: todas as ferramentas podem ser instaladas em um único comando pela CLI `mentor` (ela grava automaticamente os arquivos no local correto de cada ferramenta), veja o final deste documento.

## Tabela de Referência Rápida

| Ferramenta | Arquivo principal (papel do agente) | Local de armazenamento | Como carregar | Outros módulos (security/style/workflow) |
|------|---------------------|---------|---------|-----------------------------------|
| Xiaomi MIMO | `AGENTS.md` | Raiz do projeto | Manual: `/skill AGENTS.md` | Carregue um a um com `/skill security.md`, etc. |
| Claude Code | `CLAUDE.md` ou `AGENTS.md` | Raiz do projeto | Carregamento automático | Referencie com `@security.md` no arquivo principal, ou coloque em um subdiretório para carregar sob demanda |
| OpenAI Codex | `AGENTS.md` | Raiz do projeto | Carregamento automático | Referencie com `@security.md` no arquivo principal |
| Cursor | `AGENTS.md` | `.cursor/rules/` | Carregamento automático (as rules podem usar glob para definir o escopo dos arquivos) | Coloque os arquivos de mesmo nome no mesmo diretório |
| Gemini CLI | `GEMINI.md` | Raiz do projeto | Carregamento automático | Renomeie e coloque-os juntos, ou referencie com `@` |
| Google Jules | `JULES.md` | Raiz do projeto | Carregamento automático | Igual ao anterior |
| Aider | `CONVENTIONS.md` | Raiz do projeto | Carregamento automático | Mescle o conteúdo ou referencie a partir de arquivos separados |
| Windsurf | `.windsurfrules` | Raiz do projeto | Carregamento automático | Igual ao anterior |
| GitHub Copilot Agent | `AGENTS.md` | Raiz do projeto | Carregamento automático | Referencie com `@security.md` |

## Detalhes por Ferramenta

### Xiaomi MIMO
1. Copie `prompts/AGENTS.md` para a raiz do projeto
2. Em uma sessão do MIMO, digite `/skill AGENTS.md` para carregar o papel de mentor
3. Quando precisar de segurança/estilo/workflow, carregue sob demanda com `/skill security.md`, `/skill style.md`, `/skill workflow.md`
4. Projetos de longo prazo: use `/dream` para consolidar as regras no MEMORY.md; em caso de desconexão, retome com `mimo --continue`

### Claude Code
1. Copie `prompts/AGENTS.md` → renomeie para `CLAUDE.md` (ou mantenha como `AGENTS.md`; as versões novas o detectam automaticamente)
2. Coloque na raiz do projeto; é carregado automaticamente em todas as sessões
3. Referencie os outros módulos em `CLAUDE.md` com `@security.md`, ou anexe e mescle diretamente
4. Um `CLAUDE.md` dentro de um subdiretório é carregado sob demanda quando você entra naquele diretório

### OpenAI Codex
1. Copie `prompts/AGENTS.md` para a raiz do projeto (o Codex carrega automaticamente o `AGENTS.md` da raiz)
2. Referencie os outros módulos em `AGENTS.md` com `@security.md`
3. Para retomar após uma desconexão, use `codex --resume` (ou `codex exec --resume`)

### Cursor
1. Copie `prompts/AGENTS.md` para o diretório `.cursor/rules/` (o Agent carrega as rules automaticamente)
2. Para que valha apenas para um escopo de arquivos, converta para o formato `.mdc` e adicione o `globs` no frontmatter
3. Coloque também os arquivos de mesmo nome dos outros módulos em `.cursor/rules/`

### Gemini CLI
1. Copie `prompts/AGENTS.md` → renomeie para `GEMINI.md`, coloque na raiz do projeto; é carregado automaticamente
2. Os outros módulos podem ser mesclados no `GEMINI.md` ou referenciados com `@` sob demanda

### Google Jules
1. Copie `prompts/AGENTS.md` → renomeie para `JULES.md`, coloque na raiz do projeto; é carregado automaticamente

### Aider
1. Copie `prompts/AGENTS.md` → renomeie para `CONVENTIONS.md`, coloque na raiz do projeto; é carregado automaticamente nas sessões de edição

### Windsurf
1. Copie `prompts/AGENTS.md` → renomeie para `.windsurfrules`, coloque na raiz do projeto; é carregado automaticamente

### GitHub Copilot Agent
1. Copie `prompts/AGENTS.md` para a raiz do projeto; é carregado automaticamente. Referencie os outros módulos com `@security.md`

## Instalação com a CLI mentor (um comando)

```bash
mentor install          # Interativo: escolha o idioma → escolha o módulo (padrão: agent) → detecta/seleciona a ferramenta automaticamente
mentor install --lang zh-CN --modules agent,security --cli claude-code
mentor add workflow     # Acrescenta um módulo
mentor list             # Exibe os módulos instalados
```

`mentor` grava automaticamente os arquivos com o nome e no local que cada ferramenta exige, seguindo as regras da tabela acima (Claude Code → `CLAUDE.md`, Cursor → `.cursor/rules/`, as demais → `AGENTS.md`, etc.).

## Prompt Consolidado

Se você não precisa dividir em módulos, pode usar diretamente o `prompts/Prompt-Completo-do-Mentor.md` (versão consolidada dos quatro módulos, carregada de uma só vez).
