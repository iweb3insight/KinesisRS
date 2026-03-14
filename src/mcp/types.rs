// src/mcp/types.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MCP 请求结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpRequest {
    pub jsonrpc: String,
    pub id: serde_json::Value,
    pub method: String,
    #[serde(default)]
    pub params: serde_json::Value,
}

impl Default for McpRequest {
    fn default() -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: serde_json::Value::Null,
            method: String::new(),
            params: serde_json::Value::Null,
        }
    }
}

/// MCP 响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResponse {
    pub jsonrpc: String,
    pub id: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<McpError>,
}

impl Default for McpResponse {
    fn default() -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: serde_json::Value::Null,
            result: None,
            error: None,
        }
    }
}

impl McpResponse {
    pub fn success(id: serde_json::Value, result: serde_json::Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    pub fn error(id: serde_json::Value, error: McpError) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(error),
        }
    }
}

/// MCP 错误结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl McpError {
    pub fn new(code: i32, message: String) -> Self {
        Self {
            code,
            message,
            data: None,
        }
    }

    pub fn with_data(code: i32, message: String, data: serde_json::Value) -> Self {
        Self {
            code,
            message,
            data: Some(data),
        }
    }

    // JSON-RPC 标准错误码
    pub fn parse_error() -> Self {
        Self::new(-32700, "Parse error".to_string())
    }

    pub fn invalid_request() -> Self {
        Self::new(-32600, "Invalid Request".to_string())
    }

    pub fn method_not_found() -> Self {
        Self::new(-32601, "Method not found".to_string())
    }

    pub fn invalid_params() -> Self {
        Self::new(-32602, "Invalid params".to_string())
    }

    pub fn internal_error() -> Self {
        Self::new(-32603, "Internal error".to_string())
    }

    // MCP 自定义错误码
    pub fn unauthorized() -> Self {
        Self::new(-32000, "Unauthorized".to_string())
    }

    pub fn license_expired() -> Self {
        Self::new(-32001, "License expired".to_string())
    }

    pub fn rate_limit_exceeded() -> Self {
        Self::new(-32002, "Rate limit exceeded".to_string())
    }

    pub fn pro_feature_required(feature: &str) -> Self {
        Self::with_data(
            -32003,
            "Pro feature required".to_string(),
            serde_json::json!({
                "feature": feature,
                "required_tier": "pro"
            }),
        )
    }

    // 业务错误
    pub fn insufficient_funds() -> Self {
        Self::new(-32100, "Insufficient funds".to_string())
    }

    pub fn slippage_exceeded() -> Self {
        Self::new(-32101, "Slippage exceeded".to_string())
    }

    pub fn honeypot_detected() -> Self {
        Self::new(-32102, "Honeypot detected".to_string())
    }

    pub fn transaction_failed(message: String) -> Self {
        Self::new(-32103, message)
    }
}

/// 工具定义
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
    pub pro_required: bool,
}

/// 工具调用结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub success: bool,
    pub data: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl ToolResult {
    pub fn success(data: serde_json::Value) -> Self {
        Self {
            success: true,
            data,
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: serde_json::Value::Null,
            error: Some(message),
        }
    }
}

/// 服务器信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
    pub tier: String,
}

impl Default for ServerInfo {
    fn default() -> Self {
        Self {
            name: "kinesis-rs".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            tier: "free".to_string(),
        }
    }
}

/// 能力声明
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capabilities {
    pub tools: ToolsCapability,
    pub resources: ResourcesCapability,
}

impl Default for Capabilities {
    fn default() -> Self {
        Self {
            tools: ToolsCapability::default(),
            resources: ResourcesCapability::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolsCapability {
    #[serde(rename = "listChanged")]
    pub list_changed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourcesCapability {
    #[serde(rename = "subscribe")]
    pub subscribe: bool,
    #[serde(rename = "listChanged")]
    pub list_changed: bool,
}

/// License 等级
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LicenseTier {
    Free,
    Pro,
    Enterprise,
}

impl Default for LicenseTier {
    fn default() -> Self {
        Self::Free
    }
}

impl LicenseTier {
    pub fn can_access(&self, pro_required: bool) -> bool {
        if pro_required {
            matches!(self, LicenseTier::Pro | LicenseTier::Enterprise)
        } else {
            true
        }
    }
}
