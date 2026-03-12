use std::env;
use thiserror::Error;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Config {
    pub bsc_rpc_urls: Vec<String>,
    pub sol_rpc_url: String,
    pub bsc_router_address: Option<String>,
    pub https_proxy: Option<String>,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingVar(String),
}

impl Config {
    /// Loads core configuration from environment variables.
    pub fn load() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();

        let bsc_rpc_raw = env::var("BSC_RPC_URL")
            .map_err(|_| ConfigError::MissingVar("BSC_RPC_URL".to_string()))?;
        
        let bsc_rpc_urls: Vec<String> = bsc_rpc_raw
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if bsc_rpc_urls.is_empty() {
            return Err(ConfigError::MissingVar("BSC_RPC_URL must contain at least one valid URL".to_string()));
        }
        
        let sol_rpc_url = env::var("SOL_RPC_URL")
            .map_err(|_| ConfigError::MissingVar("SOL_RPC_URL".to_string()))?;

        let bsc_router_address = env::var("BSC_ROUTER_ADDRESS").ok();

        let https_proxy = env::var("HTTPS_PROXY").ok();

        Ok(Self {
            bsc_rpc_urls,
            sol_rpc_url,
            bsc_router_address,
            https_proxy,
        })
    }

    /// Dynamically loads a BSC private key based on the wallet index.
    pub fn get_bsc_private_key(&self, index: u32) -> Result<String, ConfigError> {
        let var_name = if index <= 1 {
            "BSC_PRIVATE_KEY_1".to_string()
        } else {
            format!("BSC_PRIVATE_KEY_{}", index)
        };

        env::var(&var_name).map_err(|_| ConfigError::MissingVar(var_name))
    }

    /// Dynamically loads a Solana private key based on the wallet index.
    pub fn get_sol_private_key(&self, index: u32) -> Result<String, ConfigError> {
        let var_name = if index <= 1 {
            "SOL_PRIVATE_KEY_1".to_string()
        } else {
            format!("SOL_PRIVATE_KEY_{}", index)
        };

        env::var(&var_name).map_err(|_| ConfigError::MissingVar(var_name))
    }
}
