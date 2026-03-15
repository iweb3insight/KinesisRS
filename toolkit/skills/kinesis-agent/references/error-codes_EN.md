# Error Codes Reference

## Error Types

| Error Type | Description | Common Causes |
|------------|-------------|---------------|
| `rpc_error` | RPC request failed | Network issues, RPC unavailable |
| `simulation_failed` | Transaction simulation failed | Logic revert, insufficient balance |
| `send_failed` | Transaction send failed | Signature issues, chain congestion |
| `config_error` | Configuration error | Private key missing, RPC not set |
| `invalid_input` | Invalid input parameters | Invalid address, negative amount |
| `contract_error` | Contract execution error | Revert, permission issues |

---

## Common Errors & Solutions

### 1. AccountNotFound

**Error Message:**
```json
{"error": {"type": "contract_error", "message": "Simulation failed: \"AccountNotFound\""}}
```

**Cause:** Wallet balance is 0.

**Solution:**
```bash
# Deposit SOL into the wallet
# Wallet address: 88DqDXNAQZHWscK5HjPavDkBCvsfzmUrDvAV9ZTY5jMv
```

---

### 2. ATA Error

**Error Message:**
```
RPC Error -32602: invalid transaction: Transaction loads an address table account that doesn't exist
```

**Cause:** Address Lookup Table (LUT) created by Raydium does not exist on Devnet/Testnet.

**Affected Networks:** Devnet, Testnet

**Solution:**
```bash
# Option 1: Use Mainnet dry-run
export SOL_RPC_URL="https://api.mainnet-beta.solana.com"

# Option 2: Use Pump.fun path (if available)
# Pump.fun does not use LUT
```

---

### 3. ROUTE_NOT_FOUND

**Error Message:**
```json
{"error": {"message": "ROUTE_NOT_FOUND"}}
```

**Cause:** Raydium API hasn't indexed the liquidity pool for this token.

**Common Scenarios:**
- Newly created Pump.fun tokens
- Tokens not yet "graduated"
- Just graduated but not synced yet

**Solution:**
```bash
# Option 1: Switch to a token with liquidity
# E.g., BONK, USDC, SOL

# Option 2: Wait for token to graduate to Raydium

# Option 3: Trade directly on Pump.fun (if not graduated)
```

---

### 4. REQ_SWAP_RESPONSE_ERROR

**Error Message:**
```json
{"error": {"message": "Failed to parse Raydium transaction: ... REQ_SWAP_RESPONSE_ERROR"}}
```

**Cause:** Raydium API returned an invalid response.

**Common Scenarios:**
- Secondary call after Quote API failure
- Temporary API issues

**Solution:**
```bash
# Retry
kinesis-rs buy <TOKEN> <AMOUNT> --dry-run

# Or wait and retry
```

---

### 5. REQ_COMPUTE_UNIT_PRICE_MICRO_LAMPORTS_ERROR

**Error Message:**
```json
{"error": {"message": "REQ_COMPUTE_UNIT_PRICE_MICRO_LAMPORTS_ERROR"}}
```

**Cause:** Issue with Compute Unit Price setting.

**Solution:**
```bash
# Wait for API recovery or retry
```

---

### 6. SlippageExceeded

**Error Message:**
```json
{"error": {"revert_reason": "SlippageExceeded"}}
```

**Cause:** Price volatility exceeded set slippage.

**Solution:**
```bash
# Increase slippage
kinesis-rs buy <TOKEN> <AMOUNT> --slippage 25 --chain solana

# Or decrease amount
kinesis-rs buy <TOKEN> <AMOUNT> --slippage 15 --chain solana
```

---

### 7. Insufficient Liquidity

**Error Message:**
```json
{"error": {"revert_reason": "FreedomRouter: INSUFFICIENT_LIQUIDITY"}}
```

**Cause:** Insufficient pool liquidity.

**Solution:**
```bash
# Decrease buy amount
kinesis-rs buy <TOKEN> 0.01 --chain solana

# Wait for liquidity recovery
```

---

### 8. Insufficient Gas / Insufficient Funds

**Error Message:**
```json
{"error": {"revert_reason": "insufficient funds for gas * price + value"}}
```

**Cause:** Insufficient balance to pay for gas fees.

**Solution:**
```bash
# Deposit native token (SOL/BNB)
```

---

### 9. Token account not found

**Error Message:**
```json
{"error": {"message": "Token account not found: <TOKEN_ADDRESS>"}}
```

**Cause:** Wallet does not hold this token.

**Solution:**
```bash
# Buy the token first to create ATA
# Or check if token address is correct
```

---

### 10. Invalid Token Address

**Error Message:**
```json
{"error": {"message": "Invalid token address"}}
```

**Cause:** Token address format error.

**Solution:**
```bash
# Check address format
# Solana: Base58 encoded, 32-44 characters
# BSC: Starts with 0x, 40 hex characters
```

---

## Error Handling Flow

```
User Request
    ↓
Execute Command
    ↓
┌─────────────────────────────────────┐
│  Success?                           │
│  ↓ Yes                              │
│  Return success response             │
│  ↓ No                               │
│  Parse error type                   │
│  ↓                                  │
│  ┌─────────────────────────────────┐│
│  │ rpc_error                       ││
│  │  - Check network connection     ││
│  │  - Change RPC                   ││
│  ├─────────────────────────────────┤│
│  │ simulation_failed                ││
│  │  - Check balance                 ││
│  │  - Check approval                ││
│  │  - Adjust slippage               ││
│  ├─────────────────────────────────┤│
│  │ contract_error                  ││
│  │  - Parse revert_reason           ││
│  │  - Refer to specific solutions   ││
│  └─────────────────────────────────┘│
└─────────────────────────────────────┘
    ↓
Return error response
    ↓
Show error + suggestions to user
```

---

## Debugging Tips

### 1. Enable Debug Logs

```bash
RUST_LOG=debug kinesis-rs --json quote ...
```

### 2. Check Network Connection

```bash
# Test RPC directly
curl -X POST https://api.mainnet-beta.solana.com -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}'

# Test Raydium API
curl "https://transaction-v1.raydium.io/compute/swap-base-in?inputMint=So11111111111111111111111111111111111111112&outputMint=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v&amount=1000000000&slippageBps=50"
```

### 3. Check Balance

```bash
kinesis-rs --json balance --chain solana
```
