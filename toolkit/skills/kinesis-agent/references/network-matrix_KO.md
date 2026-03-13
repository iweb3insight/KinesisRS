# 네트워크 매트릭스

## 지원 네트워크

| 네트워크 | 체인 ID | RPC URL | 상태 |
|----------|----------|---------|------|
| BSC Mainnet | 56 | bsc-dataseed.binance.org | ✅ |
| BSC Testnet | 97 | data-seed-prebsc-1-s1.binance.org:8545 | ✅ |
| Solana Mainnet | - | api.mainnet-beta.solana.com | ✅ |
| Solana Devnet | - | api.devnet.solana.com | ⚠️ |
| Solana Testnet | - | api.testnet.solana.com | ⚠️ |

---

## Solana 네트워크 상세 비교

### Mainnet (api.mainnet-beta.solana.com)

| 기능 | 상태 | 설명 |
|------|------|------|
| Quote | ✅ | 정상 작동 |
| Buy (Raydium) | ✅ | SOL 잔액 필요 |
| Buy (Pump.fun) | ✅ | SOL 잔액 필요 |
| Sell | ✅ | 토큰 잔액 필요 |
| Balance | ✅ | 정상 작동 |
| Detect | ✅ | 정상 작동 |

**테스트용 개인키 잔액:** 0 SOL (실제 자금 없음)

---

### Devnet (api.devnet.solana.com)

| 기능 | 상태 | 설명 |
|------|------|------|
| Quote | ✅ | 정상 작동 |
| Buy (Raydium) | ❌ | ATA 오류 |
| Buy (Pump.fun) | ⚠️ | 테스트되지 않음 |
| Sell | ❌ | ATA 오류 |
| Balance | ✅ | 정상 작동 |
| Detect | ✅ | 정상 작동 |

**테스트용 개인키 잔액:** 3.67 SOL

**알려진 문제:**
- Raydium 거래는 Address Lookup Table (LUT)을 사용합니다.
- Devnet에 LUT가 존재하지 않아 `Transaction loads an address table account that doesn't exist` 오류가 발생합니다.

---

### Testnet (api.testnet.solana.com)

| 기능 | 상태 | 설명 |
|------|------|------|
| Quote | ✅ | 정상 작동 |
| Buy (Raydium) | ❌ | ATA 오류 |
| Buy (Pump.fun) | ⚠️ | 테스트되지 않음 |
| Sell | ❌ | ATA 오류 |
| Balance | ✅ | 정상 작동 |
| Detect | ✅ | 정상 작동 |

**테스트용 개인키 잔액:** 3.00 SOL

**알려진 문제:** Devnet과 동일

---

## BSC 네트워크

### Mainnet

| 기능 | 상태 | 설명 |
|------|------|------|
| Quote | ✅ | PancakeSwap |
| Buy | ✅ | BNB 잔액 필요 |
| Sell | ✅ | 토큰 잔액 필요 |
| Approve | ✅ | 자동 처리됨 |
| Balance | ✅ | 정상 작동 |

### Testnet

| 기능 | 상태 | 설명 |
|------|------|------|
| Quote | ✅ | PancakeSwap |
| Buy | ✅ | 테스트용 BNB 필요 |
| Sell | ✅ | 테스트용 토큰 필요 |
| Balance | ✅ | 정상 작동 |

---

## 네트워크 선택 권장 사항

### 개발/테스트

| 시나리오 | 권장 네트워크 | 이유 |
|----------|---------------|------|
| 빠른 견적 테스트 | Devnet/Testnet | 무료, 빠름 |
| Pump.fun 테스트 | Mainnet (dry-run) | Devnet에 Pump.fun 없음 |
| Raydium 테스트 | Mainnet (dry-run) | Devnet ATA 문제 |
| 통합 테스트 | Testnet | 메인넷과 더 유사함 |

### 프로덕션 환경

| 시나리오 | 권장 네트워크 | 이유 |
|----------|---------------|------|
| 실제 거래 | Mainnet | 유일한 옵션 |
| 거래 검증 | Mainnet (dry-run) | 실제 환경 시뮬레이션 |

---

## 환경 변수 설정

```bash
# Solana
export SOL_RPC_URL="https://api.mainnet-beta.solana.com"  # Mainnet
export SOL_RPC_URL="https://api.devnet.solana.com"       # Devnet
export SOL_RPC_URL="https://api.testnet.solana.com"      # Testnet

# BSC
export BSC_RPC_URL="https://bsc-dataseed.binance.org/"   # Mainnet
export BSC_RPC_URL="https://data-seed-prebsc-1-s1.binance.org:8545/"  # Testnet
```

---

## 멀티 RPC 설정

쉼표로 구분된 여러 RPC를 지원합니다:

```bash
# BSC
export BSC_RPC_URL="https://bsc-dataseed.binance.org/,https://bsc-dataseed1.binance.org/,https://bsc-dataseed2.binance.org/"

# Solana (멀티 RPC 백업)
export SOL_RPC_URL="https://api.mainnet-beta.solana.com,https://solana-api.projectserum.com"
```

---

## 테스트 토큰

### Solana Devnet/Testnet

| 토큰 | 주소 | 용도 |
|------|------|------|
| - | - | 현재 테스트 가능한 토큰 없음 |

### Solana Mainnet

| 토큰 | 주소 | 유동성 |
|------|------|--------|
| BONK | DezXAX8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 | 높음 |
| USDC | EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v | 높음 |
| SOL | So11111111111111111111111111111111111111112 | 최고 |

### BSC Testnet

| 토큰 | 주소 |
|------|------|
| BNB | - (네이티브) |
| 테스트 토큰 | 0x... |

---

## 문제 해결

### Devnet/Testnet ATA 오류

```
RPC Error -32602: invalid transaction: Transaction loads an address table account that doesn't exist
```

**해결 방법:**
1. Mainnet dry-run으로 테스트
2. 향후 버전의 수정을 대기

### ROUTE_NOT_FOUND

```
Raydium API error: ROUTE_NOT_FOUND
```

**원인:** 토큰이 Raydium에 유동성 풀을 가지고 있지 않음

**해결 방법:**
1. 유동성이 있는 토큰으로 변경
2. 토큰이 Raydium으로 졸업할 때까지 대기
