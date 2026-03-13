// src/mcp/stdio_transport.rs

use crate::mcp::types::*;
use crate::mcp::protocol::McpProtocol;
use crate::mcp::errors::*;
use crate::mcp::tool_registry::ToolRegistry;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

/// stdio 传输层处理器
pub struct StdioTransport {
    protocol: Arc<Mutex<McpProtocol>>,
    running: Arc<Mutex<bool>>,
    registry: Arc<RwLock<ToolRegistry>>,
}

impl StdioTransport {
    pub fn new() -> Self {
        Self {
            protocol: Arc::new(Mutex::new(McpProtocol::new())),
            running: Arc::new(Mutex::new(false)),
            registry: Arc::new(RwLock::new(ToolRegistry::new())),
        }
    }

    /// 启动 stdio 服务
    pub async fn start(&self) -> McpResult<()> {
        let mut running = self.running.lock().await;
        *running = true;
        drop(running);

        tracing::info!("MCP stdio 服务启动");

        let stdin = tokio::io::stdin();
        let mut reader = BufReader::new(stdin);
        let mut line = String::new();

        let protocol = self.protocol.clone();
        let running = self.running.clone();

        loop {
            // 检查是否仍在运行
            {
                let is_running = running.lock().await;
                if !*is_running {
                    tracing::info!("MCP stdio 服务停止");
                    break;
                }
            }

            // 读取一行输入
            line.clear();
            let bytes_read = match reader.read_line(&mut line).await {
                Ok(0) => break, // EOF
                Ok(n) => n,
                Err(e) => {
                    tracing::error!(error = %e, "读取 stdin 失败");
                    continue;
                }
            };

            if bytes_read == 0 {
                continue;
            }

            let input = line.trim();
            if input.is_empty() {
                continue;
            }

            tracing::debug!(input = %input, "收到 MCP 请求");

            // 处理请求
            let response = {
                let protocol = protocol.lock().await;
                self.handle_request(&protocol, input).await
            };

            // 写入响应
            let mut stdout = tokio::io::stdout();
            if let Err(e) = stdout.write_all(response.as_bytes()).await {
                tracing::error!(error = %e, "写入 stdout 失败");
                continue;
            }
            if let Err(e) = stdout.write_all(b"\n").await {
                tracing::error!(error = %e, "写入换行符失败");
                continue;
            }
            if let Err(e) = stdout.flush().await {
                tracing::error!(error = %e, "flush 失败");
            }
        }

        Ok(())
    }

    /// 停止服务
    pub async fn stop(&self) {
        let mut running = self.running.lock().await;
        *running = false;
        tracing::info!("MCP stdio 服务停止中...");
    }

    /// 获取工具注册表
    pub fn get_registry(&self) -> Arc<RwLock<ToolRegistry>> {
        self.registry.clone()
    }

    /// 处理单个请求
    async fn handle_request(&self, protocol: &McpProtocol, input: &str) -> String {
        // 解析请求
        let request = match protocol.parse_request(input) {
            Ok(req) => req,
            Err(e) => {
                let error = match e {
                    McpProtocolError::ParseError(_) => McpError::parse_error(),
                    McpProtocolError::InvalidRequest(msg) => {
                        McpError::with_data(-32600, "Invalid Request".to_string(), serde_json::json!({"reason": msg}))
                    }
                    _ => McpError::internal_error(),
                };
                return protocol.format_error(serde_json::Value::Null, error);
            }
        };

        // 路由到对应处理器
        let result = match request.method.as_str() {
            "initialize" => self.handle_initialize(protocol, &request.params).await,
            "tools/list" => self.handle_tools_list(protocol, &request.params).await,
            "tools/call" => self.handle_tools_call(protocol, &request.params).await,
            "ping" => Ok(serde_json::json!({})),
            _ => Err(McpProtocolError::MethodNotFound(request.method.clone())),
        };

        // 格式化响应
        match result {
            Ok(data) => protocol.format_response(request.id, data),
            Err(e) => {
                let error = match e {
                    McpProtocolError::MethodNotFound(_) => McpError::method_not_found(),
                    McpProtocolError::InvalidParams(msg) => {
                        McpError::with_data(-32602, "Invalid params".to_string(), serde_json::json!({"reason": msg}))
                    }
                    _ => McpError::internal_error(),
                };
                protocol.format_error(request.id, error)
            }
        }
    }

    /// 处理 initialize 方法
    async fn handle_initialize(&self, protocol: &McpProtocol, params: &serde_json::Value) -> McpResult<serde_json::Value> {
        Ok(protocol.handle_initialize(params))
    }

    /// 处理 tools/list 方法
    async fn handle_tools_list(&self, _protocol: &McpProtocol, _params: &serde_json::Value) -> McpResult<serde_json::Value> {
        let registry = self.registry.read().await;
        let tools = registry.list_tools().await;
        Ok(serde_json::json!({
            "tools": tools
        }))
    }

    /// 处理 tools/call 方法
    async fn handle_tools_call(&self, _protocol: &McpProtocol, params: &serde_json::Value) -> McpResult<serde_json::Value> {
        let name = params
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| McpProtocolError::InvalidParams("Missing 'name' parameter".to_string()))?;

        let arguments = params.get("arguments").unwrap_or(&serde_json::Value::Null);

        tracing::info!(tool = %name, "调用工具");

        let registry = self.registry.read().await;
        match registry.call_tool(name, arguments.clone()).await {
            Ok(result) => {
                Ok(serde_json::json!({
                    "success": true,
                    "data": result.data
                }))
            }
            Err(e) => {
                Err(McpProtocolError::InternalError(format!("Tool execution failed: {}", e)))
            }
        }
    }
}

impl Default for StdioTransport {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_stdio_transport_creation() {
        let transport = StdioTransport::new();
        assert!(transport.running.try_lock().is_ok());
    }

    #[tokio::test]
    async fn test_handle_initialize() {
        let transport = StdioTransport::new();
        let protocol = McpProtocol::new();
        let params = json!({
            "protocol_version": "1.0",
            "client_info": {"name": "test"}
        });

        let result = transport.handle_initialize(&protocol, &params).await;
        assert!(result.is_ok());
    }
}
