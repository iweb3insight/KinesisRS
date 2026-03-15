---
name: kinesis-agent
description: Agente de execução de negociação multiplataforma unificado suportando opencode/Gemini/Claude/OpenClaw. Baixa automaticamente o binário, executa negociações Solana/BSC.
binary:
  latest: https://github.com/iweb3insight/KinesisRS/releases/latest
  version: v0.6.5
  install: |
    # Instalação automática
    curl -sL https://raw.githubusercontent.com/iweb3insight/KinesisRS/main/scripts/install.sh | bash
    
    # Ou download manual
    # https://github.com/iweb3insight/KinesisRS/releases/latest
platforms:
  - darwin-arm64
  - darwin-amd64
  - linux-amd64
  - windows-amd64
---

# Agente Kinesis - Execução de Negociação Multiplataforma

## Instalação

### Instalação Automática

```bash
curl -sL https://raw.githubusercontent.com/iweb3insight/KinesisRS/main/scripts/install.sh | bash
```

### Instalação Manual

```bash
# macOS ARM64
curl -sL https://github.com/iweb3insight/KinesisRS/releases/download/v0.6.5/kinesis-rs-vv0.6.5-macos-arm64.tar.gz -o /tmp/kinesis.tar.gz
tar -xzf /tmp/kinesis.tar.gz -C /tmp/
chmod +x /tmp/kinesis-rs && mv /tmp/kinesis-rs ~/.kinesis/

# Linux
curl -sL https://github.com/iweb3insight/KinesisRS/releases/download/v0.6.5/kinesis-rs-vv0.6.5-linux-amd64.tar.gz -o /tmp/kinesis.tar.gz

# Windows
# Baixe o .zip e extraia
```

### Verificar Instalação

```bash
~/.kinesis/kinesis-rs --version
~/.kinesis/kinesis-rs wallet
```

## Configurar Variáveis de Ambiente

```bash
# Rede Solana
export SOL_RPC_URL="https://api.mainnet-beta.solana.com"  # Mainnet
# export SOL_RPC_URL="https://api.devnet.solana.com"       # Devnet
# export SOL_RPC_URL="https://api.testnet.solana.com"      # Testnet

# Rede BSC
export BSC_RPC_URL="https://bsc-dataseed.binance.org/"

# Opcional: Proxy
export HTTPS_PROXY="http://proxy:8080"
```

## Executar Negociações (Verificação em Três Etapas)

```
1. Cotação (Quote) → Obter cotação
2. Simulação (Dry-run) → Simular negociação
3. Execução → Negociação real
```

---

## Fluxos de Trabalho Principais

### Fluxo de Compra (Buy)

```
Passo 1: quote <TOKEN> <AMOUNT>
        ↓
Passo 2: buy <TOKEN> <AMOUNT> --dry-run
        ↓
Passo 3: buy <TOKEN> <AMOUNT> --no-dry-run (Usuário confirma)
```

### Fluxo de Venda (Sell)

```
Passo 1: sell <TOKEN> <AMOUNT> --dry-run
        ↓
Passo 2: sell <TOKEN> <AMOUNT> --no-dry-run
```

---

## Adaptadores de Plataforma

| Plataforma | Método de Chamada | Referência |
|------------|-------------------|------------|
| opencode   | Shell             | adapters/opencode.md |
| openclaw   | Shell             | adapters/openclaw.md |
| Gemini     | mcp__local__execute | adapters/gemini.md |
| Claude     | MCP/Bash          | adapters/claude.md |

---

## Redes Suportadas

| Rede    | SOL_RPC_URL | Saldo | Status de Compra |
|---------|-------------|-------|------------------|
| Mainnet | api.mainnet-beta.solana.com | Necessário | ✅ |
| Devnet  | api.devnet.solana.com | 3.67 SOL | ⚠️ ATA |
| Testnet | api.testnet.solana.com | 3.00 SOL | ⚠️ ATA |

Veja: references/network-matrix.md

---

## Manuseio de Erros

| Erro | Causa | Solução |
|-------|-------|----------|
| AccountNotFound | Saldo 0 | Depositar SOL |
| Erro ATA | Sem LUT na Devnet | Usar caminho Pump.fun |
| ROUTE_NOT_FOUND | Token não indexado | Aguardar ou trocar token |

Veja: references/error-codes.md

---

## Materiais de Referência

- [API de Negociação](references/trading-api.md)
- [Matriz de Rede](references/network-matrix.md)
- [Códigos de Erro](references/error-codes.md)
