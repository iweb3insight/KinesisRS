// src/mcp/tools/buy.rs

use crate::mcp::types::ToolResult as McpToolResult;
use crate::mcp::errors::ToolError;
use crate::config::Config;
use crate::bsc::executor::BscExecutor;
use crate::solana::executor::SolanaExecutor;
use alloy_primitives::utils::parse_ether;
use std::str::FromStr;

/// kinesis_buy 工具 - 真实实现
pub struct BuyTool {
    config: Config,
}

impl BuyTool {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    async fn execute_buy_bsc(&self, token: &str, amount: f64, slippage: f32, dry_run: bool) -> Result<McpToolResult, ToolError> {
        // 加载 BSC 私钥
        let bsc_key = self.config.get_bsc_private_key(1)
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to load BSC key: {}", e)))?;

        // 创建执行器
        let executor = BscExecutor::new(self.config.clone(), bsc_key)
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to create executor: {}", e)))?;

        // 解析代币地址
        let token_addr = alloy_primitives::Address::from_str(token)
            .map_err(|_| ToolError::InvalidTokenAddress(token.to_string()))?;

        // 解析金额
        let amount_in = parse_ether(&amount.to_string())
            .map_err(|_| ToolError::InvalidAmount(format!("Invalid amount: {}", amount)))?;

        // 计算滑点
        let slippage_bps = (slippage * 100.0) as u16;
        
        if dry_run {
            // Dry-run 模式：获取报价并模拟
            let quote_result = executor.quote_buy(token_addr, amount_in)
                .await
                .map_err(|e| ToolError::ExecutionFailed(format!("Quote failed: {}", e)))?;

            // 计算最小输出金额 (使用 U256 运算)
            let slippage_factor = alloy_primitives::U256::from(10000u16 - slippage_bps);
            let amount_out_min = quote_result * slippage_factor / alloy_primitives::U256::from(10000u16);

            let simulate_result = executor.buy(token_addr, amount_in, amount_out_min, alloy_primitives::U256::from(0), true)
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
                    "gas_estimate": exec_res.gas_used.to_string(),
                    "message": exec_res.message
                }))),
                Err(e) => Err(ToolError::ExecutionFailed(format!("Simulation failed: {}", e))),
            }
        } else {
            // 真实交易模式
            todo!("Real transaction implementation")
        }
    }

    async fn execute_buy_solana(&self, token: &str, amount: f64, slippage: f32, dry_run: bool) -> Result<McpToolResult, ToolError> {
        // 加载 Solana 私钥
        let sol_key = self.config.get_sol_private_key(1)
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to load Solana key: {}", e)))?;

        // 创建执行器
        let executor = SolanaExecutor::new(self.config.sol_rpc_url.clone(), &sol_key)
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to create executor: {}", e)))?;

        // 计算 Jito tip (lamports)
        let jito_tip_lamports = None;

        if dry_run {
            // Dry-run 模式
            let slippage_bps = (slippage * 100.0) as u16;
            
            match executor.buy(token, amount, slippage_bps, true, jito_tip_lamports).await {
                Ok(sig) => Ok(McpToolResult::success(serde_json::json!({
                    "success": true,
                    "tx_hash": sig,
                    "amount_in": amount.to_string(),
                    "token": token,
                    "chain": "solana",
                    "dry_run": true
                }))),
                Err(e) => Err(ToolError::ExecutionFailed(format!("Buy failed: {}", e))),
            }
        } else {
            // 真实交易模式
            todo!("Real transaction implementation")
        }
    }
}

#[async_trait::async_trait]
impl crate::mcp::tool_registry::ToolHandler for BuyTool {
    async fn handle(&self, args: serde_json::Value) -> Result<McpToolResult, ToolError> {
        tracing::info!("kinesis_buy 被调用");

        // 解析参数
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

        // 验证参数
        if token.is_empty() {
            return Err(ToolError::InvalidTokenAddress("Token address cannot be empty".to_string()));
        }

        if amount <= 0.0 {
            return Err(ToolError::InvalidAmount("Amount must be positive".to_string()));
        }

        if slippage < 0.0 || slippage > 100.0 {
            return Err(ToolError::ValidationError("Slippage must be between 0 and 100".to_string()));
        }

        // 根据链执行
        match chain {
            "bsc" => self.execute_buy_bsc(token, amount, slippage, dry_run).await,
            "solana" => self.execute_buy_solana(token, amount, slippage, dry_run).await,
            _ => Err(ToolError::UnsupportedChain(chain.to_string())),
        }
    }

    fn schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "token": {"type": "string", "description": "代币合约地址"},
                "amount": {"type": "number", "description": "买入金额 (SOL/BNB)"},
                "slippage": {"type": "number", "default": 15, "description": "滑点容忍度 %"},
                "chain": {"type": "string", "enum": ["bsc", "solana"], "default": "solana"},
                "dry_run": {"type": "boolean", "default": true}
            },
            "required": ["token", "amount"]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Integration tests are in tests/mcp_integration_test.rs
}
