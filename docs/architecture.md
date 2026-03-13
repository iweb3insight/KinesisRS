# Kinesis.rs Rust v1.0 Project Architecture

**Document Version**: 1.0  
**Last Updated**: 2026-03-11  
**Architecture Style**: Alibaba Layered Architecture

---

## 1. System Overview

```mermaid
flowchart TB
    subgraph External["External Systems"]
        RPC_BSC[BSC RPC Nodes<br/>Mainnet/Testnet]
        RPC_SOL[Solana RPC Nodes]
        ENV[Environment Variables<br/>Private Keys/Config]
        USER[User/Agent]
    end

    subgraph Kinesis["Kinesis.rs Rust v1.0"]
        direction TB
        
        subgraph CLI["CLI Access Layer"]
            Parser[Clap Argument Parsing]
            Validator[Parameter Validation]
            Output[JSON/Human-readable Output]
        end

        subgraph Business["Business Logic Layer"]
            subgraph BSC["BSC Execution Module"]
                BscExecutor[BscExecutor]
                Router[Router Selection<br/>FreedomRouter/PancakeSwap]
                Approve[Auto-approval]
                MultiRPC[Multi-RPC Load Balancing]
            end
            
            subgraph SOL["Solana Execution Module"]
                SolExecutor[SolExecutor]
                PathDetector[Path Detector]
                JitoClient[Jito Bundle]
            end
        end

        subgraph Core["Core Service Layer"]
            Config[Configuration Management]
            Types[Type Definitions<br/>TradeResult/Stage]
            Error[Error Handling]
            Logger[Log Tracing]
        end

        subgraph Infra["Infrastructure Layer"]
            Alloy[Alloy 0.8.3<br/>EVM Interaction Library]
            Tokio[Tokio<br/>Async Runtime]
            Reqwest[Reqwest<br/>HTTP Client]
        end
    end

    USER -->|Command Arguments| Parser
    Parser --> Validator
    Validator --> BscExecutor
    BscExecutor --> Router
    BscExecutor --> MultiRPC
    MultiRPC --> RPC_BSC
    Config -->|Load| ENV
    BscExecutor -->|Output| Output
    Output -->|JSON/Text| USER
    
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

## 2. Layered Architecture Details

```mermaid
flowchart LR
    subgraph L1["Layer 1: Access Layer"]
        A1[CLI Argument Parsing<br/>clap]
        A2[Env Variable Loading<br/>dotenvy]
        A3[Output Formatting<br/>serde_json]
    end

    subgraph L2["Layer 2: Business Layer"]
        B1[BSC Trade Execution<br/>buy/sell/quote]
        B2[Allowance Management<br/>approve/approveIfNeeded]
        B3[Balance Inquiry<br/>balance]
        B4[Multi-wallet Management<br/>wallet selector]
    end

    subgraph L3["Layer 3: Domain Layer"]
        D1[TradeResult<br/>Execution Results]
        D2[Stage<br/>Execution Phases]
        D3[TradeError<br/>Error Types]
        D4[Chain<br/>Chain Enumeration]
    end

    subgraph L4["Layer 4: Infrastructure Layer"]
        I1[Alloy 0.8.3<br/>Blockchain Interaction]
        I2[Tokio<br/>Async Runtime]
        I3[Reqwest<br/>HTTP Client]
        I4[tracing<br/>Structured Logging]
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

## 3. Module Dependencies

```mermaid
graph TD
    subgraph Root["Root"]
        Lib[lib.rs]
        Main[main.rs]
    end

    subgraph Modules["Functional Modules"]
        CLI[cli.rs<br/>Args Definition]
        CFG[config.rs<br/>Configuration]
        TYP[types.rs<br/>Type Definitions]
        BSC[bsc/mod.rs<br/>BSC Entry]
    end

    subgraph BSCInternal["BSC Sub-modules"]
        EXEC[bsc/executor.rs<br/>Core Executor]
    end

    subgraph Tests["Test Modules"]
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

## 4. BscExecutor Internal Architecture

```mermaid
flowchart TB
    subgraph BscExecutor["BscExecutor Struct"]
        Providers[providers: Vec<Arc<RootProvider>> <br/>Multi-RPC Support]
        Signer[signer: PrivateKeySigner <br/>Local Signer]
        Router[router_address: Address <br/>Router Contract]
        ChainId[chain_id: u64 <br/>Chain ID]
    end

    subgraph PublicAPI["Public API"]
        QB[quote_buy<br/>Buy Quote]
        QS[quote_sell<br/>Sell Quote]
        Buy[buy<br/>Execute Buy]
        Sell[sell<br/>Execute Sell]
        Approve[approve<br/>Approve Token]
        ApproveIfNeed[approve_if_needed<br/>Conditional Approval]
        Balance[get_balance<br/>Balance Inquiry]
        Allowance[get_allowance<br/>Allowance Inquiry]
    end

    subgraph Internal["Internal Methods"]
        GetWETH[get_weth_address<br/>Fetch WETH Address]
        HandleErr[handle_rpc_error<br/>Error Handling]
        ExtractRevert[extract_revert_reason<br/>Extract Revert Reason]
        DecodeRevert[decode_revert_data<br/>Decode Revert Data]
    end

    subgraph Contracts["Contract Interfaces"]
        IBscRouter[IBscRouter<br/>FreedomRouter/PancakeSwap]
        IERC20[IERC20<br/>ERC20 Token Contract]
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

## 5. Trade Execution Sequence

```mermaid
sequenceDiagram
    participant User as User/Agent
    participant CLI as CLI Layer
    participant Config as Config Module
    participant Executor as BscExecutor
    participant Router as Router Contract
    participant Token as Token Contract
    participant RPC as RPC Node

    User->>CLI: buy 0xToken 0.1 --json --dry-run
    CLI->>CLI: Parse & Validate Args
    
    CLI->>Config: get_bsc_private_key(wallet_id)
    Config-->>CLI: Private Key (from ENV)
    
    CLI->>Executor: new(config, private_key)
    Executor->>RPC: eth_chainId
    RPC-->>Executor: chain_id
    Executor-->>CLI: BscExecutor Instance
    
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
    
    CLI->>CLI: Build TradeResult
    CLI->>CLI: Add Stages (Timing)
    CLI-->>User: JSON TradeResult
```

---

## 6. Sell Flow (with Auto-Approval)

```mermaid
sequenceDiagram
    participant User as User/Agent
    participant CLI as CLI Layer
    participant Executor as BscExecutor
    participant Router as Router Contract
    participant Token as Token Contract
    participant RPC as RPC Node

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
        Executor-->>CLI: None (No Approval Needed)
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

## 7. Error Handling Architecture

```mermaid
flowchart TB
    subgraph ErrorSources["Error Sources"]
        RPC_ERR[RPC Error<br/>Network/Limit/Timeout]
        CONTRACT_ERR[Contract Error<br/>Revert/Exception]
        CONFIG_ERR[Config Error<br/>Missing ENV]
        INPUT_ERR[Input Error<br/>Invalid Params]
    end

    subgraph ErrorHandling["Error Handling Layer"]
        HANDLE_RPC[handle_rpc_error]
        EXTRACT[extract_revert_reason]
        DECODE[decode_revert_data]
    end

    subgraph ErrorTypes["ExecutorError Enum"]
        E1[ClientBuilderError]
        E2[InvalidKey]
        E3[ContractError]
        E4[TransactionError]
        E5[RpcError]
        E6[AllRpcsFailed]
        E7[SimulationFailed]
    end

    subgraph TradeError["TradeError Output"]
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

## 8. Revert Reason Decoding Flow

```mermaid
flowchart LR
    subgraph Input["Error Input"]
        ERR[RPC Error Message<br/>"execution reverted: ..."]
    end

    subgraph Extract["Extraction Phase"]
        E1[Find "execution reverted:"]
        E2[Extract Reason String]
        E3[Detect 0x Prefix]
    end

    subgraph Decode["Decoding Phase"]
        D1{0x Prefix?}
        D2[Return String Directly]
        D3[Hex Decode]
        D4{First 4 Bytes Selector}
    end

    subgraph Output["Output Types"]
        O1[Error string<br/>08c379a0]
        O2[Panic uint256<br/>4e487b71]
        O3[Custom Error<br/>Other Selector]
        O4[REVERT_NO_DATA]
    end

    Input --> E1
    E1 --> E2
    E2 --> E3
    E3 --> D1
    D1 -->|No| D2
    D1 -->|Yes| D3
    D3 --> D4
    D4 -->|08c379a0| O1
    D4 -->|4e487b71| O2
    D4 -->|Other| O3
    
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

## 9. Data Model Architecture

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

## 10. CLI Command Architecture

```mermaid
flowchart TB
    subgraph GlobalFlags["Global Flags"]
        G1[--json<br/>JSON Output Mode]
        G2[--dry-run<br/>Simulation Mode (Default)]
        G3[--wallet N<br/>Wallet Selection]
    end

    subgraph Commands["Commands"]
        C1[buy<br/>Execute Buy]
        C2[sell<br/>Execute Sell]
        C3[quote<br/>Price Quote]
        C4[balance<br/>Balance Inquiry]
        C5[approve<br/>Token Approval]
        C6[config<br/>Show Config]
        C7[wallet<br/>Show Addresses]
    end

    subgraph BuyArgs["buy Arguments"]
        B1[token_address: String]
        B2[amount: f64]
        B3[--chain: Chain]
        B4[--slippage: f32 0-100]
        B5[--tip_rate: f32 0-5]
    end

    subgraph SellArgs["sell Arguments"]
        S1[token_address: String]
        S2[amount: f64]
        S3[--chain: Chain]
        S4[--slippage: f32 0-100]
        S5[--tip_rate: f32 0-5]
    end

    subgraph QuoteArgs["quote Arguments"]
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

## 11. Configuration Management

```mermaid
flowchart TB
    subgraph EnvVars["Environment Variables"]
        E1[BSC_RPC_URL<br/>Comma-separated RPCs]
        E2[SOL_RPC_URL]
        E3[BSC_ROUTER_ADDRESS<br/>Optional]
        E4[HTTPS_PROXY<br/>Optional]
        E5[BSC_PRIVATE_KEY_N<br/>N=1,2,3...]
    end

    subgraph ConfigStruct["Config Struct"]
        C1[bsc_rpc_urls: Vec~String~]
        C2[sol_rpc_url: String]
        C3[bsc_router_address: Option~String~]
        C4[https_proxy: Option~String~]
    end

    subgraph Methods["Methods"]
        M1[load<br/>Load from ENV]
        M2[get_bsc_private_key<br/>Fetch by Index]
    end

    subgraph Validation["Validation"]
        V1[RPC URL Non-empty]
        V2[Key Format Check]
        V3[Slippage Range 0-100]
        V4[Tip Rate Range 0-5]
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

## 12. Test Architecture

```mermaid
flowchart TB
    subgraph UnitTests["Unit Tests"]
        U1[main.rs::tests<br/>Clap Logic]
    end

    subgraph IntegrationTests["Integration Tests"]
        I1[bsc_executor.rs<br/>Quote/Approve/Balance/Sim]
        I2[bsc_executor_errors.rs<br/>Error Handling]
        I3[bsc_executor_logic.rs<br/>Tip Encoding]
        I4[cli_e2e.rs<br/>CLI Command Flow]
        I5[output_observability.rs<br/>Output Validation]
        I6[rpc_robustness.rs<br/>Multi-RPC Reliability]
    end

    subgraph E2ETests["E2E Tests"]
        E1[bsc_testnet_e2e.rs<br/>Real Network (Ignored)]
    end

    subgraph Coverage["Coverage Metrics"]
        C1[CLI Parsing: 90%]
        C2[BSC Execution: 73%]
        C3[Observability: 100%]
        C4[RPC Robustness: 75%]
        C5[Unit Tests: 100%]
        C6[Total: 88.6%]
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

## 13. Dependency Architecture

```mermaid
flowchart TB
    subgraph AppDeps["App Dependencies"]
        A1[tokio 1.x<br/>Runtime]
        A2[clap 4.x<br/>CLI Parser]
        A3[serde 1.x<br/>Serialization]
        A4[serde_json 1.x<br/>JSON]
        A5[thiserror 1.x<br/>Errors]
        A6[tracing 0.1.x<br/>Logs]
        A7[dotenvy 0.15<br/>ENV Loader]
        A8[reqwest 0.12<br/>HTTP]
    end

    subgraph AlloyDeps["Alloy Dependencies"]
        B1[alloy 0.8.3<br/>Metapackage]
        B2[alloy-sol-types 0.8.26]
        B3[alloy-primitives 0.8.26]
        B4[alloy-contract 0.8.3]
    end

    subgraph DevDeps["Dev Dependencies"]
        C1[mockito 1.4<br/>HTTP Mocking]
        C2[assert_cmd 2.0<br/>CLI Testing]
        C3[predicates 3.1<br/>Assertions]
    end

    subgraph Core["Core Modules"]
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

## 14. Deployment Architecture

```mermaid
flowchart TB
    subgraph Build["Build Environment"]
        B1[cargo build<br/>--release]
        B2[target/release/<br/>kinesis-rs]
    end

    subgraph Runtime["Runtime Environment"]
        R1[Linux/macOS]
        R2[Environment Variables]
        R3[.env File]
    end

    subgraph Execution["Execution Modes"]
        E1[JSON Mode<br/>--json]
        E2[Human-readable Mode<br/>Default]
        E3[Simulation Mode<br/>--dry-run]
        E4[Real Transaction<br/>--no-dry-run]
    end

    subgraph External["External Dependencies"]
        X1[BSC RPC Nodes]
        X2[Solana RPC Nodes]
        X3[Proxy Server<br/>Optional]
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

**Document Status**: Finalized  
**Audit Status**: Approved  
**Related Docs**: `Cargo.toml`, `src/`
