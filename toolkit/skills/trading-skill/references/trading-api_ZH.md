# CLI 命令映射

所有命令均支持 `--json` 以获取结构化输出。

## `quote`
获取实时兑换数量。
- `token_address`: 目标代币地址。
- `amount`: 输入数量。
- `--action`: `buy` 或 `sell`。
- `--chain`: `bsc` (默认) 或 `solana`。

## `buy`
- `token_address`: 要收到的代币。
- `amount`: 要花费的原生 SOL/BNB 数量。
- `--slippage`: 0-100 (例如，15.0 表示 15%)。
- `--jito-tip`: (仅限 Solana) SOL 数量。
- `--no-dry-run`: 执行真实交易。

## `sell`
- `token_address`: 要卖出的代币。
- `amount`: 要卖出的代币单位数量。
- `--no-dry-run`: 执行真实交易。

## `balance`
- `--token-address`: SPL/BEP20 地址 (可选)。
- `--chain`: `bsc` 或 `solana`。

## `wallet`
显示当前活动钱包索引的派生地址。
