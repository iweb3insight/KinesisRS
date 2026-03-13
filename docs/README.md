# Kinesis.rs Rust v1.0

A stateless, JSON-first, multi-chain crypto trading execution layer for LLM Agents.

## Features
- **Stateless & Agent-First**: Communicates via standard JSON `TradeResult` protocol.
- **True Dry-run**: Real `eth_call` and `estimate_gas` simulation for all trades.
- **Robust Security**: Private keys loaded exclusively from environment variables.
- **Detailed Observability**: Staged logging with duration metrics and revert reason decoding.
- **Automatic Approval**: Intelligent `approveIfNeeded` logic for sell transactions.

## Installation
```bash
cargo build --release
```

## Quick Start
```bash
# Get a quote
./target/release/solana_claw_coin_cli quote <TOKEN_ADDRESS> 0.1 --chain bsc

# Buy with JSON output (Dry-run by default)
./target/release/solana_claw_coin_cli --json buy <TOKEN_ADDRESS> 0.1 --slippage 5

# Check balance
./target/release/solana_claw_coin_cli balance --token-address <TOKEN_ADDRESS>
```

## Progress
- **BSC**: Functional (Quote, Balance, Approve, Simulation, Real Send).
- **Solana**: Path Detector pending implementation.
- **Tests**: 25/68 passed.

## Architecture
See `PROGRESS.md` for detailed milestone tracking.
