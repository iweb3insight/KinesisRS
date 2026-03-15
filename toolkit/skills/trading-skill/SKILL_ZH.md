---
name: kinesis-trading-skill
description: 适用于 BSC 和 Solana 的多链交易执行。用于在 PancakeSwap、Pump.fun 和 Raydium 上买卖代币、获取报价和查询余额。支持 Jito bundle 提交和多 RPC 竞速。
---

# 交易执行技能 (KinesisRS)

此技能使 Gemini CLI 能够充当高性能加密货币交易代理。

## 核心工作流

### 1. 买入工作流 (安全执行)
1. **获取报价 (Quote)**：执行 `quote` 以获取实时价格。
2. **模拟 (Simulate)**：使用 `--dry-run` 运行以验证逻辑和 Gas 费。
3. **执行 (Execute)**：与用户确认后，使用 `--no-dry-run` 运行。

### 2. 卖出工作流 (自动授权)
1. **模拟 (Simulate)**：BSC 自动处理 `approveIfNeeded`。
2. **执行 (Execute)**：使用 `--no-dry-run` 运行。

## 参考资料

- **[QUICK_START.md](references/trading-api.md)**: CLI 命令映射和 JSON 示例。
- **[USAGE_GUIDE.md](references/usage-guide.md)**: 代理交互模式和故障排除的深入探讨。
- **[SETUP.md](references/setup.md)**: 环境变量和构建指令。

## 验证
运行 `./kinesis-trading-skill/scripts/check_config.cjs` 以验证您的环境。
