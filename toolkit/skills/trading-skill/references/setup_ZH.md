# 环境与安装

## 环境变量
- `BSC_RPC_URL`: 以逗号分隔的 RPC URL。
- `SOL_RPC_URL`: Solana RPC 终结点。
- `BSC_PRIVATE_KEY_1`, `BSC_PRIVATE_KEY_2` 等。
- `SOL_PRIVATE_KEY_1`, `SOL_PRIVATE_KEY_2` 等。
- `JITO_RPC_URL`: (可选) Jito Block Engine URL。

## 二进制文件使用
此技能假设 `solana_claw_coin_cli` 二进制文件已构建，并可在项目根目录中找到，或已添加到 PATH 中。
构建命令：`cargo build --release`
二进制文件位置：`target/release/solana_claw_coin_cli`
