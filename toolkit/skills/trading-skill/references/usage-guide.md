# KinesisRS 详细使用指南

## 1. 核心设计理念
KinesisRS 是一个 **Agent-First** 的交易系统。它的所有设计都是为了让 LLM Agent（如 Gemini, Claude）能够安全、精确地执行复杂交易。

- **无状态 (Stateless)**: 每一条命令都包含执行所需的所有上下文。
- **JSON 优先**: 推荐始终使用 `--json` 标志，以便 Agent 精确解析 `TradeResult`。
- **安全第一**: 默认开启 `--dry-run`，强制执行模拟验证。

## 2. 交互最佳实践 (对于 Agent)

### 买入流程
1. **询价**: 执行 `quote`。解析 `amount_out` 并向用户展示。
2. **风险评估**: 执行 `buy --dry-run`。
   - 检查 `stages` 中的耗时。
   - 检查 `gas_estimate`。
   - 如果成功，展示模拟结果并请求用户二次确认。
3. **真实成交**: 用户确认后，执行 `buy --no-dry-run`。

### 卖出流程
- 卖出命令会自动检测是否需要 `approve` (针对 BSC)。
- 如果 `stages` 中出现 `approve` 阶段，说明发生了授权操作。

## 3. Solana 特色功能使用

### Jito Bundle 加速
在 Solana 链上，为了防止抢跑（MEV）或在极端拥堵时成交，必须使用 Jito：
```bash
./solana_claw_coin_cli buy <TOKEN> 0.1 --chain solana --jito-tip 0.001
```
- **参数**: `--jito-tip` 单位为 SOL。建议范围：0.0001 - 0.01。

### Raydium 智能路由
对于非 Pump.fun 代币（如 USDC, SOL/USDT 池），执行器会自动调用 Raydium V3 Trade API 进行最优路径搜索。

## 4. 常见错误代码与处理建议

| 错误消息 | 原因 | 建议 |
| :--- | :--- | :--- |
| `AccountNotFound` | 钱包余额为 0 或未初始化 | 充值原生代币 (BNB/SOL) |
| `SlippageExceeded` | 价格波动过大 | 增加 `--slippage` (例如 25.0) |
| `RouteNotFound` | 代币流动性不足或 API 未索引 | 检查代币地址，或尝试小额买入 |
| `Simulation failed` | 逻辑执行 Revert | 检查 `raw_revert_data` 获取详细原因 |

## 5. 性能审计
通过解析 `TradeResult` 中的 `stages` 数组，Agent 可以计算出：
- **API 延迟**: `quote` 阶段的 `duration_ms`。
- **执行延迟**: `buy`/`sell` 阶段的 `duration_ms`。
- **总耗时**: 所有阶段 `duration_ms` 的总和。
