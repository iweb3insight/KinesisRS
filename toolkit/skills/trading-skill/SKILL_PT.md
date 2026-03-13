---
name: kinesis-trading-skill
description: Execução de negociação multi-chain para BSC e Solana. Use para comprar/vender tokens, cotar preços e verificar saldos no PancakeSwap, Pump.fun e Raydium. Suporta submissão de Jito bundle e multi-RPC racing.
---

# Habilidade de Execução de Negociação (KinesisRS)

Esta habilidade permite que o Gemini CLI atue como um agente de negociação de cripto de alto desempenho.

## Fluxos de Trabalho Principais

### 1. Fluxo de Compra (Execução Segura)
1. **Obter Cotação (Quote)**: Execute `quote` para obter o preço em tempo real.
2. **Simular**: Execute com `--dry-run` para verificar a lógica e o gás.
3. **Executar**: Confirme com o usuário e execute com `--no-dry-run`.

### 2. Fluxo de Venda (Auto-Aprovação)
1. **Simular**: BSC lida com `approveIfNeeded` automaticamente.
2. **Executar**: Execute com `--no-dry-run`.

## Materiais de Referência

- **[QUICK_START.md](references/trading-api.md)**: Mapeamento de comandos CLI e exemplos JSON.
- **[USAGE_GUIDE.md](references/usage-guide.md)**: Aprofundamento nos padrões de interação do Agente e resolução de problemas.
- **[SETUP.md](references/setup.md)**: Variáveis de ambiente e instruções de build.

## Verificação
Execute `./kinesis-trading-skill/scripts/check_config.cjs` para verificar seu ambiente.
