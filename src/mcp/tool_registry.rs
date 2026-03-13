// src/mcp/tool_registry.rs

use crate::mcp::types::*;
use crate::mcp::errors::*;
use crate::config::Config;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// 明确使用 types::ToolResult 而不是 std::result::Result
use crate::mcp::types::ToolResult as McpToolResult;
// 导出 ToolDefinition
pub use crate::mcp::types::ToolDefinition;

// 导入真实工具
use crate::mcp::tools::{BuyTool, SellTool, QuoteTool, BalanceTool, DetectTool, ConfigTool, WalletTool};

/// 工具处理器 trait
#[async_trait::async_trait]
pub trait ToolHandler: Send + Sync {
    async fn handle(&self, args: serde_json::Value) -> Result<McpToolResult, ToolError>;
    fn schema(&self) -> serde_json::Value;
}

/// 工具注册表
pub struct ToolRegistry {
    tools: Arc<RwLock<HashMap<String, Arc<dyn ToolHandler>>>>,
    definitions: Arc<RwLock<HashMap<String, ToolDefinition>>>,
    license_tier: Arc<RwLock<LicenseTier>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: Arc::new(RwLock::new(HashMap::new())),
            definitions: Arc::new(RwLock::new(HashMap::new())),
            license_tier: Arc::new(RwLock::new(LicenseTier::Free)),
        }
    }

    /// 设置 License 等级
    pub async fn set_license_tier(&self, tier: LicenseTier) {
        let mut current = self.license_tier.write().await;
        *current = tier;
    }

    /// 获取当前 License 等级
    pub async fn get_license_tier(&self) -> LicenseTier {
        *self.license_tier.read().await
    }

    /// 注册工具
    pub async fn register(&self, name: String, handler: Arc<dyn ToolHandler>, definition: ToolDefinition) {
        let mut tools = self.tools.write().await;
        let mut definitions = self.definitions.write().await;

        tools.insert(name.clone(), handler);
        definitions.insert(name, definition);

        tracing::info!("工具已注册");
    }

    /// 列出所有可用工具
    pub async fn list_tools(&self) -> Vec<ToolDefinition> {
        let tier = self.get_license_tier().await;
        let definitions = self.definitions.read().await;

        definitions
            .values()
            .filter(|def| tier.can_access(def.pro_required))
            .cloned()
            .collect()
    }

    /// 调用工具
    pub async fn call_tool(&self, name: &str, args: serde_json::Value) -> Result<McpToolResult, McpProtocolError> {
        // 获取 handler 的 Arc 引用
        let handler = {
            let tools = self.tools.read().await;
            let definitions = self.definitions.read().await;
            let tier = self.get_license_tier().await;

            // 检查工具是否存在
            let handler = tools
                .get(name)
                .ok_or_else(|| McpProtocolError::MethodNotFound(name.to_string()))?
                .clone();

            // 检查权限
            let def = definitions
                .get(name)
                .ok_or_else(|| McpProtocolError::InternalError("Tool definition not found".to_string()))?;

            if !tier.can_access(def.pro_required) {
                return Err(McpProtocolError::ProFeatureRequired(name.to_string()));
            }

            handler
        };

        // 调用工具
        match handler.handle(args).await {
            Ok(result) => Ok(result),
            Err(e) => {
                tracing::error!(tool = %name, error = %e, "工具执行失败");
                Err(McpProtocolError::InternalError(e.to_string()))
            }
        }
    }

    /// 获取工具数量
    pub async fn count(&self) -> usize {
        self.tools.read().await.len()
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// 注册所有基础工具
pub async fn register_default_tools(registry: Arc<RwLock<ToolRegistry>>, config: Config) {
    // kinesis_buy
    {
        let reg = registry.write().await;
        reg.register(
            "kinesis_buy".to_string(),
            Arc::new(BuyTool::new(config.clone())),
            ToolDefinition {
                name: "kinesis_buy".to_string(),
                description: "买入代币".to_string(),
                input_schema: BuyTool::new(config.clone()).schema(),
                pro_required: false,
            },
        )
        .await;
    }

    // kinesis_sell
    {
        let reg = registry.write().await;
        reg.register(
            "kinesis_sell".to_string(),
            Arc::new(SellTool::new(config.clone())),
            ToolDefinition {
                name: "kinesis_sell".to_string(),
                description: "卖出代币".to_string(),
                input_schema: SellTool::new(config.clone()).schema(),
                pro_required: false,
            },
        )
        .await;
    }

    // kinesis_quote
    {
        let reg = registry.write().await;
        reg.register(
            "kinesis_quote".to_string(),
            Arc::new(QuoteTool::new(config.clone())),
            ToolDefinition {
                name: "kinesis_quote".to_string(),
                description: "获取报价".to_string(),
                input_schema: QuoteTool::new(config.clone()).schema(),
                pro_required: false,
            },
        )
        .await;
    }

    // kinesis_balance
    {
        let reg = registry.write().await;
        reg.register(
            "kinesis_balance".to_string(),
            Arc::new(BalanceTool::new(config.clone())),
            ToolDefinition {
                name: "kinesis_balance".to_string(),
                description: "查询余额".to_string(),
                input_schema: BalanceTool::new(config.clone()).schema(),
                pro_required: false,
            },
        )
        .await;
    }

    // kinesis_detect
    {
        let reg = registry.write().await;
        reg.register(
            "kinesis_detect".to_string(),
            Arc::new(DetectTool::new(config.clone())),
            ToolDefinition {
                name: "kinesis_detect".to_string(),
                description: "检测代币路径".to_string(),
                input_schema: DetectTool::new(config.clone()).schema(),
                pro_required: false,
            },
        )
        .await;
    }

    // kinesis_config
    {
        let reg = registry.write().await;
        reg.register(
            "kinesis_config".to_string(),
            Arc::new(ConfigTool::new(config.clone())),
            ToolDefinition {
                name: "kinesis_config".to_string(),
                description: "查看配置".to_string(),
                input_schema: ConfigTool::new(config.clone()).schema(),
                pro_required: false,
            },
        )
        .await;
    }

    // kinesis_wallet
    {
        let reg = registry.write().await;
        reg.register(
            "kinesis_wallet".to_string(),
            Arc::new(WalletTool::new(config.clone())),
            ToolDefinition {
                name: "kinesis_wallet".to_string(),
                description: "查看钱包地址".to_string(),
                input_schema: WalletTool::new(config.clone()).schema(),
                pro_required: false,
            },
        )
        .await;
    }

    tracing::info!("已注册 7 个基础工具");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tool_registry_creation() {
        let registry = ToolRegistry::new();
        let tool_count = registry.count().await;
        assert_eq!(tool_count, 0);
    }

    #[tokio::test]
    async fn test_license_tier() {
        let registry = ToolRegistry::new();
        
        // Default is Free
        assert_eq!(registry.get_license_tier().await, LicenseTier::Free);
        
        // Set to Pro
        registry.set_license_tier(LicenseTier::Pro).await;
        assert_eq!(registry.get_license_tier().await, LicenseTier::Pro);
    }

    // Integration tests with real tools are in tests/mcp_integration_test.rs
}
