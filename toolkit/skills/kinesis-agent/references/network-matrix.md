# Network Matrix

## 支持的网络

| 网络 | Chain ID | RPC URL | 状态 |
|------|----------|---------|------|
| BSC Mainnet | 56 | bsc-dataseed.binance.org | ✅ |
| BSC Testnet | 97 | data-seed-prebsc-1-s1.binance.org:8545 | ✅ |
| Solana Mainnet | - | api.mainnet-beta.solana.com | ✅ |
| Solana Devnet | - | api.devnet.solana.com | ⚠️ |
| Solana Testnet | - | api.testnet.solana.com | ⚠️ |

---

## Solana 网络详细对比

### Mainnet (api.mainnet-beta.solana.com)

| 功能 | 状态 | 说明 |
|------|------|------|
| Quote | ✅ | 正常工作 |
| Buy (Raydium) | ✅ | 需要 SOL 余额 |
| Buy (Pump.fun) | ✅ | 需要 SOL 余额 |
| Sell | ✅ | 需要代币余额 |
| Balance | ✅ | 正常工作 |
| Detect | ✅ | 正常工作 |

**测试私钥余额:** 0 SOL (无真实资金)

---

### Devnet (api.devnet.solana.com)

| 功能 | 状态 | 说明 |
|------|------|------|
| Quote | ✅ | 正常工作 |
| Buy (Raydium) | ❌ | ATA 错误 |
| Buy (Pump.fun) | ⚠️ | 未测试 |
| Sell | ❌ | ATA 错误 |
| Balance | ✅ | 正常工作 |
| Detect | ✅ | 正常工作 |

**测试私钥余额:** 3.67 SOL

**已知问题:**
- Raydium 交易使用 Address Lookup Table (LUT)
- Devnet 上 LUT 不存在，导致 `Transaction loads an address table account that doesn't exist`

---

### Testnet (api.testnet.solana.com)

| 功能 | 状态 | 说明 |
|------|------|------|
| Quote | ✅ | 正常工作 |
| Buy (Raydium) | ❌ | ATA 错误 |
| Buy (Pump.fun) | ⚠️ | 未测试 |
| Sell | ❌ | ATA 错误 |
| Balance | ✅ | 正常工作 |
| Detect | ✅ | 正常工作 |

**测试私钥余额:** 3.00 SOL

**已知问题:** 同 Devnet

---

## BSC 网络

### Mainnet

| 功能 | 状态 | 说明 |
|------|------|------|
| Quote | ✅ | PancakeSwap |
| Buy | ✅ | 需要 BNB 余额 |
| Sell | ✅ | 需要代币余额 |
| Approve | ✅ | 自动处理 |
| Balance | ✅ | 正常工作 |

### Testnet

| 功能 | 状态 | 说明 |
|------|------|------|
| Quote | ✅ | PancakeSwap |
| Buy | ✅ | 需要测试 BNB |
| Sell | ✅ | 需要测试代币 |
| Balance | ✅ | 正常工作 |

---

## 网络选择建议

### 开发/测试

| 场景 | 推荐网络 | 原因 |
|------|---------|------|
| 快速测试 Quote | Devnet/Testnet | 免费，快速 |
| 测试 Pump.fun | Mainnet (dry-run) | Devnet 无 Pump.fun |
| 测试 Raydium | Mainnet (dry-run) | Devnet ATA 问题 |
| 集成测试 | Testnet | 更接近主网 |

### 生产环境

| 场景 | 推荐网络 | 原因 |
|------|---------|------|
| 真实交易 | Mainnet | 唯一选择 |
| 交易验证 | Mainnet (dry-run) | 模拟真实环境 |

---

## 环境变量配置

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

## 多 RPC 配置

支持逗号分隔多个 RPC：

```bash
# BSC
export BSC_RPC_URL="https://bsc-dataseed.binance.org/,https://bsc-dataseed1.binance.org/,https://bsc-dataseed2.binance.org/"

# Solana (多 RPC 备选)
export SOL_RPC_URL="https://api.mainnet-beta.solana.com,https://solana-api.projectserum.com"
```

---

## 测试代币

### Solana Devnet/Testnet

| 代币 | 地址 | 用途 |
|------|------|------|
| - | - | 暂无可测试代币 |

### Solana Mainnet

| 代币 | 地址 | 流动性 |
|------|------|--------|
| BONK | DezXAX8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 | 高 |
| USDC | EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v | 高 |
| SOL | So11111111111111111111111111111111111111112 | 最高 |

### BSC Testnet

| 代币 | 地址 |
|------|------|
| BNB | - (原生) |
| 测试代币 | 0x... |

---

## 故障排除

### Devnet/Testnet ATA 错误

```
RPC Error -32602: invalid transaction: Transaction loads an address table account that doesn't exist
```

**解决方案:**
1. 使用 Mainnet dry-run 测试
2. 等待未来版本修复

### ROUTE_NOT_FOUND

```
Raydium API error: ROUTE_NOT_FOUND
```

**原因:** 代币在 Raydium 上没有流动性池

**解决方案:**
1. 更换有流动性的代币
2. 等待代币毕业到 Raydium
