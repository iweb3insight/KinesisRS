# Guía de Uso Detallada de KinesisRS

## 1. Filosofía de Diseño Principal
KinesisRS es un sistema de trading **Agent-First**. Cada elección de diseño se realiza para garantizar que los Agentes LLM (como Gemini, Claude) puedan ejecutar operaciones complejas de forma segura y precisa.

- **Sin Estado (Stateless)**: Cada comando contiene todo el contexto necesario para la ejecución.
- **JSON-First**: Recomendamos usar siempre el flag `--json` para que el Agente pueda analizar con precisión el `TradeResult`.
- **Seguridad Primero**: `--dry-run` está habilitado por defecto para forzar la verificación de la simulación.

## 2. Mejores Prácticas de Interacción (Para Agentes)

### Flujo de Compra
1. **Cotización (Quote)**: Ejecute `quote`. Analice `amount_out` y preséntelo al usuario.
2. **Evaluación de Riesgo**: Ejecute `buy --dry-run`.
   - Verifique `duration_ms` en `stages`.
   - Verifique `gas_estimate`.
   - Si tiene éxito, muestre el resultado de la simulación y pida la confirmación del usuario.
3. **Trade Real**: Tras la confirmación, ejecute `buy --no-dry-run`.

### Flujo de Venta
- Los comandos de venta detectan automáticamente si se necesita un `approve` (para BSC).
- si aparece una etapa de `approve` en `stages`, significa que se produjo una operación de aprobación.

## 3. Funciones Específicas de Solana

### Aceleración Jito Bundle
En Solana, para evitar el front-running (MEV) o para asegurar que las operaciones se realicen durante la congestión, se debe usar Jito:
```bash
./solana_claw_coin_cli buy <TOKEN> 0.1 --chain solana --jito-tip 0.001
```
- **Parámetro**: `--jito-tip` está en SOL. Rango recomendado: 0.0001 - 0.01.

### Enrutamiento Inteligente Raydium
Para tokens que no son de Pump.fun (ej. USDC, pools SOL/USDT), el ejecutor llama automáticamente a la API de Trade de Raydium V3 para buscar la ruta óptima.

## 4. Códigos de Error Comunes y Manejo

| Mensaje de Error | Causa | Sugerencia |
| :--- | :--- | :--- |
| `AccountNotFound` | El saldo de la billetera es 0 o no está inicializado | Deposite el token nativo (BNB/SOL) |
| `SlippageExceeded` | Alta volatilidad de precios | Aumente el `--slippage` (ej. 25.0) |
| `RouteNotFound` | Liquidez insuficiente o API no indexada | Verifique la dirección del token o intente con un monto pequeño |
| `Simulation failed` | La ejecución falló (revert) | Verifique `raw_revert_data` para más detalles |

## 5. Auditoría de Rendimiento
Al analizar el array `stages` en el `TradeResult`, el Agente puede calcular:
- **Latencia de API**: `duration_ms` de la etapa `quote`.
- **Latencia de Ejecución**: `duration_ms` de la etapa `buy`/`sell`.
- **Duración Total**: Suma de `duration_ms` de todas las etapas.
