---
name: kinesis-agent
description: 统一的跨平台交易执行 Agent，支持 opencode/Gemini/Claude/OpenClaw。自动下载二进制，执行 Solana/BSC 交易。
binary:
  latest: https://github.com/iweb3insight/KinesisRS/releases/latest
  version: v0.6.5
  install: |
    # 自动安装
    curl -sL https://raw.githubusercontent.com/iweb3insight/KinesisRS/main/scripts/install.sh | bash
    
    # 或手动下载
    # https://github.com/iweb3insight/KinesisRS/releases/latest
platforms:
  - darwin-arm64
  - darwin-amd64
  - linux-amd64
  - windows-amd64
---

# Kinesis Agent - 跨平台交易执行

## 安装

### 自动安装

```bash
curl -sL https://raw.githubusercontent.com/iweb3insight/KinesisRS/main/scripts/install.sh | bash
```

### 手动安装

```bash
# macOS ARM64
curl -sL https://github.com/iweb3insight/KinesisRS/releases/download/v0.6.5/kinesis-rs-vv0.6.5-macos-arm64.tar.gz -o /tmp/kinesis.tar.gz
tar -xzf /tmp/kinesis.tar.gz -C /tmp/
chmod +x /tmp/kinesis-rs && mv /tmp/kinesis-rs ~/.kinesis/

# Linux
curl -sL https://github.com/iweb3insight/KinesisRS/releases/download/v0.6.5/kinesis-rs-vv0.6.5-linux-amd64.tar.gz -o /tmp/kinesis.tar.gz

# Windows
# 下载 .zip 文件并解压
```

### 验证安装

```bash
~/.kinesis/kinesis-rs --version
~/.kinesis/kinesis-rs wallet
```

## 配置环境变量

```bash
# Solana 网络
export SOL_RPC_URL="https://api.mainnet-beta.solana.com"  # Mainnet
# export SOL_RPC_URL="https://api.devnet.solana.com"       # Devnet
# export SOL_RPC_URL="https://api.testnet.solana.com"      # Testnet

# BSC 网络
export BSC_RPC_URL="https://bsc-dataseed.binance.org/"

# 可选: 代理
export HTTPS_PROXY="http://proxy:8080"
```

## 执行交易 (三步验证)

```
1. Quote    → 获取报价
2. Dry-run  → 模拟交易
3. Execute  → 真实交易
```

---

## 核心工作流

### 买入流程 (Buy)

```
Step 1: quote <TOKEN> <AMOUNT>
        ↓
Step 2: buy <TOKEN> <AMOUNT> --dry-run
        ↓
Step 3: buy <TOKEN> <AMOUNT> --no-dry-run (用户确认)
```

### 卖出流程 (Sell)

```
Step 1: sell <TOKEN> <AMOUNT> --dry-run
        ↓
Step 2: sell <TOKEN> <AMOUNT> --no-dry-run
```

---

## 平台适配

| 平台 | 调用方式 | 参考文档 |
|------|---------|----------|
| opencode | Shell | adapters/opencode.md |
| openclaw | Shell | adapters/openclaw.md |
| Gemini | mcp__local__execute | adapters/gemini.md |
| Claude | MCP/Bash | adapters/claude.md |

---

## 支持的网络

| 网络 | SOL_RPC_URL | 余额 | Buy 状态 |
|------|-------------|------|----------|
| Mainnet | api.mainnet-beta.solana.com | 需充值 | ✅ |
| Devnet | api.devnet.solana.com | 3.67 SOL | ⚠️ ATA |
| Testnet | api.testnet.solana.com | 3.00 SOL | ⚠️ ATA |

详见: references/network-matrix.md

---

## 错误处理

| 错误 | 原因 | 解决方案 |
|------|------|----------|
| AccountNotFound | 余额为0 | 充值 SOL |
| ATA 错误 | Devnet 无 LUT | 使用 Pump.fun 路径 |
| ROUTE_NOT_FOUND | 代币未索引 | 等待或换代币 |

详见: references/error-codes.md

---

## 参考文档

- [Trading API](references/trading-api.md)
- [Network Matrix](references/network-matrix.md)
- [Error Codes](references/error-codes.md)
