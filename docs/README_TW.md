# Kinesis.rs v0.6.0

> "Kinesis：感應市場情緒，點亮鏈上生機"

Kinesis.rs 是一款專為 LLM Agent 設計的無狀態、JSON 優先、Agent 原生的多鏈加密貨幣交易執行層，具備極致的 **Agent 友好性 (Agentic Friendly)**。

<div align="center">
  <img src="kinesis-rs.png" width="640" />
</div>

## 安全架構 (Security Architecture)

Kinesis.rs 採用「零信任」原則管理私鑰與鏈上互動。

### 1. 私鑰保護
- **不傳送至伺服器**: 私鑰永遠不會上傳到任何伺服器或雲端。它們僅從本地環境變數載入，且僅在本地執行程序中使用。
- **執行上下文隔離**: 私鑰明文永遠不會離開本地二進位程序。加密與簽章僅在受保護的記憶體中進行，確保私鑰絕不會洩漏給 Agent 提示詞 (Prompt) 或外部日誌記錄。
- **本地加密標準**: 我們建議在本地持久化儲存時使用 PBKDF2 + AES-256-GCM 對私鑰進行加密。
- **自動鎖定機制**: 執行上下文支援逾時鎖定，最大限度縮短私鑰在記憶體中的解密暴露窗口。

### 2. 鏈上防禦
- **交易時效保護**: 所有 BSC 側的 `buy` 與 `sell` 操作均強制包含鏈上 Deadline，防止過時交易被惡意執行。
- **動態授權目標**: `approve` 邏輯直接使用經核實合約回傳的 `approveTarget`，杜絕向惡意地址授權的風險。
- **公開透明**: 核心邏輯完全開源可審計，FreedomRouter 代理與實現合約已在 BscScan 完成驗證。

### 3. 模擬優先
- **強制 Dry-run**: 系統預設開啟 `--dry-run` 模式，必須顯式指定 `--no-dry-run` 才能發送真實交易，有效規避邏輯或參數錯誤導致的資產損失。

## 特性
- **多鏈支援**: 原生支援 BNB 智能鏈 (BSC) 和 Solana。
- **Agent 優先**: JSON 優先的通訊協定，與 LLM 無縫整合。
- **極致性能**: 支援多 RPC 競速與交易預建置邏輯。
- **全平台支援**: 通過 GitHub Releases 提供針對 Linux (amd64)、macOS (Intel/M1) 及 Windows (amd64) 的預編譯二進位包。
- **智慧路由**: 自動偵測並執行 Solana 上的 Pump.fun 與 Raydium V3 路徑（含 Token-2022 支援）。

## 快速開始
1. 複製儲存庫。
2. 將 `.env.example` 複製為 `.env` 並填入私鑰。
3. 編譯: `cargo build --release`。
4. 執行: `./target/release/kinesis-rs balance --chain solana`。

## CLI 使用說明

```text
Usage: kinesis-rs [OPTIONS] <COMMAND>

Commands:
  buy      Buy a token on a supported chain
  sell     Sell a token on a supported chain
  quote    Get a quote for a trade
  balance  Check balance of native or token
  approve  Approve a token for trading
  config   Display current configuration
  wallet   Display wallet address
  detect   Detect Solana token path (Pump.fun or Raydium)
  help     Print this message or the help of the given subcommand(s)

Options:
      --json             Global flag to force JSON output for agent consumption
      --dry-run          Global flag to simulate transactions without sending them
      --no-dry-run       Global flag to disable simulation and send the real transaction
      --wallet <WALLET>  Selects the wallet to use based on environment variable suffix (e.g., _1, _2) [default: 1]
  -h, --help             Print help (see more with '--help')
  -V, --version          Print version
```

## 贊助支持

Kinesis.rs 致力於推動 Agent 驅動的自動交易未來。為了支持我們在主網（Solana/BSC/ETH）持續進行功能驗證與實戰測試，歡迎通過以下地址進行贊助：

- **SOL**: `UFePGDrDS8xmutWkLKKGfgKUvacvLLSyQZ66AacKYUZ`
- **BNB**: `0x1580b9604c47Dbef3A61ae5a3deFF7f6611f3C28`
- **ETH**: `0x1580b9604c47Dbef3A61ae5a3deFF7f6611f3C28`

*所有贊助資金均將直接用於支付真實環境驗證所需的 Gas 費用及流動性測試成本。*

## 常見問題處理
