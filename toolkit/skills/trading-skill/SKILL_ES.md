---
name: kinesis-trading-skill
description: Ejecución de trading multi-chain para BSC y Solana. Úselo para comprar/vender tokens, cotizar precios y verificar saldos en PancakeSwap, Pump.fun y Raydium. Soporta envío de Jito bundle y multi-RPC racing.
---

# Habilidad de Ejecución de Trading (KinesisRS)

Esta habilidad permite que Gemini CLI actúe como un agente de trading de cripto de alto rendimiento.

## Flujos de Trabajo Principales

### 1. Flujo de Compra (Ejecución Segura)
1. **Obtener Cotización (Quote)**: Ejecute `quote` para obtener el precio en tiempo real.
2. **Simular**: Ejecute con `--dry-run` para verificar la lógica y el gas.
3. **Ejecutar**: Confirme con el usuario y ejecute con `--no-dry-run`.

### 2. Flujo de Venta (Auto-Aprobación)
1. **Simular**: BSC maneja `approveIfNeeded` automáticamente.
2. **Ejecutar**: Ejecute con `--no-dry-run`.

## Materiales de Referência

- **[QUICK_START.md](references/trading-api.md)**: Mapeo de comandos CLI y ejemplos JSON.
- **[USAGE_GUIDE.md](references/usage-guide.md)**: Profundización en los patrones de interacción del Agente y resolución de problemas.
- **[SETUP.md](references/setup.md)**: Variables de entorno e instrucciones de build.

## Verificación
Ejecute `./kinesis-trading-skill/scripts/check_config.cjs` para verificar su entorno.
