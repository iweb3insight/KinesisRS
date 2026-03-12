use serde::{Serialize, Deserialize};

/// Represents the final, structured result of a trade execution.
/// This is the primary data structure for communication with an LLM agent.
#[derive(Debug, Serialize, Deserialize)]
pub struct TradeResult {
    pub success: bool,
    pub chain: Chain,
    pub stages: Vec<Stage>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_out: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_used: Option<u128>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_estimate: Option<u128>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_impact_percent: Option<f64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_info: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<TradeError>,
}

/// Represents a single step in the trade execution process for fine-grained observability.
#[derive(Debug, Serialize, Deserialize)]
pub struct Stage {
    pub name: String,
    pub duration_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<serde_json::Value>,
}

/// Represents detailed diagnostic information for failed trades.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TradeError {
    RpcError { message: String },
    SimulationFailed {
        revert_reason: Option<String>,
        raw_revert_data: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        decoded_custom: Option<CustomRevert>,
    },
    SendFailed {
        code: i64,
        message: String,
    },
    ConfigError { message: String },
    InvalidInput { message: String },
    ContractError { message: String },
}

/// Represents a decoded custom contract error.
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomRevert {
    pub selector: String,
    pub name: Option<String>,
    pub args: Vec<String>,
}

/// An enumeration of possible chains.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, clap::ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Chain {
    Bsc,
    Solana,
}

/// A simplified enumeration of error codes for basic categorization.
/// (Maintained for backward compatibility or simple logic where full TradeError is overkill)
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    InvalidInput,
    InvalidPrivateKey,
    ConfigurationError,
    RpcError,
    InsufficientLiquidity,
    SlippageExceeded,
    InsufficientFunds,
    TransactionFailed,
    SimulationFailed,
    ContractError,
    Unknown,
}
