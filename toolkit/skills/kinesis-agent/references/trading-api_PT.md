# Referência da API de Negociação

## Formato do Comando

```bash
kinesis-rs <COMANDO> [OPÇÕES]
```

## Opções Globais

| Opção | Descrição | Valor Padrão |
|--------|-------------|---------------|
| `--json` | Saída em formato JSON | false |
| `--chain` | Tipo de blockchain | bsc |
| `--wallet` | Índice da carteira | 1 |
| `--dry-run` | Simular negociação | true |
| `--no-dry-run` | Negociação real | false |

## Comandos

### quote

Obter cotação de preço do token.

```bash
kinesis-rs quote <TOKEN_ADDRESS> <AMOUNT> [OPÇÕES]

# Parâmetros
TOKEN_ADDRESS  # Endereço do contrato do token
AMOUNT         # Valor

# Opções
--action buy|sell    # Direção da negociação (Padrão: buy)
-c, --chain bsc|solana  # Blockchain (Padrão: bsc)

# Exemplo
kinesis-rs --json quote DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.1 --action buy --chain solana
```

**Exemplo de Saída:**
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
kinesis-rs buy <TOKEN_ADDRESS> <AMOUNT> [OPÇÕES]

# Parâmetros
TOKEN_ADDRESS  # Endereço do token alvo (Token a comprar)
AMOUNT         # Valor de tokens nativos a gastar (SOL/BNB)

# Opções
--slippage PERCENT    # Tolerância de slippage % (Padrão: 15)
--tip-rate PERCENT    # Gorjeta do desenvolvedor % (Solana, Padrão: 0)
--jito-tip SOL       # Gorjeta Jito (Solana, Padrão: 0)
-c, --chain          # Blockchain

# Exemplo
# Simulação (Dry-run, Padrão)
kinesis-rs --json buy DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --slippage 15 --chain solana

# Negociação real
kinesis-rs --json buy DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --slippage 15 --chain solana --no-dry-run

# Com gorjeta Jito
kinesis-rs --json buy DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --jito-tip 0.001 --chain solana
```

**Exemplo de Saída (Dry-run):**
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
kinesis-rs sell <TOKEN_ADDRESS> <AMOUNT> [OPÇÕES]

# Parâmetros
TOKEN_ADDRESS  # Endereço do token (Token a vender)
AMOUNT         # Quantidade de tokens a vender

# Opções
--slippage PERCENT    # Tolerância de slippage % (Padrão: 15)
--tip-rate PERCENT    # Gorjeta do desenvolvedor % (Solana)
--jito-tip SOL       # Gorjeta Jito (Solana)
-c, --chain          # Blockchain

# Exemplo
kinesis-rs --json sell DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 1000 --slippage 15 --chain solana
```

**Nota:** BSC lida automaticamente com `approveIfNeeded`.

---

### balance

Consultar saldo.

```bash
kinesis-rs balance [OPÇÕES]

# Opções
--token-address ADDRESS  # Endereço do token (Consulta token nativo se vazio)
-c, --chain              # Blockchain

# Exemplo
# Consultar saldo de SOL
kinesis-rs --json balance --chain solana

# Consultar saldo de token
kinesis-rs --json balance --token-address DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 --chain solana
```

**Exemplo de Saída:**
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

Exibir endereços da carteira.

```bash
kinesis-rs wallet [OPÇÕES]

# Opções
--wallet INDEX  # Índice da carteira (Padrão: 1)

# Exemplo
kinesis-rs --json wallet
```

**Exemplo de Saída:**
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

Exibir configuração atual.

```bash
kinesis-rs --json config
```

---

### detect

Detectar caminho do token (Apenas Solana).

```bash
kinesis-rs detect <TOKEN_ADDRESS> --chain solana

# Exemplo
kinesis-rs --json detect DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 --chain solana
```

**Exemplo de Saída:**
```json
{
  "success": true,
  "token_address": "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263",
  "path": "Raydium"
}
```

---

## Formato de Resposta de Erro

```json
{
  "success": false,
  "chain": "solana",
  "stages": [...],
  "error": {
    "type": "rpc_error|simulation_failed|send_failed|config_error|invalid_input|contract_error",
    "message": "Descrição do erro",
    "revert_reason": "Razão do revert do contrato (se houver)",
    "raw_revert_data": "Dados brutos do revert (se houver)"
  }
}
```
