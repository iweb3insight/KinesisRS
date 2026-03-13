// src/mcp/tools/wallet.rs

use crate::mcp::types::ToolResult as McpToolResult;
use crate::mcp::errors::ToolError;
use crate::config::Config;
use crate::bsc::executor::BscExecutor;
use crate::solana::executor::SolanaExecutor;

/// kinesis_wallet 工具 - 真实实现
pub struct WalletTool {
    config: Config,
}

impl WalletTool {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn execute_wallet(&self, wallet_index: u32) -> Result<McpToolResult, ToolError> {
        // 获取 BSC 地址
        let bsc_addr = match self.config.get_bsc_private_key(wallet_index) {
            Ok(key) => {
                BscExecutor::new(self.config.clone(), key)
                    .await
                    .map(|e| e.wallet_address().to_string())
                    .ok()
            }
            Err(_) => None,
        };

        // 获取 Solana 地址
        let sol_addr = match self.config.get_sol_private_key(wallet_index) {
            Ok(key) => {
                SolanaExecutor::new(self.config.sol_rpc_url.clone(), &key)
                    .await
                    .map(|e| e.wallet_address().to_string())
                    .ok()
            }
            Err(_) => None,
        };

        Ok(McpToolResult::success(serde_json::json!({
            "success": true,
            "wallet_index": wallet_index,
            "bsc": bsc_addr.unwrap_or_else(|| "Not configured".to_string()),
            "solana": sol_addr.unwrap_or_else(|| "Not configured".to_string())
        })))
    }
}

#[async_trait::async_trait]
impl crate::mcp::tool_registry::ToolHandler for WalletTool {
    async fn handle(&self, args: serde_json::Value) -> Result<McpToolResult, ToolError> {
        tracing::info!("kinesis_wallet 被调用");

        let wallet_index = args
            .get("wallet_index")
            .and_then(|v| v.as_u64())
            .unwrap_or(1) as u32;

        self.execute_wallet(wallet_index).await
    }

    fn schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "wallet_index": {"type": "integer", "default": 1, "description": "钱包索引"}
            }
        })
    }
}
