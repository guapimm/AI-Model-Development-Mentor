🌍 Outros idiomas → [English](../README.md) · [中文](../zh-CN/README.md)

# AI Model Mentor (Português)

> **Transforme o seu assistente de codificação de IA em um mentor full-stack cauteloso com 10 anos de experiência — apenas prompts puros, zero dependências.**

---

## O que é isto?

Um **framework baseado apenas em prompts** que molda o seu assistente de codificação de IA em um **arquiteto full-stack e mentor de desenvolvimento com 10 anos de experiência**, feito para iniciantes em programação sem nenhuma base.

Ele obriga a IA a seguir um conjunto de "regras de ferro" — tornando *Segurança em Primeiro Lugar, Lógica Transparente, Documentação em Primeiro Lugar, Eficiência de Tokens, Implementação em Etapas e Controle de Recursos* o seu comportamento padrão. O resultado: uma IA que não apenas *escreve código*, mas escreve código **seguro, fácil de manter e documentado**.

> 📦 O conteúdo dos prompts é independente da ferramenta de IA — consulte o [COMPATIBILITY.md](./COMPATIBILITY.md) para o guia de carregamento de cada ferramenta (opencode / Claude Code / Codex / Cursor, etc.).

## Módulos Principais (compatível com várias ferramentas)

| Módulo | Arquivo | Objetivo |
|--------|------|---------|
| 🧑‍🏫 Papel de Mentor | [AGENTS.md](./prompts/AGENTS.md) | Persona de arquiteto-mentor full-stack + 6 regras de ferro + lista de autoverificação de segurança e desempenho ★ núcleo, de uso obrigatório |
| 🛡️ Normas de Segurança | [security.md](./prompts/security.md) | 8 domínios de segurança: gerenciamento de segredos / validação de entrada / banco de dados / XSS / sistema de arquivos / requisições externas / tratamento de exceções / desempenho e recursos |
| 🎨 Estilo de Interação | [style.md](./prompts/style.md) | Analogias da vida cotidiana, etiquetas de fase, confirmar antes de executar, complexidade progressiva |
| 📋 Fluxo de Desenvolvimento | [workflow.md](./prompts/workflow.md) | Sistema de documentos / estimativa de recursos / design de banco de dados / protocolo de posicionamento do front-end / implantação e recuperação de desastres / ciclo fechado de testes e autoverificação / âncoras de versão |

## 📦 Mais documentos

- [COMPATIBILITY.md](./COMPATIBILITY.md) — guia de carregamento para cada ferramenta de IA (opencode / Claude Code / Codex / Cursor, etc.)
- [Prompt-Completo-do-Mentor.md](./prompts/Prompt-Completo-do-Mentor.md) — prompt completo consolidado (todos os módulos unificados)

### As 6 Regras de Ferro

1. **Código como documentação** — todo o código carrega comentários que explicam o "porquê"
2. **Segurança em primeiro lugar** — nada de segredos codificados no código, validação rigorosa de entrada, consultas parametrizadas, prevenção de XSS
3. **Zero mudanças destrutivas** — analise as dependências primeiro, marque as alterações como [Obrigatória] / [Opcional]
4. **Execução em etapas** — nunca mais de 300 linhas por saída, aguarde a confirmação em cada etapa
5. **Isolamento modular** — máximo de 500 linhas por arquivo, reserve interfaces de extensão
6. **Desempenho e recursos em primeiro lugar** — design de banco de dados com plano de índices, paginação por padrão nas consultas de listas, estimativa de recursos em três níveis (memória/disco/CPU) no início do projeto e mecanismo de liberação para operações com alto consumo de memória

## ⬇️ Instalação e uso do mentor CLI

**Opção A: binário Go (recomendado, zero dependências, multiplataforma)**

Baixe o executável `mentor` para a sua plataforma nos GitHub Releases (v0.1.0, compatível com Windows / Linux / macOS) e coloque-o no PATH:

```bash
mentor install                        # assistente interativo: escolha o idioma → escolha os módulos (padrão: agent) → detecta a ferramenta automaticamente
mentor install --lang zh-CN --modules agent,security --cli claude-code --dir ./proj
mentor add workflow                   # adiciona módulos
mentor list                           # lista os módulos instalados
mentor detect                         # detecta as ferramentas de IA usadas no projeto
mentor pack                           # gera um diretório de skill compatível
```

O `mentor` grava automaticamente o nome e a localização corretos do arquivo conforme a ferramenta: opencode/Codex → `AGENTS.md`, Claude Code → `CLAUDE.md`, Cursor → `.cursor/rules/`.

**Opção B: cópia manual**

Siga as instruções do [COMPATIBILITY.md](./COMPATIBILITY.md) e copie os arquivos de `prompts/` para as localizações correspondentes no projeto.

> Comandos suportados: `install` / `add` / `remove` / `list` / `detect` / `pack`; módulos: agent (padrão) / security / style / workflow / complete; ferramentas: opencode / claude-code / codex / cursor / other.

## 📖 Guia de uso (opencode)

### Comandos rápidos

| Cenário | Ação |
|------|------|
| Desenvolvimento diário | Entre no projeto → o opencode carrega AGENTS.md automaticamente → converse normalmente |
| Projeto de longo prazo | Mantenha as regras no AGENTS.md (atualize com `/init`) |
| Conexão inesperadamente perdida | Recupere com `opencode --continue`; as regras continuam lá |
| Abrir uma nova sessão por conta própria | Basta iniciar o `opencode` — o AGENTS.md é carregado automaticamente |

### Estrutura de arquivos do projeto

```
📁 my-project/
├── 📄 AGENTS.md          ← prompt principal
├── 📄 security.md        ← normas de segurança
├── 📄 workflow.md        ← normas de fluxo de trabalho
├── 📄 style.md           ← estilo de interação
└── 📁 src/
```

---

### Demonstração de cenários concretos

#### Cenário 1: escrever código no dia a dia (carregar apenas o AGENTS.md)

> Você: "Me ajude a escrever uma API para obter a lista de usuários"

A carregar: AGENTS.md (já carregado automaticamente, nenhuma ação necessária)

A IA fará automaticamente:

- código com comentários em português
- marcar a lista de segurança antes de enviar o código
- executar em etapas (≤300 linhas)
- no máximo 500 linhas por arquivo

#### Cenário 2: escrever a interface de login/cadastro (carregar AGENTS.md + security.md)

> Você: "Me ajude a escrever a função de login do usuário, seguindo os requisitos do security.md"

A carregar:

```bash
@security.md
```

A IA também fará:

- armazenar as senhas com hash bcrypt
- definir expiração para o Token JWT
- prevenir força bruta (limite de tentativas de login)
- prevenir injeção de SQL (consultas parametrizadas)

#### Cenário 3: iniciar um projeto do zero (carregar AGENTS.md + workflow.md)

> Você: "Quero criar um sistema de blog; com base no workflow.md, me ajude a montar o esqueleto do projeto"

A carregar:

```bash
@workflow.md
```

A IA também fará:

- criar docs/architecture.md (seleção da stack + diagrama de arquitetura)
- criar docs/dev_log.md (modelo de log de desenvolvimento)
- criar docs/api_interface.md (modelo de contrato de interface)
- criar docs/SNAPSHOT.md (instantâneo do projeto)
- gerar os scripts backup.sh e rollback.sh

#### Cenário 4: a explicação da IA está obscura demais (carregar style.md)

> Você: "Seguindo o estilo do style.md, me explique com analogias da vida cotidiana o que é JWT"

A carregar:

```bash
@style.md
```

A IA também fará:

- explicar JWT com o "cartão de fidelidade do restaurante"
- adicionar a etiqueta de fase [📋 Análise de requisitos]
- dar a conclusão primeiro e os detalhes depois
- oferecer 2 a 3 opções

#### Cenário 5: implantação em produção (carregar AGENTS.md + workflow.md)

> Você: "Seguindo as normas de implantação do workflow.md, me ajude a escrever a configuração de implantação com Docker"

A carregar:

```bash
@workflow.md
```

A IA também fará:

- distinguir configurações do ambiente de desenvolvimento/produção
- gerar docker-compose.yml
- gerar health_check.sh
- lembrar das etapas de backup e rollback

### ⚠️ Quando NÃO carregar?

| Quando não carregar | Motivo |
|---------------|------|
| Fazer perguntas puramente técnicas (como "como usar React useEffect") | AGENTS.md já é suficiente; adicionar o workflow só atrapalha |
| Alterar um estilo CSS | Não precisa das normas de segurança nem do fluxo de implantação |
| Pedir para a IA traduzir um texto | Não precisa de nenhum módulo |
| Refatorar levemente código existente | A lista de segurança do AGENTS.md já cobre isso |

### 💡 Resumo em uma frase

> AGENTS.md é a "pele" padrão; os outros três são plug-ins de efeitos especiais — ative-os apenas quando precisar; no dia a dia, deixe-os desligados, economizando Tokens e mantendo tudo limpo.

## Início Rápido (3 passos)

```bash
# 1. Copie o papel de mentor para o seu projeto (renomeie-o)
cp prompts/AGENTS.md AGENTS.md

# 2. (Recomendado) Adicione também as normas de segurança / estilo / fluxo de trabalho
cp prompts/security.md security.md
cp prompts/style.md style.md
cp prompts/workflow.md workflow.md
```

3. Inicie o opencode e diga:

> "Sou um iniciante completo. Aqui está a minha Especificação de Requisitos do Projeto: nome do projeto ____, objetivos principais ____, papéis dos usuários ____, fluxos de trabalho principais ____, dados a persistir ____. Comece pela Fase 0: Preparação do Ambiente e Seleção da Stack Tecnológica e me guie passo a passo."

A IA avançará por "Design → Lógica Principal → UI → Testes", aguardando a sua confirmação em cada estágio.

## Estrutura de Arquivos

```
AI_Model_Development_Mentor/
├── README.md            # Página de entrada em inglês + seletor de idiomas
├── LICENSE              # Licença MIT
├── cli/                 # CLI mentor (Go, binário único) — instalação em um clique
├── zh-CN/  en-US/  ja-JP/  ko-KR/  es-ES/  fr-FR/  de-DE/  pt-BR/  ru-RU/
└── <idioma>/
    ├── README.md        # página de entrada do idioma + guia de uso
    ├── COMPATIBILITY.md # instruções de carregamento por ferramenta (o "arquivo adaptador")
    └── prompts/         # conteúdo independente de ferramenta (por idioma)
        ├── AGENTS.md    # papel de mentor ★ obrigatório
        ├── security.md  # normas de segurança
        ├── style.md     # estilo de interação
        ├── workflow.md  # fluxo de desenvolvimento
        └── <completo>.md # prompt consolidado em um único arquivo
```

> 📦 Novas ferramentas são adicionadas como linhas no [COMPATIBILITY.md](./COMPATIBILITY.md); não é mais necessário criar diretórios por produto.

## FAQ

**P: Preciso de todos os 4 módulos?**
R: Não. `AGENTS.md` é o único indispensável. Adicione `security.md` para proteções mais rigorosas e `style.md` para uma experiência de conversa mais amigável.

**P: Funciona com outros produtos de IA?**
R: Sim. O conteúdo dos prompts é independente da ferramenta — a diferença está apenas na forma de carregamento. Veja o [COMPATIBILITY.md](./COMPATIBILITY.md) para o guia de cada ferramenta (opencode / Claude Code / Codex / Cursor, etc.).

## Licença

[Licença MIT](../LICENSE) © 2026 guapimm
