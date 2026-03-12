# CLI Command Mapping

All commands support `--json` for structured output.

## `quote`
Get real-time swap amounts.
- `token_address`: Target token.
- `amount`: Input amount.
- `--action`: `buy` or `sell`.
- `--chain`: `bsc` (default) or `solana`.

## `buy`
- `token_address`: Token to receive.
- `amount`: Native SOL/BNB to spend.
- `--slippage`: 0-100 (e.g., 15.0 for 15%).
- `--jito-tip`: (Solana only) SOL amount.
- `--no-dry-run`: Execute real trade.

## `sell`
- `token_address`: Token to spend.
- `amount`: Token units to sell.
- `--no-dry-run`: Execute real trade.

## `balance`
- `--token-address`: SPL/BEP20 address (optional).
- `--chain`: `bsc` or `solana`.

## `wallet`
Shows derived addresses for the active wallet index.
