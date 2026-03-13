# Kinesis.rs Rust v1.0

Kinesis.rs 是一个专为 LLM Agent 设计的无状态、JSON 优先、多链加密交易执行层。

## 特性
- **多链支持**: 支持 BNB 智能链 (BSC) 和 Solana 的原生执行。
- **Agent 优先设计**: 采用 JSON 优先的通信协议，实现与 LLM 的无缝集成。
- **高性能**: 支持多 RPC 竞速和交易预构造。
- **Solana 智能路由**: 自动检测并执行 Pump.fun 和 Raydium V3 路径。
- **安全保障**: 强制执行 dry-run 模拟和安全的私钥管理。

## 快速开始
1. 克隆仓库。
2. 将 `.env.example` 复制为 `.env` 并添加您的私钥。
3. 编译: `cargo build --release`。
4. 运行: `./target/release/solana_claw_coin_cli balance --chain solana`。

## Skills
查看 `skills/` 目录，了解如何与 Gemini CLI 和其他 Agent 框架集成。
