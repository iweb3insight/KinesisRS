# Referência de Códigos de Erro

## Tipos de Erro

| Tipo de Erro        | Descrição                         | Causas Comuns                  |
|---------------------|-----------------------------------|--------------------------------|
| `rpc_error`         | Falha na requisição RPC           | Problemas de rede, RPC indisponível |
| `simulation_failed` | Falha na simulação da transação   | Lógica de revert, saldo insuficiente |
| `send_failed`       | Falha no envio da transação       | Problemas de assinatura, congestão da rede |
| `config_error`      | Erro de configuração              | Chave privada ausente, RPC não configurado |
| `invalid_input`     | Parâmetros de entrada inválidos   | Endereço inválido, valor negativo |
| `contract_error`    | Erro de execução do contrato      | Revert, problemas de permissão |

---

## Erros Comuns & Soluções

### 1. AccountNotFound

**Mensagem de Erro:**
```json
{"error": {"type": "contract_error", "message": "Simulation failed: \"AccountNotFound\""}}
```

**Causa:** O saldo da carteira é 0.

**Solução:**
```bash
# Deposite SOL na carteira
# Endereço da carteira: 88DqDXNAQZHWscK5HjPavDkBCvsfzmUrDvAV9ZTY5jMv
```

---

### 2. Erro de ATA

**Mensagem de Erro:**
```
RPC Error -32602: invalid transaction: Transaction loads an address table account that doesn't exist
```

**Causa:** A Address Lookup Table (LUT) criada pelo Raydium não existe na Devnet/Testnet.

**Redes Afetadas:** Devnet, Testnet

**Solução:**
```bash
# Opção 1: Use a simulação (dry-run) na Mainnet
export SOL_RPC_URL="https://api.mainnet-beta.solana.com"

# Opção 2: Use o caminho Pump.fun (se disponível)
# Pump.fun não utiliza LUT
```

---

### 3. ROUTE_NOT_FOUND

**Mensagem de Erro:**
```json
{"error": {"message": "ROUTE_NOT_FOUND"}}
```

**Causa:** A API do Raydium ainda não indexou o pool de liquidez para este token.

**Cenários Comuns:**
- Tokens Pump.fun recém-criados
- Tokens que ainda não "graduaram"
- Acabaram de graduar, mas ainda não foram sincronizados

**Solução:**
```bash
# Opção 1: Troque para um token com liquidez
# Ex: BONK, USDC, SOL

# Opção 2: Aguarde o token graduar para o Raydium

# Opção 3: Negocie diretamente no Pump.fun (se não graduado)
```

---

### 4. REQ_SWAP_RESPONSE_ERROR

**Mensagem de Erro:**
```json
{"error": {"message": "Failed to parse Raydium transaction: ... REQ_SWAP_RESPONSE_ERROR"}}
```

**Causa:** A API do Raydium retornou uma resposta inválida.

**Cenários Comuns:**
- Chamada secundária após falha na API de Cotação (Quote)
- Problemas temporários na API

**Solução:**
```bash
# Tente novamente
kinesis-rs buy <TOKEN> <AMOUNT> --dry-run

# Ou aguarde e tente novamente
```

---

### 5. REQ_COMPUTE_UNIT_PRICE_MICRO_LAMPORTS_ERROR

**Mensagem de Erro:**
```json
{"error": {"message": "REQ_COMPUTE_UNIT_PRICE_MICRO_LAMPORTS_ERROR"}}
```

**Causa:** Problema com a configuração do Compute Unit Price.

**Solução:**
```bash
# Aguarde a recuperação da API ou tente novamente
```

---

### 6. SlippageExceeded

**Mensagem de Erro:**
```json
{"error": {"revert_reason": "SlippageExceeded"}}
```

**Causa:** A volatilidade do preço excedeu o slippage configurado.

**Solução:**
```bash
# Aumente o slippage
kinesis-rs buy <TOKEN> <AMOUNT> --slippage 25 --chain solana

# Ou diminua o valor
kinesis-rs buy <TOKEN> <AMOUNT> --slippage 15 --chain solana
```

---

### 7. Insufficient Liquidity

**Mensagem de Erro:**
```json
{"error": {"revert_reason": "FreedomRouter: INSUFFICIENT_LIQUIDITY"}}
```

**Causa:** Liquidez insuficiente no pool.

**Solução:**
```bash
# Diminua o valor de compra
kinesis-rs buy <TOKEN> 0.01 --chain solana

# Aguarde a recuperação da liquidez
```

---

### 8. Insufficient Gas / Insufficient Funds

**Mensagem de Erro:**
```json
{"error": {"revert_reason": "insufficient funds for gas * price + value"}}
```

**Causa:** Saldo insuficiente para pagar as taxas de gás.

**Solução:**
```bash
# Deposite o token nativo (SOL/BNB)
```

---

### 9. Token account not found

**Mensagem de Erro:**
```json
{"error": {"message": "Token account not found: <TOKEN_ADDRESS>"}}
```

**Causa:** A carteira não possui este token.

**Solução:**
```bash
# Compre o token primeiro para criar a ATA
# Ou verifique se o endereço do token está correto
```

---

### 10. Endereço de Token Inválido

**Mensagem de Erro:**
```json
{"error": {"message": "Invalid token address"}}
```

**Causa:** Erro no formato do endereço do token.

**Solução:**
```bash
# Verifique o formato do endereço
# Solana: Codificado em Base58, 32-44 caracteres
# BSC: Começa com 0x, 40 caracteres hexadecimais
```

---

## Fluxo de Manuseio de Erros

```
Requisição do Usuário
    ↓
Executar Comando
    ↓
┌─────────────────────────────────────┐
│  Sucesso?                            │
│  ↓ Sim                               │
│  Retornar resposta de sucesso        │
│  ↓ Não                               │
│  Analisar tipo de erro               │
│  ↓                                   │
│  ┌─────────────────────────────────┐│
│  │ rpc_error                       ││
│  │  - Verificar conexão de rede    ││
│  │  - Alterar RPC                  ││
│  ├─────────────────────────────────┤│
│  │ simulation_failed                ││
│  │  - Verificar saldo              ││
│  │  - Verificar aprovação          ││
│  │  - Ajustar slippage             ││
│  ├─────────────────────────────────┤│
│  │ contract_error                  ││
│  │  - Analisar revert_reason       ││
│  │  - Consultar soluções específic.││
└─────────────────────────────────────┘
    ↓
Retornar resposta de erro
    ↓
Exibir erro + sugestões ao usuário
```

---

## Dicas de Depuração

### 1. Habilitar Logs de Debug

```bash
RUST_LOG=debug kinesis-rs --json quote ...
```

### 2. Verificar Conexão de Rede

```bash
# Testar RPC diretamente
curl -X POST https://api.mainnet-beta.solana.com -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}'

# Testar API do Raydium
curl "https://transaction-v1.raydium.io/compute/swap-base-in?inputMint=So11111111111111111111111111111111111111112&outputMint=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v&amount=1000000000&slippageBps=50"
```

### 3. Verificar Saldo

```bash
kinesis-rs --json balance --chain solana
```
