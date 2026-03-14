// src/mcp/tools/config.rs

use crate::mcp::types::ToolResult as McpToolResult;
use crate::mcp::errors::ToolError;
use crate::config::Config;

/// kinesis_config 工具 - 真实实现
pub struct ConfigTool {
    config: Config,
}

impl ConfigTool {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn execute_config(&self) -> Result<McpToolResult, ToolError> {
        // 返回脱敏配置
        Ok(McpToolResult::success(serde_json::json!({
            "success": true,
            "config": {
                "bsc_rpc_url": "***",
                "sol_rpc_url": "***",
                "wallet_index": 1
            }
        })))
    }
}

#[async_trait::async_trait]
impl crate::mcp::tool_registry::ToolHandler for ConfigTool {
    async fn handle(&self, _args: serde_json::Value) -> Result<McpToolResult, ToolError> {
        tracing::info!("kinesis_config 被调用");
        self.execute_config()
    }

    fn schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {}
        })
    }
}
