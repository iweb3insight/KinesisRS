# Referencia de Códigos de Error

## Tipos de Error

| Tipo de Error       | Descripción                       | Causas Comunes                 |
|---------------------|-----------------------------------|--------------------------------|
| `rpc_error`         | Error en la petición RPC          | Problemas de red, RPC no disponible |
| `simulation_failed` | Error en la simulación del trade  | Lógica de revert, saldo insuficiente |
| `send_failed`       | Error al enviar la transacción    | Problemas de firma, congestión de red |
| `config_error`      | Error de configuración            | Falta la clave privada, RPC no configurado |
| `invalid_input`     | Parámetros de entrada inválidos   | Dirección inválida, monto negativo |
| `contract_error`    | Error de ejecución del contrato   | Revert, problemas de permisos |

---

## Errores Comunes y Soluciones

### 1. AccountNotFound

**Mensaje de Error:**
```json
{"error": {"type": "contract_error", "message": "Simulation failed: \"AccountNotFound\""}}
```

**Causa:** El saldo de la billetera es 0.

**Solución:**
```bash
# Deposite SOL en la billetera
# Dirección de la billetera: 88DqDXNAQZHWscK5HjPavDkBCvsfzmUrDvAV9ZTY5jMv
```

---

### 2. Error de ATA

**Mensaje de Error:**
```
RPC Error -32602: invalid transaction: Transaction loads an address table account that doesn't exist
```

**Causa:** La Address Lookup Table (LUT) creada por Raydium no existe en Devnet/Testnet.

**Redes Afectadas:** Devnet, Testnet

**Solución:**
```bash
# Opción 1: Use la simulación (dry-run) en Mainnet
export SOL_RPC_URL="https://api.mainnet-beta.solana.com"

# Opción 2: Use la ruta de Pump.fun (si está disponible)
# Pump.fun no utiliza LUT
```

---

### 3. ROUTE_NOT_FOUND

**Mensaje de Error:**
```json
{"error": {"message": "ROUTE_NOT_FOUND"}}
```

**Causa:** La API de Raydium aún no ha indexado el pool de liquidez para este token.

**Escenarios Comunes:**
- Tokens de Pump.fun recién creados
- Tokens que aún no se han "graduado"
- Recién graduados pero aún no sincronizados

**Solución:**
```bash
# Opción 1: Cambie a un token con liquidez
# Ej. BONK, USDC, SOL

# Opción 2: Espere a que el token se gradúe a Raydium

# Opción 3: Opere directamente en Pump.fun (si no se ha graduado)
```

---

### 4. REQ_SWAP_RESPONSE_ERROR

**Mensaje de Error:**
```json
{"error": {"message": "Failed to parse Raydium transaction: ... REQ_SWAP_RESPONSE_ERROR"}}
```

**Causa:** La API de Raydium devolvió una respuesta inválida.

**Escenarios Comunes:**
- Llamada secundaria tras fallo en la API de Cotización (Quote)
- Problemas temporales de la API

**Solución:**
```bash
# Reintente
kinesis-rs buy <TOKEN> <AMOUNT> --dry-run

# O espere y reintente
```

---

### 5. REQ_COMPUTE_UNIT_PRICE_MICRO_LAMPORTS_ERROR

**Mensaje de Error:**
```json
{"error": {"message": "REQ_COMPUTE_UNIT_PRICE_MICRO_LAMPORTS_ERROR"}}
```

**Causa:** Problema con la configuración del Compute Unit Price.

**Solución:**
```bash
# Espere a la recuperación de la API o reintente
```

---

### 6. SlippageExceeded

**Mensaje de Error:**
```json
{"error": {"revert_reason": "SlippageExceeded"}}
```

**Causa:** La volatilidad del precio superó el slippage configurado.

**Solución:**
```bash
# Aumente el slippage
kinesis-rs buy <TOKEN> <AMOUNT> --slippage 25 --chain solana

# O disminuya el monto
kinesis-rs buy <TOKEN> <AMOUNT> --slippage 15 --chain solana
```

---

### 7. Insufficient Liquidity

**Mensaje de Error:**
```json
{"error": {"revert_reason": "FreedomRouter: INSUFFICIENT_LIQUIDITY"}}
```

**Causa:** Liquidez insuficiente en el pool.

**Solución:**
```bash
# Disminuya el monto de compra
kinesis-rs buy <TOKEN> 0.01 --chain solana

# Espere a que se recupere la liquidez
```

---

### 8. Insufficient Gas / Insufficient Funds

**Mensaje de Error:**
```json
{"error": {"revert_reason": "insufficient funds for gas * price + value"}}
```

**Causa:** Saldo insuficiente para pagar las tarifas de gas.

**Solución:**
```bash
# Deposite el token nativo (SOL/BNB)
```

---

### 9. Token account not found

**Mensaje de Error:**
```json
{"error": {"message": "Token account not found: <TOKEN_ADDRESS>"}}
```

**Causa:** La billetera no posee este token.

**Solución:**
```bash
# Compre el token primero para crear la ATA
# O verifique si la dirección del token es correcta
```

---

### 10. Dirección de Token Inválida

**Mensaje de Error:**
```json
{"error": {"message": "Invalid token address"}}
```

**Causa:** Error en el formato de la dirección del token.

**Solución:**
```bash
# Verifique el formato de la dirección
# Solana: Codificado en Base58, 32-44 caracteres
# BSC: Comienza con 0x, 40 caracteres hexadecimales
```

---

## Flujo de Manejo de Erros

```
Petición del Usuario
    ↓
Ejecutar Comando
    ↓
┌─────────────────────────────────────┐
│  ¿Éxito?                            │
│  ↓ Sí                               │
│  Retornar respuesta de éxito        │
│  ↓ No                               │
│  Analizar tipo de error             │
│  ↓                                   │
│  ┌─────────────────────────────────┐│
│  │ rpc_error                       ││
│  │  - Verificar conexión de red    ││
│  │  - Cambiar RPC                  ││
│  ├─────────────────────────────────┤│
│  │ simulation_failed                ││
│  │  - Verificar saldo              ││
│  │  - Verificar aprobación          ││
│  │  - Ajustar slippage             ││
│  ├─────────────────────────────────┤│
│  │ contract_error                  ││
│  │  - Analizar revert_reason       ││
│  │  - Consultar soluc. específicas ││
└─────────────────────────────────────┘
    ↓
Retornar respuesta de error
    ↓
Mostrar error + sugerencias al usuario
```

---

## Consejos de Depuración

### 1. Habilitar Logs de Depuración

```bash
RUST_LOG=debug kinesis-rs --json quote ...
```

### 2. Verificar Conexión de Red

```bash
# Probar RPC directamente
curl -X POST https://api.mainnet-beta.solana.com -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}'

# Probar API de Raydium
curl "https://transaction-v1.raydium.io/compute/swap-base-in?inputMint=So11111111111111111111111111111111111111112&outputMint=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v&amount=1000000000&slippageBps=50"
```

### 3. Verificar Saldo

```bash
kinesis-rs --json balance --chain solana
```
