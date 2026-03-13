# Mapeo de Comandos CLI

Todos los comandos soportan `--json` para salida estructurada.

## `quote`
Obtenga montos de swap en tiempo real.
- `token_address`: Token de destino.
- `amount`: Monto de entrada.
- `--action`: `buy` o `sell`.
- `--chain`: `bsc` (por defecto) o `solana`.

## `buy`
- `token_address`: Token a recibir.
- `amount`: SOL/BNB nativo para gastar.
- `--slippage`: 0-100 (ej: 15.0 para 15%).
- `--jito-tip`: (Solo Solana) Monto en SOL.
- `--no-dry-run`: Ejecutar trade real.

## `sell`
- `token_address`: Token a gastar.
- `amount`: Unidades de token para vender.
- `--no-dry-run`: Ejecutar trade real.

## `balance`
- `--token-address`: Dirección SPL/BEP20 (opcional).
- `--chain`: `bsc` o `solana`.

## `wallet`
Muestra direcciones derivadas para el índice de billetera activo.
