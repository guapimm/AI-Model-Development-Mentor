# Manual Detalhado de Normas de Segurança

## 1. Gerenciamento de Chaves e Configurações

- Proibido codificar diretamente no código qualquer chave, senha ou Token de API
- Use variáveis de ambiente de forma uniforme; o código deve referenciar apenas os nomes das variáveis
- Extraia todos os itens de configuração para o arquivo `.env.example` (sem valores reais, apenas os nomes das variáveis)
- Adicione o arquivo `.env` do ambiente de produção ao `.gitignore`

## 2. Validação de Entrada do Usuário

- Toda entrada do usuário deve passar por validação de tipo (por exemplo, campos numéricos devem rejeitar strings)
- Defina limites de tamanho razoáveis (por exemplo, nome de usuário com 2 a 50 caracteres)
- Rejeite injeção de caracteres especiais (palavras-chave de SQL, tags de HTML etc.)
- Restrinja o tipo e o tamanho dos arquivos enviados; valide o tipo MIME

## 3. Segurança do Banco de Dados

- É obrigatório usar consultas parametrizadas ou instruções pré-compiladas do ORM
- Proibido montar SQL por concatenação de strings
- Campos sensíveis (senhas) devem ser armazenados como hash (bcrypt/argon2)
- A senha da string de conexão do banco de dados deve ser armazenada em variável de ambiente
- Configure obrigatoriamente um limite máximo para o pool de conexões do banco de dados, evitando que o serviço trave quando o número de conexões se esgota

## 4. Proteção contra XSS no Front-end

- Todo o conteúdo renderizado dinamicamente deve passar por escape de HTML
- Use os mecanismos de escape integrados do framework (como `{}` no React, `{{}}` no Vue)
- Proibido renderizar entrada do usuário diretamente com `innerHTML` ou `v-html`
- Configure os sinalizadores `HttpOnly` e `Secure` nos Cookies

## 5. Segurança do Sistema de Arquivos

- Todas as operações de caminho de arquivo devem ser validadas para prevenir travessia de diretório (`../`)
- Use listas de permissão (allowlist) para restringir o alcance dos diretórios acessíveis
- Renomeie os arquivos enviados para UUIDs aleatórios, sem preservar o nome original do arquivo
- Defina um limite rígido (hard limit) para o tamanho de cada arquivo; para arquivos muito grandes, exija upload em partes (chunked)

## 6. Segurança de Requisições Externas

- Todas as requisições HTTP devem ter timeout (recomendado: 5 a 10 segundos)
- Implemente política de novas tentativas (no máximo 3 vezes, com backoff exponencial)
- Valide os certificados SSL; é proibido pular a verificação do certificado

## 7. Tratamento de Exceções

- Todas as exceções devem ser capturadas com try-catch
- Em produção, não retorne o stack trace original ao cliente
- Registre logs de erro (com horário, ID da requisição e tipo do erro)
- Registre logs de auditoria para operações sensíveis (falha de login, permissões insuficientes)

## 8. Desempenho e Segurança de Recursos

- Todas as interfaces de listas devem ter paginação ativada por padrão, com limite máximo de itens por página (padrão: 100), proibindo consultas de dados completos
- Configure limite de frequência (rate limiting) nas interfaces de acordo com a concorrência estimada (em nível de IP e de usuário), prevenindo ataques de esgotamento de recursos
- Arquivos grandes/processamento de grandes volumes de dados devem usar leitura e escrita em fluxo (streaming), evitando carregar tudo de uma vez na memória e causar estouro (overflow)
- Os campos essenciais de consulta devem ter índices criados; é proibida a varredura completa da tabela sem índices
- Limpe periodicamente logs expirados e arquivos temporários, controlando o crescimento ilimitado do uso do disco
