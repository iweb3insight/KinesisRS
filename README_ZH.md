# Kinesis.rs v0.6.0

> "Kinesis: 将市场的耳语，转化为链上的动能。"

Kinesis.rs 是一个专为 LLM Agent 设计的无状态、JSON 优先、多链加密交易执行层。

## 安全架构 (Security Architecture)

Kinesis.rs 采用“零信任”原则管理私钥与链上交互。

### 1. 私钥保护
- **不上传服务器**: 私钥永远不会上传到任何服务器或云端。它们仅从本地环境变量加载，且仅在本地执行进程中使用。
- **执行上下文隔离**: 私钥明文永远不会离开本地二进制进程。解密与签名仅在受保护内存中进行，确保私钥绝不会泄露给 Agent Prompt 或外部日志记录。
- **本地加密标准**: 我们建议在本地持久化存储时使用 PBKDF2 + AES-256-GCM 对私钥进行加密。
- **自动锁定机制**: 执行上下文支持超时锁定，最大限度缩短私钥在内存中的解密暴露窗口。

### 2. 链上防御
- **交易时效保护**: 所有 BSC 侧的 `buy` 与 `sell` 操作均强制包含链上 Deadline，防止过时交易被恶意执行。
- **动态授权目标**: `approve` 逻辑直接使用经核实合约返回的 `approveTarget`，杜绝向恶意地址授权的风险。
- **公开透明**: 核心逻辑完全开源可审计，FreedomRouter 代理与实现合约已在 BscScan 完成验证。

### 3. 模拟优先
- **强制 Dry-run**: 系统默认开启 `--dry-run` 模式，必须显式指定 `--no-dry-run` 才能发送真实交易，有效规避逻辑或参数错误导致的资产损失。

## 特性
- **多链支持**: 原生支持 BNB 智能链 (BSC) 和 Solana。
- **Agent 优先**: JSON 优先的通信协议，与 LLM 无缝集成。
- **极致性能**: 支持多 RPC 竞速与交易预构造逻辑。
- **全平台支持**: 通过 GitHub Releases 提供针对 Linux (amd64)、macOS (Intel/M1) 及 Windows (amd64) 的预编译二进制包。- **智能路由**: 自动检测并执行 Solana 上的 Pump.fun 与 Raydium V3 路径（含 Token-2022 支持）。

## 快速开始
1. 克隆仓库。
2. 将 `.env.example` 复制为 `.env` 并填入私钥。
3. 编译: `cargo build --release`。
4. 运行: `./target/release/kinesis-rs balance --chain solana`。


## 赞助支持

Kinesis.rs 致力于推动 Agent 驱动的自动交易未来。为了支持我们在主网（Solana/BSC/ETH）持续进行功能验证与实战测试，欢迎通过以下地址进行赞助：

- **SOL**: `UFePGDrDS8xmutWkLKKGfgKUvacvLLSyQZ66AacKYUZ`
- **BNB**: `0x1580b9604c47Dbef3A61ae5a3deFF7f6611f3C28`
- **ETH**: `0x1580b9604c47Dbef3A61ae5a3deFF7f6611f3C28`

*所有赞助资金均将直接用于支付真实环境验证所需的 Gas 费用及流动性测试成本。*

## 常见问题处理 (macOS)

如果在 macOS 上运行二进制文件时出现 `[1] killed` 错误，这是由于系统安全隔离（Gatekeeper）导致的。请执行以下命令解除限制：

```bash
xattr -d com.apple.quarantine ~/Downloads/kinesis-rs
chmod +x ~/Downloads/kinesis-rs
./kinesis-rs --version
```
