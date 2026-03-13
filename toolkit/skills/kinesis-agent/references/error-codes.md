# Error Codes Reference

## 错误类型

| 错误类型 | 说明 | 常见原因 |
|----------|------|----------|
| `rpc_error` | RPC 请求失败 | 网络问题, RPC 不可用 |
| `simulation_failed` | 交易模拟失败 | 逻辑 revert, 余额不足 |
| `send_failed` | 交易发送失败 | 签名问题, 链拥堵 |
| `config_error` | 配置错误 | 私钥不存在, RPC 未设置 |
| `invalid_input` | 输入参数错误 | 无效地址, 负数金额 |
| `contract_error` | 合约执行错误 | Revert, 权限问题 |

---

## 常见错误及解决方案

### 1. AccountNotFound

**错误信息:**
```json
{"error": {"type": "contract_error", "message": "Simulation failed: \"AccountNotFound\""}}
```

**原因:** 钱包余额为 0

**解决方案:**
```bash
# 充值 SOL 到钱包
# 钱包地址: 88DqDXNAQZHWscK5HjPavDkBCvsfzmUrDvAV9ZTY5jMv
```

---

### 2. ATA 错误

**错误信息:**
```
RPC Error -32602: invalid transaction: Transaction loads an address table account that doesn't exist
```

**原因:** Devnet/Testnet 不存在 Raydium 创建的 Address Lookup Table

**影响网络:** Devnet, Testnet

**解决方案:**
```bash
# 方案 1: 使用 Mainnet dry-run
export SOL_RPC_URL="https://api.mainnet-beta.solana.com"

# 方案 2: 使用 Pump.fun 路径 (如果可用)
# Pump.fun 不使用 LUT
```

---

### 3. ROUTE_NOT_FOUND

**错误信息:**
```json
{"error": {"message": "ROUTE_NOT_FOUND"}}
```

**原因:** Raydium API 未索引该代币的流动性池

**常见场景:**
- 新创建的 Pump.fun 代币
- 未毕业的代币
- 刚毕业但未同步的代币

**解决方案:**
```bash
# 方案 1: 更换有流动性的代币
# 例如: BONK, USDC, SOL

# 方案 2: 等待代币毕业到 Raydium

# 方案 3: 使用 Pump.fun 直接交易 (如果代币未毕业)
```

---

### 4. REQ_SWAP_RESPONSE_ERROR

**错误信息:**
```json
{"error": {"message": "Failed to parse Raydium transaction: ... REQ_SWAP_RESPONSE_ERROR"}}
```

**原因:** Raydium API 返回无效响应

**常见场景:**
- Quote API 失败后的二次调用
- API 临时性问题

**解决方案:**
```bash
# 重试
kinesis-rs buy <TOKEN> <AMOUNT> --dry-run

# 或者等待后重试
```

---

### 5. REQ_COMPUTE_UNIT_PRICE_MICRO_LAMPORTS_ERROR

**错误信息:**
```json
{"error": {"message": "REQ_COMPUTE_UNIT_PRICE_MICRO_LAMPORTS_ERROR"}}
```

**原因:** Compute Unit Price 设置问题

**解决方案:**
```bash
# 等待 API 恢复或重试
```

---

### 6. SlippageExceeded

**错误信息:**
```json
{"error": {"revert_reason": "SlippageExceeded"}}
```

**原因:** 价格波动超过设置的滑点

**解决方案:**
```bash
# 提高滑点
kinesis-rs buy <TOKEN> <AMOUNT> --slippage 25 --chain solana

# 或降低金额
kinesis-rs buy <TOKEN> <AMOUNT> --slippage 15 --chain solana
```

---

### 7. Insufficient Liquidity

**错误信息:**
```json
{"error": {"revert_reason": "FreedomRouter: INSUFFICIENT_LIQUIDITY"}}
```

**原因:** 池子流动性不足

**解决方案:**
```bash
# 降低买入金额
kinesis-rs buy <TOKEN> 0.01 --chain solana

# 等待流动性恢复
```

---

### 8. Insufficient Gas / Insufficient Funds

**错误信息:**
```json
{"error": {"revert_reason": "insufficient funds for gas * price + value"}}
```

**原因:** 余额不足以支付 Gas 费

**解决方案:**
```bash
# 充值原生代币 (SOL/BNB)
```

---

### 9. Token account not found

**错误信息:**
```json
{"error": {"message": "Token account not found: <TOKEN_ADDRESS>"}}
```

**原因:** 钱包未持有该代币

**解决方案:**
```bash
# 先买入代币创建 ATA
# 或检查代币地址是否正确
```

---

### 10. Invalid Token Address

**错误信息:**
```json
{"error": {"message": "Invalid token address"}}
```

**原因:** 代币地址格式错误

**解决方案:**
```bash
# 检查地址格式
# Solana: Base58 编码, 32-44 字符
# BSC: 0x 开头, 40 十六进制字符
```

---

## 错误处理流程

```
用户请求
    ↓
执行命令
    ↓
┌─────────────────────────────────────┐
│  成功?                              │
│  ↓ Yes                              │
│  返回成功响应                       │
│  ↓ No                               │
│  解析错误类型                       │
│  ↓                                  │
│  ┌─────────────────────────────────┐│
│  │ rpc_error                       ││
│  │  - 检查网络连接                  ││
│  │  - 更换 RPC                      ││
│  ├─────────────────────────────────┤│
│  │ simulation_failed                ││
│  │  - 检查余额                      ││
│  │  - 检查授权                      ││
│  │  - 调整滑点                      ││
│  ├─────────────────────────────────┤│
│  │ contract_error                  ││
│  │  - 解析 revert_reason            ││
│  │  - 参考具体解决方案              ││
│  └─────────────────────────────────┘│
└─────────────────────────────────────┘
    ↓
返回错误响应
    ↓
向用户展示错误 + 建议
```

---

## 调试技巧

### 1. 开启调试日志

```bash
RUST_LOG=debug kinesis-rs --json quote ...
```

### 2. 检查网络连接

```bash
# 直接测试 RPC
curl -X POST https://api.mainnet-beta.solana.com -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}'

# 测试 Raydium API
curl "https://transaction-v1.raydium.io/compute/swap-base-in?inputMint=So11111111111111111111111111111111111111112&outputMint=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v&amount=1000000000&slippageBps=50"
```

### 3. 检查余额

```bash
kinesis-rs --json balance --chain solana
```
