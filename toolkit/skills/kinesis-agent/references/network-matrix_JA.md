# ネットワークマトリックス

## サポートされているネットワーク

| ネットワーク | チェーン ID | RPC URL | 状態 |
|------|----------|---------|------|
| BSC Mainnet | 56 | bsc-dataseed.binance.org | ✅ |
| BSC Testnet | 97 | data-seed-prebsc-1-s1.binance.org:8545 | ✅ |
| Solana Mainnet | - | api.mainnet-beta.solana.com | ✅ |
| Solana Devnet | - | api.devnet.solana.com | ⚠️ |
| Solana Testnet | - | api.testnet.solana.com | ⚠️ |

---

## Solana ネットワーク詳細比較

### Mainnet (api.mainnet-beta.solana.com)

| 機能 | 状態 | 説明 |
|------|------|------|
| Quote | ✅ | 正常に動作 |
| Buy (Raydium) | ✅ | SOL残高が必要 |
| Buy (Pump.fun) | ✅ | SOL残高が必要 |
| Sell | ✅ | トークン残高が必要 |
| Balance | ✅ | 正常に動作 |
| Detect | ✅ | 正常に動作 |

**テスト用秘密鍵の残高:** 0 SOL (実際の資金なし)

---

### Devnet (api.devnet.solana.com)

| 機能 | 状態 | 説明 |
|------|------|------|
| Quote | ✅ | 正常に動作 |
| Buy (Raydium) | ❌ | ATA エラー |
| Buy (Pump.fun) | ⚠️ | 未テスト |
| Sell | ❌ | ATA エラー |
| Balance | ✅ | 正常に動作 |
| Detect | ✅ | 正常に動作 |

**テスト用秘密鍵の残高:** 3.67 SOL

**既知の問題:**
- Raydium 取引は Address Lookup Table (LUT) を使用します
- Devnet 上に LUT が存在しないため、`Transaction loads an address table account that doesn't exist` エラーが発生します

---

### Testnet (api.testnet.solana.com)

| 機能 | 状態 | 説明 |
|------|------|------|
| Quote | ✅ | 正常に動作 |
| Buy (Raydium) | ❌ | ATA エラー |
| Buy (Pump.fun) | ⚠️ | 未テスト |
| Sell | ❌ | ATA エラー |
| Balance | ✅ | 正常に動作 |
| Detect | ✅ | 正常に動作 |

**テスト用秘密鍵の残高:** 3.00 SOL

**既知の問題:** Devnet と同様

---

## BSC ネットワーク

### Mainnet

| 機能 | 状態 | 説明 |
|------|------|------|
| Quote | ✅ | PancakeSwap |
| Buy | ✅ | BNB残高が必要 |
| Sell | ✅ | トークン残高が必要 |
| Approve | ✅ | 自動的に処理 |
| Balance | ✅ | 正常に動作 |

### Testnet

| 機能 | 状態 | 説明 |
|------|------|------|
| Quote | ✅ | PancakeSwap |
| Buy | ✅ | テスト用BNBが必要 |
| Sell | ✅ | テスト用トークンが必要 |
| Balance | ✅ | 正常に動作 |

---

## ネットワーク選択の推奨事項

### 開発・テスト

| シナリオ | 推奨ネットワーク | 理由 |
|------|---------|------|
| クオートの迅速なテスト | Devnet/Testnet | 無料、高速 |
| Pump.fun のテスト | Mainnet (dry-run) | Devnet に Pump.fun がない |
| Raydium のテスト | Mainnet (dry-run) | Devnet の ATA 問題 |
| 統合テスト | Testnet | 本番環境に近い |

### 本番環境

| シナリオ | 推奨ネットワーク | 理由 |
|------|---------|------|
| 実際の取引 | Mainnet | 唯一の選択肢 |
| 取引の検証 | Mainnet (dry-run) | 実際の環境をシミュレート |

---

## 環境変数の設定

```bash
# Solana
export SOL_RPC_URL="https://api.mainnet-beta.solana.com"  # Mainnet
export SOL_RPC_URL="https://api.devnet.solana.com"       # Devnet
export SOL_RPC_URL="https://api.testnet.solana.com"      # Testnet

# BSC
export BSC_RPC_URL="https://bsc-dataseed.binance.org/"   # Mainnet
export BSC_RPC_URL="https://data-seed-prebsc-1-s1.binance.org:8545/"  # Testnet
```

---

## マルチ RPC 設定

カンマ区切りで複数の RPC をサポートしています：

```bash
# BSC
export BSC_RPC_URL="https://bsc-dataseed.binance.org/,https://bsc-dataseed1.binance.org/,https://bsc-dataseed2.binance.org/"

# Solana (マルチ RPC バックアップ)
export SOL_RPC_URL="https://api.mainnet-beta.solana.com,https://solana-api.projectserum.com"
```

---

## テスト用トークン

### Solana Devnet/Testnet

| トークン | アドレス | 用途 |
|------|------|------|
| - | - | 現在テスト用トークンはありません |

### Solana Mainnet

| トークン | アドレス | 流動性 |
|------|------|--------|
| BONK | DezXAX8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 | 高 |
| USDC | EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v | 高 |
| SOL | So11111111111111111111111111111111111111112 | 最高 |

### BSC Testnet

| トークン | アドレス |
|------|------|
| BNB | - (ネイティブ) |
| テストトークン | 0x... |

---

## トラブルシューティング

### Devnet/Testnet ATA エラー

```
RPC Error -32602: invalid transaction: Transaction loads an address table account that doesn't exist
```

**解決策:**
1. Mainnet dry-run でテストする
2. 将来のバージョンでの修正を待つ

### ROUTE_NOT_FOUND

```
Raydium API error: ROUTE_NOT_FOUND
```

**原因:** トークンが Raydium 上に流動性プールを持っていない

**解決策:**
1. 流動性のあるトークンに変更する
2. トークンが Raydium に卒業するのを待つ
