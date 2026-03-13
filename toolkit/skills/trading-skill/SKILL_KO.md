---
name: kinesis-trading-skill
description: BSC 및 Solana를 위한 멀티 체인 거래 실행. PancakeSwap, Pump.fun 및 Raydium에서 토큰 구매/판매, 견적(쿼트) 및 잔액 확인에 사용. Jito 번들 제출 및 멀티 RPC 레이싱 지원.
---

# 거래 실행 스킬 (KinesisRS)

이 스킬은 Gemini CLI가 고성능 암호화폐 거래 에이전트로 작동할 수 있게 합니다.

## 핵심 워크플로우

### 1. 구매 워크플로우 (안전한 실행)
1. **견적 가져오기 (Quote)**: `quote`를 실행하여 실시간 가격을 확인합니다.
2. **시뮬레이션 (Simulate)**: `--dry-run`으로 실행하여 로직과 가스비를 확인합니다.
3. **실행 (Execute)**: 사용자 확인 후 `--no-dry-run`으로 실행합니다.

### 2. 판매 워크플로우 (자동 승인)
1. **시뮬레이션 (Simulate)**: BSC는 `approveIfNeeded`를 자동으로 처리합니다.
2. **실행 (Execute)**: `--no-dry-run`으로 실행합니다.

## 참고 자료

- **[QUICK_START.md](references/trading-api.md)**: CLI 명령 매핑 및 JSON 예시.
- **[USAGE_GUIDE.md](references/usage-guide.md)**: 에이전트 상호작용 패턴 및 문제 해결 심층 분석.
- **[SETUP.md](references/setup.md)**: 환경 변수 및 빌드 지침.

## 검증
`./kinesis-trading-skill/scripts/check_config.cjs`를 실행하여 환경을 확인하세요.
