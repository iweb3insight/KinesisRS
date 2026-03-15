# Matriz de Red

## Redes Soportadas

| Red            | Chain ID | URL RPC                                 | Estado |
|----------------|----------|-----------------------------------------|--------|
| BSC Mainnet    | 56       | bsc-dataseed.binance.org                | ✅     |
| BSC Testnet    | 97       | data-seed-prebsc-1-s1.binance.org:8545  | ✅     |
| Solana Mainnet | -        | api.mainnet-beta.solana.com             | ✅     |
| Solana Devnet  | -        | api.devnet.solana.com                   | ⚠️     |
| Solana Testnet | -        | api.testnet.solana.com                  | ⚠️     |

---

## Comparación Detallada de la Red Solana

### Mainnet (api.mainnet-beta.solana.com)

| Función        | Estado | Descripción                 |
|----------------|--------|-----------------------------|
| Cotización (Quote)| ✅   | Funcionando normalmente     |
| Compra (Raydium)| ✅     | Requiere saldo en SOL       |
| Compra (Pump.fun)| ✅    | Requiere saldo en SOL       |
| Venta          | ✅     | Requiere saldo de tokens    |
| Saldo          | ✅     | Funcionando normalmente     |
| Detectar       | ✅     | Funcionando normalmente     |

**Saldo de la Clave Privada de Prueba:** 0 SOL (Sin fondos reales)

---

### Devnet (api.devnet.solana.com)

| Función        | Estado | Descripción                 |
|----------------|--------|-----------------------------|
| Cotización (Quote)| ✅   | Funcionando normalmente     |
| Compra (Raydium)| ❌     | Error de ATA               |
| Compra (Pump.fun)| ⚠️    | No probado                 |
| Venta          | ❌     | Error de ATA               |
| Saldo          | ✅     | Funcionando normalmente     |
| Detectar       | ✅     | Funcionando normalmente     |

**Saldo de la Clave Privada de Prueba:** 3.67 SOL

**Problemas Conocidos:**
- Las transacciones de Raydium usan Address Lookup Table (LUT)
- LUT no existe en Devnet, causando `Transaction loads an address table account that doesn't exist`

---

### Testnet (api.testnet.solana.com)

| Función        | Estado | Descripción                 |
|----------------|--------|-----------------------------|
| Cotización (Quote)| ✅   | Funcionando normalmente     |
| Compra (Raydium)| ❌     | Error de ATA               |
| Compra (Pump.fun)| ⚠️    | No probado                 |
| Venta          | ❌     | Error de ATA               |
| Saldo          | ✅     | Funcionando normalmente     |
| Detectar       | ✅     | Funcionando normalmente     |

**Saldo de la Clave Privada de Prueba:** 3.00 SOL

**Problemas Conocidos:** Lo mismo que en Devnet

---

## Red BSC

### Mainnet

| Función        | Estado | Descripción                 |
|----------------|--------|-----------------------------|
| Cotización (Quote)| ✅   | PancakeSwap                 |
| Compra         | ✅     | Requiere saldo en BNB       |
| Venta          | ✅     | Requiere saldo de tokens    |
| Aprobar        | ✅     | Manejado automáticamente    |
| Saldo          | ✅     | Funcionando normalmente     |

### Testnet

| Función        | Estado | Descripción                 |
|----------------|--------|-----------------------------|
| Cotización (Quote)| ✅   | PancakeSwap                 |
| Compra         | ✅     | Requiere BNB de prueba      |
| Venta          | ✅     | Requiere tokens de prueba   |
| Saldo          | ✅     | Funcionando normalmente     |

---

## Recomendaciones para la Selección de Red

### Desarrollo/Pruebas

| Escenario            | Red Recomendada     | Razón                       |
|----------------------|---------------------|-----------------------------|
| Cotización Rápida    | Devnet/Testnet      | Gratis, rápido              |
| Probar Pump.fun      | Mainnet (dry-run)   | No hay Pump.fun en Devnet   |
| Probar Raydium       | Mainnet (dry-run)   | Problemas de ATA en Devnet  |
| Prueba de Integración| Testnet             | Más cerca de Mainnet        |

### Entorno de Producción

| Escenario            | Red Recomendada     | Razón                       |
|----------------------|---------------------|-----------------------------|
| Trading Real         | Mainnet             | Única opción                |
| Verificación de Trade| Mainnet (dry-run)   | Simula el entorno real      |

---

## Configuración de Variables de Entorno

```bash
# Solana
export SOL_RPC_URL="https://api.mainnet-beta.solana.com"  # Mainnet
export SOL_RPC_URL="https://api.devnet.solana.com"       # Devnet
export SOL_RPC_URL="https://api.testnet.solana.com"      # Testnet

# BSC
export BSC_RPC_URL="https://bsc-dataseed.binance.org/"   # Mainnet
export BSC_RPC_URL="https://data-seed-prebsc-1-s1.binance.org:8545/"  # Testnet
```

---

## Configuración Multi-RPC

Soporta múltiples RPCs separados por comas:

```bash
# BSC
export BSC_RPC_URL="https://bsc-dataseed.binance.org/,https://bsc-dataseed1.binance.org/,https://bsc-dataseed2.binance.org/"

# Solana (Respaldo Multi-RPC)
export SOL_RPC_URL="https://api.mainnet-beta.solana.com,https://solana-api.projectserum.com"
```

---

## Tokens de Prueba

### Solana Devnet/Testnet

| Token | Dirección | Caso de Uso |
|-------|---------|----------|
| - | - | No hay tokens de prueba disponibles actualmente |

### Solana Mainnet

| Token | Dirección | Liquidez |
|-------|---------|-----------|
| BONK | DezXAX8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 | Alta |
| USDC | EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v | Alta |
| SOL | So11111111111111111111111111111111111111112 | La más alta |

### BSC Testnet

| Token | Dirección |
|-------|---------|
| BNB | - (Nativo) |
| Token de Prueba | 0x... |

---

## Solución de Problemas

### Error de ATA en Devnet/Testnet

```
RPC Error -32602: invalid transaction: Transaction loads an address table account that doesn't exist
```

**Solución:**
1. Pruebe con Mainnet dry-run
2. Espere a la corrección en una versión futura

### ROUTE_NOT_FOUND

```
Raydium API error: ROUTE_NOT_FOUND
```

**Causa:** El token no tiene pool de liquidez en Raydium

**Solución:**
1. Cambie a un token con liquidez
2. Espere a que el token se gradúe a Raydium
