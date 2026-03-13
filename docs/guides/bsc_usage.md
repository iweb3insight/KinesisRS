# FreedomAgent BSC 执行层使用指南

本指南详细介绍了 FreedomAgent Rust v1.0 在 BSC（币安智能链）上的功能、配置及调用方式。该工具作为“交易原语”，专为 LLM Agent 和自动化流程设计。

---

## 1. 环境准备 (Setup)

在使用之前，请确保已配置以下环境变量（或创建 `.env` 文件）：

| 变量名 | 必填 | 说明 | 示例 |
| :--- | :--- | :--- | :--- |
| `BSC_RPC_URL` | 是 | BSC 节点的 HTTP RPC 地址 | `https://bsc-dataseed.binance.org/` |
| `BSC_PRIVATE_KEY_1` | 是 | 1号钱包私钥（默认） | `0x...` |
| `BSC_PRIVATE_KEY_2` | 否 | 2号钱包私钥 | `0x...` |

> **安全红线**：私钥仅在内存中临时用于签名，**绝不会**出现在任何日志、JSON 输出或磁盘持久化文件中。

---

## 2. 全局参数 (Global Flags)

这些标志可附加在任何命令前，用于控制输出和执行模式：

- `--json`: **(Agent 必备)** 强制以结构化 JSON 格式输出结果。
- `--dry-run`: (默认开启) 仅进行链上模拟 (`eth_call`) 和 Gas 预估，不发送真实交易。
- `--no-dry-run`: 关闭模拟模式，直接进行真实签名并广播交易。
- `--wallet <INDEX>`: 指定使用的钱包索引（1, 2, ...），对应环境变量后缀。

---

## 3. 核心命令 (Commands)

### 3.1 买入代币 (Buy)
使用 BNB 一键买入目标代币。系统会自动选择最优路径（Pancake/Four.meme/Flap）。

```bash
# 模拟：用 0.1 BNB 买入，滑点 15%
./freedomagent --json buy <TOKEN_ADDRESS> 0.1 --slippage 15

# 真实交易：
./freedomagent --json --no-dry-run buy <TOKEN_ADDRESS> 0.1
```

### 3.2 卖出代币 (Sell)
将代币卖回 BNB。**内置自动授权 (ApproveIfNeeded) 逻辑**。

```bash
# 模拟：卖出 1000 个代币。若未授权，输出将自动包含模拟 approve 阶段。
./freedomagent --json sell <TOKEN_ADDRESS> 1000
```

### 3.3 报价查询 (Quote)
纯只读查询，不涉及模拟或 gas 预估。

```bash
# 查询 0.1 BNB 预期获得的代币数量
./freedomagent quote <TOKEN_ADDRESS> 0.1 --action buy
```

### 3.4 资产查询 (Balance)
查询钱包余额。

```bash
# 查询原生 BNB 余额
./freedomagent balance

# 查询指定代币余额
./freedomagent balance --token-address <TOKEN_ADDRESS>
```

---

## 4. 深度诊断与可观测性

### 4.1 TradeResult JSON 结构
所有命令（在 `--json` 模式下）均返回一致的结构，便于 Agent 解析。

```json
{
  "success": true,
  "chain": "bsc",
  "stages": [...],
  "amount_out": "25000000000000000000",
  "gas_estimate": 125000,
  "tx_hash": "0x...", // 仅在真实交易成功后出现
  "revert_reason": null
}
```

### 4.2 错误分类
系统通过 `TradeError` 提供了精细化的错误反馈：

- **`simulation_failed`**: 合约层逻辑错误。
  - 会自动解析 Revert Reason (如 `SLIPPAGE_EXCEEDED`, `INSUFFICIENT_LIQUIDITY`)。
- **`rpc_error`**: 基础设施错误。
  - 自动识别：`Network timeout` (超时), `Rate limit exceeded` (429), `Nonce/Gas conflict` (Nonce 冲突)。
- **`config_error` / `invalid_input`**: 配置或参数错误。

---

## 5. 极致稳固特性说明

1. **Gas 聚合统计**：在 `sell` 流程中，如果触发了 `approve`，`gas_estimate` 会自动累加授权和交换两步的预期消耗。
2. **Receipt 确认等待**：在 `--no-dry-run` 模式下，程序会同步等待 Receipt，返回真实的 `gas_used`。
3. **静默日志**：使用 `--json` 时，所有人类可读的日志（Tracing）会自动重定向，不干扰 JSON 解析。

---

## 6. 调用建议

1. **组合调用**：建议 Agent 先运行一次 `dry-run` 获取 `gas_estimate`，评估成本后再决定是否执行真实交易。
2. **重试策略**：若捕获到 `rpc_error` 中的 Nonce 冲突，建议 Agent 等待 1-2 个区块后重试。
3. **多钱包并发**：由于程序无状态，可通过指定不同 `--wallet` 索引实现完全并行的交易流。
