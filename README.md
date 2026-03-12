# Kinesis.rs v0.6.0

> "Kinesis: Converting market whispers into on-chain momentum."

Kinesis.rs is a stateless, JSON-first, multi-chain crypto trading execution layer designed primarily for LLM Agents.

## Security Architecture

Kinesis.rs is built with a "Zero-Trust" approach to private key management and on-chain interaction.

### 1. Private Key Protection
- **No Server-Side Storage**: Private keys are never uploaded to any server or cloud. They are loaded exclusively from local environment variables and used only within the local execution process.
- **Execution Context Isolation**: Plaintext keys never exit the local binary process. Decryption and signing occur only within protected memory during the execution phase, ensuring no leak to the Agent Prompt or external logs.
- **Local Encryption Standard**: We advocate for PBKDF2 + AES-256-GCM encryption for key persistence in local environments.
- **Auto-Lock Mechanism**: Execution contexts support timeouts to minimize the window of decrypted key exposure in memory.

### 2. On-Chain Safeguards
- **Transaction Deadlines**: All BSC `buy` and `sell` operations include mandatory on-chain deadlines to prevent "stuck" transactions from being executed at stale prices.
- **Dynamic Approval Targeting**: The `approve` logic uses `approveTarget` returned directly by verified contracts, eliminating the risk of granting allowance to malicious or incorrect addresses.

### 3. Simulation First
- **Mandatory Dry-runs**: The system defaults to `--dry-run` mode, requiring explicit opt-out via `--no-dry-run`. This prevents unintended asset exposure due to logic or parameter errors.

## Features
- **Multi-Chain Support**: Native execution for BNB Smart Chain (BSC) and Solana.
- **Agent-First Design**: JSON-first communication protocol for seamless LLM integration.
- **High Performance**: Parallel RPC racing and transaction pre-construction.
- **Multi-Platform Support**: Pre-compiled binaries available for Linux (amd64), macOS (Intel/M1), and Windows (amd64) via GitHub Releases.- **Solana Pathing**: Automatic detection and execution for Pump.fun and Raydium V3 (inc. Token-2022 support).

## Getting Started
1. Clone the repo.
2. Copy `.env.example` to `.env` and add your keys.
3. Build: `cargo build --release`.
4. Run: `./target/release/kinesis-rs balance --chain solana`.


## Support the Mission

Kinesis.rs is built for the agentic future. To support the ongoing testing and validation of features on Mainnet (Solana/BSC/ETH), consider donating to the following addresses:

- **SOL**: `UFePGDrDS8xmutWkLKKGfgKUvacvLLSyQZ66AacKYUZ`
- **BNB**: `0x1580b9604c47Dbef3A61ae5a3deFF7f6611f3C28`
- **ETH**: `0x1580b9604c47Dbef3A61ae5a3deFF7f6611f3C28`

*All donations are used to cover network fees and liquidity costs for real-world execution testing.*

## Troubleshooting (macOS)

If you see `[1] killed` when running the binary on macOS, it is due to Gatekeeper quarantine. Run the following commands:

```bash
xattr -d com.apple.quarantine ~/Downloads/kinesis-rs
chmod +x ~/Downloads/kinesis-rs
./kinesis-rs --version
```
