# Trading API Reference

## 命令格式

```bash
kinesis-rs <COMMAND> [OPTIONS]
```

## 全局选项

| 选项 | 说明 | 默认值 |
|------|------|--------|
| `--json` | JSON 格式输出 | false |
| `--chain` | 区块链类型 | bsc |
| `--wallet` | 钱包索引 | 1 |
| `--dry-run` | 模拟交易 | true |
| `--no-dry-run` | 真实交易 | false |

## Commands

### quote

获取代币报价。

```bash
kinesis-rs quote <TOKEN_ADDRESS> <AMOUNT> [OPTIONS]

# 参数
TOKEN_ADDRESS  # 代币合约地址
AMOUNT         # 数量

# 选项
--action buy|sell    # 交易方向 (默认: buy)
-c, --chain bsc|solana  # 区块链 (默认: bsc)

# 示例
kinesis-rs --json quote DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.1 --action buy --chain solana
```

**输出示例:**
```json
{
  "success": true,
  "amount_out": "1450172355",
  "path": "Raydium"
}
```

---

### buy

买入代币。

```bash
kinesis-rs buy <TOKEN_ADDRESS> <AMOUNT> [OPTIONS]

# 参数
TOKEN_ADDRESS  # 目标代币地址 (要买入的代币)
AMOUNT         # 花费的原生代币数量 (SOL/BNB)

# 选项
--slippage PERCENT    # 滑点容忍度 % (默认: 15)
--tip-rate PERCENT    # 开发者小费 % (Solana, 默认: 0)
--jito-tip SOL       # Jito 小费 (Solana, 默认: 0)
-c, --chain          # 区块链

# 示例
# Dry-run (默认)
kinesis-rs --json buy DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --slippage 15 --chain solana

# 真实交易
kinesis-rs --json buy DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --slippage 15 --chain solana --no-dry-run

# 带 Jito 小费
kinesis-rs --json buy DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --jito-tip 0.001 --chain solana
```

**输出示例 (Dry-run):**
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

卖出代币。

```bash
kinesis-rs sell <TOKEN_ADDRESS> <AMOUNT> [OPTIONS]

# 参数
TOKEN_ADDRESS  # 代币地址 (要卖出的代币)
AMOUNT         # 卖出的代币数量

# 选项
--slippage PERCENT    # 滑点容忍度 % (默认: 15)
--tip-rate PERCENT    # 开发者小费 % (Solana)
--jito-tip SOL       # Jito 小费 (Solana)
-c, --chain          # 区块链

# 示例
kinesis-rs --json sell DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 1000 --slippage 15 --chain solana
```

**注意:** BSC 会自动处理 `approveIfNeeded`。

---

### balance

查询余额。

```bash
kinesis-rs balance [OPTIONS]

# 选项
--token-address ADDRESS  # 代币地址 (空则查询原生代币)
-c, --chain              # 区块链

# 示例
# 查询 SOL 余额
kinesis-rs --json balance --chain solana

# 查询代币余额
kinesis-rs --json balance --token-address DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 --chain solana
```

**输出示例:**
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

显示钱包地址。

```bash
kinesis-rs wallet [OPTIONS]

# 选项
--wallet INDEX  # 钱包索引 (默认: 1)

# 示例
kinesis-rs --json wallet
```

**输出示例:**
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

显示当前配置。

```bash
kinesis-rs --json config
```

---

### detect

检测代币路径 (仅 Solana)。

```bash
kinesis-rs detect <TOKEN_ADDRESS> --chain solana

# 示例
kinesis-rs --json detect DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 --chain solana
```

**输出示例:**
```json
{
  "success": true,
  "token_address": "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263",
  "path": "Raydium"
}
```

---

## 错误响应格式

```json
{
  "success": false,
  "chain": "solana",
  "stages": [...],
  "error": {
    "type": "rpc_error|simulation_failed|send_failed|config_error|invalid_input|contract_error",
    "message": "错误描述",
    "revert_reason": "合约 revert 原因 (如果有)",
    "raw_revert_data": "原始 revert 数据 (如果有)"
  }
}
```
