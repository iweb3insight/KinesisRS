# Trading API 참조

## 명령 형식

```bash
kinesis-rs <COMMAND> [OPTIONS]
```

## 글로벌 옵션

| 옵션 | 설명 | 기본값 |
|--------|-------------|---------------|
| `--json` | JSON 형식 출력 | false |
| `--chain` | 블록체인 유형 | bsc |
| `--wallet` | 지갑 인덱스 | 1 |
| `--dry-run` | 거래 시뮬레이션 | true |
| `--no-dry-run` | 실제 거래 실행 | false |

## 명령 목록

### quote

토큰 견적(쿼트)을 가져옵니다.

```bash
kinesis-rs quote <TOKEN_ADDRESS> <AMOUNT> [OPTIONS]

# 매개변수
TOKEN_ADDRESS  # 토큰 컨트랙트 주소
AMOUNT         # 수량

# 옵션
--action buy|sell    # 거래 방향 (기본값: buy)
-c, --chain bsc|solana  # 블록체인 (기본값: bsc)

# 예시
kinesis-rs --json quote DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.1 --action buy --chain solana
```

**출력 예시:**
```json
{
  "success": true,
  "amount_out": "1450172355",
  "path": "Raydium"
}
```

---

### buy

토큰을 구매합니다.

```bash
kinesis-rs buy <TOKEN_ADDRESS> <AMOUNT> [OPTIONS]

# 매개변수
TOKEN_ADDRESS  # 대상 토큰 주소 (구매할 토큰)
AMOUNT         # 지불할 네이티브 토큰(SOL/BNB) 수량

# 옵션
--slippage PERCENT    # 슬리피지 허용 범위 % (기본값: 15)
--tip-rate PERCENT    # 개발자 팁 % (Solana, 기본값: 0)
--jito-tip SOL       # Jito 팁 (Solana, 기본값: 0)
-c, --chain          # 블록체인

# 예시
# Dry-run (기본값)
kinesis-rs --json buy DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --slippage 15 --chain solana

# 실제 거래
kinesis-rs --json buy DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --slippage 15 --chain solana --no-dry-run

# Jito 팁 포함
kinesis-rs --json buy DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --jito-tip 0.001 --chain solana
```

**출력 예시 (Dry-run):**
```json
{
  "success": true,
  "chain": "solana",
  "stages": [
    {"name": "cli_parse", "duration_ms": 5},
    {"name": "executor_init", "duration_ms": 120},
    {"name": "quote", "duration_ms": 350},
    {"name": "simulate_execution", "duration_ms": 580}
  ],
  "amount_out": "1450172355",
  "gas_estimate": 5000,
  "tx_hash": null,
  "error": null
}
```

---

### sell

토큰을 판매합니다.

```bash
kinesis-rs sell <TOKEN_ADDRESS> <AMOUNT> [OPTIONS]

# 매개변수
TOKEN_ADDRESS  # 토큰 주소 (판매할 토큰)
AMOUNT         # 판매할 토큰 수량

# 옵션
--slippage PERCENT    # 슬리피지 허용 범위 % (기본값: 15)
--tip-rate PERCENT    # 개발자 팁 % (Solana)
--jito-tip SOL       # Jito 팁 (Solana)
-c, --chain          # 블록체인

# 예시
kinesis-rs --json sell DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 1000 --slippage 15 --chain solana
```

**참고:** BSC는 자동으로 `approveIfNeeded`를 처리합니다.

---

### balance

잔액을 조회합니다.

```bash
kinesis-rs balance [OPTIONS]

# 옵션
--token-address ADDRESS  # 토큰 주소 (비어 있으면 네이티브 토큰 조회)
-c, --chain              # 블록체인

# 예시
# SOL 잔액 조회
kinesis-rs --json balance --chain solana

# 토큰 잔액 조회
kinesis-rs --json balance --token-address DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 --chain solana
```

**출력 예시:**
```json
{
  "success": true,
  "asset": "Native SOL",
  "balance_formatted": "3.675360878",
  "balance_raw": "3675360878",
  "owner": "88DqDXNAQZHWscK5HjPavDkBCvsfzmUrDvAV9ZTY5jMv"
}
```

---

### wallet

지갑 주소를 표시합니다.

```bash
kinesis-rs wallet [OPTIONS]

# 옵션
--wallet INDEX  # 지갑 인덱스 (기본값: 1)

# 예시
kinesis-rs --json wallet
```

**출력 예시:**
```json
{
  "success": true,
  "wallets": {
    "1": {
      "bsc": "0x993D6C2e4FfeE5Fed15F5c0861d27a5BA62fCdBE",
      "solana": "88DqDXNAQZHWscK5HjPavDkBCvsfzmUrDvAV9ZTY5jMv"
    }
  }
}
```

---

### config

현재 설정을 표시합니다.

```bash
kinesis-rs --json config
```

---

### detect

토큰 경로를 감지합니다 (Solana 전용).

```bash
kinesis-rs detect <TOKEN_ADDRESS> --chain solana

# 예시
kinesis-rs --json detect DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 --chain solana
```

**출력 예시:**
```json
{
  "success": true,
  "token_address": "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263",
  "path": "Raydium"
}
```

---

## 오류 응답 형식

```json
{
  "success": false,
  "chain": "solana",
  "stages": [...],
  "error": {
    "type": "rpc_error|simulation_failed|send_failed|config_error|invalid_input|contract_error",
    "message": "오류 설명",
    "revert_reason": "컨트랙트 리버트 원인 (있는 경우)",
    "raw_revert_data": "원시 리버트 데이터 (있는 경우)"
  }
}
```
