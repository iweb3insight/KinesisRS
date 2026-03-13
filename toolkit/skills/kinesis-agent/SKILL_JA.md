---
name: kinesis-agent
description: opencode/Gemini/Claude/OpenClawをサポートする統一クロスプラットフォーム取引実行エージェント。バイナリを自動ダウンロードし、Solana/BSC取引を実行します。
binary:
  latest: https://github.com/iweb3insight/KinesisRS/releases/latest
  version: v0.6.5
  install: |
    # 自動インストール
    curl -sL https://raw.githubusercontent.com/iweb3insight/KinesisRS/main/scripts/install.sh | bash
    
    # または手動ダウンロード
    # https://github.com/iweb3insight/KinesisRS/releases/latest
platforms:
  - darwin-arm64
  - darwin-amd64
  - linux-amd64
  - windows-amd64
---

# Kinesis Agent - クロスプラットフォーム取引実行

## インストール

### 自動インストール

```bash
curl -sL https://raw.githubusercontent.com/iweb3insight/KinesisRS/main/scripts/install.sh | bash
```

### 手動インストール

```bash
# macOS ARM64
curl -sL https://github.com/iweb3insight/KinesisRS/releases/download/v0.6.5/kinesis-rs-vv0.6.5-macos-arm64.tar.gz -o /tmp/kinesis.tar.gz
tar -xzf /tmp/kinesis.tar.gz -C /tmp/
chmod +x /tmp/kinesis-rs && mv /tmp/kinesis-rs ~/.kinesis/

# Linux
curl -sL https://github.com/iweb3insight/KinesisRS/releases/download/v0.6.5/kinesis-rs-vv0.6.5-linux-amd64.tar.gz -o /tmp/kinesis.tar.gz

# Windows
# .zipファイルをダウンロードして解凍
```

### インストールの検証

```bash
~/.kinesis/kinesis-rs --version
~/.kinesis/kinesis-rs wallet
```

## 環境変数の設定

```bash
# Solana ネットワーク
export SOL_RPC_URL="https://api.mainnet-beta.solana.com"  # Mainnet
# export SOL_RPC_URL="https://api.devnet.solana.com"       # Devnet
# export SOL_RPC_URL="https://api.testnet.solana.com"      # Testnet

# BSC ネットワーク
export BSC_RPC_URL="https://bsc-dataseed.binance.org/"

# オプション: プロキシ
export HTTPS_PROXY="http://proxy:8080"
```

## 取引の実行 (3段階検証)

```
1. Quote    → 見積（クオート）取得
2. Dry-run  → 取引シミュレーション
3. Execute  → 実際の取引
```

---

## コアワークフロー

### 購入フロー (Buy)

```
Step 1: quote <TOKEN> <AMOUNT>
        ↓
Step 2: buy <TOKEN> <AMOUNT> --dry-run
        ↓
Step 3: buy <TOKEN> <AMOUNT> --no-dry-run (ユーザー確認)
```

### 売却フロー (Sell)

```
Step 1: sell <TOKEN> <AMOUNT> --dry-run
        ↓
Step 2: sell <TOKEN> <AMOUNT> --no-dry-run
```

---

## プラットフォームアダプター

| プラットフォーム | 呼び出し方法 | 参考ドキュメント |
|------|---------|----------|
| opencode | Shell | adapters/opencode.md |
| openclaw | Shell | adapters/openclaw.md |
| Gemini | mcp__local__execute | adapters/gemini.md |
| Claude | MCP/Bash | adapters/claude.md |

---

## サポートされているネットワーク

| ネットワーク | SOL_RPC_URL | 残高 | Buy 状態 |
|------|-------------|------|----------|
| Mainnet | api.mainnet-beta.solana.com | 要入金 | ✅ |
| Devnet | api.devnet.solana.com | 3.67 SOL | ⚠️ ATA |
| Testnet | api.testnet.solana.com | 3.00 SOL | ⚠️ ATA |

詳細は: references/network-matrix.md

---

## エラー処理

| エラー | 原因 | 解決策 |
|------|------|----------|
| AccountNotFound | 残高が0 | SOLを入金 |
| ATA エラー | DevnetにLUTがない | Pump.fun パスを使用 |
| ROUTE_NOT_FOUND | トークンがインデックスされていない | 待機またはトークンを変更 |

詳細は: references/error-codes.md

---

## 参考資料

- [Trading API](references/trading-api.md)
- [Network Matrix](references/network-matrix.md)
- [Error Codes](references/error-codes.md)
