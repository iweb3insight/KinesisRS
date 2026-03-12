use clap::{Parser, Subcommand};
use crate::types::Chain;
use serde::Serialize;

#[derive(Parser, Debug, Serialize)]
#[command(author, version, about, long_about = "FreedomAgent Agentic CLI - A stateless, JSON-first, multi-chain crypto trading execution layer.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Global flag to force JSON output for agent consumption.
    #[arg(long, global = true, default_value_t = false)]
    pub json: bool,

    /// Global flag to simulate transactions without sending them.
    #[arg(long, global = true, default_value_t = true, action = clap::ArgAction::SetTrue)]
    pub dry_run: bool,

    /// Global flag to disable simulation and send the real transaction.
    #[arg(long, global = true, default_value_t = false, action = clap::ArgAction::SetTrue)]
    pub no_dry_run: bool,
    
    /// Selects the wallet to use based on environment variable suffix (e.g., _1, _2).
    #[arg(long, global = true, default_value_t = 1)]
    pub wallet: u32,
}

#[derive(Subcommand, Debug, Serialize)]
pub enum Commands {
    /// Buy a token on a supported chain.
    Buy(BuyArgs),
    
    /// Sell a token on a supported chain.
    Sell(SellArgs),

    /// Get a quote for a trade.
    Quote(QuoteArgs),

    /// Check balance of native or token.
    Balance(BalanceArgs),

    /// Approve a token for trading.
    Approve(ApproveArgs),

    /// Display current configuration.
    Config,

    /// Display wallet address.
    Wallet,

    /// Detect Solana token path (Pump.fun or Raydium).
    Detect(DetectArgs),
}

#[derive(Parser, Debug, Serialize)]
pub struct BuyArgs {
    /// The contract address of the token to buy.
    pub token_address: String,

    /// The amount of the native currency (e.g., BNB, SOL) to spend.
    pub amount: f64,

    /// The blockchain to execute the trade on.
    #[arg(long, short, value_enum, default_value_t = Chain::Bsc)]
    pub chain: Chain,

    /// Slippage tolerance in percentage (0-100).
    #[arg(long, default_value_t = 15.0)]
    pub slippage: f32,

    /// Tip for the developer in percentage (0-5%).
    #[arg(long, default_value_t = 0.0)]
    pub tip_rate: f32,

    /// Jito Tip amount in SOL (e.g., 0.0001).
    #[arg(long)]
    pub jito_tip: Option<f64>,
}

#[derive(Parser, Debug, Serialize)]
pub struct SellArgs {
    /// The contract address of the token to sell.
    pub token_address: String,

    /// The amount of the token to sell.
    pub amount: f64,

    /// The blockchain to execute the trade on.
    #[arg(long, short, value_enum, default_value_t = Chain::Bsc)]
    pub chain: Chain,
    
    /// Slippage tolerance in percentage (0-100).
    #[arg(long, default_value_t = 15.0)]
    pub slippage: f32,

    /// Tip for the developer in percentage (0-5%).
    #[arg(long, default_value_t = 0.0)]
    pub tip_rate: f32,

    /// Jito Tip amount in SOL (e.g., 0.0001).
    #[arg(long)]
    pub jito_tip: Option<f64>,
}

#[derive(Parser, Debug, Serialize)]
pub struct QuoteArgs {
    /// The contract address of the token.
    pub token_address: String,

    /// The amount to quote for.
    pub amount: f64,

    /// The action to quote for (buy or sell).
    #[arg(long, default_value = "buy")]
    pub action: String,

    /// The blockchain to execute the trade on.
    #[arg(long, short, value_enum, default_value_t = Chain::Bsc)]
    pub chain: Chain,
}

#[derive(Parser, Debug, Serialize)]
pub struct BalanceArgs {
    /// The contract address of the token (optional, defaults to native).
    #[arg(long, short)]
    pub token_address: Option<String>,

    /// The blockchain to check balance on.
    #[arg(long, short, value_enum, default_value_t = Chain::Bsc)]
    pub chain: Chain,
}

#[derive(Parser, Debug, Serialize)]
pub struct ApproveArgs {
    /// The contract address of the token to approve.
    pub token_address: String,

    /// The amount to approve (optional, defaults to max).
    #[arg(long, short)]
    pub amount: Option<f64>,

    /// The blockchain to execute the approve on.
    #[arg(long, short, value_enum, default_value_t = Chain::Bsc)]
    pub chain: Chain,
}

pub fn validate_args(slippage: f32, tip_rate: f32) -> Result<(), String> {
    if slippage < 0.0 || slippage > 100.0 {
        return Err("Slippage must be between 0 and 100".to_string());
    }
    if tip_rate < 0.0 || tip_rate > 5.0 {
        return Err("Tip rate must be between 0 and 5".to_string());
    }
    Ok(())
}

#[derive(Parser, Debug, Serialize)]
pub struct DetectArgs {
    /// The token address (mint account) to detect the path for.
    pub token_address: String,

    /// The blockchain to detect the path on (only Solana supported).
    #[arg(long, short, value_enum)]
    pub chain: Chain,
}
