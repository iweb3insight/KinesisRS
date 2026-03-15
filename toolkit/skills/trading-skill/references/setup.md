# Environment & Setup

## Environment Variables
- `BSC_RPC_URL`: Comma-separated RPC URLs.
- `SOL_RPC_URL`: Solana RPC endpoint.
- `BSC_PRIVATE_KEY_1`, `BSC_PRIVATE_KEY_2`, etc.
- `SOL_PRIVATE_KEY_1`, `SOL_PRIVATE_KEY_2`, etc.
- `JITO_RPC_URL`: (Optional) Jito Block Engine URL.

## Binary Usage
The skill assumes the `solana_claw_coin_cli` binary is built and available in the project root or added to PATH.
To build: `cargo build --release`
Binary location: `target/release/solana_claw_coin_cli`
