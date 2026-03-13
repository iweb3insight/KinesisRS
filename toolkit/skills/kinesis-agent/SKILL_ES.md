---
name: kinesis-agent
description: Agente de ejecución de trading multiplataforma unificado que soporta opencode/Gemini/Claude/OpenClaw. Descarga automáticamente el binario, ejecuta trades en Solana/BSC.
binary:
  latest: https://github.com/iweb3insight/KinesisRS/releases/latest
  version: v0.6.5
  install: |
    # Instalación automática
    curl -sL https://raw.githubusercontent.com/iweb3insight/KinesisRS/main/scripts/install.sh | bash
    
    # O descarga manual
    # https://github.com/iweb3insight/KinesisRS/releases/latest
platforms:
  - darwin-arm64
  - darwin-amd64
  - linux-amd64
  - windows-amd64
---

# Agente Kinesis - Ejecución de Trading Multiplataforma

## Instalación

### Instalación Automática

```bash
curl -sL https://raw.githubusercontent.com/iweb3insight/KinesisRS/main/scripts/install.sh | bash
```

### Instalación Manual

```bash
# macOS ARM64
curl -sL https://github.com/iweb3insight/KinesisRS/releases/download/v0.6.5/kinesis-rs-vv0.6.5-macos-arm64.tar.gz -o /tmp/kinesis.tar.gz
tar -xzf /tmp/kinesis.tar.gz -C /tmp/
chmod +x /tmp/kinesis-rs && mv /tmp/kinesis-rs ~/.kinesis/

# Linux
curl -sL https://github.com/iweb3insight/KinesisRS/releases/download/v0.6.5/kinesis-rs-vv0.6.5-linux-amd64.tar.gz -o /tmp/kinesis.tar.gz

# Windows
# Descargue el .zip y extraiga
```

### Verificar Instalación

```bash
~/.kinesis/kinesis-rs --version
~/.kinesis/kinesis-rs wallet
```

## Configurar Variables de Entorno

```bash
# Red Solana
export SOL_RPC_URL="https://api.mainnet-beta.solana.com"  # Mainnet
# export SOL_RPC_URL="https://api.devnet.solana.com"       # Devnet
# export SOL_RPC_URL="https://api.testnet.solana.com"      # Testnet

# Red BSC
export BSC_RPC_URL="https://bsc-dataseed.binance.org/"

# Opcional: Proxy
export HTTPS_PROXY="http://proxy:8080"
```

## Ejecutar Trades (Verificación de Tres Pasos)

```
1. Cotización (Quote) → Obtener cotización
2. Simulación (Dry-run) → Simular trade
3. Ejecución → Trade real
```

---

## Flujos de Trabajo Principales

### Flujo de Compra (Buy)

```
Paso 1: quote <TOKEN> <AMOUNT>
        ↓
Paso 2: buy <TOKEN> <AMOUNT> --dry-run
        ↓
Paso 3: buy <TOKEN> <AMOUNT> --no-dry-run (El usuario confirma)
```

### Flujo de Venta (Sell)

```
Paso 1: sell <TOKEN> <AMOUNT> --dry-run
        ↓
Paso 2: sell <TOKEN> <AMOUNT> --no-dry-run
```

---

## Adaptadores de Plataforma

| Plataforma | Método de Llamada | Referencia |
|------------|-------------------|------------|
| opencode   | Shell             | adapters/opencode.md |
| openclaw   | Shell             | adapters/openclaw.md |
| Gemini     | mcp__local__execute | adapters/gemini.md |
| Claude     | MCP/Bash          | adapters/claude.md |

---

## Redes Soportadas

| Red     | SOL_RPC_URL | Saldo | Estado de Compra |
|---------|-------------|-------|------------------|
| Mainnet | api.mainnet-beta.solana.com | Necesario | ✅ |
| Devnet  | api.devnet.solana.com | 3.67 SOL | ⚠️ ATA |
| Testnet | api.testnet.solana.com | 3.00 SOL | ⚠️ ATA |

Vea: references/network-matrix.md

---

## Manejo de Errores

| Error | Causa | Solución |
|-------|-------|----------|
| AccountNotFound | Saldo 0 | Depositar SOL |
| Error ATA | Sin LUT en Devnet | Usar ruta Pump.fun |
| ROUTE_NOT_FOUND | Token no indexado | Esperar o cambiar token |

Vea: references/error-codes.md

---

## Materiales de Referencia

- [API de Trading](references/trading-api.md)
- [Matriz de Red](references/network-matrix.md)
- [Códigos de Error](references/error-codes.md)
