// src/mcp/mod.rs

//! MCP (Model Context Protocol) 模块
//!
//! 提供基于 JSON-RPC 2.0 的 MCP 协议实现，支持 stdio 传输层。
//!
//! # 模块结构
//!
//! - `types`: 类型定义 (McpRequest, McpResponse, McpError, etc.)
//! - `errors`: 错误类型 (McpProtocolError, ToolError, ValidationError)
//! - `protocol`: JSON-RPC 协议解析和格式化
//! - `stdio_transport`: stdio 传输层实现
//! - `tool_registry`: 工具注册表和工具实现
//! - `tools`: 具体工具实现 (buy, sell, quote, etc.)

pub mod types;
pub mod errors;
pub mod protocol;
pub mod stdio_transport;
pub mod tool_registry;
pub mod tools;

// 导出常用类型
pub use types::*;
pub use errors::*;
pub use protocol::McpProtocol;
pub use stdio_transport::StdioTransport;
pub use tool_registry::{ToolRegistry, ToolHandler, register_default_tools};
pub use tool_registry::ToolDefinition;

use crate::config::Config;

/// MCP 服务
pub struct McpService {
    registry: ToolRegistry,
    transport: StdioTransport,
    config: Config,
}

impl McpService {
    pub fn new(config: Config) -> Self {
        Self {
            registry: ToolRegistry::new(),
            transport: StdioTransport::new(),
            config,
        }
    }

    /// 初始化 MCP 服务
    pub async fn initialize(&self) {
        tracing::info!("初始化 MCP 服务");

        // 注册默认工具到 transport 的 registry
        register_default_tools(self.transport.get_registry(), self.config.clone()).await;

        let tool_count = self.transport.get_registry().read().await.count().await;
        tracing::info!("已注册 {} 个工具", tool_count);
    }

    /// 启动 MCP 服务 (stdio 模式)
    pub async fn start(&self) -> crate::mcp::errors::McpResult<()> {
        tracing::info!("启动 MCP stdio 服务");
        self.transport.start().await
    }

    /// 停止 MCP 服务
    pub async fn stop(&self) {
        tracing::info!("停止 MCP 服务");
        self.transport.stop().await;
    }

    /// 获取工具注册表
    pub fn registry(&self) -> &ToolRegistry {
        &self.registry
    }
}

// Note: Default implementation removed - Config is required
// impl Default for McpService {
//     fn default() -> Self {
//         Self::new()
//     }
// }

#[cfg(test)]
mod tests {
    // Tests require full Config setup - tested separately in integration tests
}
