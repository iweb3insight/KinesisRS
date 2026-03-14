// src/mcp/tools/quote.rs

use crate::mcp::types::ToolResult as McpToolResult;
use crate::mcp::errors::ToolError;
use crate::config::Config;
use crate::bsc::executor::BscExecutor;
use crate::solana::executor::SolanaExecutor;
use alloy_primitives::utils::parse_ether;
use std::str::FromStr;

/// kinesis_quote 工具 - 改进错误处理
pub struct QuoteTool {
    config: Config,
}

impl QuoteTool {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    async fn execute_quote_bsc(&self, token: &str, amount: f64, action: &str) -> Result<McpToolResult, ToolError> {
        let bsc_key = match self.config.get_bsc_private_key(1) {
            Ok(key) => key,
            Err(e) => {
                return Ok(McpToolResult::success(serde_json::json!({
                    "success": false,
                    "error": "configuration_error",
                    "message": format!("Failed to load BSC key: {}", e),
                    "suggestion": "Please check your BSC_PRIVATE_KEY_1 environment variable"
                })));
            }
        };

        let executor = match BscExecutor::new(self.config.clone(), bsc_key).await {
            Ok(exec) => exec,
            Err(e) => {
                return Ok(McpToolResult::success(serde_json::json!({
                    "success": false,
                    "error": "executor_creation_failed",
                    "message": format!("Failed to create executor: {}", e),
                    "suggestion": "Please check your BSC_RPC_URL configuration"
                })));
            }
        };

        let token_addr = match alloy_primitives::Address::from_str(token) {
            Ok(addr) => addr,
            Err(_) => {
                return Ok(McpToolResult::success(serde_json::json!({
                    "success": false,
                    "error": "invalid_token_address",
                    "message": format!("Invalid BSC token address: {}", token),
                    "suggestion": "Please check the token address is a valid 20-byte hex address"
                })));
            }
        };

        let amount_in = match parse_ether(&amount.to_string()) {
            Ok(amt) => amt,
            Err(_) => {
                return Ok(McpToolResult::success(serde_json::json!({
                    "success": false,
                    "error": "invalid_amount",
                    "message": format!("Invalid amount: {}", amount),
                    "suggestion": "Please provide a valid numeric amount"
                })));
            }
        };

        let result = match action {
            "buy" => executor.quote_buy(token_addr, amount_in).await,
            "sell" => executor.quote_sell(token_addr, amount_in).await,
            _ => {
                return Ok(McpToolResult::success(serde_json::json!({
                    "success": false,
                    "error": "invalid_action",
                    "message": "Invalid action. Use 'buy' or 'sell'",
                    "suggestion": "Please specify 'buy' or 'sell' as the action parameter"
                })));
            }
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
            Err(e) => {
                let error_msg = e.to_string().to_lowercase();
                
                if error_msg.contains("liquidity") || error_msg.contains("pool not found") {
                    return Ok(McpToolResult::success(serde_json::json!({
                        "success": false,
                        "error": "insufficient_liquidity",
                        "message": "Insufficient liquidity or trading pair not found",
                        "token": token,
                        "suggestion": "This token may not have a trading pair on PancakeSwap yet"
                    })));
                }
                
                if error_msg.contains("not found") {
                    return Ok(McpToolResult::success(serde_json::json!({
                        "success": false,
                        "error": "token_not_found",
                        "message": format!("Token not found: {}", e),
                        "token": token,
                        "suggestion": "Please verify the token address on BscScan"
                    })));
                }
                
                Ok(McpToolResult::success(serde_json::json!({
                    "success": false,
                    "error": "quote_failed",
                    "message": format!("Quote failed: {}", e),
                    "token": token,
                    "suggestion": "Please check token address and try again"
                })))
            }
        }
    }

    async fn execute_quote_solana(&self, token: &str, amount: f64, action: &str) -> Result<McpToolResult, ToolError> {
        let sol_key = match self.config.get_sol_private_key(1) {
            Ok(key) => key,
            Err(e) => {
                return Ok(McpToolResult::success(serde_json::json!({
                    "success": false,
                    "error": "configuration_error",
                    "message": format!("Failed to load Solana key: {}", e),
                    "suggestion": "Please check your SOL_PRIVATE_KEY_1 environment variable"
                })));
            }
        };

        let executor = match SolanaExecutor::new(self.config.sol_rpc_url.clone(), &sol_key).await {
            Ok(exec) => exec,
            Err(e) => {
                return Ok(McpToolResult::success(serde_json::json!({
                    "success": false,
                    "error": "executor_creation_failed",
                    "message": format!("Failed to create executor: {}", e),
                    "suggestion": "Please check your SOL_RPC_URL configuration"
                })));
            }
        };

        const SOL_MINT: &str = "So11111111111111111111111111111111111111112";

        let (input_mint, output_mint, amount_lamports) = match action {
            "buy" => (SOL_MINT, token, (amount * 1_000_000_000.0) as u64),
            "sell" => (token, SOL_MINT, (amount * 1_000_000.0) as u64),
            _ => {
                return Ok(McpToolResult::success(serde_json::json!({
                    "success": false,
                    "error": "invalid_action",
                    "message": "Invalid action. Use 'buy' or 'sell'",
                    "suggestion": "Please specify 'buy' or 'sell' as the action parameter"
                })));
            }
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
            Err(e) => {
                let error_msg = e.to_string().to_lowercase();
                
                // Liquidity issues
                if error_msg.contains("liquidity") || error_msg.contains("pool not found") || error_msg.contains("no routes") {
                    return Ok(McpToolResult::success(serde_json::json!({
                        "success": false,
                        "error": "insufficient_liquidity",
                        "message": "Insufficient liquidity or trading pair not found",
                        "token": token,
                        "suggestion": "This token may not have a trading pair on Raydium or Pump.fun yet"
                    })));
                }
                
                // Token not found
                if error_msg.contains("not found") || error_msg.contains("accountnotfound") {
                    return Ok(McpToolResult::success(serde_json::json!({
                        "success": false,
                        "error": "token_not_found",
                        "message": format!("Token not found: {}", e),
                        "token": token,
                        "suggestion": "Please verify the token address on Solscan or check if it's launched on pump.fun"
                    })));
                }
                
                // Network timeout
                if error_msg.contains("timeout") || error_msg.contains("timed out") {
                    return Ok(McpToolResult::success(serde_json::json!({
                        "success": false,
                        "error": "network_timeout",
                        "message": "Network request timed out",
                        "token": token,
                        "suggestion": "Please check your network connection or try again later"
                    })));
                }
                
                // Slippage issues
                if error_msg.contains("slippage") {
                    return Ok(McpToolResult::success(serde_json::json!({
                        "success": false,
                        "error": "slippage_too_high",
                        "message": "Price slippage too high for this token",
                        "token": token,
                        "suggestion": "This token has high price volatility. Consider a smaller amount."
                    })));
                }
                
                // Generic quote failure
                Ok(McpToolResult::success(serde_json::json!({
                    "success": false,
                    "error": "quote_failed",
                    "message": format!("Quote failed: {}", e),
                    "token": token,
                    "suggestion": "Please check token address and try again, or verify on pump.fun directly"
                })))
            }
        }
    }
}

#[async_trait::async_trait]
impl crate::mcp::tool_registry::ToolHandler for QuoteTool {
    async fn handle(&self, args: serde_json::Value) -> Result<McpToolResult, ToolError> {
        tracing::info!("kinesis_quote called");

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
            _ => Ok(McpToolResult::success(serde_json::json!({
                "success": false,
                "error": "unsupported_chain",
                "message": format!("Unsupported chain: {}", chain),
                "suggestion": "Please use 'bsc' or 'solana' as the chain parameter"
            })))
        }
    }

    fn schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "token": {"type": "string", "description": "Token contract address"},
                "amount": {"type": "number", "description": "Amount to trade"},
                "action": {"type": "string", "enum": ["buy", "sell"], "default": "buy"},
                "chain": {"type": "string", "enum": ["bsc", "solana"], "default": "solana"}
            },
            "required": ["token", "amount"]
        })
    }
}
