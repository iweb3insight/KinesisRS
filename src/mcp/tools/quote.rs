// src/mcp/tools/quote.rs

use crate::mcp::types::ToolResult as McpToolResult;
use crate::mcp::errors::ToolError;
use crate::config::Config;
use crate::bsc::executor::BscExecutor;
use crate::solana::executor::SolanaExecutor;
use alloy_primitives::utils::parse_ether;
use std::str::FromStr;

/// kinesis_quote 工具 - 真实实现
pub struct QuoteTool {
    config: Config,
}

impl QuoteTool {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    async fn execute_quote_bsc(&self, token: &str, amount: f64, action: &str) -> Result<McpToolResult, ToolError> {
        let bsc_key = self.config.get_bsc_private_key(1)
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to load BSC key: {}", e)))?;

        let executor = BscExecutor::new(self.config.clone(), bsc_key)
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to create executor: {}", e)))?;

        let token_addr = alloy_primitives::Address::from_str(token)
            .map_err(|_| ToolError::InvalidTokenAddress(token.to_string()))?;

        let amount_in = parse_ether(&amount.to_string())
            .map_err(|_| ToolError::InvalidAmount(format!("Invalid amount: {}", amount)))?;

        let result = match action {
            "buy" => executor.quote_buy(token_addr, amount_in).await,
            "sell" => executor.quote_sell(token_addr, amount_in).await,
            _ => return Err(ToolError::ValidationError("Invalid action. Use 'buy' or 'sell'".to_string())),
        };

        match result {
            Ok(amount_out) => Ok(McpToolResult::success(serde_json::json!({
                "success": true,
                "amount_out": amount_out.to_string(),
                "action": action,
                "input_amount": amount.to_string(),
                "token": token,
                "chain": "bsc"
            }))),
            Err(e) => Err(ToolError::ExecutionFailed(format!("Quote failed: {}", e))),
        }
    }

    async fn execute_quote_solana(&self, token: &str, amount: f64, action: &str) -> Result<McpToolResult, ToolError> {
        let sol_key = self.config.get_sol_private_key(1)
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to load Solana key: {}", e)))?;

        let executor = SolanaExecutor::new(self.config.sol_rpc_url.clone(), &sol_key)
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to create executor: {}", e)))?;

        const SOL_MINT: &str = "So11111111111111111111111111111111111111112";

        let (input_mint, output_mint, amount_lamports) = match action {
            "buy" => (SOL_MINT, token, (amount * 1_000_000_000.0) as u64),
            "sell" => (token, SOL_MINT, (amount * 1_000_000.0) as u64),
            _ => return Err(ToolError::ValidationError("Invalid action. Use 'buy' or 'sell'".to_string())),
        };

        match executor.quote(input_mint, output_mint, amount_lamports).await {
            Ok((amount_out, path)) => Ok(McpToolResult::success(serde_json::json!({
                "success": true,
                "amount_out": amount_out.to_string(),
                "path": format!("{:?}", path),
                "action": action,
                "input_amount": amount.to_string(),
                "token": token,
                "chain": "solana"
            }))),
            Err(e) => Err(ToolError::ExecutionFailed(format!("Quote failed: {}", e))),
        }
    }
}

#[async_trait::async_trait]
impl crate::mcp::tool_registry::ToolHandler for QuoteTool {
    async fn handle(&self, args: serde_json::Value) -> Result<McpToolResult, ToolError> {
        tracing::info!("kinesis_quote 被调用");

        let token = args
            .get("token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::ValidationError("Missing 'token' parameter".to_string()))?;

        let amount = args
            .get("amount")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| ToolError::ValidationError("Missing 'amount' parameter".to_string()))?;

        let action = args
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("buy");

        let chain = args
            .get("chain")
            .and_then(|v| v.as_str())
            .unwrap_or("solana");

        if token.is_empty() {
            return Err(ToolError::InvalidTokenAddress("Token address cannot be empty".to_string()));
        }

        if amount <= 0.0 {
            return Err(ToolError::InvalidAmount("Amount must be positive".to_string()));
        }

        match chain {
            "bsc" => self.execute_quote_bsc(token, amount, action).await,
            "solana" => self.execute_quote_solana(token, amount, action).await,
            _ => Err(ToolError::UnsupportedChain(chain.to_string())),
        }
    }

    fn schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "token": {"type": "string", "description": "代币合约地址"},
                "amount": {"type": "number", "description": "数量"},
                "action": {"type": "string", "enum": ["buy", "sell"], "default": "buy"},
                "chain": {"type": "string", "enum": ["bsc", "solana"], "default": "solana"}
            },
            "required": ["token", "amount"]
        })
    }
}
