// src/mcp/protocol.rs

use crate::mcp::types::*;
use crate::mcp::errors::*;
use serde_json::Value;

/// MCP 协议处理器
pub struct McpProtocol {
    version: String,
    server_info: ServerInfo,
}

impl Default for McpProtocol {
    fn default() -> Self {
        Self::new()
    }
}

impl McpProtocol {
    pub fn new() -> Self {
        Self {
            version: "1.0".to_string(),
            server_info: ServerInfo::default(),
        }
    }

    /// 解析 JSON-RPC 请求
    pub fn parse_request(&self, raw: &str) -> McpResult<McpRequest> {
        // 解析 JSON
        let request: McpRequest = serde_json::from_str(raw).map_err(|e| {
            tracing::error!(error = %e, "JSON 解析失败");
            McpProtocolError::ParseError(e)
        })?;

        // 验证 jsonrpc 版本
        if request.jsonrpc != "2.0" {
            tracing::warn!(version = %request.jsonrpc, "不支持的 JSON-RPC 版本");
            return Err(McpProtocolError::InvalidRequest(
                "Only JSON-RPC 2.0 is supported".to_string(),
            ));
        }

        // 验证方法存在
        if request.method.is_empty() {
            return Err(McpProtocolError::InvalidRequest(
                "Method is required".to_string(),
            ));
        }

        tracing::debug!(
            method = %request.method,
            id = ?request.id,
            "MCP 请求解析成功"
        );

        Ok(request)
    }

    /// 格式化成功响应
    pub fn format_response(&self, id: Value, result: Value) -> String {
        let response = McpResponse::success(id, result);
        serde_json::to_string(&response).unwrap_or_else(|e| {
            tracing::error!(error = %e, "响应序列化失败");
            r#"{"jsonrpc":"2.0","id":null,"error":{"code":-32603,"message":"Internal error"}}"#.to_string()
        })
    }

    /// 格式化错误响应
    pub fn format_error(&self, id: Value, error: McpError) -> String {
        let response = McpResponse::error(id, error);
        serde_json::to_string(&response).unwrap_or_else(|e| {
            tracing::error!(error = %e, "错误响应序列化失败");
            r#"{"jsonrpc":"2.0","id":null,"error":{"code":-32603,"message":"Internal error"}}"#.to_string()
        })
    }

    /// 获取服务器信息
    pub fn server_info(&self) -> &ServerInfo {
        &self.server_info
    }

    /// 获取协议版本
    pub fn version(&self) -> &str {
        &self.version
    }

    /// 处理 initialize 方法
    pub fn handle_initialize(&self, params: &Value) -> Value {
        tracing::info!("处理 initialize 请求");

        let client_info = params.get("client_info").unwrap_or(&Value::Null);
        tracing::debug!(client_info = ?client_info, "客户端信息");

        serde_json::json!({
            "protocol_version": self.version,
            "server_info": self.server_info,
            "capabilities": {
                "tools": {
                    "listChanged": true
                },
                "resources": {
                    "subscribe": false,
                    "listChanged": false
                }
            }
        })
    }

    /// 处理 tools/list 方法
    pub fn handle_tools_list(&self, tools: &[ToolDefinition]) -> Value {
        tracing::debug!(count = tools.len(), "列出工具");

        serde_json::json!({
            "tools": tools
        })
    }

    /// 处理 tools/call 方法结果
    pub fn handle_tools_call(&self, result: ToolResult) -> Value {
        if result.success {
            serde_json::json!({
                "success": true,
                "data": result.data
            })
        } else {
            serde_json::json!({
                "success": false,
                "error": result.error.unwrap_or_else(|| "Unknown error".to_string())
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_valid_request() {
        let protocol = McpProtocol::new();
        let raw = r#"{
            "jsonrpc": "2.0",
            "id": "test-001",
            "method": "tools/call",
            "params": {"name": "kinesis_buy"}
        }"#;

        let result = protocol.parse_request(raw);
        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.id, json!("test-001"));
        assert_eq!(request.method, "tools/call");
    }

    #[test]
    fn test_parse_invalid_json() {
        let protocol = McpProtocol::new();
        let raw = r#"{"jsonrpc": "2.0", invalid}"#;

        let result = protocol.parse_request(raw);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            McpProtocolError::ParseError(_)
        ));
    }

    #[test]
    fn test_parse_invalid_version() {
        let protocol = McpProtocol::new();
        let raw = r#"{
            "jsonrpc": "1.0",
            "id": "1",
            "method": "test"
        }"#;

        let result = protocol.parse_request(raw);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            McpProtocolError::InvalidRequest(_)
        ));
    }

    #[test]
    fn test_parse_missing_method() {
        let protocol = McpProtocol::new();
        let raw = r#"{
            "jsonrpc": "2.0",
            "id": "1"
        }"#;

        let result = protocol.parse_request(raw);
        // 缺少 method 字段会被 serde_json 解析为无效 JSON
        assert!(result.is_err());
        // 可能是 ParseError 或 InvalidRequest
        match result.unwrap_err() {
            McpProtocolError::ParseError(_) | McpProtocolError::InvalidRequest(_) => {},
            _ => panic!("Expected ParseError or InvalidRequest"),
        }
    }

    #[test]
    fn test_format_response() {
        let protocol = McpProtocol::new();
        let result = protocol.format_response(json!("1"), json!({"success": true}));

        assert!(result.contains(r#""jsonrpc":"2.0""#));
        assert!(result.contains(r#""id":"1""#));
        assert!(result.contains(r#""success":true"#));
        assert!(!result.contains("error"));
    }

    #[test]
    fn test_format_error() {
        let protocol = McpProtocol::new();
        let error = McpError::invalid_request();
        let result = protocol.format_error(json!("1"), error);

        assert!(result.contains(r#""jsonrpc":"2.0""#));
        assert!(result.contains(r#""code":-32600"#));
        assert!(result.contains(r#""Invalid Request""#));
    }

    #[test]
    fn test_handle_initialize() {
        let protocol = McpProtocol::new();
        let params = json!({
            "protocol_version": "1.0",
            "client_info": {"name": "test-agent", "version": "1.0.0"}
        });

        let result = protocol.handle_initialize(&params);
        assert!(result.get("protocol_version").is_some());
        assert!(result.get("server_info").is_some());
        assert!(result.get("capabilities").is_some());
    }
}
