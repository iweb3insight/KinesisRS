# FreedomAgent 상세 사용 가이드

## 1. 핵심 설계 이념
FreedomAgent는 **에이전트 우선(Agent-First)** 거래 시스템입니다. 모든 설계는 LLM 에이전트(Gemini, Claude 등)가 복잡한 거래를 안전하고 정확하게 실행할 수 있도록 이루어졌습니다.

- **상태 비저장(Stateless)**: 각 명령에는 실행에 필요한 모든 컨텍스트가 포함되어 있습니다.
- **JSON 우선**: 에이전트가 `TradeResult`를 정확하게 파싱할 수 있도록 항상 `--json` 플래그를 사용하는 것을 권장합니다.
- **보안 제일**: 시뮬레이션 검증을 강제하기 위해 기본적으로 `--dry-run`이 활성화되어 있습니다.

## 2. 상호작용 베스트 프랙티스 (에이전트용)

### 매수 워크플로우
1. **견적(Quote)**: `quote`를 실행합니다. `amount_out`을 파싱하여 사용자에게 제시합니다.
2. **리스크 평가**: `buy --dry-run`을 실행합니다.
   - `stages` 내의 `duration_ms`를 확인합니다.
   - `gas_estimate`를 확인합니다.
   - 성공하면 시뮬레이션 결과를 보여주고 사용자에게 최종 확인을 요청합니다.
3. **실제 거래**: 사용자 확인 후, `buy --no-dry-run`을 실행합니다.

### 매도 워크플로우
- 매도 명령은 승인(`approve`)이 필요한지 자동으로 감지합니다(BSC의 경우).
- `stages`에 `approve` 단계가 나타나면 승인 작업이 발생했음을 의미합니다.

## 3. Solana 특화 기능 사용

### Jito 번들 가속
Solana 체인에서는 프런트 러닝(MEV)을 방지하거나 혼잡 시 거래를 성사시키기 위해 Jito를 사용해야 합니다.
```bash
./kinesis_rs buy <TOKEN> 0.1 --chain solana --jito-tip 0.001
```
- **파라미터**: `--jito-tip` 단위는 SOL입니다. 권장 범위: 0.0001 - 0.01.

### Raydium 스마트 라우팅
Pump.fun 이외의 토큰(예: USDC, SOL/USDT 풀 등)의 경우, 실행기는 Raydium V3 Trade API를 자동으로 호출하여 최적의 경로를 탐색합니다.

## 4. 주요 오류 코드 및 처리 제안

| 오류 메시지 | 원인 | 제안 |
| :--- | :--- | :--- |
| `AccountNotFound` | 지갑 잔액이 0이거나 초기화되지 않음 | 기본 토큰(BNB/SOL)을 입금하세요 |
| `SlippageExceeded` | 가격 변동성이 너무 큼 | `--slippage`를 늘리세요 (예: 25.0) |
| `RouteNotFound` | 유동성 부족 또는 API 미색인 | 토큰 주소를 확인하거나 소액으로 시도하세요 |
| `Simulation failed` | 로직 실행이 리버트됨 | `raw_revert_data`를 확인하여 상세 원인을 파악하세요 |

## 5. 성능 감사
`TradeResult` 내의 `stages` 배열을 파싱함으로써 에이전트는 다음을 계산할 수 있습니다.
- **API 지연 시간**: `quote` 단계의 `duration_ms`.
- **실행 지연 시간**: `buy`/`sell` 단계의 `duration_ms`.
- **총 소요 시간**: 모든 단계의 `duration_ms` 합계.
