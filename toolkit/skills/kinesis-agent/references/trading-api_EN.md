# Trading API Reference

## Command Format

```bash
kinesis-rs <COMMAND> [OPTIONS]
```

## Global Options

| Option | Description | Default Value |
|--------|-------------|---------------|
| `--json` | JSON format output | false |
| `--chain` | Blockchain type | bsc |
| `--wallet` | Wallet index | 1 |
| `--dry-run` | Simulate trade | true |
| `--no-dry-run` | Real trade | false |

## Commands

### quote

Get token price quote.

```bash
kinesis-rs quote <TOKEN_ADDRESS> <AMOUNT> [OPTIONS]

# Parameters
TOKEN_ADDRESS  # Token contract address
AMOUNT         # Amount

# Options
--action buy|sell    # Trading direction (Default: buy)
-c, --chain bsc|solana  # Blockchain (Default: bsc)

# Example
kinesis-rs --json quote DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.1 --action buy --chain solana
```

**Output Example:**
```json
{
  "success": true,
  "amount_out": "1450172355",
  "path": "Raydium"
}
```

---

### buy

Buy tokens.

```bash
kinesis-rs buy <TOKEN_ADDRESS> <AMOUNT> [OPTIONS]

# Parameters
TOKEN_ADDRESS  # Target token address (Token to buy)
AMOUNT         # Amount of native tokens to spend (SOL/BNB)

# Options
--slippage PERCENT    # Slippage tolerance % (Default: 15)
--tip-rate PERCENT    # Developer tip % (Solana, Default: 0)
--jito-tip SOL       # Jito tip (Solana, Default: 0)
-c, --chain          # Blockchain

# Example
# Dry-run (Default)
kinesis-rs --json buy DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --slippage 15 --chain solana

# Real trade
kinesis-rs --json buy DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --slippage 15 --chain solana --no-dry-run

# With Jito tip
kinesis-rs --json buy DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --jito-tip 0.001 --chain solana
```

**Output Example (Dry-run):**
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

Sell tokens.

```bash
kinesis-rs sell <TOKEN_ADDRESS> <AMOUNT> [OPTIONS]

# Parameters
TOKEN_ADDRESS  # Token address (Token to sell)
AMOUNT         # Amount of tokens to sell

# Options
--slippage PERCENT    # Slippage tolerance % (Default: 15)
--tip-rate PERCENT    # Developer tip % (Solana)
--jito-tip SOL       # Jito tip (Solana)
-c, --chain          # Blockchain

# Example
kinesis-rs --json sell DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 1000 --slippage 15 --chain solana
```

**Note:** BSC automatically handles `approveIfNeeded`.

---

### balance

Query balance.

```bash
kinesis-rs balance [OPTIONS]

# Options
--token-address ADDRESS  # Token address (Query native token if empty)
-c, --chain              # Blockchain

# Example
# Query SOL balance
kinesis-rs --json balance --chain solana

# Query token balance
kinesis-rs --json balance --token-address DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 --chain solana
```

**Output Example:**
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

Display wallet addresses.

```bash
kinesis-rs wallet [OPTIONS]

# Options
--wallet INDEX  # Wallet index (Default: 1)

# Example
kinesis-rs --json wallet
```

**Output Example:**
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

Display current configuration.

```bash
kinesis-rs --json config
```

---

### detect

Detect token path (Solana only).

```bash
kinesis-rs detect <TOKEN_ADDRESS> --chain solana

# Example
kinesis-rs --json detect DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 --chain solana
```

**Output Example:**
```json
{
  "success": true,
  "token_address": "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263",
  "path": "Raydium"
}
```

---

## Error Response Format

```json
{
  "success": false,
  "chain": "solana",
  "stages": [...],
  "error": {
    "type": "rpc_error|simulation_failed|send_failed|config_error|invalid_input|contract_error",
    "message": "Error description",
    "revert_reason": "Contract revert reason (if any)",
    "raw_revert_data": "Raw revert data (if any)"
  }
}
```
