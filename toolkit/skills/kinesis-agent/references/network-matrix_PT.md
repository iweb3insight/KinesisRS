# Matriz de Rede

## Redes Suportadas

| Rede           | Chain ID | URL RPC                                 | Status |
|----------------|----------|-----------------------------------------|--------|
| BSC Mainnet    | 56       | bsc-dataseed.binance.org                | ✅     |
| BSC Testnet    | 97       | data-seed-prebsc-1-s1.binance.org:8545  | ✅     |
| Solana Mainnet | -        | api.mainnet-beta.solana.com             | ✅     |
| Solana Devnet  | -        | api.devnet.solana.com                   | ⚠️     |
| Solana Testnet | -        | api.testnet.solana.com                  | ⚠️     |

---

## Comparação Detalhada da Rede Solana

### Mainnet (api.mainnet-beta.solana.com)

| Recurso        | Status | Descrição                   |
|----------------|--------|-----------------------------|
| Cotação (Quote)| ✅     | Funcionando normalmente     |
| Compra (Raydium)| ✅     | Requer saldo em SOL         |
| Compra (Pump.fun)| ✅    | Requer saldo em SOL         |
| Venda          | ✅     | Requer saldo de tokens      |
| Saldo          | ✅     | Funcionando normalmente     |
| Detectar       | ✅     | Funcionando normalmente     |

**Saldo da Chave Privada de Teste:** 0 SOL (Sem fundos reais)

---

### Devnet (api.devnet.solana.com)

| Recurso        | Status | Descrição                   |
|----------------|--------|-----------------------------|
| Cotação (Quote)| ✅     | Funcionando normalmente     |
| Compra (Raydium)| ❌     | Erro de ATA                 |
| Compra (Pump.fun)| ⚠️    | Não testado                 |
| Venda          | ❌     | Erro de ATA                 |
| Saldo          | ✅     | Funcionando normalmente     |
| Detectar       | ✅     | Funcionando normalmente     |

**Saldo da Chave Privada de Teste:** 3.67 SOL

**Problemas Conhecidos:**
- Transações Raydium usam Address Lookup Table (LUT)
- LUT não existe na Devnet, causando `Transaction loads an address table account that doesn't exist`

---

### Testnet (api.testnet.solana.com)

| Recurso        | Status | Descrição                   |
|----------------|--------|-----------------------------|
| Cotação (Quote)| ✅     | Funcionando normalmente     |
| Compra (Raydium)| ❌     | Erro de ATA                 |
| Compra (Pump.fun)| ⚠️    | Não testado                 |
| Venda          | ❌     | Erro de ATA                 |
| Saldo          | ✅     | Funcionando normalmente     |
| Detectar       | ✅     | Funcionando normalmente     |

**Saldo da Chave Privada de Teste:** 3.00 SOL

**Problemas Conhecidos:** Mesmo que na Devnet

---

## Rede BSC

### Mainnet

| Recurso        | Status | Descrição                   |
|----------------|--------|-----------------------------|
| Cotação (Quote)| ✅     | PancakeSwap                 |
| Compra         | ✅     | Requer saldo em BNB         |
| Venda          | ✅     | Requer saldo de tokens      |
| Aprovar        | ✅     | Lida automaticamente        |
| Saldo          | ✅     | Funcionando normalmente     |

### Testnet

| Recurso        | Status | Descrição                   |
|----------------|--------|-----------------------------|
| Cotação (Quote)| ✅     | PancakeSwap                 |
| Compra         | ✅     | Requer BNB de teste         |
| Venda          | ✅     | Requer tokens de teste      |
| Saldo          | ✅     | Funcionando normalmente     |

---

## Recomendações de Seleção de Rede

### Desenvolvimento/Teste

| Cenário              | Rede Recomendada    | Razão                       |
|----------------------|---------------------|-----------------------------|
| Teste de Cotação Rápida| Devnet/Testnet    | Gratuito, rápido            |
| Testar Pump.fun      | Mainnet (dry-run)   | Sem Pump.fun na Devnet      |
| Testar Raydium      | Mainnet (dry-run)   | Problemas de ATA na Devnet  |
| Teste de Integração  | Testnet             | Mais próximo da Mainnet     |

### Ambiente de Produção

| Cenário              | Rede Recomendada    | Razão                       |
|----------------------|---------------------|-----------------------------|
| Negociação Real      | Mainnet             | Única opção                 |
| Verificação de Negoc. | Mainnet (dry-run)   | Simula ambiente real        |

---

## Configuração de Variáveis de Ambiente

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

## Configuração Multi-RPC

Suporta múltiplos RPCs separados por vírgula:

```bash
# BSC
export BSC_RPC_URL="https://bsc-dataseed.binance.org/,https://bsc-dataseed1.binance.org/,https://bsc-dataseed2.binance.org/"

# Solana (Backup Multi-RPC)
export SOL_RPC_URL="https://api.mainnet-beta.solana.com,https://solana-api.projectserum.com"
```

---

## Tokens de Teste

### Solana Devnet/Testnet

| Token | Endereço | Caso de Uso |
|-------|---------|----------|
| - | - | Nenhum token de teste disponível atualmente |

### Solana Mainnet

| Token | Endereço | Liquidez |
|-------|---------|-----------|
| BONK | DezXAX8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 | Alta |
| USDC | EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v | Alta |
| SOL | So11111111111111111111111111111111111111112 | Altíssima |

### BSC Testnet

| Token | Endereço |
|-------|---------|
| BNB | - (Nativo) |
| Token de Teste | 0x... |

---

## Resolução de Problemas

### Erro de ATA na Devnet/Testnet

```
RPC Error -32602: invalid transaction: Transaction loads an address table account that doesn't exist
```

**Solução:**
1. Teste com Mainnet dry-run
2. Aguarde correção em versão futura

### ROUTE_NOT_FOUND

```
Raydium API error: ROUTE_NOT_FOUND
```

**Causa:** O token não possui pool de liquidez no Raydium

**Solução:**
1. Troque para um token com liquidez
2. Aguarde o token "graduar" para o Raydium
