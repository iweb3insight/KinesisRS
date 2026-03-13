# FreedomAgent Rust v1.0 项目架构图

**文档版本**: 1.0  
**最后更新**: 2026-03-11  
**架构风格**: 阿里巴巴分层架构

---

## 1. 系统整体架构概览

```mermaid
flowchart TB
    subgraph External["外部系统层"]
        RPC_BSC[BSC RPC 节点<br/>主网/测试网]
        RPC_SOL[Solana RPC 节点]
        ENV[环境变量<br/>私钥/配置]
        USER[用户/Agent]
    end

    subgraph FreedomAgent["FreedomAgent Rust v1.0"]
        direction TB
        
        subgraph CLI["CLI 接入层"]
            Parser[Clap 参数解析]
            Validator[参数验证]
            Output[JSON/人类可读输出]
        end

        subgraph Business["业务逻辑层"]
            subgraph BSC["BSC 执行模块"]
                BscExecutor[BscExecutor]
                Router[路由选择<br/>FreedomRouter/PancakeSwap]
                Approve[自动授权]
                MultiRPC[多 RPC 负载均衡]
            end
            
            subgraph SOL["Solana 执行模块<br/>待实现"]
                SolExecutor[SolExecutor]
                PathDetector[Path Detector]
                JitoClient[Jito Bundle]
            end
        end

        subgraph Core["核心服务层"]
            Config[配置管理]
            Types[类型定义<br/>TradeResult/Stage]
            Error[错误处理]
            Logger[日志追踪]
        end

        subgraph Infra["基础设施层"]
            Alloy[Alloy 0.8.3<br/>EVM 交互库]
            Tokio[Tokio<br/>异步运行时]
            Reqwest[Reqwest<br/>HTTP 客户端]
        end
    end

    USER -->|命令行参数 | Parser
    Parser --> Validator
    Validator --> BscExecutor
    BscExecutor --> Router
    BscExecutor --> MultiRPC
    MultiRPC --> RPC_BSC
    Config -->|加载 | ENV
    BscExecutor -->|输出 | Output
    Output -->|JSON/文本 | USER
    
    classDef external fill:#e8f4f8,stroke:#1890ff
    classDef cli fill:#f6ffed,stroke:#52c41a
    classDef business fill:#fff7e6,stroke:#fa8c16
    classDef core fill:#f9f0ff,stroke:#722ed1
    classDef infra fill:#fff1f0,stroke:#f5222d
    
    class External external
    class CLI cli
    class Business business
    class Core core
    class Infra infra
```

---

## 2. 分层架构详解

```mermaid
flowchart LR
    subgraph L1["第一层：接入层 (Access Layer)"]
        A1[CLI 参数解析<br/>clap]
        A2[环境变量加载<br/>dotenvy]
        A3[输出格式化<br/>serde_json]
    end

    subgraph L2["第二层：业务层 (Business Layer)"]
        B1[BSC 交易执行<br/>buy/sell/quote]
        B2[授权管理<br/>approve/approveIfNeeded]
        B3[余额查询<br/>balance]
        B4[多钱包管理<br/>wallet selector]
    end

    subgraph L3["第三层：领域层 (Domain Layer)"]
        D1[TradeResult<br/>交易结果]
        D2[Stage<br/>执行阶段]
        D3[TradeError<br/>错误类型]
        D4[Chain<br/>链枚举]
    end

    subgraph L4["第四层：基础设施层 (Infrastructure Layer)"]
        I1[Alloy 0.8.3<br/>区块链交互]
        I2[Tokio<br/>异步运行时]
        I3[Reqwest<br/>HTTP 客户端]
        I4[tracing<br/>日志追踪]
    end

    L1 --> L2
    L2 --> L3
    L3 --> L4
    
    classDef l1 fill:#e6f7ff,stroke:#1890ff
    classDef l2 fill:#f6ffed,stroke:#52c41a
    classDef l3 fill:#fff7e6,stroke:#fa8c16
    classDef l4 fill:#f9f0ff,stroke:#722ed1
    
    class L1 l1
    class L2 l2
    class L3 l3
    class L4 l4
```

---

## 3. 模块依赖关系图

```mermaid
graph TD
    subgraph Root["根模块"]
        Lib[lib.rs]
        Main[main.rs]
    end

    subgraph Modules["功能模块"]
        CLI[cli.rs<br/>参数定义]
        CFG[config.rs<br/>配置管理]
        TYP[types.rs<br/>类型定义]
        BSC[bsc/mod.rs<br/>BSC 模块入口]
    end

    subgraph BSCInternal["BSC 子模块"]
        EXEC[bsc/executor.rs<br/>执行器核心]
    end

    subgraph Tests["测试模块"]
        T1[tests/bsc_executor.rs]
        T2[tests/bsc_executor_errors.rs]
        T3[tests/bsc_executor_logic.rs]
        T4[tests/cli_e2e.rs]
        T5[tests/output_observability.rs]
        T6[tests/rpc_robustness.rs]
        T7[tests/bsc_testnet_e2e.rs]
    end

    Lib --> CLI
    Lib --> CFG
    Lib --> TYP
    Lib --> BSC
    BSC --> EXEC
    Main --> CLI
    Main --> CFG
    Main --> TYP
    Main --> BSC
    
    T1 --> EXEC
    T2 --> EXEC
    T3 --> EXEC
    T4 --> CLI
    T5 --> TYP
    T6 --> EXEC
    T7 --> EXEC
    
    classDef root fill:#ffe58f,stroke:#faad14
    classDef mod fill:#95de64,stroke:#52c41a
    classDef bsc fill:#40a9ff,stroke:#1890ff
    classDef test fill:#d3adf7,stroke:#722ed1
    
    class Root root
    class Modules mod
    class BSCInternal bsc
    class Tests test
```

---

## 4. BSC 执行器内部架构

```mermaid
flowchart TB
    subgraph BscExecutor["BscExecutor 结构体"]
        Providers[providers: Vec<Arc<RootProvider>> <br/>多 RPC 支持]
        Signer[signer: PrivateKeySigner <br/>本地签名器]
        Router[router_address: Address <br/>路由合约地址]
        ChainId[chain_id: u64 <br/>链 ID]
    end

    subgraph PublicAPI["公共 API"]
        QB[quote_buy<br/>买入报价]
        QS[quote_sell<br/>卖出报价]
        Buy[buy<br/>买入执行]
        Sell[sell<br/>卖出执行]
        Approve[approve<br/>授权]
        ApproveIfNeed[approve_if_needed<br/>条件授权]
        Balance[get_balance<br/>余额查询]
        Allowance[get_allowance<br/>授权额度查询]
    end

    subgraph Internal["内部方法"]
        GetWETH[get_weth_address<br/>获取 WETH 地址]
        HandleErr[handle_rpc_error<br/>错误处理]
        ExtractRevert[extract_revert_reason<br/>提取回滚原因]
        DecodeRevert[decode_revert_data<br/>解码回滚数据]
    end

    subgraph Contracts["合约接口"]
        IBscRouter[IBscRouter<br/>FreedomRouter/PancakeSwap]
        IERC20[IERC20<br/>代币合约]
    end

    BscExecutor --> PublicAPI
    PublicAPI --> Internal
    PublicAPI --> Contracts
    
    QB --> IBscRouter
    QS --> IBscRouter
    Buy --> IBscRouter
    Sell --> IBscRouter
    Approve --> IERC20
    ApproveIfNeed --> IERC20
    Balance --> IERC20
    Allowance --> IERC20
    
    HandleErr --> ExtractRevert
    ExtractRevert --> DecodeRevert
    
    classDef struct fill:#ffe58f,stroke:#faad14
    classDef api fill:#95de64,stroke:#52c41a
    classDef internal fill:#40a9ff,stroke:#1890ff
    classDef contract fill:#ffccc7,stroke:#ff4d4f
    
    class BscExecutor struct
    class PublicAPI api
    class Internal internal
    class Contracts contract
```

---

## 5. 交易执行流程时序图

```mermaid
sequenceDiagram
    participant User as 用户/Agent
    participant CLI as CLI 层
    participant Config as 配置模块
    participant Executor as BscExecutor
    participant Router as 路由合约
    participant Token as 代币合约
    participant RPC as RPC 节点

    User->>CLI: buy 0xToken 0.1 --json --dry-run
    CLI->>CLI: 参数解析 & 验证
    
    CLI->>Config: get_bsc_private_key(wallet_id)
    Config-->>CLI: 私钥 (从 ENV)
    
    CLI->>Executor: new(config, private_key)
    Executor->>RPC: eth_chainId
    RPC-->>Executor: chain_id
    Executor-->>CLI: BscExecutor 实例
    
    CLI->>Executor: quote_buy(token, amount)
    Executor->>RPC: eth_call (quoteBuy)
    RPC-->>Executor: amount_out
    Executor-->>CLI: quote_result
    
    CLI->>Executor: buy(token, amount_in, amount_out_min, tip_rate, dry_run)
    Executor->>RPC: eth_estimateGas
    RPC-->>Executor: gas_estimate
    Executor->>RPC: eth_call (simulate)
    RPC-->>Executor: simulation_result
    
    Executor-->>CLI: ExecutionResult
    
    CLI->>CLI: 构建 TradeResult
    CLI->>CLI: 添加 stages (耗时统计)
    CLI-->>User: JSON TradeResult
```

---

## 6. 卖出交易流程 (含自动授权)

```mermaid
sequenceDiagram
    participant User as 用户/Agent
    participant CLI as CLI 层
    participant Executor as BscExecutor
    participant Router as 路由合约
    participant Token as 代币合约
    participant RPC as RPC 节点

    User->>CLI: sell 0xToken 10 --json
    CLI->>Executor: quote_sell(token, amount)
    Executor->>RPC: eth_call (quoteSell)
    RPC-->>Executor: amount_out
    Executor-->>CLI: quote_result
    
    CLI->>Executor: approve_if_needed(token, owner, amount)
    Executor->>RPC: eth_call (allowance)
    RPC-->>Executor: allowance_value
    
    alt allowance < amount
        Executor->>RPC: eth_sendRawTransaction (approve)
        RPC-->>Executor: approve_receipt
        Executor-->>CLI: Some(approve_result)
    else allowance >= amount
        Executor-->>CLI: None (无需授权)
    end
    
    CLI->>Executor: sell(token, amount_in, amount_out_min, tip_rate, dry_run)
    Executor->>RPC: eth_estimateGas
    RPC-->>Executor: gas_estimate
    Executor->>RPC: eth_call (simulate)
    RPC-->>Executor: simulation_result
    Executor-->>CLI: ExecutionResult
    
    CLI-->>User: TradeResult { stages: [approve, quote, simulate] }
```

---

## 7. 错误处理架构

```mermaid
flowchart TB
    subgraph ErrorSources["错误来源"]
        RPC_ERR[RPC 错误<br/>网络/限流/超时]
        CONTRACT_ERR[合约错误<br/>Revert/异常]
        CONFIG_ERR[配置错误<br/>缺失 ENV]
        INPUT_ERR[输入错误<br/>无效参数]
    end

    subgraph ErrorHandling["错误处理层"]
        HANDLE_RPC[handle_rpc_error]
        EXTRACT[extract_revert_reason]
        DECODE[decode_revert_data]
    end

    subgraph ErrorTypes["ExecutorError 枚举"]
        E1[ClientBuilderError]
        E2[InvalidKey]
        E3[ContractError]
        E4[TransactionError]
        E5[RpcError]
        E6[AllRpcsFailed]
        E7[SimulationFailed]
    end

    subgraph TradeError["TradeError 输出"]
        T1[RpcError]
        T2[SimulationFailed<br/>revert_reason<br/>raw_revert_data<br/>decoded_custom]
        T3[SendFailed]
        T4[ConfigError]
        T5[InvalidInput]
        T6[ContractError]
    end

    ErrorSources --> ErrorHandling
    ErrorHandling --> ErrorTypes
    ErrorTypes --> TradeError
    
    RPC_ERR --> HANDLE_RPC
    CONTRACT_ERR --> EXTRACT
    EXTRACT --> DECODE
    
    classDef source fill:#ffccc7,stroke:#ff4d4f
    classDef handle fill:#ffe58f,stroke:#faad14
    classDef exec fill:#95de64,stroke:#52c41a
    classDef trade fill:#40a9ff,stroke:#1890ff
    
    class ErrorSources source
    class ErrorHandling handle
    class ErrorTypes exec
    class TradeError trade
```

---

## 8. Revert Reason 解码流程

```mermaid
flowchart LR
    subgraph Input["错误输入"]
        ERR[RPC 错误消息<br/>"execution reverted: ..."]
    end

    subgraph Extract["提取阶段"]
        E1[查找"execution reverted:"]
        E2[提取原因字符串]
        E3[检测 0x 前缀]
    end

    subgraph Decode["解码阶段"]
        D1{是否有 0x 前缀？}
        D2[直接返回原因字符串]
        D3[hex 解码]
        D4{前 4 字节 selector}
    end

    subgraph Output["输出类型"]
        O1[Error string<br/>08c379a0]
        O2[Panic uint256<br/>4e487b71]
        O3[Custom Error<br/>其他 selector]
        O4[REVERT_NO_DATA]
    end

    Input --> E1
    E1 --> E2
    E2 --> E3
    E3 --> D1
    D1 -->|否 | D2
    D1 -->|是 | D3
    D3 --> D4
    D4 -->|08c379a0| O1
    D4 -->|4e487b71| O2
    D4 -->|其他 | O3
    
    classDef input fill:#ffccc7,stroke:#ff4d4f
    classDef extract fill:#ffe58f,stroke:#faad14
    classDef decode fill:#95de64,stroke:#52c41a
    classDef output fill:#40a9ff,stroke:#1890ff
    
    class Input input
    class Extract extract
    class Decode decode
    class Output output
```

---

## 9. 数据模型架构

```mermaid
classDiagram
    class TradeResult {
        +success: bool
        +chain: Chain
        +stages: Vec~Stage~
        +tx_hash: Option~String~
        +amount_out: Option~String~
        +gas_used: Option~u128~
        +gas_estimate: Option~u128~
        +price_impact_percent: Option~f64~
        +route_info: Option~String~
        +error: Option~TradeError~
    }

    class Stage {
        +name: String
        +duration_ms: u64
        +input: Option~Value~
        +output: Option~Value~
    }

    class TradeError {
        <<enumeration>>
        RpcError
        SimulationFailed
        SendFailed
        ConfigError
        InvalidInput
        ContractError
    }

    class CustomRevert {
        +selector: String
        +name: Option~String~
        +args: Vec~String~
    }

    class Chain {
        <<enumeration>>
        Bsc
        Solana
    }

    class ErrorCode {
        <<enumeration>>
        InvalidInput
        InvalidPrivateKey
        ConfigurationError
        RpcError
        InsufficientLiquidity
        SlippageExceeded
        InsufficientFunds
        TransactionFailed
        SimulationFailed
        ContractError
        Unknown
    }

    TradeResult "1" *-- "1..*" Stage
    TradeResult "1" *-- "0..1" TradeError
    TradeError "1" *-- "0..1" CustomRevert
    TradeResult "1" *-- "1" Chain
    
    classDef model fill:#e6f7ff,stroke:#1890ff
    classDef enum fill:#f6ffed,stroke:#52c41a
    classDef struct fill:#fff7e6,stroke:#fa8c16
    
    class TradeResult model
    class Stage struct
    class TradeError model
    class CustomRevert struct
    class Chain enum
    class ErrorCode enum
```

---

## 10. CLI 命令架构

```mermaid
flowchart TB
    subgraph GlobalFlags["全局标志"]
        G1[--json<br/>JSON 输出模式]
        G2[--dry-run<br/>模拟模式 默认=true]
        G3[--wallet N<br/>钱包选择]
    end

    subgraph Commands["命令"]
        C1[buy<br/>买入]
        C2[sell<br/>卖出]
        C3[quote<br/>报价]
        C4[balance<br/>余额]
        C5[approve<br/>授权]
        C6[config<br/>配置]
        C7[wallet<br/>钱包地址]
    end

    subgraph BuyArgs["buy 参数"]
        B1[token_address: String]
        B2[amount: f64]
        B3[--chain: Chain]
        B4[--slippage: f32 0-100]
        B5[--tip_rate: f32 0-5]
    end

    subgraph SellArgs["sell 参数"]
        S1[token_address: String]
        S2[amount: f64]
        S3[--chain: Chain]
        S4[--slippage: f32 0-100]
        S5[--tip_rate: f32 0-5]
    end

    subgraph QuoteArgs["quote 参数"]
        Q1[token_address: String]
        Q2[amount: f64]
        Q3[--action: buy/sell]
        Q4[--chain: Chain]
    end

    GlobalFlags --> Commands
    C1 --> BuyArgs
    C2 --> SellArgs
    C3 --> QuoteArgs
    
    classDef global fill:#ffe58f,stroke:#faad14
    classDef cmd fill:#95de64,stroke:#52c41a
    classDef args fill:#40a9ff,stroke:#1890ff
    
    class GlobalFlags global
    class Commands cmd
    class BuyArgs args
    class SellArgs args
    class QuoteArgs args
```

---

## 11. 配置管理架构

```mermaid
flowchart TB
    subgraph EnvVars["环境变量"]
        E1[BSC_RPC_URL<br/>逗号分隔多 RPC]
        E2[SOL_RPC_URL]
        E3[BSC_ROUTER_ADDRESS<br/>可选]
        E4[HTTPS_PROXY<br/>可选]
        E5[BSC_PRIVATE_KEY_N<br/>N=1,2,3...]
    end

    subgraph ConfigStruct["Config 结构体"]
        C1[bsc_rpc_urls: Vec~String~]
        C2[sol_rpc_url: String]
        C3[bsc_router_address: Option~String~]
        C4[https_proxy: Option~String~]
    end

    subgraph Methods["方法"]
        M1[load<br/>从 ENV 加载配置]
        M2[get_bsc_private_key<br/>按索引获取私钥]
    end

    subgraph Validation["验证"]
        V1[RPC URL 非空]
        V2[私钥格式验证]
        V3[slippage 0-100]
        V4[tip_rate 0-5]
    end

    EnvVars --> ConfigStruct
    ConfigStruct --> Methods
    Methods --> Validation
    
    classDef env fill:#ffccc7,stroke:#ff4d4f
    classDef struct fill:#ffe58f,stroke:#faad14
    classDef method fill:#95de64,stroke:#52c41a
    classDef valid fill:#40a9ff,stroke:#1890ff
    
    class EnvVars env
    class ConfigStruct struct
    class Methods method
    class Validation valid
```

---

## 12. 测试架构

```mermaid
flowchart TB
    subgraph UnitTests["单元测试"]
        U1[main.rs::tests<br/>3 个测试]
    end

    subgraph IntegrationTests["集成测试"]
        I1[bsc_executor.rs<br/>6 个测试<br/>报价/授权/余额/模拟]
        I2[bsc_executor_errors.rs<br/>4 个测试<br/>错误处理]
        I3[bsc_executor_logic.rs<br/>1 个测试<br/>小费编码]
        I4[cli_e2e.rs<br/>9 个测试<br/>CLI 命令]
        I5[output_observability.rs<br/>5 个测试<br/>输出验证]
        I6[rpc_robustness.rs<br/>3 个测试<br/>RPC 鲁棒性]
    end

    subgraph E2ETests["E2E 测试"]
        E1[bsc_testnet_e2e.rs<br/>1 个测试<br/>真实网络 忽略]
    end

    subgraph Coverage["覆盖率"]
        C1[CLI 参数：90%]
        C2[BSC 执行：73%]
        C3[输出观测：100%]
        C4[RPC 鲁棒：75%]
        C5[单元：100%]
        C6[总计：88.6%]
    end

    UnitTests --> Coverage
    IntegrationTests --> Coverage
    E2ETests --> Coverage
    
    classDef unit fill:#ffe58f,stroke:#faad14
    classDef integration fill:#95de64,stroke:#52c41a
    classDef e2e fill:#40a9ff,stroke:#1890ff
    classDef cov fill:#f9f0ff,stroke:#722ed1
    
    class UnitTests unit
    class IntegrationTests integration
    class E2ETests e2e
    class Coverage cov
```

---

## 13. 依赖关系架构

```mermaid
flowchart TB
    subgraph AppDeps["应用依赖"]
        A1[tokio 1.x<br/>异步运行时]
        A2[clap 4.x<br/>CLI 解析]
        A3[serde 1.x<br/>序列化]
        A4[serde_json 1.x<br/>JSON]
        A5[thiserror 1.x<br/>错误处理]
        A6[tracing 0.1.x<br/>日志]
        A7[dotenvy 0.15<br/>ENV 加载]
        A8[reqwest 0.12<br/>HTTP]
    end

    subgraph AlloyDeps["Alloy 依赖"]
        B1[alloy 0.8.3<br/>元包]
        B2[alloy-sol-types 0.8.26]
        B3[alloy-primitives 0.8.26]
        B4[alloy-contract 0.8.3]
    end

    subgraph DevDeps["开发依赖"]
        C1[mockito 1.4<br/>HTTP Mock]
        C2[assert_cmd 2.0<br/>CLI 测试]
        C3[predicates 3.1<br/>断言]
    end

    subgraph Core["核心模块"]
        D1[lib.rs]
        D2[main.rs]
        D3[bsc/executor.rs]
    end

    AppDeps --> Core
    AlloyDeps --> Core
    DevDeps -.-> Core
    
    classDef app fill:#e6f7ff,stroke:#1890ff
    classDef alloy fill:#f6ffed,stroke:#52c41a
    classDef dev fill:#fff7e6,stroke:#fa8c16
    classDef core fill:#f9f0ff,stroke:#722ed1
    
    class AppDeps app
    class AlloyDeps alloy
    class DevDeps dev
    class Core core
```

---

## 14. 部署架构

```mermaid
flowchart TB
    subgraph Build["构建环境"]
        B1[cargo build<br/>--release]
        B2[target/release/<br/>solana_claw_coin_cli]
    end

    subgraph Runtime["运行环境"]
        R1[Linux/macOS]
        R2[环境变量]
        R3[.env 文件]
    end

    subgraph Execution["执行模式"]
        E1[JSON 模式<br/>--json]
        E2[人类可读模式<br/>默认]
        E3[模拟模式<br/>--dry-run 默认]
        E4[真实交易<br/>--no-dry-run]
    end

    subgraph External["外部依赖"]
        X1[BSC RPC 节点]
        X2[Solana RPC 节点]
        X3[代理服务器<br/>可选]
    end

    Build --> Runtime
    Runtime --> Execution
    Execution --> External
    
    classDef build fill:#ffe58f,stroke:#faad14
    classDef runtime fill:#95de64,stroke:#52c41a
    classDef exec fill:#40a9ff,stroke:#1890ff
    classDef ext fill:#ffccc7,stroke:#ff4d4f
    
    class Build build
    class Runtime runtime
    class Execution exec
    class External ext
```

---

## 附录：架构图例说明

| 颜色 | 含义 |
|------|------|
| 🔵 蓝色 | 核心业务模块 |
| 🟢 绿色 | 业务逻辑/功能模块 |
| 🟡 黄色 | 配置/结构体 |
| 🔴 红色 | 外部系统/错误 |
| 🟣 紫色 | 核心服务/类型 |

---

**文档状态**: 已完成  
**审核状态**: 待审核  
**关联文档**: `Cargo.toml`, `src/`, `docs/alloy-dependency.md`
