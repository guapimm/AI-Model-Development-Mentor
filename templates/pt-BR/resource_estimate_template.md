# Tabela de estimativa de recursos do projeto (obrigatória na Fase 0)

> Preenchida no início do projeto, guiada pela IA mentora, como base para a escolha da stack e o planejamento do deploy.
> Após preencher, arquive esta tabela em `docs/architecture.md` e mantenha-a atualizada nas fases seguintes.

## 1. Informações básicas do projeto

| Item | Conteúdo |
|------|----------|
| Nome do projeto | |
| Linhas de código estimadas | (menos de 500 linhas ativa o «modo leve», mantendo apenas um README.md) |
| Escala de usuários-alvo | Uso pessoal / equipe pequena / produto público |
| Pico de usuários simultâneos | |
| Tipo de dados | Texto simples / imagens / áudio-vídeo / arquivos grandes |

## 2. Estimativa de recursos em três níveis

| Dimensão | Mínimo (dev/demo) | Recomendado (lançamento pequeno) | Alta disponibilidade (produto público) |
|----------|-------------------|----------------------------------|----------------------------------------|
| Memória | | | |
| Disco | | | |
| Núcleos de CPU | | | |
| Largura de banda | | | |
| Banco de dados | SQLite / em memória | MySQL / PostgreSQL | Cluster + separação leitura/escrita |

## 3. Dependências de serviços de terceiros

| Serviço | Finalidade | Obrigatório? | Plano gratuito é suficiente? |
|---------|------------|--------------|------------------------------|
| Servidor na nuvem | | | |
| Armazenamento de objetos (arquivos/imagens) | | | |
| SMS / e-mail | | | |
| Pagamento | | | |
| Outro | | | |

## 4. Plano de desempenho e recursos

- [ ] Endpoints de listagem paginam por padrão; sem varreduras de tabela completa
- [ ] O design do banco de dados inclui um plano de índices
- [ ] Operações com arquivos/dados grandes usam streaming
- [ ] Operações com grande uso de memória têm mecanismo de liberação explícito
- [ ] Requisições externas definem políticas de timeout e nova tentativa

## 5. Estimativa de custo mensal

| Item | Mínimo | Recomendado |
|------|--------|-------------|
| Servidor | | |
| Armazenamento | | |
| Serviços de terceiros | | |
| **Total** | | |
