# opencode 适配器

## 环境要求

- macOS / Linux / Windows
- curl 或 wget
- 解压工具 (tar/unzip)

## 自动安装

```bash
# 方式 1: 使用安装脚本
curl -sL https://raw.githubusercontent.com/iweb3insight/KinesisRS/main/scripts/install.sh | bash

# 方式 2: 手动指定路径
export KINESIS_BINARY="$HOME/.kinesis/kinesis-rs"
```

## 手动安装

```bash
# macOS ARM64
curl -sL https://github.com/iweb3insight/KinesisRS/releases/download/v0.6.5/kinesis-rs-vv0.6.5-macos-arm64.tar.gz -o kinesis.tar.gz
tar -xzf kinesis.tar.gz
chmod +x kinesis-rs
mv kinesis-rs ~/bin/

# Linux
curl -sL https://github.com/iweb3insight/KinesisRS/releases/download/v0.6.5/kinesis-rs-vv0.6.5-linux-amd64.tar.gz -o kinesis.tar.gz
```

## 环境变量

```bash
# 必需
export SOL_RPC_URL="https://api.mainnet-beta.solana.com"  # 或 devnet/testnet

# 可选
export BSC_RPC_URL="https://bsc-dataseed.binance.org/"
export HTTPS_PROXY="http://proxy:8080"
```

## 使用方法

### 1. 设置别名

```bash
alias kinesis='$HOME/.kinesis/kinesis-rs'
```

### 2. 执行命令

```bash
# Quote - 获取报价
SOL_RPC_URL="https://api.mainnet-beta.solana.com" \
  $HOME/.kinesis/kinesis-rs --json quote DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --action buy --chain solana

# Balance - 查询余额
SOL_RPC_URL="https://api.mainnet-beta.solana.com" \
  $HOME/.kinesis/kinesis-rs --json balance --chain solana

# Buy - 买入 (dry-run)
SOL_RPC_URL="https://api.mainnet-beta.solana.com" \
  $HOME/.kinesis/kinesis-rs --json buy DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --slippage 15 --chain solana --dry-run

# Buy - 买入 (真实交易)
SOL_RPC_URL="https://api.mainnet-beta.solana.com" \
  $HOME/.kinesis/kinesis-rs --json buy DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --slippage 15 --chain solana --no-dry-run
```

### 3. 完整示例

```bash
#!/bin/bash
# kinesis-trade.sh

KINESIS="$HOME/.kinesis/kinesis-rs"
TOKEN="DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263"
AMOUNT=0.001
SLIPPAGE=15
CHAIN="solana"

# Step 1: Quote
echo "=== Step 1: Quote ==="
$KINESIS --json quote $TOKEN $AMOUNT --action buy --chain $CHAIN

# Step 2: Dry-run
echo "=== Step 2: Dry-run ==="
$KINESIS --json buy $TOKEN $AMOUNT --slippage $SLIPPAGE --chain $CHAIN --dry-run

# Step 3: Real (需要用户确认)
echo "=== Step 3: Execute ==="
# $KINESIS --json buy $TOKEN $AMOUNT --slippage $SLIPPAGE --chain $CHAIN --no-dry-run
```

---

## 常见问题

### Q: 如何切换网络?

```bash
# Devnet
export SOL_RPC_URL="https://api.devnet.solana.com"

# Testnet  
export SOL_RPC_URL="https://api.testnet.solana.com"

# Mainnet
export SOL_RPC_URL="https://api.mainnet-beta.solana.com"
```

### Q: 二进制路径在哪?

```bash
# 默认路径
$HOME/.kinesis/kinesis-rs

# 自定义路径
export KINESIS_BINARY="/path/to/kinesis-rs"
```

### Q: 如何调试?

```bash
# 开启调试日志
RUST_LOG=debug $HOME/.kinesis/kinesis-rs --json quote ...
```
