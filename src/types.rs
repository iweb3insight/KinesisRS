use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;

/// Represents the final, structured result of a trade execution.
/// This is the primary data structure for communication with an LLM agent.
#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Stage {
    pub name: String,
    pub duration_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<serde_json::Value>,
}

/// Represents detailed diagnostic information for failed trades.
#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomRevert {
    pub selector: String,
    pub name: Option<String>,
    pub args: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TransactionStatus {
    Pending,
    Success,
    Failed,
    NotFound,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionStatusResponse {
    pub status: TransactionStatus,
    pub tx_hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmations: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// An enumeration of possible chains.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, clap::ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Chain {
    Bsc,
    Solana,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HealthResult {
    pub success: bool,
    pub chain: Chain,
    pub rpc_status: RpcStatus,
    pub wallet_status: WalletStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RpcStatus {
    pub connected: bool,
    pub latency_ms: u64,
    pub endpoint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_height: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WalletStatus {
    pub ready: bool,
    pub address: String,
    pub balance: String,
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

/// Transaction history entry for local storage
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionHistoryEntry {
    pub id: String,
    pub timestamp: String,
    #[serde(rename = "type")]
    pub tx_type: String,
    pub token: String,
    pub amount_in: String,
    pub amount_out: Option<String>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub chain: Chain,
    pub stages: Vec<Stage>,
}

impl TransactionHistoryEntry {
    pub fn from_trade_result(trade_result: &TradeResult, tx_type: &str, token: &str, amount_in: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            tx_type: tx_type.to_string(),
            token: token.to_string(),
            amount_in: amount_in.to_string(),
            amount_out: trade_result.amount_out.clone(),
            status: if trade_result.success { "success".to_string() } else { "failed".to_string() },
            tx_hash: trade_result.tx_hash.clone(),
            error: trade_result.error.as_ref().map(|e| format!("{:?}", e)),
            chain: trade_result.chain,
            stages: trade_result.stages.clone(),
        }
    }
}

/// History storage manager
pub struct HistoryManager {
    storage_path: PathBuf,
}

impl HistoryManager {
    pub fn new() -> Self {
        let mut path = dirs::home_dir().unwrap_or_default();
        path.push(".kinesis");
        path.push("history.json");
        Self { storage_path: path }
    }

    pub fn save(&self, entry: &TransactionHistoryEntry) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = self.storage_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut entries = self.load()?;
        entries.insert(0, entry.clone());
        
        let json = serde_json::to_string_pretty(&entries)?;
        fs::write(&self.storage_path, json)?;
        Ok(())
    }

    pub fn load(&self) -> Result<Vec<TransactionHistoryEntry>, Box<dyn std::error::Error>> {
        if !self.storage_path.exists() {
            return Ok(Vec::new());
        }
        let content = fs::read_to_string(&self.storage_path)?;
        let entries: Vec<TransactionHistoryEntry> = serde_json::from_str(&content)?;
        Ok(entries)
    }

    pub fn get_history(&self, limit: usize, status: &str, chain: Chain) -> Vec<TransactionHistoryEntry> {
        let entries = self.load().unwrap_or_default();
        entries
            .into_iter()
            .filter(|e| e.chain == chain)
            .filter(|e| status == "all" || e.status == status)
            .take(limit)
            .collect()
    }
}
