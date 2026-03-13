# Kinesis.rs Rust v1.0

Kinesis.rs は、主に LLM エージェント向けに設計された、ステートレスで JSON ファーストなマルチチェーン暗号資産取引実行レイヤーです。

## 特徴
- **マルチチェーン対応**: BNB スマートチェーン (BSC) および Solana のネイティブ実行をサポート。
- **エージェントファースト設計**: LLM とのシームレスな統合を実現する JSON ファーストの通信プロトコル。
- **ハイパフォーマンス**: パラレル RPC レーシングとトランザクションの事前構築。
- **Solana パス検出**: Pump.fun および Raydium V3 の自動検出と実行。
- **セキュリティ**: 強制的な dry-run シミュレーションと安全な秘密鍵管理。

## クイックスタート
1. リポジトリをクローンします。
2. `.env.example` を `.env` にコピーし、秘密鍵を追加します。
3. ビルド: `cargo build --release`
4. 実行: `./target/release/solana_claw_coin_cli balance --chain solana`

## Skills
Gemini CLI やその他のエージェントフレームワークとの統合方法については、`skills/` ディレクトリを確認してください。
