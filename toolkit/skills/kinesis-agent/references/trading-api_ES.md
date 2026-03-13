# Referencia de la API de Trading

## Formato del Comando

```bash
kinesis-rs <COMANDO> [OPCIONES]
```

## Opciones Globales

| Opción | Descripción | Valor por Defecto |
|--------|-------------|---------------|
| `--json` | Salida en formato JSON | false |
| `--chain` | Tipo de blockchain | bsc |
| `--wallet` | Índice de la billetera | 1 |
| `--dry-run` | Simular trade | true |
| `--no-dry-run` | Trade real | false |

## Comandos

### quote

Obtener cotización de precio del token.

```bash
kinesis-rs quote <TOKEN_ADDRESS> <AMOUNT> [OPCIONES]

# Parámetros
TOKEN_ADDRESS  # Dirección del contrato del token
AMOUNT         # Monto

# Opciones
--action buy|sell    # Dirección del trade (Por defecto: buy)
-c, --chain bsc|solana  # Blockchain (Por defecto: si está vacío)

# Ejemplo
kinesis-rs --json quote DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.1 --action buy --chain solana
```

**Ejemplo de Salida:**
```json
{
  "success": true,
  "amount_out": "1450172355",
  "path": "Raydium"
}
```

---

### buy

Comprar tokens.

```bash
kinesis-rs buy <TOKEN_ADDRESS> <AMOUNT> [OPCIONES]

# Parámetros
TOKEN_ADDRESS  # Dirección del token de destino (Token a comprar)
AMOUNT         # Monto de tokens nativos a gastar (SOL/BNB)

# Opciones
--slippage PERCENT    # Tolerancia de slippage % (Por defecto: 15)
--tip-rate PERCENT    # Propina del desarrollador % (Solana, Por defecto: 0)
--jito-tip SOL       # Propina de Jito (Solana, Por defecto: 0)
-c, --chain          # Blockchain

# Ejemplo
# Simulación (Dry-run, Por defecto)
kinesis-rs --json buy DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --slippage 15 --chain solana

# Trade real
kinesis-rs --json buy DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --slippage 15 --chain solana --no-dry-run

# Con propina de Jito
kinesis-rs --json buy DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --jito-tip 0.001 --chain solana
```

**Ejemplo de Salida (Dry-run):**
```json
{
  "success": true,
  "chain": "solana",
  "stages": [
    {"name": "cli_parse", "duration_ms": 5},
    {"name": "executor_init", "duration_ms": 120},
    {"name": "quote", "duration_ms": 350},
    {"name": "simulate_execution", "duration_ms": 580}
  ],
  "amount_out": "1450172355",
  "gas_estimate": 5000,
  "tx_hash": null,
  "error": null
}
```

---

### sell

Vender tokens.

```bash
kinesis-rs sell <TOKEN_ADDRESS> <AMOUNT> [OPCIONES]

# Parámetros
TOKEN_ADDRESS  # Dirección del token (Token a vender)
AMOUNT         # Cantidad de tokens a vender

# Opciones
--slippage PERCENT    # Tolerancia de slippage % (Por defecto: 15)
--tip-rate PERCENT    # Propina del desarrollador % (Solana)
--jito-tip SOL       # Propina de Jito (Solana)
-c, --chain          # Blockchain

# Ejemplo
kinesis-rs --json sell DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 1000 --slippage 15 --chain solana
```

**Nota:** BSC maneja automáticamente `approveIfNeeded`.

---

### balance

Consultar saldo.

```bash
kinesis-rs balance [OPCIONES]

# Opciones
--token-address ADDRESS  # Dirección del token (Consulta token nativo si está vacío)
-c, --chain              # Blockchain

# Ejemplo
# Consultar saldo de SOL
kinesis-rs --json balance --chain solana

# Consultar saldo de token
kinesis-rs --json balance --token-address DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 --chain solana
```

**Ejemplo de Salida:**
```json
{
  "success": true,
  "asset": "Native SOL",
  "balance_formatted": "3.675360878",
  "balance_raw": "3675360878",
  "owner": "88DqDXNAQZHWscK5HjPavDkBCvsfzmUrDvAV9ZTY5jMv"
}
```

---

### wallet

Mostrar direcciones de la billetera.

```bash
kinesis-rs wallet [OPCIONES]

# Opciones
--wallet INDEX  # Índice de la billetera (Por defecto: 1)

# Ejemplo
kinesis-rs --json wallet
```

**Ejemplo de Salida:**
```json
{
  "success": true,
  "wallets": {
    "1": {
      "bsc": "0x993D6C2e4FfeE5Fed15F5c0861d27a5BA62fCdBE",
      "solana": "88DqDXNAQZHWscK5HjPavDkBCvsfzmUrDvAV9ZTY5jMv"
    }
  }
}
```

---

### config

Mostrar configuración actual.

```bash
kinesis-rs --json config
```

---

### detect

Detectar ruta del token (Solo Solana).

```bash
kinesis-rs detect <TOKEN_ADDRESS> --chain solana

# Ejemplo
kinesis-rs --json detect DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 --chain solana
```

**Ejemplo de Salida:**
```json
{
  "success": true,
  "token_address": "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263",
  "path": "Raydium"
}
```

---

## Formato de Respuesta de Error

```json
{
  "success": false,
  "chain": "solana",
  "stages": [...],
  "error": {
    "type": "rpc_error|simulation_failed|send_failed|config_error|invalid_input|contract_error",
    "message": "Descripción del error",
    "revert_reason": "Razón del revert del contrato (si la hay)",
    "raw_revert_data": "Datos brutos del revert (si los hay)"
  }
}
```
