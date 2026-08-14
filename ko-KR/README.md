🌍 다른 언어 → [English](../README.md)

# AI Model Mentor (한국어)

> **AI 코딩 어시스턴트를 신중한 10년 경력 풀스택 멘토로 만들어 드립니다 — 순수 프롬프트, 제로 의존성.**

---

## 이게 무엇인가요?

**순수 프롬프트(Prompt) 프레임워크**입니다. AI 코딩 어시스턴트를 **10년 경력의 풀스택 아키텍트 겸 개발 멘토**로 만들어, 코딩 경험이 전혀 없는 초보자를 위한 개발 안내자가 되게 합니다.

이 프레임워크는 AI가 일련의 "철칙"을 따르도록 강제합니다 — *보안 최우선, 투명한 로직, 문서 우선, Token 효율, 단계적 구현, 자원 통제*를 AI의 기본 동작으로 만듭니다. 그 결과, AI는 단순히 *코드를 작성하는 것*을 넘어 **안전하고, 유지보수하기 쉬우며, 문서화된 코드**를 작성하게 됩니다.

> ⚠️ 다중 도구 호환: 프롬프트는 모든 LLM 기반 코딩 도구(MIMO / Claude Code / Codex / Cursor 등)에서 사용할 수 있습니다. 도구별 로딩 방법은 [COMPATIBILITY.md](./COMPATIBILITY.md)를 참고하세요.

## 핵심 모듈 (다중 도구 호환)

| 모듈 | 파일 | 용도 |
|------|------|------|
| 🧑‍🏫 멘토 역할 | [AGENTS.md](./prompts/AGENTS.md) | 풀스택 아키텍트 멘토 페르소나 + 6대 철칙 + 보안·성능 자가 점검 체크리스트 ★ 핵심, 필수 |
| 🛡️ 보안 규범 | [security.md](./prompts/security.md) | 8대 보안 영역 규범: 키 관리 / 입력 검증 / 데이터베이스 / XSS / 파일 시스템 / 외부 요청 / 예외 처리 / 성능·리소스 |
| 🎨 상호작용 스타일 | [style.md](./prompts/style.md) | 생활 속 비유, 단계 태그, 먼저 확인 후 실행, 점진적 복잡도 |
| 📋 개발 워크플로우 | [workflow.md](./prompts/workflow.md) | 문서 체계 / 리소스 예측 / 데이터베이스 설계 / 프론트엔드 연동 프로토콜 / 배포·재해 복구 / 테스트 자가 점검 루프 / 버전 앵커 |

## 📦 추가 문서

- [COMPATIBILITY.md](./COMPATIBILITY.md) — 각 AI 도구(MIMO / Claude Code / Codex / Cursor 등) 로딩 방법
- [개발 멘토 완전판 프롬프트.md](./prompts/개발 멘토 완전판 프롬프트.md) — 모든 모듈을 통합한 한 번에 로드하는 완전판 프롬프트

## ⬇️ mentor CLI 설치 및 사용법

**방식 A: Go 바이너리 (권장, 제로 의존성, 크로스 플랫폼)**

GitHub Releases에서 해당 플랫폼의 `mentor` 실행 파일(v0.1.0, Windows / Linux / macOS 지원)을 다운로드하여 PATH에 추가한 후:

```bash
mentor install                        # 대화형 가이드: 언어 선택 → 모듈 선택(기본값 agent) → 도구 자동 감지
mentor install --lang zh-CN --modules agent,security --cli claude-code --dir ./proj
mentor add workflow                   # 모듈 추가
mentor list                           # 설치된 모듈 확인
mentor detect                         # 프로젝트에서 사용 중인 AI 도구 감지
mentor pack                           # 호환 가능한 skill 디렉터리 생성
```

`mentor`는 도구에 따라 올바른 파일 이름과 위치를 자동으로 기록합니다: MIMO/Codex → `AGENTS.md`, Claude Code → `CLAUDE.md`, Cursor → `.cursor/rules/`.

**방식 B: 수동 복사**

[COMPATIBILITY.md](./COMPATIBILITY.md)의 안내에 따라 `prompts/` 아래의 파일을 프로젝트의 해당 위치에 복사하세요.

> 지원 명령어: `install` / `add` / `remove` / `list` / `detect` / `pack`; 모듈: agent(기본값) / security / style / workflow / complete; 도구: mimo / claude-code / codex / cursor / other.

## 📖 사용 가이드（MIMO CLI）

### 명령어 빠른 요약

| 시나리오 | 조작 |
|------|------|
| 일상 개발 | 프로젝트 진입 → `/skill AGENTS.md` → 일반 대화 |
| 장기 프로젝트 | 처음 로드한 후 `/dream`으로 규칙을 MEMORY.md에 정착 |
| 예기치 못한 연결 끊김 | `mimo --continue`로 복구, skill 규칙은 그대로 유지 |
| 새 세션 직접 시작 | `/new` 실행 후, `/skill AGENTS.md`를 다시 실행해야 함 |

### 프로젝트 파일 구조

```
📁 my-project/
├── 📄 AGENTS.md          ← 메인 프롬프트
├── 📄 security.md        ← 보안 규범
├── 📄 workflow.md        ← 워크플로우 규범
├── 📄 style.md           ← 상호작용 스타일
└── 📁 src/
```

---

### 구체적 시나리오 데모

#### 시나리오 1: 일상적인 코드 작성 (AGENTS.md만 로드)

> 사용자: "사용자 목록을 가져오는 API를 작성해 주세요"

로드 필요: AGENTS.md(자동 로드됨, 별도 조작 불필요)

AI가 자동으로 수행하는 작업:

- 코드에 한국어 주석 포함
- 출력 전 안전 체크리스트 체크
- 단계별 실행(≤300줄)
- 단일 파일 500줄 이하

#### 시나리오 2: 로그인/회원가입 인터페이스 작성 (AGENTS.md + security.md 로드)

> 사용자: "사용자 로그인 기능을 작성해 주세요. security.md의 요구사항에 따라 진행해 주세요"

로드 필요:

```bash
/skill security.md
```

AI가 추가로 수행하는 작업:

- 비밀번호를 bcrypt 해시로 저장
- JWT Token에 만료 시간 설정
- 무차별 대입 공격 방어(로그인 실패 제한)
- SQL 주입 방어(파라미터화 쿼리)

#### 시나리오 3: 프로젝트를 처음부터 시작 (AGENTS.md + workflow.md 로드)

> 사용자: "블로그 시스템을 만들려고 합니다. workflow.md를 참고해서 프로젝트 골격을 잡아 주세요"

로드 필요:

```bash
/skill workflow.md
```

AI가 추가로 수행하는 작업:

- docs/architecture.md 생성(기술 스택 선정 + 아키텍처 다이어그램)
- docs/dev_log.md 생성(개발 로그 템플릿)
- docs/api_interface.md 생성(인터페이스 계약 템플릿)
- docs/SNAPSHOT.md 생성(프로젝트 스냅샷)
- backup.sh와 rollback.sh 스크립트 생성

#### 시나리오 4: AI 설명이 너무 어려움 (style.md 로드)

> 사용자: "style.md 방식으로 JWT가 무엇인지 생활 속 비유로 설명해 주세요"

로드 필요:

```bash
/skill style.md
```

AI가 추가로 수행하는 작업:

- "식당 멤버십 카드" 비유로 JWT 설명
- 단계 태그 [📋 요구사항 분석] 추가
- 결론 먼저, 세부사항은 나중에
- 2-3개의 선택지 제공

#### 시나리오 5: 배포 및 출시 (AGENTS.md + workflow.md 로드)

> 사용자: "workflow.md의 배포 규범에 따라 Docker 배포 설정을 작성해 주세요"

로드 필요:

```bash
/skill workflow.md
```

AI가 추가로 수행하는 작업:

- 개발/프로덕션 환경 설정 구분
- docker-compose.yml 생성
- health_check.sh 생성
- 백업 및 롤백 절차 안내

### ⚠️ 언제 로드하지 않아도 되나요?

| 로드할 필요가 없는 상황 | 이유 |
|---------------|------|
| 순수 기술 질문(예: "React useEffect는 어떻게 쓰나요") | AGENTS.md로 충분하며, workflow를 추가하면 오히려 방해 |
| CSS 스타일 하나 수정 | 보안 규범과 배포 절차가 필요 없음 |
| AI에게 문장 번역 요청 | 어떤 skill도 전혀 필요 없음 |
| 기존 코드의 간단한 리팩터링 | AGENTS.md의 안전 체크리스트로 충분 |

### 💡 한 문장 요약

> AGENTS.md는 기본 스킨이고, 나머지 세 개는 이펙트 플러그인입니다 — 필요할 때만 켜고 평소에는 꺼 두세요. Token도 아끼고 깔끔합니다.

### 6대 철칙

1. **코드 = 문서** — 모든 코드에 "왜 이렇게 하는지"를 설명하는 주석 포함
2. **보안 우선** — 하드코딩 키 금지, 엄격한 입력 검증, 파라미터화 쿼리, XSS 방지
3. **무파괴 변경** — 먼저 의존성을 분석하고, 수정을 【필수 수정】/【선택적 최적화】로 표시
4. **단계별 실행** — 한 번의 출력에 300줄을 초과하지 않고, 각 단계마다 확인을 기다림
5. **모듈화 격리** — 파일당 최대 500줄, 확장 인터페이스 예약
6. **성능과 자원 우선** — 데이터베이스 설계와 함께 인덱스 방안 출력, 조회 인터페이스 기본 페이징, 프로젝트 초기 3단계(메모리/디스크/CPU) 자원 예측 완료, 대용량 메모리 작업에는 해제 메커니즘 필수

## 빠른 시작 (3단계)

```bash
# 1. 멘토 역할을 프로젝트로 복사 (이름을 AGENTS.md로 변경)
cp prompts/AGENTS.md AGENTS.md

# 2. (권장) 보안/스타일/워크플로우 규범도 함께 추가
cp prompts/security.md security.md
cp prompts/style.md style.md
cp prompts/workflow.md workflow.md
```

3. Xiaomi MIMO를 실행하고 다음과 같이 말하세요:

> "저는 완전한 초보입니다. 제【프로젝트 요구사항 명세서】는 다음과 같습니다: 프로젝트 이름 ____, 핵심 목표 ____, 사용자 역할 ____, 핵심 운영 절차 ____, 반드시 저장할 데이터 ____. 단계 0: 환경 준비 및 기술 스택 선정부터 시작해 한 단계씩 안내해 주세요."

AI는 "설계 → 핵심 로직 → 화면 → 테스트" 순서로 진행하며, 모든 단계에서 여러분의 확인을 기다립니다.

## 파일 구조

```
AI_Model_Development_Mentor/
├── README.md            # 다국어 랜딩 페이지
├── LICENSE              # MIT License
├── zh-CN/               # 중국어
│   ├── README.md        # 중국어 입구
│   └── prompts/         # 프롬프트 모듈 (다중 도구 호환)
│       ├── AGENTS.md    # 멘토 역할 (ZH)
│       ├── security.md  # 보안 규범 (ZH)
│       ├── style.md     # 상호작용 스타일 (ZH)
│       └── workflow.md  # 개발 워크플로우 (ZH)
├── en-US/               # 영어
│   ├── README.md        # 영어 입구
│   └── prompts/         # 프롬프트 모듈 (다중 도구 호환)
│       ├── AGENTS.md    # 멘토 역할 (EN)
│       ├── security.md  # 보안 규범 (EN)
│       ├── style.md     # 상호작용 스타일 (EN)
│       └── workflow.md  # 개발 워크플로우 (EN)
└── ko-KR/               # 한국어
    ├── README.md        # 한국어 입구 (이 파일)
    └── prompts/         # 프롬프트 모듈 (다중 도구 호환)
        ├── AGENTS.md    # 멘토 역할 (KO)
        ├── security.md  # 보안 규범 (KO)
        ├── style.md     # 상호작용 스타일 (KO)
        └── workflow.md  # 개발 워크플로우 (KO)
```

> 📦 새 도구는 [COMPATIBILITY.md](./COMPATIBILITY.md)에 행으로 추가됩니다. 더 이상 제품별 디렉터리를 만들 필요가 없습니다.

## 자주 묻는 질문

**Q: 4개 모듈이 모두 필요한가요?**
A: 아닙니다. `AGENTS.md`만 필수입니다. 더 강한 안전 장치가 필요하면 `security.md`를, 더 친근한 대화 경험이 필요하면 `style.md`를 추가하세요.

**Q: 다른 AI 제품에서도 동작하나요?**
A: 네. 프롬프트는 특정 도구에 종속되지 않으므로 모든 LLM 기반 코딩 도구에서 동작합니다. 도구별 로딩 방법은 [COMPATIBILITY.md](./COMPATIBILITY.md)를 참고하세요.

## 라이선스

[MIT License](../LICENSE) © 2026 guapimm
