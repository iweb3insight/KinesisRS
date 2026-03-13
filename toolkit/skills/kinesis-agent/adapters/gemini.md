# Gemini CLI 适配器

## 环境要求

- Gemini CLI 已安装
- 网络访问 GitHub 下载二进制

## 安装

### 方式 1: 自动安装

```bash
# 在 Gemini CLI 中执行
mcp__local__execute: curl -sL https://raw.githubusercontent.com/iweb3insight/KinesisRS/main/scripts/install.sh | bash
```

### 方式 2: 手动配置

```bash
# 1. 下载二进制
curl -sL https://github.com/iweb3insight/KinesisRS/releases/download/v0.6.5/kinesis-rs-vv0.6.5-macos-arm64.tar.gz -o /tmp/kinesis.tar.gz

# 2. 解压并移动
tar -xzf /tmp/kinesis.tar.gz -C /tmp/
chmod +x /tmp/kinesis-rs
mv /tmp/kinesis-rs ~/.kinesis/

# 3. 设置权限
chmod +x ~/.kinesis/kinesis-rs
```

## 配置 MCP 工具

### 创建自定义工具

在 Gemini CLI 中注册 Kinesis 工具：

```json
{
  "name": "kinesis_quote",
  "description": "获取 Solana 代币报价",
  "command": "~/.kinesis/kinesis-rs",
  "args": ["--json", "quote", "$TOKEN", "$AMOUNT", "--action", "buy", "--chain", "solana"],
  "env": {
    "SOL_RPC_URL": "https://api.mainnet-beta.solana.com"
  }
}
```

### 工具列表

| 工具名 | 功能 | 参数 |
|--------|------|------|
| kinesis_quote | 获取报价 | token, amount |
| kinesis_balance | 查询余额 | chain |
| kinesis_buy | 买入代币 | token, amount, slippage, dry_run |
| kinesis_sell | 卖出代币 | token, amount, slippage, dry_run |
| kinesis_wallet | 查询钱包 | chain |

## 使用方法

### 1. 基本调用

```
# Quote
mcp__local__execute: ~/.kinesis/kinesis-rs --json quote DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --action buy --chain solana

# Balance
mcp__local__execute: ~/.kinesis/kinesis-rs --json balance --chain solana

# Buy (dry-run)
mcp__local__execute: ~/.kinesis/kinesis-rs --json buy DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --slippage 15 --chain solana --dry-run
```

### 2. Agent 模式

```python
class KinesisAgent:
    def __init__(self, binary_path="~/.kinesis/kinesis-rs"):
        self.binary = binary_path
        self.rpc_url = "https://api.mainnet-beta.solana.com"
    
    def quote(self, token: str, amount: float) -> dict:
        cmd = f"{self.binary} --json quote {token} {amount} --action buy --chain solana"
        return self.execute(cmd)
    
    def buy(self, token: str, amount: float, slippage: int = 15, dry_run: bool = True):
        dry_flag = "--dry-run" if dry_run else "--no-dry-run"
        cmd = f"{self.binary} --json buy {token} {amount} --slippage {slippage} --chain solana {dry_flag}"
        return self.execute(cmd)
    
    def execute(self, cmd: str) -> dict:
        # 通过 mcp__local__execute 执行
        result = mcp__local__execute(cmd, env={"SOL_RPC_URL": self.rpc_url})
        return json.loads(result)
```

### 3. 完整示例

```
用户: 买入 0.001 SOL 的 BONK

Agent:
  1. Quote
     mcp__local__execute: ~/.kinesis/kinesis-rs --json quote DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --action buy --chain solana
     → {"amount_out": "1450172355", "success": true}
  
  2. Dry-run
     mcp__local__execute: ~/.kinesis/kinesis-rs --json buy DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 0.001 --slippage 15 --chain solana --dry-run
     → {"success": true, "tx_hash": "..."}
  
  3. 展示结果给用户确认
```

---

## 网络切换

```
# Devnet
mcp__local__execute: SOL_RPC_URL=https://api.devnet.solana.com ~/.kinesis/kinesis-rs --json balance --chain solana

# Testnet
mcp__local__execute: SOL_RPC_URL=https://api.testnet.solana.com ~/.kinesis/kinesis-rs --json balance --chain solana

# Mainnet
mcp__local__execute: SOL_RPC_URL=https://api.mainnet-beta.solana.com ~/.kinesis/kinesis-rs --json balance --chain solana
```

---

## 常见问题

### Q: 如何处理环境变量?

```python
# 方式 1: 在命令中设置
cmd = f"SOL_RPC_URL=https://api.mainnet-beta.solana.com {self.binary} ..."

# 方式 2: 通过 env 参数
mcp__local__execute(cmd, env={"SOL_RPC_URL": "https://api.mainnet-beta.solana.com"})
```

### Q: 如何调试?

```bash
# 开启调试
mcp__local__execute: RUST_LOG=debug ~/.kinesis/kinesis-rs --json quote ...
```

### Q: 二进制自动下载?

目前需要手动下载。未来计划支持:
```bash
mcp__local__execute: curl -sL https://github.com/iweb3insight/KinesisRS/releases/latest/download/kinesis-rs | install.sh
```
