# FreedomAgent Detailed Usage Guide

## 1. Core Design Philosophy
FreedomAgent is an **Agent-First** trading system. Every design choice is made to ensure LLM Agents (like Gemini, Claude) can execute complex trades safely and precisely.

- **Stateless**: Each command contains all the context needed for execution.
- **JSON-First**: We recommend always using the `--json` flag so the Agent can precisely parse the `TradeResult`.
- **Security First**: `--dry-run` is enabled by default to force simulation verification.

## 2. Interaction Best Practices (For Agents)

### Buy Workflow
1. **Quote**: Execute `quote`. Parse `amount_out` and present it to the user.
2. **Risk Assessment**: Execute `buy --dry-run`.
   - Check `duration_ms` in `stages`.
   - Check `gas_estimate`.
   - If successful, show the simulation result and ask for user confirmation.
3. **Real Trade**: After confirmation, execute `buy --no-dry-run`.

### Sell Workflow
- Sell commands automatically detect if an `approve` is needed (for BSC).
- if an `approve` stage appears in `stages`, it means an approval operation occurred.

## 3. Solana Specific Features

### Jito Bundle Acceleration
On Solana, to prevent front-running (MEV) or to land trades during congestion, Jito must be used:
```bash
./kinesis_rs buy <TOKEN> 0.1 --chain solana --jito-tip 0.001
```
- **Parameter**: `--jito-tip` is in SOL. Recommended range: 0.0001 - 0.01.

### Raydium Smart Routing
For non-Pump.fun tokens (e.g., USDC, SOL/USDT pools), the executor automatically calls the Raydium V3 Trade API for optimal path searching.

## 4. Common Error Codes & Handling

| Error Message | Cause | Suggestion |
| :--- | :--- | :--- |
| `AccountNotFound` | Wallet balance is 0 or uninitialized | Deposit native token (BNB/SOL) |
| `SlippageExceeded` | High price volatility | Increase `--slippage` (e.g., 25.0) |
| `RouteNotFound` | Insufficient liquidity or API not indexed | Check token address or try small amount |
| `Simulation failed` | Execution reverted | Check `raw_revert_data` for details |

## 5. Performance Auditing
By parsing the `stages` array in `TradeResult`, the Agent can calculate:
- **API Latency**: `duration_ms` of the `quote` stage.
- **Execution Latency**: `duration_ms` of the `buy`/`sell` stage.
- **Total Duration**: Sum of `duration_ms` of all stages.
