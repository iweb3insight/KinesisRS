# CLIコマンドマッピング

すべてのコマンドは、構造化出力のために`--json`をサポートしています。

## `quote`
リアルタイムのスワップ額を取得します。
- `token_address`: ターゲットトークン。
- `amount`: 入力額。
- `--action`: `buy` または `sell`。
- `--chain`: `bsc` (デフォルト) または `solana`。

## `buy`
- `token_address`: 受け取るトークン。
- `amount`: 支払うネイティブSOL/BNBの額。
- `--slippage`: 0-100 (例: 15%の場合は15.0)。
- `--jito-tip`: (Solanaのみ) SOLの額。
- `--no-dry-run`: 実際の取引を実行します。

## `sell`
- `token_address`: 支払うトークン。
- `amount`: 売却するトークンの単位数。
- `--no-dry-run`: 実際の取引を実行します。

## `balance`
- `--token-address`: SPL/BEP20アドレス (任意)。
- `--chain`: `bsc` または `solana`。

## `wallet`
アクティブなウォレットインデックスの派生アドレスを表示します。
