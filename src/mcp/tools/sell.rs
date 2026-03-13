// src/mcp/tools/sell.rs

use crate::mcp::types::ToolResult as McpToolResult;
use crate::mcp::errors::ToolError;
use crate::config::Config;
use crate::bsc::executor::BscExecutor;
use crate::solana::executor::SolanaExecutor;
use alloy_primitives::utils::parse_ether;
use std::str::FromStr;

/// kinesis_sell 工具 - 真实实现
pub struct SellTool {
    config: Config,
}

impl SellTool {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    async fn execute_sell_bsc(&self, token: &str, amount: f64, slippage: f32, dry_run: bool) -> Result<McpToolResult, ToolError> {
        let bsc_key = self.config.get_bsc_private_key(1)
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to load BSC key: {}", e)))?;

        let executor = BscExecutor::new(self.config.clone(), bsc_key)
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to create executor: {}", e)))?;

        let token_addr = alloy_primitives::Address::from_str(token)
            .map_err(|_| ToolError::InvalidTokenAddress(token.to_string()))?;

        let amount_in = parse_ether(&amount.to_string())
            .map_err(|_| ToolError::InvalidAmount(format!("Invalid amount: {}", amount)))?;

        let slippage_bps = (slippage * 100.0) as u16;

        if dry_run {
            // 获取报价
            let quote_result = executor.quote_sell(token_addr, amount_in)
                .await
                .map_err(|e| ToolError::ExecutionFailed(format!("Quote failed: {}", e)))?;

            // 计算最小输出金额
            let slippage_factor = alloy_primitives::U256::from(10000u16 - slippage_bps);
            let amount_out_min = quote_result * slippage_factor / alloy_primitives::U256::from(10000u16);

            // 模拟卖出
            let simulate_result = executor.sell(token_addr, amount_in, amount_out_min, alloy_primitives::U256::from(0), true)
                .await;

            match simulate_result {
                Ok(exec_res) => Ok(McpToolResult::success(serde_json::json!({
                    "success": true,
                    "tx_hash": "SIMULATED",
                    "amount_in": amount.to_string(),
                    "amount_out": quote_result.to_string(),
                    "token": token,
                    "chain": "bsc",
                    "dry_run": true,
                    "gas_estimate": exec_res.gas_used.to_string()
                }))),
                Err(e) => Err(ToolError::ExecutionFailed(format!("Simulation failed: {}", e))),
            }
        } else {
            todo!("Real transaction implementation")
        }
    }

    async fn execute_sell_solana(&self, token: &str, amount: f64, slippage: f32, dry_run: bool) -> Result<McpToolResult, ToolError> {
        let sol_key = self.config.get_sol_private_key(1)
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to load Solana key: {}", e)))?;

        let executor = SolanaExecutor::new(self.config.sol_rpc_url.clone(), &sol_key)
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to create executor: {}", e)))?;

        // Solana 卖出金额转换为 base units (假设 6 decimals)
        let amount_base = (amount * 1_000_000.0) as u64;
        let slippage_bps = (slippage * 100.0) as u16;
        let jito_tip_lamports = None;

        if dry_run {
            match executor.sell(token, amount_base, slippage_bps, true, jito_tip_lamports).await {
                Ok(sig) => Ok(McpToolResult::success(serde_json::json!({
                    "success": true,
                    "tx_hash": sig,
                    "amount_in": amount.to_string(),
                    "token": token,
                    "chain": "solana",
                    "dry_run": true
                }))),
                Err(e) => Err(ToolError::ExecutionFailed(format!("Sell failed: {}", e))),
            }
        } else {
            todo!("Real transaction implementation")
        }
    }
}

#[async_trait::async_trait]
impl crate::mcp::tool_registry::ToolHandler for SellTool {
    async fn handle(&self, args: serde_json::Value) -> Result<McpToolResult, ToolError> {
        tracing::info!("kinesis_sell 被调用");

        let token = args
            .get("token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::ValidationError("Missing 'token' parameter".to_string()))?;

        let amount = args
            .get("amount")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| ToolError::ValidationError("Missing 'amount' parameter".to_string()))?;

        let slippage = args
            .get("slippage")
            .and_then(|v| v.as_f64())
            .unwrap_or(15.0) as f32;

        let chain = args
            .get("chain")
            .and_then(|v| v.as_str())
            .unwrap_or("solana");

        let dry_run = args
            .get("dry_run")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        if token.is_empty() {
            return Err(ToolError::InvalidTokenAddress("Token address cannot be empty".to_string()));
        }

        if amount <= 0.0 {
            return Err(ToolError::InvalidAmount("Amount must be positive".to_string()));
        }

        match chain {
            "bsc" => self.execute_sell_bsc(token, amount, slippage, dry_run).await,
            "solana" => self.execute_sell_solana(token, amount, slippage, dry_run).await,
            _ => Err(ToolError::UnsupportedChain(chain.to_string())),
        }
    }

    fn schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "token": {"type": "string", "description": "代币合约地址"},
                "amount": {"type": "number", "description": "卖出数量"},
                "slippage": {"type": "number", "default": 15, "description": "滑点容忍度 %"},
                "chain": {"type": "string", "enum": ["bsc", "solana"], "default": "solana"},
                "dry_run": {"type": "boolean", "default": true}
            },
            "required": ["token", "amount"]
        })
    }
}
