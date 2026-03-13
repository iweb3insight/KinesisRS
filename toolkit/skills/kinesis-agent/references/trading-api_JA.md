# Trading API リファレンス

## コマンド形式

```bash
kinesis-rs <COMMAND> [OPTIONS]
```

## グローバルオプション

| オプション | 説明 | デフォルト値 |
|------|------|--------|
| `--json` | JSON形式で出力 | false |
| `--chain` | ブロックチェーンタイプ | bsc |
| `--wallet` | ウォレットインデックス | 1 |
| `--dry-run` | 取引をシミュレーション | true |
| `--no-dry-run` | 実際の取引を実行 | false |

## コマンド

### quote

トークンの見積（クオート）を取得します。

```bash
kinesis-rs quote <TOKEN_ADDRESS> <AMOUNT> [OPTIONS]

# 引数
TOKEN_ADDRESS  # トークンのコントラクトアドレス
AMOUNT         # 数量

# オプション
--action buy|sell    # 取引方向 (デフォルト: buy)
-c, --chain bsc|solana  # ブロックチェーン (デフォルト: bsc)

# 例
kinesis-rs --json quote DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.1 --action buy --chain solana
```

**出力例:**
```json
{
  "success": true,
  "amount_out": "1450172355",
  "path": "Raydium"
}
```

---

### buy

トークンを購入します。

```bash
kinesis-rs buy <TOKEN_ADDRESS> <AMOUNT> [OPTIONS]

# 引数
TOKEN_ADDRESS  # ターゲットトークンのアドレス (購入するトークン)
AMOUNT         # 支払うネイティブトークン(SOL/BNB)の数量

# オプション
--slippage PERCENT    # スリッページ許容度 % (デフォルト: 15)
--tip-rate PERCENT    # 開発者チップ % (Solana, デフォルト: 0)
--jito-tip SOL       # Jito チップ (Solana, デフォルト: 0)
-c, --chain          # ブロックチェーン

# 例
# Dry-run (デフォルト)
kinesis-rs --json buy DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --slippage 15 --chain solana

# 実際の取引
kinesis-rs --json buy DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --slippage 15 --chain solana --no-dry-run

# Jitoチップ付き
kinesis-rs --json buy DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --jito-tip 0.001 --chain solana
```

**出力例 (Dry-run):**
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

トークンを売却します。

```bash
kinesis-rs sell <TOKEN_ADDRESS> <AMOUNT> [OPTIONS]

# 引数
TOKEN_ADDRESS  # トークンアドレス (売却するトークン)
AMOUNT         # 売却するトークンの数量

# オプション
--slippage PERCENT    # スリッページ許容度 % (デフォルト: 15)
--tip-rate PERCENT    # 開発者チップ % (Solana)
--jito-tip SOL       # Jito チップ (Solana)
-c, --chain          # ブロックチェーン

# 例
kinesis-rs --json sell DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 1000 --slippage 15 --chain solana
```

**注意:** BSCは自動的に `approveIfNeeded` を処理します。

---

### balance

残高を照会します。

```bash
kinesis-rs balance [OPTIONS]

# オプション
--token-address ADDRESS  # トークンアドレス (空の場合はネイティブトークンを照会)
-c, --chain              # ブロックチェーン

# 例
# SOL 残高を照会
kinesis-rs --json balance --chain solana

# トークン残高を照会
kinesis-rs --json balance --token-address DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 --chain solana
```

**出力例:**
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

ウォレットアドレスを表示します。

```bash
kinesis-rs wallet [OPTIONS]

# オプション
--wallet INDEX  # ウォレットインデックス (デフォルト: 1)

# 例
kinesis-rs --json wallet
```

**出力例:**
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

現在の設定を表示します。

```bash
kinesis-rs --json config
```

---

### detect

トークンのパスを検出します (Solanaのみ)。

```bash
kinesis-rs detect <TOKEN_ADDRESS> --chain solana

# 例
kinesis-rs --json detect DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 --chain solana
```

**出力例:**
```json
{
  "success": true,
  "token_address": "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263",
  "path": "Raydium"
}
```

---

## エラーレスポンス形式

```json
{
  "success": false,
  "chain": "solana",
  "stages": [...],
  "error": {
    "type": "rpc_error|simulation_failed|send_failed|config_error|invalid_input|contract_error",
    "message": "エラーの説明",
    "revert_reason": "コントラクトのリバート理由 (ある場合)",
    "raw_revert_data": "生のリバートデータ (ある場合)"
  }
}
```
