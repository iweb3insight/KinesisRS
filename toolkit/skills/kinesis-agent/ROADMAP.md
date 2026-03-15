# Kinesis Agent Roadmap

## 当前能力

### 已实现

| 能力 | 状态 |
|------|------|
| Quote 报价 | ✅ |
| Buy 买入 (dry-run) | ✅ |
| Sell 卖出 | ✅ |
| Balance 余额查询 | ✅ |
| Wallet 钱包地址 | ✅ |
| Detect 代币路径检测 | ✅ |
| 多链支持 (BSC/Solana) | ✅ |
| 跨平台适配 | ✅ |

### 缺失能力

1. 交易状态追踪
2. 历史记录
3. 多钱包管理
4. 风控规则
5. 实时价格监控
6. 批量交易
7. 交易重试
8. 通知机制

---

## Roadmap

### Phase 1: 交易核心增强 (v0.7.0)

#### 1.1 交易状态追踪

```bash
kinesis-rs tx-status <TX_HASH> [--chain solana|bsc]
```

**功能**:
- 查询链上交易确认状态
- 返回 confirmations 数量
- 超时自动轮询

**输出**:
```json
{
  "tx_hash": "...",
  "status": "confirmed|pending|failed",
  "confirmations": 12,
  "block_height": 12345678
}
```

#### 1.2 自动重试

```bash
kinesis-rs buy <TOKEN> <AMOUNT> --retry 3 --retry-interval 5s
```

**功能**:
- 交易失败自动重试
- 可配置重试次数和间隔
- 指数退避策略

#### 1.3 交易历史

```bash
kinesis-rs history [--limit 10] [--status all|success|failed]
```

**功能**:
- 本地存储交易记录
- 支持 JSON/SQLite 存储
- 按时间/状态过滤

**存储格式**:
```json
{
  "id": "uuid",
  "timestamp": "2026-03-12T12:00:00Z",
  "type": "buy|sell",
  "token": "...",
  "amount_in": "0.001",
  "amount_out": "1450172355",
  "status": "success|failed",
  "tx_hash": "...",
  "error": null,
  "chain": "solana",
  "stages": [...]
}
```

#### 1.4 交易取消

```bash
kinesis-rs cancel <TX_HASH> [--chain solana]
```

**功能**:
- 取消待确认交易 (Solana)
- 使用 `cancel` 指令

---

### Phase 2: 风控与管理 (v0.8.0)

#### 2.1 止盈止损

```bash
kinesis-rs buy <TOKEN> <AMOUNT> --take-profit 50 --stop-loss 10
```

**功能**:
- 设置止盈百分比
- 设置止损百分比
- 触发时自动卖出

**逻辑**:
```
买入价格 = P
止盈价格 = P * (1 + 50%)
止损价格 = P * (1 - 10%)
```

#### 2.2 仓位管理

```bash
kinesis-rs buy <TOKEN> <AMOUNT> --max-position 100
```

**功能**:
- 限制单币种持仓上限
- 超过阈值禁止买入

#### 2.3 交易限额

```bash
kinesis-rs config set daily-limit 1.0    # 每日限额 1 SOL
kinesis-rs config set single-limit 0.5  # 单笔限额 0.5 SOL
```

**功能**:
- 单笔交易限额
- 每日累计限额
- 超限拒绝交易

#### 2.4 授权管理

```bash
kinesis-rs approve <TOKEN> --revoke     # 撤销授权
kinesis-rs approvals list               # 列出授权
```

**功能**:
- 查看当前授权
- 撤销不需要的授权
- 批量管理

---

### Phase 3: 监控与通知 (v0.9.0)

#### 3.1 价格监控

```bash
kinesis-rs watch <TOKEN> --threshold 0.001 [--interval 10s]
kinesis-rs watch stop
```

**功能**:
- 实时监控代币价格
- 价格达到阈值触发回调
- 支持 Telegram/Discord 通知

#### 3.2 Telegram 通知

```bash
kinesis-rs config set notify.telegram.bot_token <TOKEN>
kinesis-rs config set notify.telegram.chat_id <ID>
```

**通知事件**:
- 交易成功
- 交易失败
- 价格告警
- 授权变更

#### 3.3 Discord 通知

```bash
kinesis-rs config set notify.discord.webhook <URL>
```

#### 3.4 Webhook 回调

```bash
kinesis-rs config set webhook https://your-server.com/callback
```

---

### Phase 4: 高级功能 (v1.0)

#### 4.1 批量交易

```bash
kinesis-rs batch --file trades.csv
```

**CSV 格式**:
```csv
type,token,amount,slippage
buy, DezXAZ8z...,0.001,15
sell, DezXAZ8z...,1000,20
```

#### 4.2 策略模板

```bash
kinesis-rs strategy grid --token <TOKEN> --range 0.001-0.01 --grids 10
kinesis-rs strategy martingale --token <TOKEN> --base 0.001 --multiplier 2
```

#### 4.3 DCA 定投

```bash
kinesis-rs dca create --token <TOKEN> --amount 0.01 --interval 24h --times 30
kinesis-rs dca list
kinesis-rs dca cancel <ID>
```

#### 4.4 多钱包管理

```bash
kinesis-rs wallet create --name hot
kinesis-rs wallet create --name cold
kinesis-rs wallet list
kinesis-rs wallet use hot
```

**功能**:
- 热钱包: 用于日常交易
- 冷钱包: 大额存储
- 定期归集

#### 4.5 Gas 优化

```bash
kinesis-rs config set gas.strategy optimal  # 智能调节
kinesis-rs config set gas.max-fee 0.001    # 最大费用
```

---

## 架构设计

```
┌─────────────────────────────────────────────────────────┐
│                    Kinesis Agent                        │
├─────────────────────────────────────────────────────────┤
│  Core Layer     │  Trading   │  Risk      │  Monitor  │
│  - CLI          │  - Order   │  - Limit   │  - Price  │
│  - RPC          │  - Status  │  - T/P/S/L │  - Alert  │
│  - Signing      │  - Retry   │  - Position│  - Notify │
│                 │  - History │            │           │
├─────────────────────────────────────────────────────────┤
│  Storage Layer                                            │
│  - Transaction History (SQLite/JSON)                     │
│  - Price Cache                                           │
│  - Config (JSON)                                         │
├─────────────────────────────────────────────────────────┤
│  Platform Layer                                          │
│  - opencode / openclaw / Gemini / Claude                │
└─────────────────────────────────────────────────────────┘
```

---

## 命令速查表

### v0.7.0

| 命令 | 功能 |
|------|------|
| `kinesis-rs tx-status <TX_HASH>` | 查询交易状态 |
| `kinesis-rs history` | 交易历史 |
| `kinesis-rs cancel <TX_HASH>` | 取消交易 |

### v0.8.0

| 命令 | 功能 |
|------|------|
| `kinesis-rs buy <TOKEN> <AMOUNT> --take-profit 50` | 止盈买入 |
| `kinesis-rs buy <TOKEN> <AMOUNT> --stop-loss 10` | 止损买入 |
| `kinesis-rs approve <TOKEN> --revoke` | 撤销授权 |

### v0.9.0

| 命令 | 功能 |
|------|------|
| `kinesis-rs watch <TOKEN> --threshold 0.001` | 价格监控 |
| `kinesis-rs config set notify.telegram...` | Telegram 通知 |

### v1.0

| 命令 | 功能 |
|------|------|
| `kinesis-rs batch --file trades.csv` | 批量交易 |
| `kinesis-rs dca create --token...` | 定投计划 |
| `kinesis-rs wallet create --name hot` | 多钱包 |

---

## 实施计划

### v0.7.0 (2周)

- [ ] 交易状态追踪
- [ ] 自动重试
- [ ] 交易历史
- [ ] 交易取消

### v0.8.0 (2周)

- [ ] 止盈止损
- [ ] 仓位管理
- [ ] 交易限额
- [ ] 授权管理

### v0.9.0 (2周)

- [ ] 价格监控
- [ ] Telegram 通知
- [ ] Discord 通知
- [ ] Webhook 回调

### v1.0 (3周)

- [ ] 批量交易
- [ ] 策略模板
- [ ] DCA 定投
- [ ] 多钱包管理
- [ ] Gas 优化

---

**最后更新**: 2026-03-12
**版本**: 1.0
