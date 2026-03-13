---
name: kinesis-agent
description: opencode/Gemini/Claude/OpenClaw를 지원하는 통합 크로스 플랫폼 거래 실행 에이전트. 바이너리를 자동 다운로드하고 Solana/BSC 거래를 실행합니다.
binary:
  latest: https://github.com/iweb3insight/KinesisRS/releases/latest
  version: v0.6.5
  install: |
    # 자동 설치
    curl -sL https://raw.githubusercontent.com/iweb3insight/KinesisRS/main/scripts/install.sh | bash
    
    # 또는 수동 다운로드
    # https://github.com/iweb3insight/KinesisRS/releases/latest
platforms:
  - darwin-arm64
  - darwin-amd64
  - linux-amd64
  - windows-amd64
---

# Kinesis Agent - 크로스 플랫폼 거래 실행

## 설치

### 자동 설치

```bash
curl -sL https://raw.githubusercontent.com/iweb3insight/KinesisRS/main/scripts/install.sh | bash
```

### 수동 설치

```bash
# macOS ARM64
curl -sL https://github.com/iweb3insight/KinesisRS/releases/download/v0.6.5/kinesis-rs-vv0.6.5-macos-arm64.tar.gz -o /tmp/kinesis.tar.gz
tar -xzf /tmp/kinesis.tar.gz -C /tmp/
chmod +x /tmp/kinesis-rs && mv /tmp/kinesis-rs ~/.kinesis/

# Linux
curl -sL https://github.com/iweb3insight/KinesisRS/releases/download/v0.6.5/kinesis-rs-vv0.6.5-linux-amd64.tar.gz -o /tmp/kinesis.tar.gz

# Windows
# .zip 파일을 다운로드하고 압축 해제
```

### 설치 확인

```bash
~/.kinesis/kinesis-rs --version
~/.kinesis/kinesis-rs wallet
```

## 환경 변수 설정

```bash
# Solana 네트워크
export SOL_RPC_URL="https://api.mainnet-beta.solana.com"  # Mainnet
# export SOL_RPC_URL="https://api.devnet.solana.com"       # Devnet
# export SOL_RPC_URL="https://api.testnet.solana.com"      # Testnet

# BSC 네트워크
export BSC_RPC_URL="https://bsc-dataseed.binance.org/"

# 선택 사항: 프록시
export HTTPS_PROXY="http://proxy:8080"
```

## 거래 실행 (3단계 검증)

```
1. Quote    → 견적(쿼트) 확인
2. Dry-run  → 거래 시뮬레이션
3. Execute  → 실제 거래
```

---

## 핵심 워크플로우

### 구매 워크플로우 (Buy)

```
Step 1: quote <TOKEN> <AMOUNT>
        ↓
Step 2: buy <TOKEN> <AMOUNT> --dry-run
        ↓
Step 3: buy <TOKEN> <AMOUNT> --no-dry-run (사용자 확인)
```

### 판매 워크플로우 (Sell)

```
Step 1: sell <TOKEN> <AMOUNT> --dry-run
        ↓
Step 2: sell <TOKEN> <AMOUNT> --no-dry-run
```

---

## 플랫폼 어댑터

| 플랫폼 | 호출 방법 | 참고 문서 |
|------|---------|----------|
| opencode | Shell | adapters/opencode.md |
| openclaw | Shell | adapters/openclaw.md |
| Gemini | mcp__local__execute | adapters/gemini.md |
| Claude | MCP/Bash | adapters/claude.md |

---

## 지원 네트워크

| 네트워크 | SOL_RPC_URL | 잔액 | Buy 상태 |
|------|-------------|------|----------|
| Mainnet | api.mainnet-beta.solana.com | 입금 필요 | ✅ |
| Devnet | api.devnet.solana.com | 3.67 SOL | ⚠️ ATA |
| Testnet | api.testnet.solana.com | 3.00 SOL | ⚠️ ATA |

상세 정보: references/network-matrix.md

---

## 오류 처리

| 오류 | 원인 | 해결책 |
|------|------|----------|
| AccountNotFound | 잔액이 0 | SOL 입금 |
| ATA 오류 | Devnet에 LUT가 없음 | Pump.fun 경로 사용 |
| ROUTE_NOT_FOUND | 토큰이 인덱싱되지 않음 | 대기 또는 토큰 변경 |

상세 정보: references/error-codes.md

---

## 참고 자료

- [Trading API](references/trading-api.md)
- [Network Matrix](references/network-matrix.md)
- [Error Codes](references/error-codes.md)
