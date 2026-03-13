# Guia de Uso Detalhado do KinesisRS

## 1. Filosofia de Design Principal
KinesisRS é um sistema de negociação **Agent-First**. Cada escolha de design é feita para garantir que Agentes LLM (como Gemini, Claude) possam executar negociações complexas com segurança e precisão.

- **Sem Estado (Stateless)**: Cada comando contém todo o contexto necessário para a execução.
- **JSON-First**: Recomendamos sempre usar a flag `--json` para que o Agente possa analisar precisamente o `TradeResult`.
- **Segurança em Primeiro Lugar**: `--dry-run` é habilitado por padrão para forçar a verificação da simulação.

## 2. Melhores Práticas de Interação (Para Agentes)

### Fluxo de Compra
1. **Cotação (Quote)**: Execute `quote`. Analise `amount_out` e apresente ao usuário.
2. **Avaliação de Risco**: Execute `buy --dry-run`.
   - Verifique `duration_ms` em `stages`.
   - Verifique `gas_estimate`.
   - Se for bem-sucedido, mostre o resultado da simulação e peça a confirmação do usuário.
3. **Negociação Real**: Após a confirmação, execute `buy --no-dry-run`.

### Fluxo de Venda
- Os comandos de venda detectam automaticamente se um `approve` é necessário (para BSC).
- se um estágio de `approve` aparecer em `stages`, isso significa que uma operação de aprovação ocorreu.

## 3. Recursos Específicos da Solana

### Aceleração Jito Bundle
Na Solana, para evitar front-running (MEV) ou para garantir a execução em momentos de congestionamento, o Jito deve ser usado:
```bash
./solana_claw_coin_cli buy <TOKEN> 0.1 --chain solana --jito-tip 0.001
```
- **Parâmetro**: `--jito-tip` é em SOL. Faixa recomendada: 0.0001 - 0.01.

### Roteamento Inteligente Raydium
Para tokens que não são do Pump.fun (ex: USDC, pools SOL/USDT), o executor chama automaticamente a API de Negociação do Raydium V3 para a busca do caminho ideal.

## 4. Códigos de Erro Comuns & Manuseio

| Mensagem de Erro | Causa | Sugestão |
| :--- | :--- | :--- |
| `AccountNotFound` | Saldo da carteira é 0 ou não inicializado | Deposite o token nativo (BNB/SOL) |
| `SlippageExceeded` | Alta volatilidade de preço | Aumente o `--slippage` (ex: 25.0) |
| `RouteNotFound` | Liquidez insuficiente ou API não indexada | Verifique o endereço do token ou tente um valor pequeno |
| `Simulation failed` | Execução revertida | Verifique `raw_revert_data` para detalhes |

## 5. Auditoria de Desempenho
Ao analisar o array `stages` no `TradeResult`, o Agente pode calcular:
- **Latência da API**: `duration_ms` do estágio `quote`.
- **Latência de Execução**: `duration_ms` do estágio `buy`/`sell`.
- **Duração Total**: Soma de `duration_ms` de todos os estágios.
