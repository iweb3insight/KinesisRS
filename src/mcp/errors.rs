// src/mcp/errors.rs

use thiserror::Error;

/// MCP 协议错误
#[derive(Error, Debug)]
pub enum McpProtocolError {
    #[error("JSON 解析错误：{0}")]
    ParseError(#[from] serde_json::Error),

    #[error("无效的请求格式：{0}")]
    InvalidRequest(String),

    #[error("方法不存在：{0}")]
    MethodNotFound(String),

    #[error("无效参数：{0}")]
    InvalidParams(String),

    #[error("内部错误：{0}")]
    InternalError(String),

    #[error("未授权访问")]
    Unauthorized,

    #[error("License 已过期")]
    LicenseExpired,

    #[error("超过速率限制")]
    RateLimitExceeded,

    #[error("需要 Pro 版本：{0}")]
    ProFeatureRequired(String),
}

/// 工具执行错误
#[derive(Error, Debug)]
pub enum ToolError {
    #[error("工具不存在：{0}")]
    ToolNotFound(String),

    #[error("参数验证失败：{0}")]
    ValidationError(String),

    #[error("执行失败：{0}")]
    ExecutionFailed(String),

    #[error("RPC 错误：{0}")]
    RpcError(String),

    #[error("余额不足")]
    InsufficientFunds,

    #[error("滑点超限")]
    SlippageExceeded,

    #[error("检测到蜜罐")]
    HoneypotDetected,

    #[error("交易失败：{0}")]
    TransactionFailed(String),

    #[error("代币地址无效：{0}")]
    InvalidTokenAddress(String),

    #[error("无效金额：{0}")]
    InvalidAmount(String),

    #[error("网络不支持：{0}")]
    UnsupportedChain(String),
}

impl ToolError {
    pub fn to_mcp_error(&self) -> super::types::McpError {
        match self {
            ToolError::InsufficientFunds => super::types::McpError::insufficient_funds(),
            ToolError::SlippageExceeded => super::types::McpError::slippage_exceeded(),
            ToolError::HoneypotDetected => super::types::McpError::honeypot_detected(),
            ToolError::TransactionFailed(msg) => super::types::McpError::transaction_failed(msg.clone()),
            ToolError::ValidationError(msg)
            | ToolError::ExecutionFailed(msg)
            | ToolError::RpcError(msg) => {
                super::types::McpError::new(-32603, msg.clone())
            }
            ToolError::ToolNotFound(msg) => {
                super::types::McpError::new(-32601, format!("Method not found: {}", msg))
            }
            ToolError::InvalidTokenAddress(msg)
            | ToolError::InvalidAmount(msg)
            | ToolError::UnsupportedChain(msg) => {
                super::types::McpError::new(-32602, msg.clone())
            }
        }
    }
}

/// 验证错误
#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("缺少必填字段：{0}")]
    MissingRequiredField(String),

    #[error("字段类型错误：{field}, 期望：{expected}, 实际：{actual}")]
    TypeMismatch {
        field: String,
        expected: String,
        actual: String,
    },

    #[error("字段值超出范围：{0}")]
    OutOfRange(String),

    #[error("格式错误：{0}")]
    InvalidFormat(String),
}

impl ValidationError {
    pub fn to_mcp_error(&self) -> super::types::McpError {
        match self {
            ValidationError::MissingRequiredField(field) => {
                super::types::McpError::with_data(
                    -32602,
                    "Missing required field".to_string(),
                    serde_json::json!({"field": field}),
                )
            }
            ValidationError::TypeMismatch { field, expected, actual } => {
                super::types::McpError::with_data(
                    -32602,
                    "Type mismatch".to_string(),
                    serde_json::json!({
                        "field": field,
                        "expected": expected,
                        "actual": actual
                    }),
                )
            }
            ValidationError::OutOfRange(msg) | ValidationError::InvalidFormat(msg) => {
                super::types::McpError::new(-32602, msg.clone())
            }
        }
    }
}

/// 结果类型别名
pub type McpResult<T> = Result<T, McpProtocolError>;
pub type ValidationResult<T> = Result<T, ValidationError>;
