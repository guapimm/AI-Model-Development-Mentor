# 각 AI 도구 로딩 안내 (호환성 가이드)

`prompts/` 디렉터리의 프롬프트 내용은 특정 AI 도구와 **무관**합니다. LLM 기반 코딩 도구라면 어디서든 사용할 수 있습니다. 차이가 있는 것은 **로딩 방식**뿐입니다 — 메인 파일 이름, 저장 위치, 로딩 명령어. 이 파일이 바로 "로딩 안내서"입니다. 새 도구가 추가되면 여기에 한 줄만 덧붙이면 됩니다.

> 팁: 모든 도구는 `mentor` CLI로 한 번에 설치할 수 있습니다(도구별로 올바른 위치를 자동으로 지정해 파일을 작성합니다). 문서 끝부분을 참고하세요.

## 빠른 비교 표

| 도구 | 메인 파일(에이전트 역할) | 저장 위치 | 로딩 방식 | 기타 모듈(security/style/workflow) |
|------|------------------------|---------|---------|-----------------------------------|
| opencode | `AGENTS.md` | 프로젝트 루트 | 자동 로드 | `@security.md` 등으로 하나씩 로드 |
| Claude Code | `CLAUDE.md` 또는 `AGENTS.md` | 프로젝트 루트 | 자동 로드 | 메인 파일에서 `@security.md`로 참조하거나 하위 디렉터리에 넣어 필요 시 로드 |
| OpenAI Codex | `AGENTS.md` | 프로젝트 루트 | 자동 로드 | 메인 파일에서 `@security.md`로 참조 |
| Cursor | `AGENTS.md` | `.cursor/rules/` | 자동 로드(rules는 glob 매칭으로 적용 범위 지정 가능) | 같은 이름의 파일을 같은 디렉터리에 함께 배치 |
| Gemini CLI | `GEMINI.md` | 프로젝트 루트 | 자동 로드 | 이름을 바꿔 함께 넣거나 `@`로 참조 |
| Google Jules | `JULES.md` | 프로젝트 루트 | 자동 로드 | 위와 동일 |
| Aider | `CONVENTIONS.md` | 프로젝트 루트 | 자동 로드 | 내용을 병합하거나 파일로 나눠 참조 |
| Windsurf | `.windsurfrules` | 프로젝트 루트 | 자동 로드 | 위와 동일 |
| GitHub Copilot Agent | `AGENTS.md` | 프로젝트 루트 | 자동 로드 | `@security.md`로 참조 |
| 모든 MCP 클라이언트 | `mentor-mcp` 경유 | stdio(`node mcp/dist/index.js`) | 자동(resources + tools) | 모든 모듈을 MCP 리소스 `mentor://prompts/{lang}/{module}`로 노출 |

## 도구별 상세 설명

### opencode
1. `prompts/AGENTS.md`를 프로젝트 루트에 복사합니다.
2. opencode는 매 세션 AGENTS.md를 자동으로 로드하므로 수동 조작이 필요 없습니다.
3. 보안/스타일/워크플로우가 필요할 때 `@security.md`, `@style.md`, `@workflow.md`를 필요 시 로드합니다.
4. 장기 프로젝트: 규칙은 AGENTS.md에 정착시킵니다. 연결이 끊기면 `opencode --continue`로 복구합니다.

### Claude Code
1. `prompts/AGENTS.md`를 복사해 `CLAUDE.md`로 이름을 바꿉니다(또는 `AGENTS.md` 그대로 유지 — 최신 버전은 자동으로 인식합니다).
2. 프로젝트 루트에 두면 매 세션마다 자동으로 로드됩니다.
3. 기타 모듈은 `CLAUDE.md`에서 `@security.md`로 참조하거나, 직접 내용을 덧붙여 병합합니다.
4. 하위 디렉터리의 `CLAUDE.md`는 해당 디렉터리에 진입할 때 필요 시 로드됩니다.

### OpenAI Codex
1. `prompts/AGENTS.md`를 프로젝트 루트에 복사합니다(Codex는 루트의 `AGENTS.md`를 자동 로드).
2. 기타 모듈은 `AGENTS.md`에서 `@security.md`로 참조합니다.
3. 연결이 끊기면 `codex --resume`(또는 `codex exec --resume`)으로 복구합니다.

### Cursor
1. `prompts/AGENTS.md`를 `.cursor/rules/` 디렉터리에 복사합니다(Agent가 rules를 자동 로드).
2. 특정 파일 범위에만 적용하려면 `.mdc` 형식으로 변환하고 frontmatter에 `globs` 매칭을 추가합니다.
3. 기타 모듈도 같은 이름의 파일로 함께 `.cursor/rules/`에 넣습니다.

### Gemini CLI
1. `prompts/AGENTS.md`를 복사해 `GEMINI.md`로 이름을 바꾸고 프로젝트 루트에 두면 자동으로 로드됩니다.
2. 기타 모듈은 `GEMINI.md`에 병합하거나 필요할 때 `@`로 참조합니다.

### Google Jules
1. `prompts/AGENTS.md`를 복사해 `JULES.md`로 이름을 바꾸고 프로젝트 루트에 두면 자동으로 로드됩니다.

### Aider
1. `prompts/AGENTS.md`를 복사해 `CONVENTIONS.md`로 이름을 바꾸고 프로젝트 루트에 두면 편집 세션에서 자동으로 로드됩니다.

### Windsurf
1. `prompts/AGENTS.md`를 복사해 `.windsurfrules`로 이름을 바꾸고 프로젝트 루트에 두면 자동으로 로드됩니다.

### GitHub Copilot Agent
1. `prompts/AGENTS.md`를 프로젝트 루트에 복사하면 자동으로 로드됩니다. 기타 모듈은 `@security.md`로 참조합니다.

### MCP(Model Context Protocol)
1. 서버 빌드: `cd mcp && npm install && npm run build`
2. MCP 클라이언트를 `node <repo>/mcp/dist/index.js`로 지정
3. 서버는 프롬프트를 리소스(`mentor://prompts/{lang}/{module}`)와 도구(`install`, `detect_tool`, `list_languages`, `list_modules`, `generate_resource_estimate`)로 노출
4. 자세한 내용은 `mcp/README.md` 참조

## mentor CLI로 한 번에 설치

```bash
mentor install          # 대화형: 언어 선택 → 모듈 선택(기본: agent) → 도구 자동 인식/선택
mentor install --lang zh-CN --modules agent,security --cli claude-code
mentor add workflow     # 모듈 추가
mentor list             # 설치된 모듈 확인
```

`mentor`는 위 표의 규칙에 따라 각 도구가 요구하는 파일 이름과 위치로 자동으로 파일을 작성합니다(Claude Code → `CLAUDE.md`, Cursor → `.cursor/rules/`, 그 외 → `AGENTS.md` 등).

## 완전판 프롬프트

모듈로 나눌 필요가 없다면 `prompts/개발 멘토 완전판 프롬프트.md`(네 모듈 병합판, 한 번에 로드)를 바로 사용할 수 있습니다.
