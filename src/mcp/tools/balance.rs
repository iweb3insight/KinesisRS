// src/mcp/tools/balance.rs

use crate::mcp::types::ToolResult as McpToolResult;
use crate::mcp::errors::ToolError;
use crate::config::Config;
use crate::bsc::executor::BscExecutor;
use crate::solana::executor::SolanaExecutor;
use alloy_primitives::utils::format_ether;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// kinesis_balance 工具 - 真实实现
pub struct BalanceTool {
    config: Config,
}

impl BalanceTool {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    async fn execute_balance_bsc(&self, token: Option<&str>) -> Result<McpToolResult, ToolError> {
        let bsc_key = self.config.get_bsc_private_key(1)
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to load BSC key: {}", e)))?;

        let executor = BscExecutor::new(self.config.clone(), bsc_key)
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to create executor: {}", e)))?;

        let owner = executor.wallet_address();
        let token_addr = match token {
            Some(t) => Some(alloy_primitives::Address::from_str(t)
                .map_err(|_| ToolError::InvalidTokenAddress(t.to_string()))?),
            None => None,
        };

        let balance = executor.get_balance(owner, token_addr)
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Balance query failed: {}", e)))?;

        let formatted = format_ether(balance);
        let asset = match token {
            Some(_) => format!("Token ({})", token.unwrap()),
            None => "BNB".to_string(),
        };

        Ok(McpToolResult::success(serde_json::json!({
            "success": true,
            "balance": balance.to_string(),
            "balance_formatted": formatted,
            "asset": asset,
            "owner": owner.to_string(),
            "chain": "bsc"
        })))
    }

    async fn execute_balance_solana(&self, token: Option<&str>) -> Result<McpToolResult, ToolError> {
        let sol_key = self.config.get_sol_private_key(1)
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to load Solana key: {}", e)))?;

        let executor = SolanaExecutor::new(self.config.sol_rpc_url.clone(), &sol_key)
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to create executor: {}", e)))?;

        let owner = executor.wallet_address();
        let token_addr = match token {
            Some(t) => Some(Pubkey::from_str(t)
                .map_err(|_| ToolError::InvalidTokenAddress(t.to_string()))?),
            None => None,
        };

        let balance = executor.get_balance(owner, token_addr)
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Balance query failed: {}", e)))?;

        let (formatted, asset) = if token_addr.is_some() {
            // Token balance (assume 6 decimals)
            ((balance as f64 / 1_000_000.0).to_string(), format!("Token ({})", token.unwrap()))
        } else {
            // SOL balance
            ((balance as f64 / 1_000_000_000.0).to_string(), "SOL".to_string())
        };

        Ok(McpToolResult::success(serde_json::json!({
            "success": true,
            "balance": balance.to_string(),
            "balance_formatted": formatted,
            "asset": asset,
            "owner": owner.to_string(),
            "chain": "solana"
        })))
    }
}

#[async_trait::async_trait]
impl crate::mcp::tool_registry::ToolHandler for BalanceTool {
    async fn handle(&self, args: serde_json::Value) -> Result<McpToolResult, ToolError> {
        tracing::info!("kinesis_balance 被调用");

        let token = args.get("token").and_then(|v| v.as_str());
        let chain = args
            .get("chain")
            .and_then(|v| v.as_str())
            .unwrap_or("solana");

        match chain {
            "bsc" => self.execute_balance_bsc(token).await,
            "solana" => self.execute_balance_solana(token).await,
            _ => Err(ToolError::UnsupportedChain(chain.to_string())),
        }
    }

    fn schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "token": {"type": "string", "description": "代币合约地址 (可选)"},
                "chain": {"type": "string", "enum": ["bsc", "solana"], "default": "solana"}
            }
        })
    }
}
