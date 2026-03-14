// src/mcp/tools/detect.rs

use crate::mcp::types::ToolResult as McpToolResult;
use crate::mcp::errors::ToolError;
use crate::config::Config;
use crate::solana::executor::SolanaExecutor;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// kinesis_detect 工具 - 改进错误处理
pub struct DetectTool {
    config: Config,
}

impl DetectTool {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn execute_detect(&self, token: &str) -> Result<McpToolResult, ToolError> {
        // Validate token address format first
        if Pubkey::from_str(token).is_err() {
            return Ok(McpToolResult::success(serde_json::json!({
                "success": false,
                "error": "invalid_token_address",
                "message": format!("Invalid Solana address format: {}", token),
                "suggestion": "Please check the token address is a valid base58-encoded Solana pubkey"
            })));
        }

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

        match executor.path_detector.detect_path(token).await {
            Ok(info) => {
                Ok(McpToolResult::success(serde_json::json!({
                    "success": true,
                    "token_address": token,
                    "path": format!("{:?}", info.path),
                    "token_program_id": info.token_program_id.to_string(),
                    "chain": "solana"
                })))
            }
            Err(e) => {
                let error_msg = e.to_string().to_lowercase();
                
                // Token account not found
                if error_msg.contains("account not found") || error_msg.contains("not found") {
                    return Ok(McpToolResult::success(serde_json::json!({
                        "success": false,
                        "error": "token_not_found",
                        "message": format!("Token account not found: {}", token),
                        "suggestion": "The token may not be launched yet, or the address is incorrect. Check on pump.fun or Solscan."
                    })));
                }
                
                // Network timeout
                if error_msg.contains("timeout") || error_msg.contains("timed out") {
                    return Ok(McpToolResult::success(serde_json::json!({
                        "success": false,
                        "error": "network_timeout",
                        "message": "Network request timed out",
                        "suggestion": "Please check your network connection or try again later"
                    })));
                }
                
                // RPC error
                if error_msg.contains("rpc") {
                    return Ok(McpToolResult::success(serde_json::json!({
                        "success": false,
                        "error": "rpc_error",
                        "message": format!("RPC error: {}", e),
                        "suggestion": "Please check your RPC node configuration"
                    })));
                }
                
                // Generic detection failure
                Ok(McpToolResult::success(serde_json::json!({
                    "success": false,
                    "error": "detection_failed",
                    "message": format!("Path detection failed: {}", e),
                    "suggestion": "This token may not be tradable or has restrictions. Verify on pump.fun directly."
                })))
            }
        }
    }
}

#[async_trait::async_trait]
impl crate::mcp::tool_registry::ToolHandler for DetectTool {
    async fn handle(&self, args: serde_json::Value) -> Result<McpToolResult, ToolError> {
        tracing::info!("kinesis_detect called");

        let token = args
            .get("token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::ValidationError("Missing 'token' parameter".to_string()))?;

        if token.is_empty() {
            return Err(ToolError::InvalidTokenAddress("Token address cannot be empty".to_string()));
        }

        self.execute_detect(token).await
    }

    fn schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "token": {"type": "string", "description": "Token contract address"},
                "chain": {"type": "string", "enum": ["solana"], "default": "solana"}
            },
            "required": ["token"]
        })
    }
}
