# Claude CLI 适配器

## 环境要求

- Claude CLI 已安装
- 网络访问 GitHub 下载二进制

## 安装

### 方式 1: 使用 Bash 工具

```bash
# 下载并安装
Bash: curl -sL https://github.com/iweb3insight/KinesisRS/releases/download/v0.6.5/kinesis-rs-vv0.6.5-macos-arm64.tar.gz -o /tmp/kinesis.tar.gz
Bash: tar -xzf /tmp/kinesis.tar.gz -C /tmp/
Bash: chmod +x /tmp/kinesis-rs && mkdir -p ~/.kinesis && mv /tmp/kinesis-rs ~/.kinesis/
```

### 方式 2: 使用 MCP (如果有)

```
/mcp install kinesis-agent
```

## 使用方法

### 1. 使用 Bash 工具

```bash
# Quote
Bash: ~/.kinesis/kinesis-rs --json quote DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --action buy --chain solana
env: {SOL_RPC_URL: "https://api.mainnet-beta.solana.com"}

# Balance
Bash: ~/.kinesis/kinesis-rs --json balance --chain solana
env: {SOL_RPC_URL: "https://api.mainnet-beta.solana.com"}

# Buy (dry-run)
Bash: ~/.kinesis/kinesis-rs --json buy DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --slippage 15 --chain solana --dry-run
env: {SOL_RPC_URL: "https://api.mainnet-beta.solana.com"}
```

### 2. 使用 MCP 工具 (如果有)

```
# 方式 1: 使用工具调用
kinesis_quote token=DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 amount=0.001

kinesis_buy token=DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 amount=0.001 slippage=15 dry_run=true

# 方式 2: 使用自然语言
"查询 BONK 的报价"
"买入 0.001 SOL 的 BONK"
```

### 3. Agent 模式

```python
# 在 Claude 中使用 Python 工具
class KinesisTrading:
    def __init__(self):
        self.binary = "~/.kinesis/kinesis-rs"
        self.rpc = "https://api.mainnet-beta.solana.com"
    
    def quote(self, token: str, amount: float) -> dict:
        cmd = f"{self.binary} --json quote {token} {amount} --action buy --chain solana"
        result = Bash(cmd, env={"SOL_RPC_URL": self.rpc})
        return json.loads(result.stdout)
    
    def buy(self, token: str, amount: float, slippage: int = 15, dry_run: bool = True):
        flag = "--dry-run" if dry_run else "--no-dry-run"
        cmd = f"{self.binary} --json buy {token} {amount} --slippage {slippage} {flag} --chain solana"
        result = Bash(cmd, env={"SOL_RPC_URL": self.rpc})
        return json.loads(result.stdout)
    
    def balance(self, chain: str = "solana") -> dict:
        cmd = f"{self.binary} --json balance --chain {chain}"
        result = Bash(cmd, env={"SOL_RPC_URL": self.rpc})
        return json.loads(result.stdout)
```

### 4. 完整对话示例

```
用户: 我想买入 0.001 SOL 的 BONK

Assistant:
  我来帮你执行买入流程。首先获取报价：
  
  Bash: ~/.kinesis/kinesis-rs --json quote DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --action buy --chain solana
  env: {SOL_RPC_URL: "https://api.mainnet-beta.solana.com"}
  
  → {"amount_out": "1450172355", "success": true}
  
  报价: 0.001 SOL ≈ 1,450,172,355 BONK
  
  现在执行 dry-run 模拟交易：
  
  Bash: ~/.kinesis/kinesis-rs --json buy DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --slippage 15 --chain solana --dry-run
  env: {SOL_RPC_URL: "https://api.mainnet-beta.solana.com"}
  
  → {"success": false, "error": {"message": "AccountNotFound"}}
  
  抱歉，模拟交易失败。错误: AccountNotFound
  
  这是因为测试钱包在主网上没有 SOL 余额。要完成真实交易，需要充值 SOL。
  
  请问要继续吗？
```

---

## 网络切换

```bash
# Devnet
Bash: ~/.kinesis/kinesis-rs --json balance --chain solana
env: {SOL_RPC_URL: "https://api.devnet.solana.com"}

# Testnet
Bash: ~/.kinesis/kinesis-rs --json balance --chain solana
env: {SOL_RPC_URL: "https://api.testnet.solana.com"}

# Mainnet
Bash: ~/.kinesis/kinesis-rs --json balance --chain solana
env: {SOL_RPC_URL: "https://api.mainnet-beta.solana.com"}
```

---

## 常见问题

### Q: 如何在 Claude 中处理环境变量?

```bash
# 方式 1: env 参数
Bash: echo $HOME
env: {MY_VAR: "value"}

# 方式 2: 命令中设置
Bash: SOL_RPC_URL=https://api.mainnet-beta.solana.com ~/.kinesis/kinesis-rs --json quote ...
```

### Q: 如何调试?

```bash
Bash: RUST_LOG=debug ~/.kinesis/kinesis-rs --json quote ... --chain solana
env: {SOL_RPC_URL: "https://api.mainnet-beta.solana.com"}
```

### Q: Claude 支持 MCP 吗?

如果 Claude 支持 MCP，可以注册工具：
```json
{
  "name": "kinesis",
  "commands": {
    "quote": "~/.kinesis/kinesis-rs --json quote $token $amount --action buy --chain solana",
    "buy": "~/.kinesis/kinesis-rs --json buy $token $amount --slippage $slippage --dry-run --chain solana"
  }
}
```

---

## 与 opencode 的差异

| 特性 | Claude | opencode |
|------|--------|----------|
| 执行方式 | Bash/MCP | Shell |
| 环境变量 | env 参数 | export |
| JSON 解析 | 自动 | 需手动 |
| 工具注册 | MCP | alias |
