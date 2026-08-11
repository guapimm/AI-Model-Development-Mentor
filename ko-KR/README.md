🌍 다른 언어 → [English](../README.md)

# AI Model Mentor (한국어)

> **AI 코딩 어시스턴트를 신중한 10년 경력 풀스택 멘토로 만들어 드립니다 — 순수 프롬프트, 제로 의존성.**

---

## 이게 무엇인가요?

**순수 프롬프트(Prompt) 프레임워크**입니다. AI 코딩 어시스턴트를 **10년 경력의 풀스택 아키텍트 겸 개발 멘토**로 만들어, 코딩 경험이 전혀 없는 초보자를 위한 개발 안내자가 되게 합니다.

이 프레임워크는 AI가 일련의 "철칙"을 따르도록 강제합니다 — *보안 최우선, 투명한 로직, 문서 우선, Token 효율, 단계적 구현*을 AI의 기본 동작으로 만듭니다. 그 결과, AI는 단순히 *코드를 작성하는 것*을 넘어 **안전하고, 유지보수하기 쉬우며, 문서화된 코드**를 작성하게 됩니다.

> ⚠️ 현재 지원: **Xiaomi MIMO CLI**. 다른 제품(Claude Code, Cursor 등)용 최적화 버전은 계획 중입니다 — 필요하시면 댓글로 남겨 주세요.

## 핵심 모듈 (Xiaomi MIMO 버전)

| 모듈 | 파일 | 용도 |
|------|------|------|
| 🧑‍🏫 멘토 역할 | [AGENTS.md](./xiaomi-mimo/AGENTS.md) | 아키텍트-멘토 페르소나 + 6대 철칙 + 보안 자가 점검 체크리스트 ★ 핵심, 필수 |
| 🛡️ 보안 규범 | [security.md](./xiaomi-mimo/security.md) | 8대 보안 영역: 키 관리 / 입력 검증 / 데이터베이스 / XSS / 파일 시스템 / 외부 요청 / 예외 처리 / 성능 |
| 🎨 상호작용 스타일 | [style.md](./xiaomi-mimo/style.md) | 생활 속 비유, 단계 태그, 먼저 확인 후 실행, 점진적 복잡도 |
| 📋 개발 워크플로우 | [workflow.md](./xiaomi-mimo/workflow.md) | 문서 체계 / 프론트엔드 매핑 프로토콜 / 배포·롤백 / 테스트 루프 / 버전 앵커 |

### 6대 철칙

1. **코드 = 문서** — 모든 코드에 "왜 이렇게 하는지"를 설명하는 주석 포함
2. **보안 우선** — 하드코딩 키 금지, 엄격한 입력 검증, 파라미터화 쿼리, XSS 방지
3. **무파괴 변경** — 먼저 의존성을 분석하고, 수정을 【필수 수정】/【선택적 최적화】로 표시
4. **단계별 실행** — 한 번의 출력에 300줄을 초과하지 않고, 각 단계마다 확인을 기다림
5. **모듈화 격리** — 파일당 최대 500줄, 확장 인터페이스 예약
6. **Token 효율** — 매 대화가 끝나면 컨텍스트 요약 + 재개 암호 생성

## 빠른 시작 (3단계)

```bash
# 1. 멘토 역할을 프로젝트로 복사 (이름을 AGENTS.md로 변경)
cp xiaomi-mimo/AGENTS.md AGENTS.md

# 2. (권장) 보안/스타일/워크플로우 규범도 함께 추가
cp xiaomi-mimo/security.md security.md
cp xiaomi-mimo/style.md style.md
cp xiaomi-mimo/workflow.md workflow.md
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
│   └── xiaomi-mimo/     # Xiaomi MIMO 버전
│       ├── AGENTS.md    # 멘토 역할 (ZH)
│       ├── security.md  # 보안 규범 (ZH)
│       ├── style.md     # 상호작용 스타일 (ZH)
│       └── workflow.md  # 개발 워크플로우 (ZH)
├── en-US/               # 영어
│   ├── README.md        # 영어 입구
│   └── xiaomi-mimo/     # Xiaomi MIMO 버전
│       ├── AGENTS.md    # 멘토 역할 (EN)
│       ├── security.md  # 보안 규범 (EN)
│       ├── style.md     # 상호작용 스타일 (EN)
│       └── workflow.md  # 개발 워크플로우 (EN)
└── ko-KR/               # 한국어
    ├── README.md        # 한국어 입구 (이 파일)
    └── xiaomi-mimo/     # Xiaomi MIMO 버전
        ├── AGENTS.md    # 멘토 역할 (KO)
        ├── security.md  # 보안 규범 (KO)
        ├── style.md     # 상호작용 스타일 (KO)
        └── workflow.md  # 개발 워크플로우 (KO)
```

> 📦 새 제품 빌드는 각 언어 디렉터리 아래에 형제 디렉터리로 추가됩니다. 예: `zh-CN/claude-code/`, `en-US/cursor/`.

## 자주 묻는 질문

**Q: 4개 모듈이 모두 필요한가요?**
A: 아닙니다. `AGENTS.md`만 필수입니다. 더 강한 안전 장치가 필요하면 `security.md`를, 더 친근한 대화 경험이 필요하면 `style.md`를 추가하세요.

**Q: 다른 AI 제품에서도 동작하나요?**
A: 현재는 Xiaomi MIMO만 지원합니다. 다른 제품용 최적화 버전은 준비 중입니다 — 필요한 것을 댓글로 남겨 주세요.

## 라이선스

[MIT License](../LICENSE) © 2026 guapimm
