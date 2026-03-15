# Mapeamento de Comandos CLI

Todos os comandos suportam `--json` para saída estruturada.

## `quote`
Obtenha valores de swap em tempo real.
- `token_address`: Token alvo.
- `amount`: Valor de entrada.
- `--action`: `buy` ou `sell`.
- `--chain`: `bsc` (padrão) ou `solana`.

## `buy`
- `token_address`: Token a receber.
- `amount`: SOL/BNB nativo para gastar.
- `--slippage`: 0-100 (ex: 15.0 para 15%).
- `--jito-tip`: (Apenas Solana) Valor em SOL.
- `--no-dry-run`: Executar negociação real.

## `sell`
- `token_address`: Token a gastar.
- `amount`: Unidades de token para vender.
- `--no-dry-run`: Executar negociação real.

## `balance`
- `--token-address`: Endereço SPL/BEP20 (opcional).
- `--chain`: `bsc` ou `solana`.

## `wallet`
Mostra endereços derivados para o índice de carteira ativo.
