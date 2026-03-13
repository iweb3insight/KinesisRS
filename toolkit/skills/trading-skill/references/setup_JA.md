# 環境とセットアップ

## 環境変数
- `BSC_RPC_URL`: カンマ区切りのRPC URL。
- `SOL_RPC_URL`: Solana RPCエンドポイント。
- `BSC_PRIVATE_KEY_1`, `BSC_PRIVATE_KEY_2` など。
- `SOL_PRIVATE_KEY_1`, `SOL_PRIVATE_KEY_2` など。
- `JITO_RPC_URL`: (任意) Jito Block EngineのURL。

## バイナリの使用
このスキルは、`solana_claw_coin_cli`バイナリがビルドされ、プロジェクトのルートディレクトリにあるか、PATHに追加されていることを前提としています。
ビルド方法: `cargo build --release`
バイナリの場所: `target/release/solana_claw_coin_cli`
