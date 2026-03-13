# 오류 코드 참조

## 오류 유형

| 오류 유형 | 설명 | 주요 원인 |
|----------|------|----------|
| `rpc_error` | RPC 요청 실패 | 네트워크 문제, RPC 사용 불가 |
| `simulation_failed` | 거래 시뮬레이션 실패 | 로직 리버트, 잔액 부족 |
| `send_failed` | 거래 전송 실패 | 서명 문제, 체인 혼잡 |
| `config_error` | 설정 오류 | 개인키 누락, RPC 미설정 |
| `invalid_input` | 입력 매개변수 오류 | 잘못된 주소, 음수 금액 |
| `contract_error` | 컨트랙트 실행 오류 | 리버트, 권한 문제 |

---

## 일반적인 오류 및 해결 방법

### 1. AccountNotFound

**오류 메시지:**
```json
{"error": {"type": "contract_error", "message": "Simulation failed: \"AccountNotFound\""}}
```

**원인:** 지갑 잔액이 0

**해결 방법:**
```bash
# 지갑에 SOL 입금
# 지갑 주소: 88DqDXNAQZHWscK5HjPavDkBCvsfzmUrDvAV9ZTY5jMv
```

---

### 2. ATA 오류

**오류 메시지:**
```
RPC Error -32602: invalid transaction: Transaction loads an address table account that doesn't exist
```

**원인:** Devnet/Testnet에 Raydium이 생성한 Address Lookup Table (LUT)이 존재하지 않음

**영향을 받는 네트워크:** Devnet, Testnet

**해결 방법:**
```bash
# 방법 1: Mainnet dry-run 사용
export SOL_RPC_URL="https://api.mainnet-beta.solana.com"

# 방법 2: Pump.fun 경로 사용 (가능한 경우)
# Pump.fun은 LUT를 사용하지 않음
```

---

### 3. ROUTE_NOT_FOUND

**오류 메시지:**
```json
{"error": {"message": "ROUTE_NOT_FOUND"}}
```

**원인:** Raydium API가 해당 토큰의 유동성 풀을 인덱싱하지 않음

**일반적인 시나리오:**
- 새로 생성된 Pump.fun 토큰
- 아직 "졸업"하지 않은 토큰
- 방금 졸업했지만 아직 동기화되지 않은 토큰

**해결 방법:**
```bash
# 방법 1: 유동성이 있는 토큰으로 변경
# 예: BONK, USDC, SOL

# 방법 2: 토큰이 Raydium으로 졸업할 때까지 대기

# 방법 3: Pump.fun에서 직접 거래 (졸업 전인 경우)
```

---

### 4. REQ_SWAP_RESPONSE_ERROR

**오류 메시지:**
```json
{"error": {"message": "Failed to parse Raydium transaction: ... REQ_SWAP_RESPONSE_ERROR"}}
```

**원인:** Raydium API가 유효하지 않은 응답을 반환함

**일반적인 시나리오:**
- Quote API 실패 후 2차 호출
- 일시적인 API 문제

**해결 방법:**
```bash
# 재시도
kinesis-rs buy <TOKEN> <AMOUNT> --dry-run

# 또는 대기 후 재시도
```

---

### 5. REQ_COMPUTE_UNIT_PRICE_MICRO_LAMPORTS_ERROR

**오류 메시지:**
```json
{"error": {"message": "REQ_COMPUTE_UNIT_PRICE_MICRO_LAMPORTS_ERROR"}}
```

**원인:** Compute Unit Price 설정 문제

**해결 방법:**
```bash
# API 복구 대기 또는 재시도
```

---

### 6. SlippageExceeded

**오류 메시지:**
```json
{"error": {"revert_reason": "SlippageExceeded"}}
```

**원인:** 가격 변동이 설정된 슬리피지를 초과함

**해결 방법:**
```bash
# 슬리피지 증가
kinesis-rs buy <TOKEN> <AMOUNT> --slippage 25 --chain solana

# 또는 금액 감소
kinesis-rs buy <TOKEN> <AMOUNT> --slippage 15 --chain solana
```

---

### 7. Insufficient Liquidity

**오류 메시지:**
```json
{"error": {"revert_reason": "FreedomRouter: INSUFFICIENT_LIQUIDITY"}}
```

**원인:** 풀 유동성 부족

**해결 방법:**
```bash
# 구매 금액 감소
kinesis-rs buy <TOKEN> 0.01 --chain solana

# 유동성 회복 대기
```

---

### 8. Insufficient Gas / Insufficient Funds

**오류 메시지:**
```json
{"error": {"revert_reason": "insufficient funds for gas * price + value"}}
```

**원인:** 가스비 지불을 위한 잔액 부족

**해결 방법:**
```bash
# 네이티브 토큰 (SOL/BNB) 입금
```

---

### 9. Token account not found

**오류 메시지:**
```json
{"error": {"message": "Token account not found: <TOKEN_ADDRESS>"}}
```

**원인:** 지갑에 해당 토큰을 보유하고 있지 않음

**해결 방법:**
```bash
# 먼저 토큰을 구매하여 ATA 생성
# 또는 토큰 주소가 정확한지 확인
```

---

### 10. Invalid Token Address

**오류 메시지:**
```json
{"error": {"message": "Invalid token address"}}
```

**원인:** 토큰 주소 형식 오류

**해결 방법:**
```bash
# 주소 형식 확인
# Solana: Base58 인코딩, 32-44자
# BSC: 0x로 시작, 40자 16진수
```

---

## 오류 처리 흐름

```
사용자 요청
    ↓
명령 실행
    ↓
┌─────────────────────────────────────┐
│  성공?                              │
│  ↓ Yes                              │
│  성공 응답 반환                     │
│  ↓ No                               │
│  오류 유형 분석                     │
│  ↓                                  │
│  ┌─────────────────────────────────┐│
│  │ rpc_error                       ││
│  │  - 네트워크 연결 확인            ││
│  │  - RPC 변경                     ││
│  ├─────────────────────────────────┤│
│  │ simulation_failed                ││
│  │  - 잔액 확인                    ││
│  │  - 승인 확인                    ││
│  │  - 슬리피지 조정                ││
│  ├─────────────────────────────────┤│
│  │ contract_error                  ││
│  │  - revert_reason 분석            ││
│  │  - 구체적인 해결 방법 참조        ││
│  └─────────────────────────────────┘│
└─────────────────────────────────────┘
    ↓
오류 응답 반환
    ↓
사용자에게 오류 + 제안 표시
```

---

## 디버깅 팁

### 1. 디버그 로그 활성화

```bash
RUST_LOG=debug kinesis-rs --json quote ...
```

### 2. 네트워크 연결 확인

```bash
# RPC 직접 테스트
curl -X POST https://api.mainnet-beta.solana.com -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}'

# Raydium API 테스트
curl "https://transaction-v1.raydium.io/compute/swap-base-in?inputMint=So11111111111111111111111111111111111111112&outputMint=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v&amount=1000000000&slippageBps=50"
```

### 3. 잔액 확인

```bash
kinesis-rs --json balance --chain solana
```
