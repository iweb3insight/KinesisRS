# Kinesis.rs v0.6.0

> "Kinesis: Converting market whispers into on-chain momentum."

Kinesis.rs is a stateless, JSON-first, Agent-native trading execution layer designed primarily for LLM Agents—built from the ground up to be perfectly **Agentic Friendly**.

<div align="center">
  <img src="docs/kinesis-rs.png" width="640" />
</div>

## Security First Commitment

At Kinesis.rs, we recognize that our users entrust us with the execution of their financial strategies. **Data security and private key protection are our highest priorities.** Every feature is designed with a "Security-by-Default" mindset to ensure that your sensitive credentials never leave your local environment.

## Security Architecture

Kinesis.rs is built with a "Zero-Trust" approach to private key management and on-chain interaction.

### 1. Private Key Protection
- **No Server-Side Storage**: Private keys are never uploaded to any server or cloud. They are loaded exclusively from local environment variables or secure storage and used only within the local execution process.
- **Execution Context Isolation**: Plaintext keys never exit the local binary process. Decryption and signing occur only within protected memory during the execution phase, ensuring no leak to the Agent Prompt or external logs.
- **Local Encryption Standard**: We advocate for PBKDF2 + AES-256-GCM encryption for key persistence in local environments.
- **Auto-Lock Mechanism**: Execution contexts support timeouts to minimize the window of decrypted key exposure in memory.

### 2. On-Chain Safeguards
- **Transaction Deadlines**: All BSC `buy` and `sell` operations include mandatory on-chain deadlines to prevent "stuck" transactions from being executed at stale prices.
- **Dynamic Approval Targeting**: The `approve` logic uses `approveTarget` returned directly by verified contracts, eliminating the risk of granting allowance to malicious or incorrect addresses.
- **Transparency & Audibility**:
    - **Open Source Core**: Every line of the execution logic is available for public audit.
    - **Verified Contracts**: FreedomRouter Proxy and Implementation are fully verified on BscScan for public scrutiny.

### 3. Simulation First
- **Mandatory Dry-runs**: The system defaults to `--dry-run` mode, requiring explicit opt-out via `--no-dry-run`. This prevents unintended asset exposure due to logic or parameter errors.

## Features
- **Multi-Chain Support**: Native execution for BNB Smart Chain (BSC) and Solana.
- **Agent-First Design**: JSON-first communication protocol for seamless LLM integration.
- **High Performance**: Parallel RPC racing and transaction pre-construction.
- **Multi-Platform Support**: Pre-compiled binaries available for Linux (amd64), macOS (Intel/M1), and Windows (amd64) via GitHub Releases.
- **Solana Pathing**: Automatic detection and execution for Pump.fun and Raydium V3 (inc. Token-2022 support).

## Getting Started
1. Clone the repo.
2. Copy `.env.example` to `.env` and add your keys.
3. Build: `cargo build --release`.
4. Run: `./target/release/kinesis-rs balance --chain solana`.

## CLI Usage

```text
Usage: kinesis-rs [OPTIONS] <COMMAND>

Commands:
  buy      Buy a token on a supported chain
  sell     Sell a token on a supported chain
  quote    Get a quote for a trade
  balance  Check balance of native or token
  approve  Approve a token for trading
  config   Display current configuration
  wallet   Display wallet address
  detect   Detect Solana token path (Pump.fun or Raydium)
  help     Print this message or the help of the given subcommand(s)

Options:
      --json             Global flag to force JSON output for agent consumption
      --dry-run          Global flag to simulate transactions without sending them
      --no-dry-run       Global flag to disable simulation and send the real transaction
      --wallet <WALLET>  Selects the wallet to use based on environment variable suffix (e.g., _1, _2) [default: 1]
  -h, --help             Print help (see more with '--help')
  -V, --version          Print version
```

## Donation

Kinesis.rs is built for the agentic future. To support the ongoing testing and validation of features on Mainnet (Solana/BSC/ETH), consider donating to the following addresses:

- **SOL**: `UFePGDrDS8xmutWkLKKGfgKUvacvLLSyQZ66AacKYUZ`
- **BNB**: `0x1580b9604c47Dbef3A61ae5a3deFF7f6611f3C28`
- **ETH**: `0x1580b9604c47Dbef3A61ae5a3deFF7f6611f3C28`

*All donations are used to cover network fees and liquidity costs for real-world execution testing.*

## Disclaimer

Kinesis.rs is a tool for **technical research and educational purposes only**.

- This project does **NOT** constitute financial, investment, or legal advice.
- Cryptocurrency trading involves significant risk of loss. The developers are not responsible for any financial losses, bugs, or security incidents incurred through the use of this software.
- **Use at your own risk.** Always perform your own research (DYOR) before executing any on-chain transactions.

## Project Roadmap
See [ROADMAP.md](docs/ROADMAP.md) for planned features and milestones.
