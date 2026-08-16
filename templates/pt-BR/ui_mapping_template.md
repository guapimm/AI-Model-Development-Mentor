# Tabela de mapeamento de elementos UI + tabela de mapeamento de eventos (obrigatória antes de escrever o frontend)

> Emitida pela IA mentora antes de escrever o código do frontend, para que usuários sem nenhuma base consigam reportar problemas com precisão.
> Após concluir, arquive em `docs/` e use junto com o contrato de API (`docs/api_interface.md`).

## 1. Wireframe da página (ASCII ou Mermaid)

```
┌──────────────────────────────────────────┐
│  Barra de navegação superior (logo / menu / avatar) │
├───────────────┬──────────────────────────┤
│               │                          │
│   Barra       │     Conteúdo principal   │
│   lateral     │                          │
└───────────────┴──────────────────────────┘
```

## 2. Tabela de mapeamento de elementos UI

| Posição visual | Componente | Caminho do arquivo | Classe/ID CSS | Descrição |
|----------------|------------|--------------------|---------------|-----------|
| Barra superior, à direita | UserAvatar | src/components/Header.tsx | .user-avatar | Avatar do usuário e menu suspenso (sair, perfil) |
| | | | | |

## 3. Tabela de mapeamento de eventos do frontend

| Nome | Ação (clique/deslizar/entrada) | Endpoint do backend chamado | Resultado esperado |
|------|--------------------------------|-----------------------------|--------------------|
| Botão de login | Clique | POST /api/login | Redirecionar para a home após validação, mostrar erro em caso de falha |
| | | | |

## 4. Guia de uso (para usuários sem nenhuma base)

1. Para reportar um problema na página, diga apenas «**posição** + **o que aconteceu**», por exemplo:
   > "O avatar no canto superior direito não responde ao clique"
2. A IA mentora localizará o componente e o endpoint exatos com as duas tabelas acima, sem precisar descrever código.
