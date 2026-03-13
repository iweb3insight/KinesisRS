// src/mcp/tools/detect.rs

use crate::mcp::types::ToolResult as McpToolResult;
use crate::mcp::errors::ToolError;
use crate::config::Config;
use crate::solana::executor::SolanaExecutor;

/// kinesis_detect 工具 - 真实实现
pub struct DetectTool {
    config: Config,
}

impl DetectTool {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn execute_detect(&self, token: &str) -> Result<McpToolResult, ToolError> {
        let sol_key = self.config.get_sol_private_key(1)
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to load Solana key: {}", e)))?;

        let executor = SolanaExecutor::new(self.config.sol_rpc_url.clone(), &sol_key)
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to create executor: {}", e)))?;

        let info = executor.path_detector.detect_path(token)
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Path detection failed: {}", e)))?;

        Ok(McpToolResult::success(serde_json::json!({
            "success": true,
            "token_address": token,
            "path": format!("{:?}", info.path),
            "token_program_id": info.token_program_id.to_string(),
            "chain": "solana"
        })))
    }
}

#[async_trait::async_trait]
impl crate::mcp::tool_registry::ToolHandler for DetectTool {
    async fn handle(&self, args: serde_json::Value) -> Result<McpToolResult, ToolError> {
        tracing::info!("kinesis_detect 被调用");

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
                "token": {"type": "string", "description": "代币合约地址"},
                "chain": {"type": "string", "enum": ["solana"], "default": "solana"}
            },
            "required": ["token"]
        })
    }
}
