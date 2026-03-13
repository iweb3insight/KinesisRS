# Network Matrix

## Supported Networks

| Network | Chain ID | RPC URL | Status |
|---------|----------|---------|--------|
| BSC Mainnet | 56 | bsc-dataseed.binance.org | ✅ |
| BSC Testnet | 97 | data-seed-prebsc-1-s1.binance.org:8545 | ✅ |
| Solana Mainnet | - | api.mainnet-beta.solana.com | ✅ |
| Solana Devnet | - | api.devnet.solana.com | ⚠️ |
| Solana Testnet | - | api.testnet.solana.com | ⚠️ |

---

## Solana Network Detailed Comparison

### Mainnet (api.mainnet-beta.solana.com)

| Feature | Status | Description |
|---------|--------|-------------|
| Quote | ✅ | Working normally |
| Buy (Raydium) | ✅ | Requires SOL balance |
| Buy (Pump.fun) | ✅ | Requires SOL balance |
| Sell | ✅ | Requires token balance |
| Balance | ✅ | Working normally |
| Detect | ✅ | Working normally |

**Test Private Key Balance:** 0 SOL (No real funds)

---

### Devnet (api.devnet.solana.com)

| Feature | Status | Description |
|---------|--------|-------------|
| Quote | ✅ | Working normally |
| Buy (Raydium) | ❌ | ATA Error |
| Buy (Pump.fun) | ⚠️ | Untested |
| Sell | ❌ | ATA Error |
| Balance | ✅ | Working normally |
| Detect | ✅ | Working normally |

**Test Private Key Balance:** 3.67 SOL

**Known Issues:**
- Raydium transactions use Address Lookup Table (LUT)
- LUT doesn't exist on Devnet, causing `Transaction loads an address table account that doesn't exist`

---

### Testnet (api.testnet.solana.com)

| Feature | Status | Description |
|---------|--------|-------------|
| Quote | ✅ | Working normally |
| Buy (Raydium) | ❌ | ATA Error |
| Buy (Pump.fun) | ⚠️ | Untested |
| Sell | ❌ | ATA Error |
| Balance | ✅ | Working normally |
| Detect | ✅ | Working normally |

**Test Private Key Balance:** 3.00 SOL

**Known Issues:** Same as Devnet

---

## BSC Network

### Mainnet

| Feature | Status | Description |
|---------|--------|-------------|
| Quote | ✅ | PancakeSwap |
| Buy | ✅ | Requires BNB balance |
| Sell | ✅ | Requires token balance |
| Approve | ✅ | Handled automatically |
| Balance | ✅ | Working normally |

### Testnet

| Feature | Status | Description |
|---------|--------|-------------|
| Quote | ✅ | PancakeSwap |
| Buy | ✅ | Requires test BNB |
| Sell | ✅ | Requires test tokens |
| Balance | ✅ | Working normally |

---

## Network Selection Recommendations

### Development/Testing

| Scenario | Recommended Network | Reason |
|----------|---------------------|--------|
| Quick Quote Test | Devnet/Testnet | Free, fast |
| Test Pump.fun | Mainnet (dry-run) | No Pump.fun on Devnet |
| Test Raydium | Mainnet (dry-run) | Devnet ATA issues |
| Integration Test | Testnet | Closer to Mainnet |

### Production Environment

| Scenario | Recommended Network | Reason |
|----------|---------------------|--------|
| Real Trading | Mainnet | Only option |
| Trade Verification | Mainnet (dry-run) | Simulates real environment |

---

## Environment Variable Configuration

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

## Multi-RPC Configuration

Supports comma-separated multiple RPCs:

```bash
# BSC
export BSC_RPC_URL="https://bsc-dataseed.binance.org/,https://bsc-dataseed1.binance.org/,https://bsc-dataseed2.binance.org/"

# Solana (Multi-RPC backup)
export SOL_RPC_URL="https://api.mainnet-beta.solana.com,https://solana-api.projectserum.com"
```

---

## Test Tokens

### Solana Devnet/Testnet

| Token | Address | Use Case |
|-------|---------|----------|
| - | - | No test tokens available currently |

### Solana Mainnet

| Token | Address | Liquidity |
|-------|---------|-----------|
| BONK | DezXAX8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 | High |
| USDC | EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v | High |
| SOL | So11111111111111111111111111111111111111112 | Highest |

### BSC Testnet

| Token | Address |
|-------|---------|
| BNB | - (Native) |
| Test Token | 0x... |

---

## Troubleshooting

### Devnet/Testnet ATA Error

```
RPC Error -32602: invalid transaction: Transaction loads an address table account that doesn't exist
```

**Solution:**
1. Test with Mainnet dry-run
2. Wait for future version fix

### ROUTE_NOT_FOUND

```
Raydium API error: ROUTE_NOT_FOUND
```

**Cause:** Token has no liquidity pool on Raydium

**Solution:**
1. Switch to a token with liquidity
2. Wait for token to graduate to Raydium
