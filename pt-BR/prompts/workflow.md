# Normas de Fluxo de Trabalho de Desenvolvimento

## 1. Inicialização do Projeto e Sistema de Documentação

### Modo Leve (menos de 500 linhas de código)
Basta o `README.md`, contendo: visão geral do projeto, stack tecnológica, estrutura principal das tabelas, lista de interfaces, passos de implantação e uma mini tabela de estimativa de recursos.

### Modo Padrão (código ≥ 500 linhas)
Ao iniciar o projeto, crie a seguinte estrutura de documentos:

```
📁 /docs/
├── architecture.md      # Justificativa da escolha da stack tecnológica (com analogias da vida cotidiana), diagramas da arquitetura do sistema (Mermaid), estrutura de diretórios
├── resource_estimate.md # Tabela de estimativa de recursos do projeto (três níveis de memória/disco/configuração, limites de escalabilidade)
├── dev_log.md           # Log de desenvolvimento: datas, alterações, resultados de testes, problemas conhecidos e soluções
├── api_interface.md     # Contrato de interface entre front-end e back-end (URL, parâmetros, valores de retorno, cenários de exceção)
└── SNAPSHOT.md          # Instantâneo principal (≤200 linhas): versões da stack, lista de nomes de tabelas, caminhos de API, diagramas de fluxo de negócios
```

### Saída Obrigatória na Fase 0: Tabela de Estimativa de Recursos do Projeto
Deve ser gerada após a confirmação dos requisitos e antes da codificação, no formato padrão:

| Nível de configuração | Pico de memória em execução | Ocupação inicial de disco | Previsão de crescimento anual de disco | Requisito mínimo de CPU | Cenário de uso |
|---------|-------------|-------------|---------------|------------|---------|
| Nível mínimo | XX MB | XX MB | XX MB | 1 núcleo | Desenvolvimento individual, baixo volume de acessos |
| Nível recomendado | XX MB | XX MB | XX MB | 2 núcleos | Uso diário, até 100 usuários |
| Nível alto | XX MB | XX GB | XX GB | 4 núcleos | Acesso concorrente, ambiente de produção |

- Condição de gatilho de escalabilidade: especifique em qual limite de usuários/volume de dados é necessário fazer upgrade da configuração
- Estimativa de consumo de Token: faixa estimada de consumo de Token para todo o fluxo do projeto

Otimização de Tokens: ao final de cada conversa, gere o 【Resumo de contexto】(progresso, nomes de variáveis, pendências, senha de retomada), mantendo cada resumo com no máximo 100 caracteres.

## 2. Normas Obrigatórias de Design do Banco de Dados

- O design da estrutura das tabelas deve ser acompanhado do plano de índices; os campos essenciais de consulta devem ter índices
- Estime o volume de dados por tabela; se ultrapassar 100 mil registros, forneça antecipadamente um plano de particionamento/otimização
- Defina o comprimento e o tipo dos campos conforme a necessidade, evitando uso excessivo de espaço de armazenamento
- Configure obrigatoriamente um limite máximo para o pool de conexões do banco de dados, evitando que o serviço trave quando o número de conexões se esgota

## 3. Protocolo de Posicionamento Visual no Front-end

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

## 4. Mecanismos de Implantação e Recuperação de Desastres

### Backup Local
- Ofereça um script de backup com um clique, o `backup.sh`, que exporta código + configurações + banco de dados para `./local_backup/`
- Antes de cada implantação, verifique se o backup local existe; caso contrário, recuse a implantação

### Rollback Gradual em Servidor em Nuvem
- Antes de implantar o novo código, compacte automaticamente a versão antiga em `backup_[timestamp].zip`
- Procedimento de rollback de emergência em três passos:
  1. `./rollback.sh latest` — descompacte o backup mais recente
  2. `docker-compose restart`(ou `pm2 restart all`)
  3. `./health_check.sh` — exiba o status do serviço
- Registre no `dev_log.md` o horário do backup, o caminho e o histórico de rollback

### Isolamento de Ambientes
- Separe as configurações do ambiente de desenvolvimento e do ambiente de produção
- Avise com antecedência os itens de configuração de segurança que precisam ser alterados em produção

## 5. Desdobramento de Requisitos e Sugestões

Depois de concluir a funcionalidade solicitada pelo usuário, gere o《Cartão de Sugestões de Melhoria de Funcionalidades》:

- ✅ **Resumo das funcionalidades concluídas** — explique claramente o que está disponível
- 🔮 **Alerta de riscos potenciais** — acesso concorrente, consistência de dados, dependências de terceiros etc.
- 🚀 **Funcionalidades de extensão recomendadas** — marque prioridade P0/P1/P2, dificuldade de implementação em ⭐, efeito esperado
- ⚡ **Sugestões de otimização de desempenho** — indique a prioridade, como P0 adicionar índice em campos essenciais, P1 adicionar cache em dados de alta frequência (hot data)
- ⚠️ **Guia para iniciantes evitarem armadilhas** — equívocos comuns, cuidados de operação

## 6. Ciclo Fechado de Testes e Autoverificação

### Casos de Teste Mínimos Verificáveis
Forneça etapas de validação que o usuário possa executar manualmente, por exemplo:
> "Clique no botão de login, insira o usuário e a senha corretos e verifique se há redirecionamento para a página inicial"

### Declaração de Coerência Lógica
Depois de enviar o código, é obrigatório declarar:
> "Eu verifiquei: ①escopo das variáveis correto ②processamento assíncrono completo ③cobertura de captura de exceções ④sem vazamento de informações sensíveis ⑤sem gargalos de desempenho evidentes ⑥uso de memória sob controle"

## 7. Âncora de Versão

Ao concluir cada marco (milestone), gere uma mensagem padronizada de Commit do Git:
```
feat: módulo de login do usuário concluído
Implementar autenticação JWT Token
Adicionar armazenamento de senha com hash
Validação do formulário de login no front-end
Author: AI Assistant
Date: 2026-08-08
```
