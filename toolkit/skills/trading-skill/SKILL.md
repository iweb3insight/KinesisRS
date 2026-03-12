---
name: kinesis-trading-skill
description: Multi-chain trading execution for BSC and Solana. Use for buying/selling tokens, quoting prices, and checking balances on PancakeSwap, Pump.fun, and Raydium. Supports Jito bundle submission and multi-RPC racing.
---

# Trading Execution Skill (Kinesis.rs)

This skill enables Gemini CLI to act as a high-performance crypto trading agent.

## Core Workflows

### 1. Buy Workflow (Safe Execution)
1. **Get Quote**: Execute `quote` to get real-time price.
2. **Simulate**: Run with `--dry-run` to verify logic and gas.
3. **Execute**: Confirm with user and run with `--no-dry-run`.

### 2. Sell Workflow (Auto-Approve)
1. **Simulate**: BSC handles `approveIfNeeded` automatically.
2. **Execute**: Run with `--no-dry-run`.

## Reference Materials

- **[QUICK_START.md](references/trading-api.md)**: CLI command mapping and JSON examples.
- **[USAGE_GUIDE.md](references/usage-guide.md)**: Deep dive into Agent interaction patterns and troubleshooting.
- **[SETUP.md](references/setup.md)**: Environment variables and build instructions.

## Verification
Run `./kinesis-trading-skill/scripts/check_config.cjs` to verify your environment.
