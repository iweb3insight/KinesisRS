---
name: kinesis-agent
description: Unified cross-platform trading execution Agent supporting opencode/Gemini/Claude/OpenClaw. Automatically downloads binary, executes Solana/BSC trades.
binary:
  latest: https://github.com/iweb3insight/KinesisRS/releases/latest
  version: v0.6.5
  install: |
    # Auto install
    curl -sL https://raw.githubusercontent.com/iweb3insight/KinesisRS/main/scripts/install.sh | bash
    
    # Or manual download
    # https://github.com/iweb3insight/KinesisRS/releases/latest
platforms:
  - darwin-arm64
  - darwin-amd64
  - linux-amd64
  - windows-amd64
---

# Kinesis Agent - Cross-platform Trading Execution

## Installation

### Auto Install

```bash
curl -sL https://raw.githubusercontent.com/iweb3insight/KinesisRS/main/scripts/install.sh | bash
```

### Manual Install

```bash
# macOS ARM64
curl -sL https://github.com/iweb3insight/KinesisRS/releases/download/v0.6.5/kinesis-rs-vv0.6.5-macos-arm64.tar.gz -o /tmp/kinesis.tar.gz
tar -xzf /tmp/kinesis.tar.gz -C /tmp/
chmod +x /tmp/kinesis-rs && mv /tmp/kinesis-rs ~/.kinesis/

# Linux
curl -sL https://github.com/iweb3insight/KinesisRS/releases/download/v0.6.5/kinesis-rs-vv0.6.5-linux-amd64.tar.gz -o /tmp/kinesis.tar.gz

# Windows
# Download .zip and extract
```

### Verify Installation

```bash
~/.kinesis/kinesis-rs --version
~/.kinesis/kinesis-rs wallet
```

## Configure Environment Variables

```bash
# Solana Network
export SOL_RPC_URL="https://api.mainnet-beta.solana.com"  # Mainnet
# export SOL_RPC_URL="https://api.devnet.solana.com"       # Devnet
# export SOL_RPC_URL="https://api.testnet.solana.com"      # Testnet

# BSC Network
export BSC_RPC_URL="https://bsc-dataseed.binance.org/"

# Optional: Proxy
export HTTPS_PROXY="http://proxy:8080"
```

## Execute Trades (Three-step Verification)

```
1. Quote    → Get quote
2. Dry-run  → Simulate trade
3. Execute  → Real trade
```

---

## Core Workflows

### Buy Workflow (Buy)

```
Step 1: quote <TOKEN> <AMOUNT>
        ↓
Step 2: buy <TOKEN> <AMOUNT> --dry-run
        ↓
Step 3: buy <TOKEN> <AMOUNT> --no-dry-run (User confirms)
```

### Sell Workflow (Sell)

```
Step 1: sell <TOKEN> <AMOUNT> --dry-run
        ↓
Step 2: sell <TOKEN> <AMOUNT> --no-dry-run
```

---

## Platform Adapters

| Platform | Call Method | Reference |
|----------|-------------|-----------|
| opencode | Shell | adapters/opencode.md |
| openclaw | Shell | adapters/openclaw.md |
| Gemini | mcp__local__execute | adapters/gemini.md |
| Claude | MCP/Bash | adapters/claude.md |

---

## Supported Networks

| Network | SOL_RPC_URL | Balance | Buy Status |
|---------|-------------|---------|------------|
| Mainnet | api.mainnet-beta.solana.com | Needed | ✅ |
| Devnet | api.devnet.solana.com | 3.67 SOL | ⚠️ ATA |
| Testnet | api.testnet.solana.com | 3.00 SOL | ⚠️ ATA |

See: references/network-matrix.md

---

## Error Handling

| Error | Cause | Solution |
|-------|-------|----------|
| AccountNotFound | 0 balance | Deposit SOL |
| ATA Error | No LUT on Devnet | Use Pump.fun path |
| ROUTE_NOT_FOUND | Token not indexed | Wait or switch token |

See: references/error-codes.md

---

## Reference Materials

- [Trading API](references/trading-api.md)
- [Network Matrix](references/network-matrix.md)
- [Error Codes](references/error-codes.md)
