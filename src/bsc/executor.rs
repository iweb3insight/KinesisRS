// src/bsc/executor.rs
use crate::config::Config;
use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{Address, U256},
    providers::{Provider, ProviderBuilder, RootProvider},
    rpc::client::RpcClient,
    rpc::types::eth::TransactionRequest,
    signers::local::PrivateKeySigner,
    sol_types::{sol, SolCall},
    transports::http::{Client, Http},
    hex,
};
use futures::future::select_all;
use thiserror::Error;
use crate::types::{TransactionStatus, TransactionStatusResponse};
use url::Url;
use std::sync::Arc;
use anyhow::Result;

// Known Router Addresses
const PANCAKE_ROUTER_MAINNET: &str = "0x10ED43C718714eb63d5aA57B78B54704E256024E";
const PANCAKE_ROUTER_TESTNET: &str = "0x9Ac64Cc6e4415144C455BD8E4837Fea55603e5c3";

// Native Token Wrappers
const WBNB_MAINNET: &str = "0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c";
const WBNB_TESTNET: &str = "0xae13d989daC2f0dEbFf460aC112a837C89BAa7cd";

sol! {
    #[sol(rpc)]
    interface IBscRouter {
        function quoteBuy(address token, uint256 amountIn) external view returns (uint256 amountOut);
        function quoteSell(address token, uint256 amountIn) external view returns (uint256 amountOut);
        function buy(address token, uint256 amountOutMin, uint256 tipRate, uint256 deadline) external payable returns (uint256 amountOut);
        function sell(address token, uint256 amountIn, uint256 amountOutMin, uint256 tipRate, uint256 deadline) external returns (uint256 amountOut);
        function getAmountsOut(uint256 amountIn, address[] memory path) external view returns (uint256[] memory amounts);
        function WETH() external view returns (address weth_addr);
    }

    #[sol(rpc)]
    interface IERC20 {
        function allowance(address owner, address spender) external view returns (uint256 allowance);
        function approve(address spender, uint256 amount) external returns (bool success);
        function balanceOf(address account) external view returns (uint256 balance);
    }
}

#[derive(Error, Debug)]
pub enum ExecutorError {
    #[error("Failed to build RPC client: {0}")]
    ClientBuilderError(#[from] anyhow::Error),
    #[error("Private key is invalid")]
    InvalidKey,
    #[error("Contract call failed: {0}")]
    ContractError(String),
    #[error("Transaction failed: {0}")]
    TransactionError(String),
    #[error("RPC infrastructure error: {0}")]
    RpcError(String),
    #[error("All RPCs failed or returned error")]
    AllRpcsFailed,
    #[error("Simulation failed - Revert Reason: {revert_reason}")]
    SimulationFailed {
        revert_reason: String,
        raw_revert_data: Option<String>,
    },
}

#[derive(Debug)]
pub struct ExecutionResult {
    pub message: String,
    pub gas_used: u128,
}

pub struct BscExecutor {
    providers: Vec<Arc<RootProvider<Http<Client>>>>,
    signer: PrivateKeySigner,
    router_address: Address,
    chain_id: u64,
    _weth_address: Address,
}

impl BscExecutor {
    pub async fn new(config: Config, private_key: String) -> Result<Self, ExecutorError> {
        let signer = private_key.parse::<PrivateKeySigner>().map_err(|_| ExecutorError::InvalidKey)?;
        let mut providers = Vec::new();

        let mut client_builder = reqwest::Client::builder().timeout(std::time::Duration::from_secs(30));
        if let Some(proxy_url) = &config.https_proxy {
            let proxy = reqwest::Proxy::all(proxy_url).map_err(|e| ExecutorError::ClientBuilderError(e.into()))?;
            client_builder = client_builder.proxy(proxy);
        }
        let http_client = client_builder.build().map_err(|e| ExecutorError::ClientBuilderError(e.into()))?;

        for rpc_url_str in &config.bsc_rpc_urls {
            let rpc_url = Url::parse(rpc_url_str).map_err(anyhow::Error::from)?;
            let transport = Http::with_client(http_client.clone(), rpc_url);
            let client = RpcClient::new(transport, true);
            let provider = ProviderBuilder::new().on_client(client);
            providers.push(Arc::new(provider));
        }

        if providers.is_empty() { return Err(ExecutorError::RpcError("No valid RPC URLs provided".to_string())); }
        let chain_id = providers[0].get_chain_id().await.map_err(|e| ExecutorError::RpcError(format!("Failed to get Chain ID: {}", e)))?;

        let router_address = if let Some(addr_str) = &config.bsc_router_address {
            addr_str.parse::<Address>().map_err(|_| ExecutorError::ContractError("Invalid router address in config".to_string()))?
        } else {
            let default_addr = if chain_id == 56 { PANCAKE_ROUTER_MAINNET } else { PANCAKE_ROUTER_TESTNET };
            default_addr.parse().unwrap()
        };

        let weth_address = if chain_id == 56 { WBNB_MAINNET.parse().unwrap() } else { WBNB_TESTNET.parse().unwrap() };
        tracing::info!(?chain_id, ?router_address, rpc_count = ?providers.len(), "Multi-RPC BSC Executor initialized");
        Ok(Self { providers, signer, router_address, chain_id, _weth_address: weth_address })
    }

    fn primary_provider(&self) -> Arc<RootProvider<Http<Client>>> { self.providers[0].clone() }

    async fn race_rpc<T, F, Fut>(&self, f: F) -> Result<T, ExecutorError> 
    where F: Fn(Arc<RootProvider<Http<Client>>>) -> Fut, Fut: std::future::Future<Output = Result<T, ExecutorError>> + Send + 'static, T: Send + 'static
    {
        let mut tasks = Vec::new();
        for p in &self.providers { tasks.push(Box::pin(f(p.clone()))); }
        if tasks.is_empty() { return Err(ExecutorError::AllRpcsFailed); }
        let (result, _index, _remaining) = select_all(tasks).await;
        result
    }

    pub async fn quote_buy(&self, token_address: Address, amount_in: U256) -> Result<U256, ExecutorError> {
        self.race_rpc(move |p| {
            let router_addr = self.router_address;
            async move {
                let router = IBscRouter::new(router_addr, &p);
                match router.quoteBuy(token_address, amount_in).call().await {
                    Ok(result) => Ok(result.amountOut),
                    Err(e) => {
                        let err_msg = e.to_string();
                        if err_msg.contains("ABI decoding failed") || err_msg.contains("overrun") {
                            let weth = router.WETH().call().await.map_err(|e| ExecutorError::ContractError(e.to_string()))?.weth_addr;
                            if token_address == weth { return Ok(amount_in); }
                            let path = vec![weth, token_address];
                            let res = router.getAmountsOut(amount_in, path).call().await.map_err(|e| ExecutorError::ContractError(e.to_string()))?;
                            Ok(res.amounts.last().cloned().unwrap_or(U256::ZERO))
                        } else { Err(ExecutorError::ContractError(err_msg)) }
                    }
                }
            }
        }).await
    }

    pub async fn get_bsc_transaction_status(&self, tx_hash: &str, timeout_secs: u64) -> Result<TransactionStatusResponse> {
        let hash: alloy::primitives::FixedBytes<32> = tx_hash.parse().map_err(|_| anyhow::anyhow!("Invalid transaction hash format"))?;
        
        // If timeout is 0, check once
        if timeout_secs == 0 {
            return self.check_bsc_transaction_status_once(hash).await;
        }

        // Poll with timeout
        let start = std::time::Instant::now();
        while start.elapsed().as_secs() < timeout_secs {
            match self.check_bsc_transaction_status_once(hash).await {
                Ok(response) => {
                    if response.status != TransactionStatus::Pending {
                        return Ok(response);
                    }
                }
                Err(_) => {} // Continue polling
            }
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }

        Ok(TransactionStatusResponse {
            status: TransactionStatus::Pending,
            tx_hash: tx_hash.to_string(),
            slot: None,
            confirmations: None,
            error: Some("Timeout waiting for transaction confirmation".to_string()),
        })
    }

    async fn check_bsc_transaction_status_once(&self, hash: alloy::primitives::FixedBytes<32>) -> Result<TransactionStatusResponse> {
        let p = self.primary_provider();
        
        // First check if the transaction exists
        let tx = match p.get_transaction_by_hash(hash).await {
            Ok(Some(tx)) => tx,
            Ok(None) => return Ok(TransactionStatusResponse {
                status: TransactionStatus::NotFound,
                tx_hash: format!("0x{:x}", hash),
                slot: None,
                confirmations: None,
                error: None,
            }),
            Err(_) => return Ok(TransactionStatusResponse {
                status: TransactionStatus::NotFound,
                tx_hash: format!("0x{:x}", hash),
                slot: None,
                confirmations: None,
                error: None,
            }),
        };

        // If transaction exists, check receipt for status
        let receipt = match p.get_transaction_receipt(hash).await {
            Ok(Some(receipt)) => receipt,
            _ => return Ok(TransactionStatusResponse {
                status: TransactionStatus::Pending,
                tx_hash: format!("0x{:x}", hash),
                slot: tx.block_number,
                confirmations: Some(0),
                error: None,
            }),
        };

        let status = if receipt.status() {
            TransactionStatus::Success
        } else {
            TransactionStatus::Failed
        };

        let latest_block = p.get_block_number().await.unwrap_or(receipt.block_number.unwrap_or(0));
        let confirmations = if let Some(bn) = receipt.block_number {
            if latest_block >= bn { latest_block - bn + 1 } else { 1 }
        } else {
            0
        };

        Ok(TransactionStatusResponse {
            status,
            tx_hash: format!("0x{:x}", hash),
            slot: receipt.block_number,
            confirmations: Some(confirmations),
            error: if !receipt.status() { Some("Transaction reverted".to_string()) } else { None },
        })
    }

    pub fn wallet_address(&self) -> Address { self.signer.address() }

    pub async fn get_balance(&self, owner: Address, token_address: Option<Address>) -> Result<U256, ExecutorError> {
        self.race_rpc(move |p| {
            async move {
                match token_address {
                    Some(token) => {
                        let erc20 = IERC20::new(token, &p);
                        let result = erc20.balanceOf(owner).call().await.map_err(|e| ExecutorError::ContractError(e.to_string()))?;
                        Ok(result.balance)
                    }
                    None => { Ok(p.get_balance(owner).await.map_err(|e| ExecutorError::ContractError(e.to_string()))?) }
                }
            }
        }).await
    }

    pub async fn get_allowance(&self, token_address: Address, owner: Address) -> Result<U256, ExecutorError> {
        let p = self.primary_provider();
        let erc20 = IERC20::new(token_address, &p);
        let result = erc20.allowance(owner, self.router_address).call().await.map_err(|e| self.handle_rpc_error(e))?;
        Ok(result.allowance)
    }

    pub async fn approve(&self, token_address: Address, amount: U256, dry_run: bool) -> Result<ExecutionResult, ExecutorError> {
        if dry_run {
            return Ok(ExecutionResult { message: format!("Dry-run: Approval for token {} simulated successfully.", token_address), gas_used: 50_000 });
        }
        let p = self.primary_provider();
        let wallet = EthereumWallet::from(self.signer.clone());
        let rpc_url = p.client().transport().url().parse().unwrap();
        let provider_with_wallet = ProviderBuilder::new().wallet(wallet).on_http(rpc_url);
        let erc20 = IERC20::new(token_address, &provider_with_wallet);
        let pending_tx = erc20.approve(self.router_address, amount).send().await.map_err(|e| self.handle_rpc_error(e))?;
        let receipt = pending_tx.get_receipt().await.map_err(|e| ExecutorError::TransactionError(e.to_string()))?;
        Ok(ExecutionResult { message: format!("Approve transaction confirmed: 0x{:x}", receipt.transaction_hash), gas_used: receipt.gas_used })
    }

    pub async fn approve_if_needed(&self, token_address: Address, owner: Address, amount: U256, dry_run: bool) -> Result<Option<ExecutionResult>, ExecutorError> {
        let allowance = self.get_allowance(token_address, owner).await?;
        if allowance < amount { Ok(Some(self.approve(token_address, amount, dry_run).await?)) } else { Ok(None) }
    }

    pub async fn quote_sell(&self, token_address: Address, amount_in: U256) -> Result<U256, ExecutorError> {
        self.race_rpc(move |p| {
            let router_addr = self.router_address;
            async move {
                let router = IBscRouter::new(router_addr, &p);
                match router.quoteSell(token_address, amount_in).call().await {
                    Ok(result) => Ok(result.amountOut),
                    Err(e) => {
                        let err_msg = e.to_string();
                        if err_msg.contains("ABI decoding failed") || err_msg.contains("overrun") {
                            let weth = router.WETH().call().await.map_err(|e| ExecutorError::ContractError(e.to_string()))?.weth_addr;
                            if token_address == weth { return Ok(amount_in); }
                            let path = vec![token_address, weth];
                            let result = router.getAmountsOut(amount_in, path).call().await.map_err(|e| ExecutorError::ContractError(e.to_string()))?;
                            Ok(result.amounts.last().cloned().unwrap_or(U256::ZERO))
                        } else { Err(ExecutorError::ContractError(err_msg)) }
                    }
                }
            }
        }).await
    }

    pub async fn buy(&self, token_address: Address, amount_in: U256, amount_out_min: U256, tip_rate: U256, dry_run: bool) -> Result<ExecutionResult, ExecutorError> {
        let p = self.primary_provider();
        let router = IBscRouter::new(self.router_address, &p);
        let deadline = U256::from(u64::MAX);
        if dry_run {
            let gas_estimate = router.buy(token_address, amount_out_min, tip_rate, deadline).value(amount_in).estimate_gas().await.map_err(|e| self.handle_rpc_error(e))?;
            let result = router.buy(token_address, amount_out_min, tip_rate, deadline).value(amount_in).call().await.map_err(|e| self.handle_rpc_error(e))?;
            Ok(ExecutionResult { message: format!("Simulated amount out: {}", result.amountOut), gas_used: gas_estimate as u128 })
        } else {
            let buy_call = IBscRouter::buyCall { token: token_address, amountOutMin: amount_out_min, tipRate: tip_rate, deadline };
            let call_data = buy_call.abi_encode();
            let from = self.signer.address();
            let nonce_fut = p.get_transaction_count(from);
            let gas_price_fut = p.get_gas_price();
            let (nonce_res, gas_price_res) = tokio::join!(nonce_fut, gas_price_fut);
            let nonce = nonce_res.map_err(|e| self.handle_rpc_error(e))?;
            let gas_price = gas_price_res.map_err(|e| self.handle_rpc_error(e))?;
            let tx = TransactionRequest::default().with_from(from).with_to(self.router_address).with_value(amount_in).with_input(call_data).with_nonce(nonce).with_chain_id(self.chain_id).with_gas_limit(300_000).with_max_fee_per_gas(gas_price * 2).with_max_priority_fee_per_gas(gas_price);
            let wallet = EthereumWallet::from(self.signer.clone());
            let mut broadcast_tasks = Vec::new();
            for rpc_p in &self.providers {
                let rpc_url = rpc_p.client().transport().url().parse().unwrap();
                let provider_with_wallet = ProviderBuilder::new().wallet(wallet.clone()).on_http(rpc_url);
                let tx_inner = tx.clone();
                broadcast_tasks.push(tokio::spawn(async move { provider_with_wallet.send_transaction(tx_inner).await }));
            }
            let mut tx_hash = None;
            let mut last_error = None;
            for task in broadcast_tasks {
                match task.await {
                    Ok(Ok(pending_tx)) => { tx_hash = Some(*pending_tx.tx_hash()); break; }
                    Ok(Err(e)) => { last_error = Some(e); }
                    Err(_) => {}
                }
            }
            let hash = if let Some(h) = tx_hash { h } else {
                return Err(if let Some(e) = last_error { self.handle_rpc_error(e) } else { ExecutorError::TransactionError("Failed to broadcast to any RPC".to_string()) });
            };
            let receipt = p.get_transaction_receipt(hash).await.map_err(|e| self.handle_rpc_error(e))?.ok_or_else(|| ExecutorError::TransactionError("Receipt not found".to_string()))?;
            Ok(ExecutionResult { message: format!("Transaction confirmed: 0x{:x}", receipt.transaction_hash), gas_used: receipt.gas_used })
        }
    }

    pub async fn sell(&self, token_address: Address, amount_in: U256, amount_out_min: U256, tip_rate: U256, dry_run: bool) -> Result<ExecutionResult, ExecutorError> {
        let p = self.primary_provider();
        let router = IBscRouter::new(self.router_address, &p);
        let deadline = U256::from(u64::MAX);
        if dry_run {
            let sell_call = router.sell(token_address, amount_in, amount_out_min, tip_rate, deadline);
            let gas_estimate = sell_call.estimate_gas().await.map_err(|e| self.handle_rpc_error(e))?;
            let result = sell_call.call().await.map_err(|e| self.handle_rpc_error(e))?;
            Ok(ExecutionResult { message: format!("Simulated amount out: {}", result.amountOut), gas_used: gas_estimate as u128 })
        } else {
            let sell_call_obj = IBscRouter::sellCall { token: token_address, amountIn: amount_in, amountOutMin: amount_out_min, tipRate: tip_rate, deadline };
            let call_data = sell_call_obj.abi_encode();
            let from = self.signer.address();
            let nonce_fut = p.get_transaction_count(from);
            let gas_price_fut = p.get_gas_price();
            let (nonce_res, gas_price_res) = tokio::join!(nonce_fut, gas_price_fut);
            let nonce = nonce_res.map_err(|e| self.handle_rpc_error(e))?;
            let gas_price = gas_price_res.map_err(|e| self.handle_rpc_error(e))?;
            let tx = TransactionRequest::default().with_from(from).with_to(self.router_address).with_input(call_data).with_nonce(nonce).with_chain_id(self.chain_id).with_gas_limit(300_000).with_max_fee_per_gas(gas_price * 2).with_max_priority_fee_per_gas(gas_price);
            let wallet = EthereumWallet::from(self.signer.clone());
            let mut broadcast_tasks = Vec::new();
            for rpc_p in &self.providers {
                let rpc_url = rpc_p.client().transport().url().parse().unwrap();
                let provider_with_wallet = ProviderBuilder::new().wallet(wallet.clone()).on_http(rpc_url);
                let tx_inner = tx.clone();
                broadcast_tasks.push(tokio::spawn(async move { provider_with_wallet.send_transaction(tx_inner).await }));
            }
            let mut tx_hash = None;
            let mut last_error = None;
            for task in broadcast_tasks {
                match task.await {
                    Ok(Ok(pending_tx)) => { tx_hash = Some(*pending_tx.tx_hash()); break; }
                    Ok(Err(e)) => { last_error = Some(e); }
                    Err(_) => {}
                }
            }
            let hash = if let Some(h) = tx_hash { h } else {
                return Err(if let Some(e) = last_error { self.handle_rpc_error(e) } else { ExecutorError::TransactionError("Failed to broadcast to any RPC".to_string()) });
            };
            let receipt = p.get_transaction_receipt(hash).await.map_err(|e| self.handle_rpc_error(e))?.ok_or_else(|| ExecutorError::TransactionError("Receipt not found".to_string()))?;
            Ok(ExecutionResult { message: format!("Transaction confirmed: 0x{:x}", receipt.transaction_hash), gas_used: receipt.gas_used })
        }
    }

    fn handle_rpc_error(&self, err: impl std::fmt::Display) -> ExecutorError {
        let err_msg = err.to_string();
        if err_msg.contains("429") || err_msg.to_lowercase().contains("too many requests") { return ExecutorError::RpcError("Rate limit exceeded (429)".to_string()); }
        if err_msg.to_lowercase().contains("timeout") || err_msg.to_lowercase().contains("timed out") { return ExecutorError::RpcError("Network timeout".to_string()); }
        if err_msg.to_lowercase().contains("nonce too low") || err_msg.to_lowercase().contains("already known") || err_msg.to_lowercase().contains("underpriced") { return ExecutorError::RpcError(format!("Nonce/Gas conflict: {}", err_msg)); }
        if let Some(reason) = self.extract_revert_reason(&err_msg) {
            let final_reason = if reason == "REVERT_NO_DATA" || reason == "REVERT_UNKNOWN" || reason == "EMPTY_REVERT_DATA" { "INSUFFICIENT_LIQUIDITY_OR_OTHER_REVERT (bare revert)".to_string() } else { reason };
            ExecutorError::SimulationFailed { revert_reason: final_reason, raw_revert_data: self.extract_raw_revert_data(&err_msg) }
        } else { ExecutorError::ContractError(err_msg) }
    }

    fn extract_revert_reason(&self, err_msg: &str) -> Option<String> {
        let msg = err_msg.to_lowercase();
        if let Some(pos) = msg.find("execution reverted: ") {
            let mut reason = err_msg[pos + "execution reverted: ".len()..].trim();
            if let Some(comma_pos) = reason.find(',') { reason = &reason[..comma_pos].trim(); }
            if reason.starts_with("0x") {
                if reason.len() <= 2 { return Some("REVERT_NO_DATA".to_string()); }
                if let Ok(data) = hex::decode(&reason[2..]) { return Some(self.decode_revert_data(&data)); }
            }
            return Some(reason.to_string());
        }
        if msg.contains("revert") || msg.contains("reverted") { return Some("REVERT_UNKNOWN".to_string()); }
        None
    }

    fn extract_raw_revert_data(&self, err_msg: &str) -> Option<String> {
        if let Some(pos) = err_msg.find("execution reverted: ") {
            let mut reason = err_msg[pos + "execution reverted: ".len()..].trim();
            if let Some(comma_pos) = reason.find(',') { reason = &reason[..comma_pos].trim(); }
            if reason.starts_with("0x") { return Some(reason.to_string()); }
        }
        None
    }

    fn decode_revert_data(&self, data: &[u8]) -> String {
        if data.is_empty() { return "EMPTY_REVERT_DATA".to_string(); }
        if data.len() < 4 { return "malformed revert data".to_string(); }
        let selector = &data[0..4];
        if selector == &[0x08, 0xc3, 0x79, 0xa0] {
            if data.len() >= 36 {
                let offset = U256::from_be_slice(&data[4..36]);
                let offset_usize: usize = offset.to::<usize>();
                if offset_usize + 36 <= data.len() {
                    let len = U256::from_be_slice(&data[offset_usize + 4 .. offset_usize + 36]).to::<usize>();
                    if offset_usize + 36 + len <= data.len() { return String::from_utf8_lossy(&data[offset_usize + 36 .. offset_usize + 36 + len]).to_string(); }
                }
            }
            "malformed Error(string) revert".to_string()
        } else if selector == &[0x4e, 0x48, 0x7b, 0x71] {
            if data.len() >= 36 {
                let code = U256::from_be_slice(&data[4..36]);
                return format!("Panic code: {}", code);
            }
            "malformed Panic(uint256) revert".to_string()
        } else { format!("Custom error selector: 0x{}", hex::encode(selector)) }
    }
}
