# エラーコードリファレンス

## エラータイプ

| エラータイプ | 説明 | 主な原因 |
|----------|------|----------|
| `rpc_error` | RPCリクエスト失敗 | ネットワークの問題, RPCが利用不可 |
| `simulation_failed` | 取引シミュレーション失敗 | ロジックのリバート, 残高不足 |
| `send_failed` | 取引送信失敗 | 署名の問題, チェーンの混雑 |
| `config_error` | 設定エラー | 非公開鍵が存在しない, RPCが設定されていない |
| `invalid_input` | 入力パラメータエラー | 無効なアドレス, 負の金額 |
| `contract_error` | コントラクト実行エラー | リバート, 権限の問題 |

---

## よくあるエラーと解決策

### 1. AccountNotFound

**エラーメッセージ:**
```json
{"error": {"type": "contract_error", "message": "Simulation failed: \"AccountNotFound\""}}
```

**原因:** ウォレット残高が 0

**解決策:**
```bash
# ウォレットにSOLを入金
# ウォレットアドレス: 88DqDXNAQZHWscK5HjPavDkBCvsfzmUrDvAV9ZTY5jMv
```

---

### 2. ATA エラー

**エラーメッセージ:**
```
RPC Error -32602: invalid transaction: Transaction loads an address table account that doesn't exist
```

**原因:** Devnet/TestnetにRaydiumが作成したAddress Lookup Table (LUT)が存在しない

**影響を受けるネットワーク:** Devnet, Testnet

**解決策:**
```bash
# 方法 1: Mainnet dry-runを使用
export SOL_RPC_URL="https://api.mainnet-beta.solana.com"

# 方法 2: Pump.fun パスを使用 (可能な場合)
# Pump.funはLUTを使用しません
```

---

### 3. ROUTE_NOT_FOUND

**エラーメッセージ:**
```json
{"error": {"message": "ROUTE_NOT_FOUND"}}
```

**原因:** Raydium APIがそのトークンの流動性プールをインデックスしていない

**よくあるシナリオ:**
- 新しく作成された Pump.fun トークン
- まだ「卒業」していないトークン
- 卒業直後で同期が済んでいないトークン

**解決策:**
```bash
# 方法 1: 流動性のあるトークンに変更
# 例: BONK, USDC, SOL

# 方法 2: トークンがRaydiumに卒業するのを待つ

# 方法 3: Pump.fun で直接取引する (卒業していない場合)
```

---

### 4. REQ_SWAP_RESPONSE_ERROR

**エラーメッセージ:**
```json
{"error": {"message": "Failed to parse Raydium transaction: ... REQ_SWAP_RESPONSE_ERROR"}}
```

**原因:** Raydium APIが無効なレスポンスを返した

**よくあるシナリオ:**
- Quote API失敗後の二次呼び出し
- APIの一時的な問題

**解決策:**
```bash
# 再試行
kinesis-rs buy <TOKEN> <AMOUNT> --dry-run

# または待機してから再試行
```

---

### 5. REQ_COMPUTE_UNIT_PRICE_MICRO_LAMPORTS_ERROR

**エラーメッセージ:**
```json
{"error": {"message": "REQ_COMPUTE_UNIT_PRICE_MICRO_LAMPORTS_ERROR"}}
```

**原因:** Compute Unit Price 設定の問題

**解決策:**
```bash
# APIの回復を待つか再試行
```

---

### 6. SlippageExceeded

**エラーメッセージ:**
```json
{"error": {"revert_reason": "SlippageExceeded"}}
```

**原因:** 価格変動が設定したスリッページを超えた

**解決策:**
```bash
# スリッページを上げる
kinesis-rs buy <TOKEN> <AMOUNT> --slippage 25 --chain solana

# または金額を下げる
kinesis-rs buy <TOKEN> <AMOUNT> --slippage 15 --chain solana
```

---

### 7. Insufficient Liquidity

**エラーメッセージ:**
```json
{"error": {"revert_reason": "FreedomRouter: INSUFFICIENT_LIQUIDITY"}}
```

**原因:** プールの流動性が不足している

**解決策:**
```bash
# 購入金額を下げる
kinesis-rs buy <TOKEN> 0.01 --chain solana

# 流動性の回復を待つ
```

---

### 8. Insufficient Gas / Insufficient Funds

**エラーメッセージ:**
```json
{"error": {"revert_reason": "insufficient funds for gas * price + value"}}
```

**原因:** ガス代を支払うための残高が不足している

**解決策:**
```bash
# ネイティブトークン (SOL/BNB) を入金
```

---

### 9. Token account not found

**エラーメッセージ:**
```json
{"error": {"message": "Token account not found: <TOKEN_ADDRESS>"}}
```

**原因:** ウォレットがそのトークンを保持していない

**解決策:**
```bash
# 最初にトークンを購入してATAを作成する
# またはトークンアドレスが正しいか確認する
```

---

### 10. Invalid Token Address

**エラーメッセージ:**
```json
{"error": {"message": "Invalid token address"}}
```

**原因:** トークンアドレスの形式エラー

**解決策:**
```bash
# アドレス形式を確認
# Solana: Base58エンコード, 32-44文字
# BSC: 0xで始まる, 40文字の16進数
```

---

## エラー処理フロー

```
ユーザーリクエスト
    ↓
コマンド実行
    ↓
┌─────────────────────────────────────┐
│  成功?                              │
│  ↓ Yes                              │
│  成功レスポンスを返す               │
│  ↓ No                               │
│  エラータイプを解析                 │
│  ↓                                  │
│  ┌─────────────────────────────────┐│
│  │ rpc_error                       ││
│  │  - ネットワーク接続を確認        ││
│  │  - RPCを変更                     ││
│  ├─────────────────────────────────┤│
│  │ simulation_failed                ││
│  │  - 残高を確認                    ││
│  │  - 承認を確認                    ││
│  │  - スリッページを調整            ││
│  ├─────────────────────────────────┤│
│  │ contract_error                  ││
│  │  - revert_reasonを解析           ││
│  │  - 具体的な解決策を参照          ││
│  └─────────────────────────────────┘│
└─────────────────────────────────────┘
    ↓
エラーレスポンスを返す
    ↓
ユーザーにエラー + 提案を表示
```

---

## デバッグテクニック

### 1. デバッグログを有効にする

```bash
RUST_LOG=debug kinesis-rs --json quote ...
```

### 2. ネットワーク接続を確認する

```bash
# RPCを直接テスト
curl -X POST https://api.mainnet-beta.solana.com -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}'

# Raydium APIをテスト
curl "https://transaction-v1.raydium.io/compute/swap-base-in?inputMint=So11111111111111111111111111111111111111112&outputMint=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v&amount=1000000000&slippageBps=50"
```

### 3. 残高を確認する

```bash
kinesis-rs --json balance --chain solana
```
