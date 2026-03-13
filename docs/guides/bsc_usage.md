# KinesisRS BSC Execution Layer Usage Guide

This guide details the features, configuration, and invocation methods of KinesisRS Rust v1.0 on BSC (BNB Smart Chain). This tool is designed as a "trading primitive" specifically for LLM Agents and automated workflows.

---

## 1. Setup

Before use, ensure the following environment variables are configured (or create a `.env` file):

| Variable Name | Required | Description | Example |
| :--- | :--- | :--- | :--- |
| `BSC_RPC_URL` | Yes | HTTP RPC address of the BSC node | `https://bsc-dataseed.binance.org/` |
| `BSC_PRIVATE_KEY_1` | Yes | Private key for wallet index 1 (default) | `0x...` |
| `BSC_PRIVATE_KEY_2` | No | Private key for wallet index 2 | `0x...` |

> **Security Redline**: Private keys are only used temporarily in memory for signing and will **never** appear in any logs, JSON output, or disk persistence files.

---

## 2. Global Flags

These flags can be attached to any command to control output and execution mode:

- `--json`: **(Essential for Agents)** Forces structured JSON format for results.
- `--dry-run`: (Enabled by default) Performs only on-chain simulation (`eth_call`) and gas estimation without sending a real transaction.
- `--no-dry-run`: Disables simulation mode, performs real signing and broadcasts the transaction.
- `--wallet <INDEX>`: Specifies the wallet index to use (1, 2, ...), corresponding to the environment variable suffix.

---

## 3. Core Commands

### 3.1 Buy Token (Buy)
Use BNB to buy a target token in one click. The system automatically selects the optimal path (Pancake/Four.meme/Flap).

```bash
# Simulation: Buy with 0.1 BNB, 15% slippage
./kinesis-rs --json buy <TOKEN_ADDRESS> 0.1 --slippage 15

# Real Transaction:
./kinesis-rs --json --no-dry-run buy <TOKEN_ADDRESS> 0.1
```

### 3.2 Sell Token (Sell)
Sell tokens back to BNB. **Includes built-in auto-approval (ApproveIfNeeded) logic**.

```bash
# Simulation: Sell 1000 tokens. If not approved, output will automatically include the simulated approve stage.
./kinesis-rs --json sell <TOKEN_ADDRESS> 1000
```

### 3.3 Quote Query (Quote)
Read-only query, does not involve simulation or gas estimation.

```bash
# Query the expected amount of tokens for 0.1 BNB
./kinesis-rs quote <TOKEN_ADDRESS> 0.1 --action buy
```

### 3.4 Asset Query (Balance)
Query wallet balance.

```bash
# Query native BNB balance
./kinesis-rs balance

# Query specific token balance
./kinesis-rs balance --token-address <TOKEN_ADDRESS>
```

---

## 4. Deep Diagnostics and Observability

### 4.1 TradeResult JSON Structure
All commands (in `--json` mode) return a consistent structure for easy parsing by Agents.

```json
{
  "success": true,
  "chain": "bsc",
  "stages": [...],
  "amount_out": "25000000000000000000",
  "gas_estimate": 125000,
  "tx_hash": "0x...", // Appears only after a successful real transaction
  "revert_reason": null
}
```

### 4.2 Error Classification
The system provides fine-grained error feedback through `TradeError`:

- **`simulation_failed`**: Contract-level logic error.
  - Automatically parses Revert Reason (e.g., `SLIPPAGE_EXCEEDED`, `INSUFFICIENT_LIQUIDITY`).
- **`rpc_error`**: Infrastructure error.
  - Auto-identifies: `Network timeout`, `Rate limit exceeded` (429), `Nonce/Gas conflict`.
- **`config_error` / `invalid_input`**: Configuration or parameter errors.

---

## 5. Robustness Features

1. **Gas Aggregation**: In the `sell` workflow, if an `approve` is triggered, `gas_estimate` will automatically sum the expected consumption of both the approval and the swap.
2. **Receipt Wait**: In `--no-dry-run` mode, the program synchronously waits for the Receipt and returns the actual `gas_used`.
3. **Silent Logs**: When using `--json`, all human-readable logs (Tracing) are automatically redirected, preventing interference with JSON parsing.

---

## 6. Usage Recommendations

1. **Combined Invocation**: It is recommended that Agents run a `dry-run` first to get the `gas_estimate` and evaluate the cost before deciding whether to execute a real transaction.
2. **Retry Strategy**: If a Nonce conflict is captured in `rpc_error`, the Agent is advised to wait for 1-2 blocks before retrying.
3. **Multi-wallet Concurrency**: Since the program is stateless, fully parallel transaction flows can be achieved by specifying different `--wallet` indices.
