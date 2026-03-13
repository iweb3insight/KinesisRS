# Kinesis.rs 迭代任务启动提示词模板

## I. 全局上下文 (Global Context)

### 项目核心 (Kinesis.rs - Agent-Native Execution Layer)
- **目标**: Kinesis.rs 是一个无状态、JSON 优先、Agent 原生的多链加密交易执行层。我们的愿景是成为 LLM Agent 最得心应手的交易大脑，实现极致的 **Agent 友好性 (Agentic Friendly)**。
- **核心原则**:
    - **零信任安全**: 私钥永不离开本地环境，不入 Prompt。
    - **确定性输出**: 所有操作返回结构化 JSON `TradeResult`。
    - **高效率**: 多 RPC 竞速，交易预构造，Tokio 异步。
    - **单向发布**: 内部私有库 (`chinaestone/freedom-agent-rust-private`) -> 脱敏镜像 -> 公开分发库 (`iweb3insight/Kinesis.rs`)。
    - **文档驱动**: 所有关键决策和流程需记录在 `docs/`。
- **SRE 宪法**: Concise, Simple, Stable, Evolutionary, Documented.
- **许可证**: Apache 2.0。

### 关键文档速览 (Quick Links)
- **项目架构**: `docs/architecture.md`
- **发布规范**: `docs/RELEASE_GUIDE.md`
- **版本路线图**: `docs/ROADMAP_MERGED.md`
- **商业分级策略**: `docs/COMMERCIAL_STRATEGY.md`
- **Agent Skill 定义**: `toolkit/skills/trading-skill/SKILL.md`

---

## II. 当前代码状态 (Current Code State)

### 项目版本
- **当前发布版本**: `v0.6.6` (已完成所有核心执行功能、安全加固和多语种文档)。
- **Git 仓库**: `https://github.com/iweb3insight/Kinesis.rs` (主分支 `main`)。

### 已完成里程碑 (v0.6.x)
- BSC & Solana (Pump.fun/Raydium V3) 交易执行、Quote、Balance。
- Jito Bundle 集成、Token-2022 兼容。
- GitHub Actions CI/CD (多平台发布)。
- 全球化文档 (EN/ZH/JA/KO/TW) 与品牌形象统一。
- GitHub Security Suite (CodeQL, Dependabot, Private Vulnerability Reporting) 已启用并修复所有告警。
- 明确的开源与商业分级策略。

### 待处理事项摘要 (Pending Items)
- **依赖升级**: 持续关注 `docs/DEPENDABOT_PR_ANALYSIS.md` 中列出的待处理 PR (例如 `rand`, `bincode`, `alloy` 系列库)。
- **下一个版本目标**: `v0.7.0` (交易核心增强: `tx-status`, 自动重试, 交易历史)。

---

## III. 增量任务 (Incremental Task)

**[请在此处插入具体的、可执行的下一个任务。示例如下：]**

"请根据 `docs/ROADMAP_MERGED.md` 中的 `v0.7.0` 阶段，开始实现 **1.1 交易状态追踪 (`tx-status`)** 功能。具体要求如下：
- 在 `src/types.rs` 中定义 `TransactionStatus` 枚举。
- 在 `src/solana/executor.rs` 中实现 `get_solana_transaction_status` 方法。
- 在 `src/bsc/executor.rs` 中实现 `get_bsc_transaction_status` 方法。
- 在 `src/main.rs` 中添加 `Commands::TxStatus` 入口，处理 CLI 参数并调用对应链的 Executor。
- 完成后，请更新 `PROGRESS.md`，并在必要时推送到 `iweb3insight/Kinesis.rs`。"

---

**Agent 行为准则**:
- **优先查阅文档**: 在执行任何修改前，务必参考相关 `docs/` 文件。
- **遵循架构**: 严格 adherence to `docs/architecture.md` 和 `docs/RELEASE_GUIDE.md`。
- **TDD 驱动**: 新功能应伴随测试。
- **输出 JSON**: 所有 CLI 输出默认 `JSON`，便于 LLM 解析。
- **迭代更新**: 完成任务后，请更新 `PROGRESS.md` 和 `docs/DEPENDABOT_PR_ANALYSIS.md` (如适用)。
