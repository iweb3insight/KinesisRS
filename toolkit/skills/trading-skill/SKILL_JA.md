---
name: kinesis-trading-skill
description: BSCおよびSolana向けマルチチェーン取引実行。PancakeSwap、Pump.fun、Raydiumでのトークン売買、見積（クオート）、残高確認に使用。Jitoバンドル送信およびマルチRPCレーシングをサポート。
---

# 取引実行スキル (KinesisRS)

このスキルにより、Gemini CLIは高性能な暗号資産取引エージェントとして動作します。

## コアワークフロー

### 1. 購入ワークフロー (安全な実行)
1. **見積取得 (Quote)**：`quote`を実行してリアルタイム価格を取得。
2. **シミュレーション (Simulate)**：`--dry-run`を付けて実行し、ロジックとガス代を検証。
3. **実行 (Execute)**：ユーザーに確認後、`--no-dry-run`を付けて実行。

### 2. 売却ワークフロー (自動承認)
1. **シミュレーション (Simulate)**：BSCは`approveIfNeeded`を自動的に処理。
2. **実行 (Execute)**：`--no-dry-run`を付けて実行。

## 参考資料

- **[QUICK_START.md](references/trading-api.md)**: CLIコマンドマッピングとJSON例。
- **[USAGE_GUIDE.md](references/usage-guide.md)**: エージェントの相互作用パターンとトラブルシューティングの詳細。
- **[SETUP.md](references/setup.md)**: 環境変数とビルド手順。

## 検証
`./kinesis-trading-skill/scripts/check_config.cjs`を実行して環境を確認してください。
