# OpenClaw 适配器

## 环境要求

- OpenClaw 已安装
- 网络访问 GitHub 下载二进制

## 安装

### 1. 下载二进制

```bash
# macOS ARM64
curl -sL https://github.com/iweb3insight/KinesisRS/releases/download/v0.6.5/kinesis-rs-vv0.6.5-macos-arm64.tar.gz -o /tmp/kinesis.tar.gz
tar -xzf /tmp/kinesis.tar.gz -C /tmp/
chmod +x /tmp/kinesis-rs && mkdir -p ~/.kinesis && mv /tmp/kinesis-rs ~/.kinesis/
```

### 2. 配置环境变量

```bash
export SOL_RPC_URL="https://api.mainnet-beta.solana.com"
export PATH="$HOME/.kinesis:$PATH"
```

## 使用方法

### 1. 使用 Shell 工具

```bash
# Quote - 获取报价
Shell: ~/.kinesis/kinesis-rs --json quote DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --action buy --chain solana

# Balance - 查询余额
Shell: ~/.kinesis/kinesis-rs --json balance --chain solana

# Buy - 买入 (dry-run)
Shell: ~/.kinesis/kinesis-rs --json buy DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --slippage 15 --chain solana --dry-run
```

### 2. 使用 MCP 工具 (如果支持)

```yaml
tools:
  - name: kinesis_quote
    command: ~/.kinesis/kinesis-rs
    args: ["--json", "quote", "$token", "$amount", "--action", "buy", "--chain", "solana"]
    env:
      SOL_RPC_URL: "https://api.mainnet-beta.solana.com"

  - name: kinesis_buy
    command: ~/.kinesis/kinesis-rs
    args: ["--json", "buy", "$token", "$amount", "--slippage", "$slippage", "--dry-run", "--chain", "solana"]
    env:
      SOL_RPC_URL: "https://api.mainnet-beta.solana.com"
```

### 3. 完整对话示例

```
用户: 买入 0.001 SOL 的 BONK

Agent:
  1. 获取报价
     Shell: ~/.kinesis/kinesis-rs --json quote DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --action buy --chain solana
     → {"amount_out": "1450172355", "success": true}
  
  2. 模拟交易
     Shell: ~/.kinesis/kinesis-rs --json buy DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --slippage 15 --chain solana --dry-run
     → {"success": false, "error": {"message": "AccountNotFound"}}
  
  3. 展示结果
     "余额不足，需要充值 SOL 才能执行真实交易"
```

---

## 网络切换

```bash
# Devnet
Shell: SOL_RPC_URL=https://api.devnet.solana.com ~/.kinesis/kinesis-rs --json balance --chain solana

# Testnet
Shell: SOL_RPC_URL=https://api.testnet.solana.com ~/.kinesis/kinesis-rs --json balance --chain solana

# Mainnet
Shell: SOL_RPC_URL=https://api.mainnet-beta.solana.com ~/.kinesis/kinesis-rs --json balance --chain solana
```

---

## 常见问题

### Q: 如何调试?

```bash
Shell: RUST_LOG=debug ~/.kinesis/kinesis-rs --json quote ... --chain solana
```

### Q: 二进制路径?

默认: `~/.kinesis/kinesis-rs`

### Q: 环境变量不生效?

```bash
# 方式 1: 命令前设置
Shell: SOL_RPC_URL=https://api.mainnet-beta.solana.com ~/.kinesis/kinesis-rs --json quote ...

# 方式 2: env 参数
Shell: ~/.kinesis/kinesis-rs --json quote ...
env: {SOL_RPC_URL: "https://api.mainnet-beta.solana.com"}
```
