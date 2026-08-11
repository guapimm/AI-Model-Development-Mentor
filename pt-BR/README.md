🌍 Outros idiomas → [English](../README.md) · [中文](../zh-CN/README.md)

# AI Model Mentor (Português)

> **Transforme o seu assistente de codificação de IA em um mentor full-stack cauteloso com 10 anos de experiência — apenas prompts puros, zero dependências.**

---

## O que é isto?

Um **framework baseado apenas em prompts** que molda o seu assistente de codificação de IA em um **arquiteto full-stack e mentor de desenvolvimento com 10 anos de experiência**, feito para iniciantes em programação sem nenhuma base.

Ele obriga a IA a seguir um conjunto de "regras de ferro" — tornando *Segurança em Primeiro Lugar, Lógica Transparente, Documentação em Primeiro Lugar, Eficiência de Tokens e Implementação em Etapas* o seu comportamento padrão. O resultado: uma IA que não apenas *escreve código*, mas escreve código **seguro, fácil de manter e documentado**.

> ⚠️ Suporte atual: **Xiaomi MIMO CLI**. Versões otimizadas para outros produtos (Claude Code, Cursor etc.) estão planejadas — deixe um comentário se precisar de uma.

## Módulos Principais (build Xiaomi MIMO)

| Módulo | Arquivo | Objetivo |
|--------|------|---------|
| 🧑‍🏫 Papel de Mentor | [AGENTS.md](./xiaomi-mimo/AGENTS.md) | Persona de arquiteto-mentor + 6 regras de ferro + lista de autoverificação de segurança ★ núcleo, de uso obrigatório |
| 🛡️ Normas de Segurança | [security.md](./xiaomi-mimo/security.md) | 8 domínios de segurança: segredos / validação de entrada / banco de dados / XSS / sistema de arquivos / requisições externas / tratamento de erros / desempenho |
| 🎨 Estilo de Interação | [style.md](./xiaomi-mimo/style.md) | Analogias da vida cotidiana, etiquetas de fase, confirmar antes de executar, complexidade progressiva |
| 📋 Fluxo de Desenvolvimento | [workflow.md](./xiaomi-mimo/workflow.md) | Sistema de documentos / protocolo de mapeamento do front-end / implantação e rollback / ciclo de testes / âncoras de versão |

### As 6 Regras de Ferro

1. **Código como documentação** — todo o código carrega comentários que explicam o "porquê"
2. **Segurança em primeiro lugar** — nada de segredos codificados no código, validação rigorosa de entrada, consultas parametrizadas, prevenção de XSS
3. **Zero mudanças destrutivas** — analise as dependências primeiro, marque as alterações como [Obrigatória] / [Opcional]
4. **Execução em etapas** — nunca mais de 300 linhas por saída, aguarde a confirmação em cada etapa
5. **Isolamento modular** — máximo de 500 linhas por arquivo, reserve interfaces de extensão
6. **Eficiência de Tokens** — gere um resumo de contexto + senha de retomada após cada conversa

## Início Rápido (3 passos)

```bash
# 1. Copie o papel de mentor para o seu projeto (renomeie-o)
cp xiaomi-mimo/AGENTS.md AGENTS.md

# 2. (Recomendado) Adicione também as normas de segurança / estilo / fluxo de trabalho
cp xiaomi-mimo/security.md security.md
cp xiaomi-mimo/style.md style.md
cp xiaomi-mimo/workflow.md workflow.md
```

3. Inicie o Xiaomi MIMO e diga:

> "Sou um iniciante completo. Aqui está a minha Especificação de Requisitos do Projeto: nome do projeto ____, objetivos principais ____, papéis dos usuários ____, fluxos de trabalho principais ____, dados a persistir ____. Comece pela Fase 0: Preparação do Ambiente e Seleção da Stack Tecnológica e me guie passo a passo."

A IA avançará por "Design → Lógica Principal → UI → Testes", aguardando a sua confirmação em cada estágio.

## Estrutura de Arquivos

```
AI_Model_Development_Mentor/
├── README.md            # Página de entrada em inglês + seletor de idiomas
├── LICENSE              # Licença MIT
├── zh-CN/  en-US/  ja-JP/  ko-KR/  es-ES/  fr-FR/  de-DE/  pt-BR/  ru-RU/
└── <idioma>/xiaomi-mimo/  # arquivos de módulo por idioma
    ├── AGENTS.md        # papel de mentor ★ obrigatório
    ├── security.md      # normas de segurança
    ├── style.md         # estilo de interação
    └── workflow.md      # fluxo de desenvolvimento
```

> 📦 Novos builds de produtos são adicionados como diretórios irmãos sob cada diretório de idioma, por exemplo: `zh-CN/claude-code/`, `en-US/cursor/`.

## FAQ

**P: Preciso de todos os 4 módulos?**
R: Não. `AGENTS.md` é o único indispensável. Adicione `security.md` para proteções mais rigorosas e `style.md` para uma experiência de conversa mais amigável.

**P: Funciona com outros produtos de IA?**
R: Por enquanto, apenas o Xiaomi MIMO é suportado. Versões otimizadas para outros produtos estão em andamento — deixe um comentário para nos informar do que você precisa.

## Licença

[Licença MIT](../LICENSE) © 2026 guapimm
