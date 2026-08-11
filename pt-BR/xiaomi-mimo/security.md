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

## 4. Proteção contra XSS no Front-end

- Todo o conteúdo renderizado dinamicamente deve passar por escape de HTML
- Use os mecanismos de escape integrados do framework (como `{}` no React, `{{}}` no Vue)
- Proibido renderizar entrada do usuário diretamente com `innerHTML` ou `v-html`
- Configure os sinalizadores `HttpOnly` e `Secure` nos Cookies

## 5. Segurança do Sistema de Arquivos

- Todas as operações de caminho de arquivo devem ser validadas para prevenir travessia de diretório (`../`)
- Use listas de permissão (allowlist) para restringir o alcance dos diretórios acessíveis
- Renomeie os arquivos enviados para UUIDs aleatórios, sem preservar o nome original do arquivo

## 6. Segurança de Requisições Externas

- Todas as requisições HTTP devem ter timeout (recomendado: 5 a 10 segundos)
- Implemente política de novas tentativas (no máximo 3 vezes, com backoff exponencial)
- Valide os certificados SSL; é proibido pular a verificação do certificado

## 7. Tratamento de Exceções

- Todas as exceções devem ser capturadas com try-catch
- Em produção, não retorne o stack trace original ao cliente
- Registre logs de erro (com horário, ID da requisição e tipo do erro)
- Registre logs de auditoria para operações sensíveis (falha de login, permissões insuficientes)

## 8. Segurança e Desempenho

- Considere os gargalos de desempenho das interfaces; adicione cache quando necessário (Redis)
- Otimize consultas lentas e adicione índices no banco de dados
- Arquivos grandes devem ser enviados com upload em partes (chunked) ou processamento por fluxo (streaming)
- Previna ataques de esgotamento de recursos (limite a frequência de requisições)
