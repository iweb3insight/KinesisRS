# Ambiente & Configuração

## Variáveis de Ambiente
- `BSC_RPC_URL`: URLs RPC separadas por vírgula.
- `SOL_RPC_URL`: Endpoint RPC da Solana.
- `BSC_PRIVATE_KEY_1`, `BSC_PRIVATE_KEY_2`, etc.
- `SOL_PRIVATE_KEY_1`, `SOL_PRIVATE_KEY_2`, etc.
- `JITO_RPC_URL`: (Opcional) URL do Jito Block Engine.

## Uso do Binário
A habilidade assume que o binário `solana_claw_coin_cli` está compilado e disponível na raiz do projeto ou adicionado ao PATH.
Para compilar: `cargo build --release`
Localização do binário: `target/release/solana_claw_coin_cli`
