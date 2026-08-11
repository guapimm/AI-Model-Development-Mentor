# Normas de Fluxo de Trabalho de Desenvolvimento

## 1. Inicialização do Projeto e Sistema de Documentação

### Modo Leve (menos de 500 linhas de código)
Basta o `README.md`, contendo: visão geral do projeto, stack tecnológica, estrutura principal das tabelas, lista de interfaces e passos de implantação.

### Modo Padrão (código ≥ 500 linhas)
Ao iniciar o projeto, crie a seguinte estrutura de documentos:

```
📁 /docs/
├── architecture.md      # Justificativa da escolha da stack tecnológica (com analogias da vida cotidiana), diagramas da arquitetura do sistema (Mermaid), estrutura de diretórios
├── dev_log.md           # Log de desenvolvimento: datas, alterações, resultados de testes, problemas conhecidos e soluções
├── api_interface.md     # Contrato de interface entre front-end e back-end (URL, parâmetros, valores de retorno, cenários de exceção)
└── SNAPSHOT.md          # Instantâneo principal (≤200 linhas): versões da stack, lista de nomes de tabelas, caminhos de API, diagramas de fluxo de negócios
```

Otimização de Tokens: ao final de cada conversa, gere o 【Resumo de contexto】(progresso, nomes de variáveis, pendências, senha de retomada).

## 2. Protocolo de Posicionamento Visual no Front-end

Antes de escrever código de front-end, primeiro gere as seguintes informações de posicionamento:

### 1. Diagrama de Layout da Página
Use wireframes ASCII ou árvores de componentes Mermaid para deixar clara a estrutura da página.

### 2. Tabela de Mapeamento de Elementos de UI

| Posição visual | Nome do componente | Caminho do arquivo correspondente | Classe/ID CSS | Descrição da função |
|---------|---------|------------|------------|---------|
| Lado direito da barra de navegação superior | UserAvatar | /src/components/Header.tsx | .user-avatar | Avatar do usuário e menu suspenso |

### 3. Tabela de Mapeamento de Eventos do Front-end

| Nome | Operação | Interface de backend chamada | Efeito esperado |
|-------|------|------------|---------|
| Botão de login | Clique | POST /api/login | Redirecionar para a página inicial e armazenar o Token |

## 3. Mecanismos de Implantação e Recuperação de Desastres

### Backup Local
- Ofereça um script de backup com um clique, o `backup.sh`, que exporta código + configurações + banco de dados para `./local_backup/`
- Antes de cada implantação, verifique se o backup local existe; caso contrário, recuse a implantação

### Rollback Gradual em Servidor em Nuvem
- Antes de implantar o novo código, compacte automaticamente a versão antiga em `backup_timestamp.zip` (backup_ + timestamp)
- Procedimento de rollback de emergência em três passos:
  1. `./rollback.sh latest` — descompacte o backup mais recente
  2. `docker-compose restart`(ou `pm2 restart all`)
  3. `./health_check.sh` — exiba o status do serviço
- Registre no `dev_log.md` o horário do backup, o caminho e o histórico de rollback

### Isolamento de Ambientes
- Separe as configurações do ambiente de desenvolvimento e do ambiente de produção
- Avise com antecedência os itens de configuração de segurança que precisam ser alterados em produção

## 4. Desdobramento de Requisitos e Sugestões

Depois de concluir a funcionalidade solicitada pelo usuário, gere o《Cartão de Sugestões de Melhoria de Funcionalidades》:

- ✅ **Resumo das funcionalidades concluídas** — explique claramente o que está disponível
- 🔮 **Alerta de riscos potenciais** — acesso concorrente, consistência de dados, dependências de terceiros etc.
- 🚀 **Funcionalidades de extensão recomendadas** — marque prioridade P0/P1/P2, dificuldade de implementação em ⭐, efeito esperado
- ⚠️ **Guia para iniciantes evitarem armadilhas** — equívocos comuns, cuidados de operação

## 5. Ciclo Fechado de Testes e Autoverificação

### Casos de Teste Mínimos Verificáveis
Forneça etapas de validação que o usuário possa executar manualmente, por exemplo:
> "Clique no botão de login, insira o usuário e a senha corretos e verifique se há redirecionamento para a página inicial"

### Declaração de Coerência Lógica
Depois de enviar o código, é obrigatório declarar:
> "Eu verifiquei: ①escopo das variáveis correto ②processamento assíncrono completo ③cobertura de captura de exceções ④sem vazamento de informações sensíveis ⑤sem gargalos de desempenho evidentes"

## 6. Âncora de Versão

Ao concluir cada marco (milestone), gere uma mensagem padronizada de Commit do Git:
```
feat: módulo de login do usuário concluído
- Implementar autenticação JWT Token
- Adicionar armazenamento de senha com hash
- Validação do formulário de login no front-end
Author: AI Assistant
Date: 2026-08-08
```
