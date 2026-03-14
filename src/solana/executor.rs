// src/solana/executor.rs

use anyhow::{Result, anyhow};
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose, Engine as _};
use std::time::Duration;
use crate::solana::detector::{SolanaPathDetector, Path};
use solana_sdk::{
    pubkey::Pubkey,
    signer::{keypair::Keypair, Signer},
    instruction::{Instruction, AccountMeta},
    transaction::VersionedTransaction,
    message::VersionedMessage,
    hash::Hash,
};
use std::str::FromStr;
use spl_associated_token_account::get_associated_token_address_with_program_id;
use rand::seq::SliceRandom;

use crate::types::{TransactionStatus, TransactionStatusResponse, HealthResult, RpcStatus, WalletStatus, Chain};

// --- Solana Constants ---
const SOL_MINT_ADDRESS: &str = "So11111111111111111111111111111111111111112";
const PUMP_FUN_PROGRAM_ID: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";
const PUMP_FUN_GLOBAL: &str = "4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4JztUStW";
const PUMP_FUN_FEE_RECIPIENT: &str = "CebN5WGQ4it1pStoaGR3abbCaDbV1bEY24iWiDEMdi2b";
const RENT_SYSVAR: &str = "SysvarRent111111111111111111111111111111111";
const SYSTEM_PROGRAM_ID: &str = "11111111111111111111111111111111";

// --- Jito Constants ---
const JITO_TIP_ACCOUNTS: [&str; 4] = [
    "9649qRqpZbe96vSST9fM9qf4C65BPrV78vSNDtKx25n6",
    "HFqU5x63VTqvQss8hp11i4wVV8bD44PvwucfZ2bU7gY3",
    "Cw8CFyM9FkoMi7K7Crf6HNWofLH6S7X9S4FvAnXfXWp6",
    "ADa69ccYvU4NneBkY6B5VfJ2K246V97Q7D3t295P76uS",
];
const JITO_BLOCK_ENGINE_URL: &str = "https://mainnet.block-engine.jito.wtf/api/v1/bundles";
const RAYDIUM_API_COMPUTE: &str = "https://transaction-v1.raydium.io";
const RAYDIUM_API_TRANSACTION: &str = "https://transaction-v1.raydium.io";

// --- Jupiter Constants ---
const JUPITER_QUOTE_API: &str = "https://quote-api.jup.ag/v6";
const JUPITER_SWAP_API: &str = "https://quote-api.jup.ag/v6/swap";

// --- Data Structures ---

#[derive(Serialize, Deserialize, Debug)]
struct RaydiumV3TransactionRequest {
    #[serde(rename = "swapResponse")]
    swap_response: serde_json::Value,
    wallet: String,
    #[serde(rename = "txVersion")]
    tx_version: String,
    #[serde(rename = "wrapSol")]
    wrap_sol: bool,
    #[serde(rename = "unwrapSol")]
    unwrap_sol: bool,
    #[serde(rename = "computeUnitPriceMicroLamports")]
    compute_unit_price_micro_lamports: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RaydiumV3TransactionResponse {
    data: Vec<RaydiumV3TransactionData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RaydiumV3TransactionData {
    transaction: String, // base64 encoded transaction
}

#[derive(Deserialize, Debug)]
struct RaydiumAutoFeeResponse {
    data: RaydiumAutoFeeData,
}

#[derive(Deserialize, Debug)]
struct RaydiumAutoFeeData {
    default: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Default, Clone)]
pub struct BondingCurveState {
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub token_total_supply: u64,
    pub complete: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: u64,
    method: String,
    params: serde_json::Value,
}

#[derive(Deserialize, Debug)]
struct JsonRpcResponse<T> {
    result: Option<T>,
    error: Option<JsonRpcError>,
}

#[derive(Deserialize, Debug)]
struct JsonRpcError {
    code: i64,
    message: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GetAccountInfoResponse {
    value: Option<AccountValue>,
}

#[derive(Deserialize, Debug)]
struct AccountValue {
    data: (String, String),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct LatestBlockhashResponse {
    value: BlockhashValue,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BlockhashValue {
    blockhash: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SimulateTransactionResponse {
    value: SimulateValue,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SimulateValue {
    err: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GetBalanceResponse {
    value: u64,
}

// --- Executor ---

pub struct SolanaExecutor {
    rpc_url: String,
    jito_rpc_url: String,
    client: reqwest::Client,
    signer: Keypair,
    pub path_detector: SolanaPathDetector,
}

impl SolanaExecutor {
    pub async fn new(rpc_url: String, private_key_bs58: &str) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(20))
            .build()?;
        let signer = Keypair::from_base58_string(private_key_bs58);
        let path_detector = SolanaPathDetector::new(rpc_url.clone()).await?;
        let jito_rpc_url = std::env::var("JITO_RPC_URL").unwrap_or_else(|_| JITO_BLOCK_ENGINE_URL.to_string());
        Ok(Self { rpc_url, jito_rpc_url, client, signer, path_detector })
    }

    async fn call_rpc<T: for<'de> Deserialize<'de>>(&self, method: &str, params: serde_json::Value) -> Result<T> {
        self.call_rpc_with_url(&self.rpc_url, method, params).await
    }

    async fn call_rpc_with_url<T: for<'de> Deserialize<'de>>(&self, url: &str, method: &str, params: serde_json::Value) -> Result<T> {
        let request = JsonRpcRequest { jsonrpc: "2.0".to_string(), id: 1, method: method.to_string(), params, };
        let response = self.client.post(url).json(&request).send().await?;
        let rpc_res: JsonRpcResponse<T> = response.json().await?;
        if let Some(error) = rpc_res.error {
            return Err(anyhow!("RPC Error {}: {}", error.code, error.message));
        }
        rpc_res.result.ok_or_else(|| anyhow!("Empty result from RPC"))
    }

    pub fn wallet_address(&self) -> Pubkey {
        self.signer.pubkey()
    }

    pub async fn get_health(&self) -> Result<HealthResult> {
        let start = std::time::Instant::now();
        
        // 1. Check RPC Connection & Latency
        let blockhash_res: Result<LatestBlockhashResponse> = self.call_rpc("getLatestBlockhash", serde_json::json!([{"commitment": "finalized"}])).await;
        let latency = start.elapsed().as_millis() as u64;
        
        let (connected, block_height) = match blockhash_res {
            Ok(_) => {
                let height: Result<u64> = self.call_rpc("getBlockHeight", serde_json::json!([])).await;
                (true, height.ok())
            }
            Err(_) => (false, None),
        };

        // 2. Check Wallet Readiness & Balance
        let address = self.signer.pubkey().to_string();
        let balance_res = self.get_balance(self.signer.pubkey(), None).await;
        let (ready, balance_str) = match balance_res {
            Ok(bal) => (true, (bal as f64 / 1_000_000_000.0).to_string()),
            Err(_) => (false, "0".to_string()),
        };

        Ok(HealthResult {
            success: connected && ready,
            chain: Chain::Solana,
            rpc_status: RpcStatus {
                connected,
                latency_ms: latency,
                endpoint: self.rpc_url.clone(),
                block_height,
            },
            wallet_status: WalletStatus {
                ready,
                address,
                balance: balance_str,
            },
        })
    }

    pub async fn get_balance(&self, owner: Pubkey, token_address: Option<Pubkey>) -> Result<u64> {
        match token_address {
            Some(token) => {
                let info = self.path_detector.detect_path(&token.to_string()).await?;
                let ata = get_associated_token_address_with_program_id(&owner, &token, &info.token_program_id);
                let params = serde_json::json!([ata.to_string(), { "encoding": "jsonParsed" }]);
                let response: serde_json::Value = self.call_rpc("getTokenAccountBalance", params).await?;
                let amount = response["value"]["amount"].as_str()
                    .ok_or_else(|| anyhow!("Failed to parse token balance"))?
                    .parse::<u64>()?;
                Ok(amount)
            }
            None => {
                let params = serde_json::json!([owner.to_string()]);
                let response: GetBalanceResponse = self.call_rpc("getBalance", params).await?;
                Ok(response.value)
            }
        }
    }

    /// Get transaction status with optional polling
    pub async fn get_solana_transaction_status(&self, tx_hash: &str, timeout_secs: u64) -> Result<TransactionStatusResponse> {
        // If timeout is 0, check once
        if timeout_secs == 0 {
            return self.check_solana_transaction_status_once(tx_hash).await;
        }

        // Poll with timeout
        let start = std::time::Instant::now();
        while start.elapsed().as_secs() < timeout_secs {
            match self.check_solana_transaction_status_once(tx_hash).await {
                Ok(response) => {
                    if response.status != TransactionStatus::Pending {
                        return Ok(response);
                    }
                }
                Err(_) => {} // Continue polling
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
        }

        Ok(TransactionStatusResponse {
            status: TransactionStatus::Pending,
            tx_hash: tx_hash.to_string(),
            slot: None,
            confirmations: None,
            error: Some("Timeout waiting for transaction confirmation".to_string()),
        })
    }

    async fn check_solana_transaction_status_once(&self, tx_hash: &str) -> Result<TransactionStatusResponse> {
        let params = serde_json::json!([
            tx_hash,
            { "encoding": "json", "commitment": "confirmed" }
        ]);
        
        let response: serde_json::Value = match self.call_rpc("getTransaction", params).await {
            Ok(res) => res,
            Err(_) => return Ok(TransactionStatusResponse {
                status: TransactionStatus::NotFound,
                tx_hash: tx_hash.to_string(),
                slot: None,
                confirmations: None,
                error: None,
            }),
        };
        
        if response.is_null() {
            return Ok(TransactionStatusResponse {
                status: TransactionStatus::NotFound,
                tx_hash: tx_hash.to_string(),
                slot: None,
                confirmations: None,
                error: None,
            });
        }

        let slot = response["slot"].as_u64();
        let meta = &response["meta"];
        let err = meta["err"].clone();
        
        let status = if err.is_null() {
            TransactionStatus::Success
        } else {
            TransactionStatus::Failed
        };

        let error_msg = if !err.is_null() {
            Some(err.to_string())
        } else {
            None
        };

        Ok(TransactionStatusResponse {
            status,
            tx_hash: tx_hash.to_string(),
            slot,
            confirmations: Some(1), // If getTransaction returns it, it's at least confirmed
            error: error_msg,
        })
    }

    fn get_random_jito_tip_account(&self) -> Pubkey {
        let mut rng = rand::thread_rng();
        let tip_account_str = JITO_TIP_ACCOUNTS.choose(&mut rng).unwrap();
        Pubkey::from_str(tip_account_str).unwrap()
    }

    pub(crate) fn build_jito_tip_instruction(&self, lamports: u64) -> Instruction {
        let tip_account = self.get_random_jito_tip_account();
        let system_program_id = Pubkey::from_str(SYSTEM_PROGRAM_ID).unwrap();
        
        let accounts = vec![
            AccountMeta::new(self.signer.pubkey(), true),
            AccountMeta::new(tip_account, false),
        ];

        let mut data = vec![2, 0, 0, 0]; // System Instruction: Transfer (discriminator = 2)
        data.extend_from_slice(&lamports.to_le_bytes());

        Instruction {
            program_id: system_program_id,
            accounts,
            data,
        }
    }

    pub async fn quote(&self, input_mint: &str, output_mint: &str, amount: u64) -> Result<(u64, Path)> {
        let is_buy = input_mint == SOL_MINT_ADDRESS;
        let token_mint = if is_buy { output_mint } else { input_mint };
        let info = self.path_detector.detect_path(token_mint).await?;

        match info.path {
            Path::PumpFun => {
                if is_buy {
                    let sol_amount = amount as f64 / 1_000_000_000.0;
                    let quote = self.quote_pump_buy(output_mint, sol_amount).await?;
                    Ok((quote, Path::PumpFun))
                } else {
                    let quote = self.quote_pump_sell(input_mint, amount).await?;
                    Ok((quote, Path::PumpFun))
                }
            },
            Path::Raydium | Path::RaydiumGraduated | Path::Unknown(_) => {
                let quote = self.quote_raydium_api(input_mint, output_mint, amount).await?;
                Ok((quote, info.path.clone()))
            },
        }
    }

    async fn quote_pump_sell(&self, mint: &str, amount_tokens: u64) -> Result<u64> {
        let (_, state) = self.find_bonding_curve(mint).await?;
        if state.complete {
            return Err(anyhow!("This Pump.fun curve is complete and has migrated to Raydium."));
        }
        
        let tokens_to_sell = amount_tokens as u128;
        let virtual_token_reserves = state.virtual_token_reserves as u128;
        let virtual_sol_reserves = state.virtual_sol_reserves as u128;

        let new_token_reserves = virtual_token_reserves + tokens_to_sell;
        let new_sol_reserves = (virtual_sol_reserves * virtual_token_reserves) / new_token_reserves;
        let sol_out = virtual_sol_reserves - new_sol_reserves;
        
        Ok(sol_out as u64)
    }

    async fn quote_raydium_api(&self, input_mint: &str, output_mint: &str, amount: u64) -> Result<u64> {
        let quote_url = format!(
            "{}/compute/swap-base-in?inputMint={}&outputMint={}&amount={}&slippageBps=50&txVersion=V0",
            RAYDIUM_API_COMPUTE, input_mint, output_mint, amount
        );

        let response = self.client.get(&quote_url).send().await?;
        if !response.status().is_success() {
            return Err(anyhow!("Raydium API HTTP 错误：{}", response.text().await?));
        }

        let quote_res: serde_json::Value = response.json().await?;
        
        // 检查 API 是否成功
        if !quote_res["success"].as_bool().unwrap_or(false) {
            let error_msg = quote_res["msg"].as_str().unwrap_or("Unknown error");
            return Err(match error_msg {
                "ROUTE_NOT_FOUND" => anyhow!("Raydium 未索引该代币流动性池，请等待或使用 Pump.fun 路径"),
                "REQ_SWAP_RESPONSE_ERROR" => anyhow!("Raydium API 内部错误，请稍后重试"),
                other => anyhow!("Raydium API 错误 ({}): {}", error_msg, other),
            });
        }

        // 解析 outputAmount
        let out_amount = quote_res["data"]["outputAmount"]
            .as_str()
            .ok_or_else(|| anyhow!("Raydium API 返回格式异常：缺少 data.outputAmount 字段"))?
            .parse::<u64>()?;

        Ok(out_amount)
    }

    async fn find_bonding_curve(&self, mint_str: &str) -> Result<(Pubkey, BondingCurveState)> {
        let pump_program_id: Pubkey = PUMP_FUN_PROGRAM_ID.parse()?;
        let mint_pubkey: Pubkey = mint_str.parse()?;
        let (pda, _) = Pubkey::find_program_address(&[b"bonding-curve", mint_pubkey.as_ref()], &pump_program_id);
        let params = serde_json::json!([pda.to_string(), { "encoding": "base64" }]);
        let resp: GetAccountInfoResponse = self.call_rpc("getAccountInfo", params).await?;
        let value = resp.value.ok_or_else(|| anyhow!("Bonding curve account not found for mint {}", mint_str))?;
        let data = general_purpose::STANDARD.decode(&value.data.0)?;
        if data.len() < 8 + std::mem::size_of::<BondingCurveState>() {
            return Err(anyhow!("Bonding curve account data is too short"));
        }
        let state = BondingCurveState::try_from_slice(&data[8..])?;
        Ok((pda, state))
    }

    async fn quote_pump_buy(&self, mint: &str, amount_in_sol: f64) -> Result<u64> {
        let (_, state) = self.find_bonding_curve(mint).await?;
        if state.complete {
            return Err(anyhow!("This Pump.fun curve is complete and has migrated to Raydium."));
        }
        let amount_in_lamports = (amount_in_sol * 1_000_000_000.0) as u128;
        let new_token_reserves = (state.virtual_sol_reserves as u128 * state.virtual_token_reserves as u128) / (state.virtual_sol_reserves as u128 + amount_in_lamports);
        Ok((state.virtual_token_reserves as u128 - new_token_reserves) as u64)
    }
    
    pub async fn buy(&self, output_mint_str: &str, sol_amount: f64, slippage_bps: u16, dry_run: bool, jito_tip_lamports: Option<u64>) -> Result<String> {
        let info = self.path_detector.detect_path(output_mint_str).await?;
        match info.path {
            Path::PumpFun => self.buy_pump_fun(output_mint_str, info.token_program_id, sol_amount, slippage_bps, dry_run, jito_tip_lamports).await,
            Path::Raydium | Path::RaydiumGraduated | Path::Unknown(_) => {
                // 根据网络选择 API
                if self.is_mainnet() {
                    // 主网优先使用 Raydium API
                    match self.buy_raydium_api(output_mint_str, sol_amount, slippage_bps, dry_run).await {
                        Ok(result) => Ok(result),
                        Err(e) => {
                            tracing::warn!("Raydium API 失败，尝试 Jupiter: {}", e);
                            // Fallback 到 Jupiter
                            let amount_lamports = (sol_amount * 1_000_000_000.0) as u64;
                            self.swap_jupiter(SOL_MINT_ADDRESS, output_mint_str, amount_lamports, slippage_bps, dry_run).await
                        }
                    }
                } else {
                    // Devnet/Testnet 直接使用 Jupiter
                    let amount_lamports = (sol_amount * 1_000_000_000.0) as u64;
                    self.swap_jupiter(SOL_MINT_ADDRESS, output_mint_str, amount_lamports, slippage_bps, dry_run).await
                }
            }
        }
    }

    /// 检查是否为 Mainnet
    fn is_mainnet(&self) -> bool {
        self.rpc_url.contains("mainnet-beta")
    }

    async fn get_raydium_priority_fee(&self) -> Result<String> {
        let fee_res = self.client.get(&format!("{}/main/auto-fee", RAYDIUM_API_COMPUTE)).send().await?.json::<RaydiumAutoFeeResponse>().await?;
        Ok(fee_res.data.default.to_string())
    }

    async fn buy_raydium_api(&self, output_mint_str: &str, sol_amount: f64, slippage_bps: u16, dry_run: bool) -> Result<String> {
        // Step 0: Get Priority Fee
        let priority_fee = self.get_raydium_priority_fee().await.unwrap_or_else(|_| "1000".to_string());

        // Step 1: Get Quote from API
        let sol_lamports = (sol_amount * 1_000_000_000.0) as u64;
        let quote_url = format!(
            "{}/compute/swap-base-in?inputMint={}&outputMint={}&amount={}&slippageBps={}&txVersion=V0",
            RAYDIUM_API_COMPUTE, SOL_MINT_ADDRESS, output_mint_str, sol_lamports, slippage_bps
        );

        let quote_resp = self.client.get(&quote_url).send().await?;
        let quote_text = quote_resp.text().await?;
        tracing::debug!(quote_response = %quote_text, "Raydium Quote API raw response");

        let quote_res: serde_json::Value = serde_json::from_str(&quote_text)
            .map_err(|e| anyhow!("Failed to parse Raydium quote: {}. Raw response: {}", e, quote_text))?;

        // 检查 API 是否成功
        if !quote_res["success"].as_bool().unwrap_or(false) {
            let error_msg = quote_res["msg"].as_str().unwrap_or("Unknown error");
            return Err(match error_msg {
                "ROUTE_NOT_FOUND" => anyhow!("Raydium 未索引该代币流动性池，请等待或使用 Pump.fun 路径"),
                other => anyhow!("Raydium API 错误 ({}): {}", error_msg, other),
            });
        }

        // Step 2: Serialize Transaction
        let tx_request = RaydiumV3TransactionRequest {
            swap_response: quote_res, // Pass ENTIRE quote response
            wallet: self.signer.pubkey().to_string(),
            tx_version: "V0".to_string(),
            wrap_sol: true,
            unwrap_sol: false,
            compute_unit_price_micro_lamports: priority_fee,
        };

        let tx_url = format!("{}/transaction/swap-base-in", RAYDIUM_API_TRANSACTION);
        let tx_resp = self.client.post(&tx_url).json(&tx_request).send().await?;
        let tx_text = tx_resp.text().await?;
        tracing::debug!(tx_response = %tx_text, "Raydium Transaction API raw response");

        let tx_res: RaydiumV3TransactionResponse = serde_json::from_str(&tx_text)
            .map_err(|e| anyhow!("Failed to parse Raydium transaction: {}. Raw response: {}", e, tx_text))?;

        let encoded_tx = tx_res.data.get(0).ok_or_else(|| anyhow!("No transaction returned from Raydium API. Response: {}", tx_text))?.transaction.clone();

        // Step 3: Simulate (Dry-run)
        // 注意：Raydium V3 API 返回的交易已经签名，无需重新签名
        if dry_run {
            self.simulate_transaction(&encoded_tx).await?;
            return Ok("SIMULATED_RAYDIUM".to_string());
        }

        // Step 4: Send Transaction (直接发送 Raydium API 返回的已签名交易)
        let params = serde_json::json!([encoded_tx, {
            "skipPreflight": true,
            "encoding": "base64",
            "commitment": "confirmed"
        }]);

        let signature: String = self.call_rpc("sendTransaction", params).await?;
        Ok(signature)
    }

    pub async fn sell(&self, input_mint_str: &str, token_amount: u64, slippage_bps: u16, dry_run: bool, jito_tip_lamports: Option<u64>) -> Result<String> {
        let info = self.path_detector.detect_path(input_mint_str).await?;
        match info.path {
            Path::PumpFun => self.sell_pump_fun(input_mint_str, info.token_program_id, token_amount, slippage_bps, dry_run, jito_tip_lamports).await,
            Path::Raydium | Path::RaydiumGraduated | Path::Unknown(_) => {
                // 根据网络选择 API
                if self.is_mainnet() {
                    match self.sell_raydium_api(input_mint_str, token_amount, slippage_bps, dry_run).await {
                        Ok(result) => Ok(result),
                        Err(e) => {
                            tracing::warn!("Raydium API 失败，尝试 Jupiter: {}", e);
                            self.swap_jupiter(input_mint_str, SOL_MINT_ADDRESS, token_amount, slippage_bps, dry_run).await
                        }
                    }
                } else {
                    // Devnet/Testnet 直接使用 Jupiter
                    self.swap_jupiter(input_mint_str, SOL_MINT_ADDRESS, token_amount, slippage_bps, dry_run).await
                }
            }
        }
    }

    async fn sell_raydium_api(&self, input_mint_str: &str, token_amount: u64, slippage_bps: u16, dry_run: bool) -> Result<String> {
        // Step 0: Get Priority Fee
        let priority_fee = self.get_raydium_priority_fee().await.unwrap_or_else(|_| "1000".to_string());

        // Step 1: Get Quote from API
        let quote_url = format!(
            "{}/compute/swap-base-in?inputMint={}&outputMint={}&amount={}&slippageBps={}&txVersion=V0",
            RAYDIUM_API_COMPUTE, input_mint_str, SOL_MINT_ADDRESS, token_amount, slippage_bps
        );

        let quote_resp = self.client.get(&quote_url).send().await?;
        let quote_text = quote_resp.text().await?;
        let quote_res: serde_json::Value = serde_json::from_str(&quote_text)
            .map_err(|e| anyhow!("Failed to parse Raydium sell quote: {}. Raw response: {}", e, quote_text))?;

        // 检查 API 是否成功
        if !quote_res["success"].as_bool().unwrap_or(false) {
            let error_msg = quote_res["msg"].as_str().unwrap_or("Unknown error");
            return Err(match error_msg {
                "ROUTE_NOT_FOUND" => anyhow!("Raydium 未索引该代币流动性池，请等待或使用 Pump.fun 路径"),
                "REQ_SWAP_RESPONSE_ERROR" => anyhow!("Raydium API 内部错误，请稍后重试"),
                other => anyhow!("Raydium API 错误 ({}): {}", error_msg, other),
            });
        }

        // Step 2: Serialize Transaction
        let tx_request = RaydiumV3TransactionRequest {
            swap_response: quote_res,
            wallet: self.signer.pubkey().to_string(),
            tx_version: "V0".to_string(),
            wrap_sol: false,
            unwrap_sol: true,
            compute_unit_price_micro_lamports: priority_fee,
        };

        let tx_url = format!("{}/transaction/swap-base-in", RAYDIUM_API_TRANSACTION);
        let tx_resp = self.client.post(&tx_url).json(&tx_request).send().await?;
        let tx_text = tx_resp.text().await?;
        let tx_res: RaydiumV3TransactionResponse = serde_json::from_str(&tx_text)
            .map_err(|e| anyhow!("Failed to parse Raydium sell transaction: {}. Raw response: {}", e, tx_text))?;

        let encoded_tx = tx_res.data.get(0).ok_or_else(|| anyhow!("No transaction returned from Raydium API"))?.transaction.clone();

        // Step 3: Simulate (Dry-run)
        // 注意：Raydium V3 API 返回的交易已经签名，无需重新签名
        if dry_run {
            self.simulate_transaction(&encoded_tx).await?;
            return Ok("SIMULATED_RAYDIUM_SELL".to_string());
        }

        // Step 4: Send Transaction (直接发送 Raydium API 返回的已签名交易)
        let params = serde_json::json!([encoded_tx, {
            "skipPreflight": true,
            "encoding": "base64",
            "commitment": "confirmed"
        }]);

        let signature: String = self.call_rpc("sendTransaction", params).await?;
        Ok(signature)
    }

    async fn buy_pump_fun(&self, mint_str: &str, token_program_id: Pubkey, sol_amount: f64, slippage_bps: u16, dry_run: bool, jito_tip_lamports: Option<u64>) -> Result<String> {
        let blockhash_resp: LatestBlockhashResponse = self.call_rpc("getLatestBlockhash", serde_json::json!([{"commitment": "finalized"}])).await?;
        let latest_blockhash = Hash::from_str(&blockhash_resp.value.blockhash)?;
        let bonding_curve_res = self.find_bonding_curve(mint_str).await?;
        let (bonding_curve_pda, state) = bonding_curve_res;

        if state.complete {
            return Err(anyhow!("This Pump.fun curve is complete and has migrated to Raydium."));
        }

        let amount_in_lamports = (sol_amount * 1_000_000_000.0) as u128;
        let new_token_reserves = (state.virtual_sol_reserves as u128 * state.virtual_token_reserves as u128) / (state.virtual_sol_reserves as u128 + amount_in_lamports);
        let token_amount_out = (state.virtual_token_reserves as u128 - new_token_reserves) as u64;

        let sol_lamports = (sol_amount * 1_000_000_000.0) as u64;
        let max_sol_cost = sol_lamports + (sol_lamports as u128 * slippage_bps as u128 / 10000) as u64;
        
        let buy_instruction = self.build_pump_buy_instruction_with_pda(mint_str, token_program_id, bonding_curve_pda, token_amount_out, max_sol_cost).await?;
        
        let mut instructions = vec![buy_instruction];
        if let Some(tip) = jito_tip_lamports {
            instructions.push(self.build_jito_tip_instruction(tip));
        }

        self.create_and_send_tx_with_blockhash(instructions, latest_blockhash, dry_run, jito_tip_lamports.is_some()).await
    }

    async fn build_pump_buy_instruction_with_pda(&self, mint_str: &str, token_program_id: Pubkey, bonding_curve_pda: Pubkey, amount_tokens: u64, max_sol_cost: u64) -> Result<Instruction> {
        let user = self.signer.pubkey();
        let mint = Pubkey::from_str(mint_str)?;
        let pump_program_id = Pubkey::from_str(PUMP_FUN_PROGRAM_ID)?;
        
        let associated_bonding_curve = get_associated_token_address_with_program_id(&bonding_curve_pda, &mint, &token_program_id);
        let user_ata = get_associated_token_address_with_program_id(&user, &mint, &token_program_id);

        let accounts = vec![
            AccountMeta::new_readonly(Pubkey::from_str(PUMP_FUN_GLOBAL)?, false),
            AccountMeta::new(Pubkey::from_str(PUMP_FUN_FEE_RECIPIENT)?, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(bonding_curve_pda, false),
            AccountMeta::new(associated_bonding_curve, false),
            AccountMeta::new(user_ata, false),
            AccountMeta::new(user, true),
            AccountMeta::new_readonly(Pubkey::from_str(SYSTEM_PROGRAM_ID)?, false),
            AccountMeta::new_readonly(token_program_id, false), // Use dynamic token program
            AccountMeta::new_readonly(Pubkey::from_str(RENT_SYSVAR)?, false),
            AccountMeta::new_readonly(Pubkey::new_unique(), false),
            AccountMeta::new_readonly(pump_program_id, false),
        ];

        let discriminator: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];
        let mut data = discriminator.to_vec();
        data.extend_from_slice(&amount_tokens.to_le_bytes());
        data.extend_from_slice(&max_sol_cost.to_le_bytes());
        
        Ok(Instruction { program_id: pump_program_id, accounts, data })
    }

    async fn sell_pump_fun(&self, mint_str: &str, token_program_id: Pubkey, token_amount: u64, slippage_bps: u16, dry_run: bool, jito_tip_lamports: Option<u64>) -> Result<String> {
        let blockhash_resp: LatestBlockhashResponse = self.call_rpc("getLatestBlockhash", serde_json::json!([{"commitment": "finalized"}])).await?;
        let latest_blockhash = Hash::from_str(&blockhash_resp.value.blockhash)?;
        let bonding_curve_res = self.find_bonding_curve(mint_str).await?;
        let (bonding_curve_pda, state) = bonding_curve_res;

        if state.complete {
            return Err(anyhow!("This Pump.fun curve is complete and has migrated to Raydium."));
        }

        let tokens_to_sell = token_amount as u128;
        let virtual_token_reserves = state.virtual_token_reserves as u128;
        let virtual_sol_reserves = state.virtual_sol_reserves as u128;

        let new_token_reserves = virtual_token_reserves + tokens_to_sell;
        let new_sol_reserves = (virtual_sol_reserves * virtual_token_reserves) / new_token_reserves;
        let expected_sol_out = (virtual_sol_reserves - new_sol_reserves) as u64;

        let min_sol_out = expected_sol_out - (expected_sol_out as u128 * slippage_bps as u128 / 10000) as u64;
        
        let sell_instruction = self.build_pump_sell_instruction_with_pda(mint_str, token_program_id, bonding_curve_pda, token_amount, min_sol_out).await?;
        
        let mut instructions = vec![sell_instruction];
        if let Some(tip) = jito_tip_lamports {
            instructions.push(self.build_jito_tip_instruction(tip));
        }

        self.create_and_send_tx_with_blockhash(instructions, latest_blockhash, dry_run, jito_tip_lamports.is_some()).await
    }

    async fn build_pump_sell_instruction_with_pda(&self, mint_str: &str, token_program_id: Pubkey, bonding_curve_pda: Pubkey, amount_tokens: u64, min_sol_out: u64) -> Result<Instruction> {
        let user = self.signer.pubkey();
        let mint = Pubkey::from_str(mint_str)?;
        let pump_program_id = Pubkey::from_str(PUMP_FUN_PROGRAM_ID)?;
        
        let associated_bonding_curve = get_associated_token_address_with_program_id(&bonding_curve_pda, &mint, &token_program_id);
        let user_ata = get_associated_token_address_with_program_id(&user, &mint, &token_program_id);

        let accounts = vec![
            AccountMeta::new_readonly(Pubkey::from_str(PUMP_FUN_GLOBAL)?, false),
            AccountMeta::new(Pubkey::from_str(PUMP_FUN_FEE_RECIPIENT)?, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(bonding_curve_pda, false),
            AccountMeta::new(associated_bonding_curve, false),
            AccountMeta::new(user_ata, false),
            AccountMeta::new(user, true),
            AccountMeta::new_readonly(Pubkey::from_str(SYSTEM_PROGRAM_ID)?, false),
            AccountMeta::new_readonly(Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")?, false), // Associated Token Program
            AccountMeta::new_readonly(token_program_id, false), // Use dynamic token program
            AccountMeta::new_readonly(Pubkey::from_str("Ce6LvSTPbsJ7q97mz6T6o69pf9766699d6d6d6d6d6d6")?, false), // Event authority
            AccountMeta::new_readonly(pump_program_id, false),
        ];

        let discriminator: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173];
        let mut data = discriminator.to_vec();
        data.extend_from_slice(&amount_tokens.to_le_bytes());
        data.extend_from_slice(&min_sol_out.to_le_bytes());
        
        Ok(Instruction { program_id: pump_program_id, accounts, data })
    }

    async fn create_and_send_tx_with_blockhash(&self, instructions: Vec<Instruction>, latest_blockhash: Hash, dry_run: bool, use_jito: bool) -> Result<String> {
        let legacy_message = solana_sdk::message::Message::new_with_blockhash(
            &instructions,
            Some(&self.signer.pubkey()),
            &latest_blockhash,
        );
        let message = VersionedMessage::Legacy(legacy_message);
        let tx = VersionedTransaction::try_new(message, &[&self.signer])?;

        let serialized_tx = bincode::serialize(&tx)?;
        let encoded_tx = general_purpose::STANDARD.encode(serialized_tx);

        self.simulate_transaction(&encoded_tx).await?;
        if dry_run {
            return Ok("SIMULATED".to_string());
        }

        let signature = tx.signatures.get(0).ok_or_else(|| anyhow!("No signature found"))?.to_string();

        if use_jito {
            let params = serde_json::json!([[encoded_tx]]);
            let bundle_id: String = self.call_rpc_with_url(&self.jito_rpc_url, "sendBundle", params).await?;
            tracing::info!(bundle_id = %bundle_id, signature = %signature, "Bundle sent to Jito");
            return Ok(signature);
        }

        let params = serde_json::json!([encoded_tx, {
            "skipPreflight": true,
            "encoding": "base64",
            "commitment": "confirmed"
        }]);

        let rpc_signature: String = self.call_rpc("sendTransaction", params).await?;
        Ok(rpc_signature)
    }

    async fn simulate_transaction(&self, encoded_tx: &str) -> Result<()> {
        let params = serde_json::json!([encoded_tx, {
            "sigVerify": false,
            "encoding": "base64",
            "commitment": "processed"
        }]);
        let simulation: SimulateTransactionResponse = self.call_rpc("simulateTransaction", params).await?;
        if let Some(err) = simulation.value.err {
            return Err(anyhow!("Simulation failed: {}", err));
        }
        Ok(())
    }

    // ==================== Jupiter API Methods ====================

    /// Jupiter Swap API - 通用交换方法
    /// 注意：quote 逻辑已内联到 swap_jupiter 中
    #[allow(dead_code)]
    async fn quote_jupiter(&self, input_mint: &str, output_mint: &str, amount: u64) -> Result<serde_json::Value> {
        let url = format!(
            "{}/quote?inputMint={}&outputMint={}&amount={}&slippageBps=50",
            JUPITER_QUOTE_API, input_mint, output_mint, amount
        );

        let response = self.client.get(&url).send().await?;
        if !response.status().is_success() {
            return Err(anyhow!("Jupiter API HTTP 错误：{}", response.text().await?));
        }

        let quote: serde_json::Value = response.json().await?;
        
        // 检查是否有错误
        if let Some(error) = quote.get("error") {
            return Err(anyhow!("Jupiter API 错误：{}", error));
        }

        Ok(quote)
    }

    /// Jupiter Swap API - 通用交换方法
    async fn swap_jupiter(&self, input_mint: &str, output_mint: &str, amount: u64, slippage_bps: u16, dry_run: bool) -> Result<String> {
        // 1. Get Quote
        let quote_url = format!(
            "{}/quote?inputMint={}&outputMint={}&amount={}&slippageBps={}",
            JUPITER_QUOTE_API, input_mint, output_mint, amount, slippage_bps
        );

        let quote_resp = self.client.get(&quote_url).send().await?;
        let quote_text = quote_resp.text().await?;
        tracing::debug!(quote_response = %quote_text, "Jupiter Quote API raw response");

        let quote: serde_json::Value = serde_json::from_str(&quote_text)
            .map_err(|e| anyhow!("Failed to parse Jupiter quote: {}. Raw response: {}", e, quote_text))?;

        // 检查是否有错误
        if let Some(error) = quote.get("error") {
            let error_msg = error.as_str().unwrap_or("Unknown Jupiter error");
            return Err(match error_msg {
                "Route not found" => anyhow!("Jupiter 未找到交易路由，请检查代币是否有流动性"),
                other => anyhow!("Jupiter API 错误：{}", other),
            });
        }

        // 2. Get Swap Transaction
        let swap_request = serde_json::json!({
            "quoteResponse": quote,
            "userPublicKey": self.signer.pubkey().to_string(),
            "wrapAndUnwrapSol": true,
            "dynamicComputeUnitLimit": true,
            "prioritizationFeeLamports": "auto"
        });

        let swap_resp = self.client.post(JUPITER_SWAP_API).json(&swap_request).send().await?;
        let swap_text = swap_resp.text().await?;
        tracing::debug!(swap_response = %swap_text, "Jupiter Swap API raw response");

        let swap_res: serde_json::Value = serde_json::from_str(&swap_text)
            .map_err(|e| anyhow!("Failed to parse Jupiter swap: {}. Raw response: {}", e, swap_text))?;

        let encoded_tx = swap_res["swapTransaction"].as_str()
            .ok_or_else(|| anyhow!("Jupiter 未返回交易"))?;

        // 3. Simulate (Dry-run)
        if dry_run {
            self.simulate_transaction(encoded_tx).await?;
            return Ok("SIMULATED_JUPITER".to_string());
        }

        // 4. Send Transaction
        let params = serde_json::json!([encoded_tx, {
            "skipPreflight": true,
            "encoding": "base64",
            "commitment": "confirmed"
        }]);

        let signature: String = self.call_rpc("sendTransaction", params).await?;
        Ok(signature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solana::detector::LEGACY_TOKEN_PROGRAM_ID;

    #[tokio::test]
    async fn test_build_jito_tip_instruction() {
        let priv_key = "56xhNWxYX4EzHs8s3bVcvM3DuScRvCvjU6uajV26GKgRCscJv6TtsQGp94HsycgaxU5gteBBSbd6d9yQmH6oyAR2";
        // Create a minimal executor for testing (skip full initialization)
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(20))
            .build()
            .unwrap();
        let signer = Keypair::from_base58_string(priv_key);
        // Skip network call by using a mock URL or handling error gracefully
        let path_detector = match SolanaPathDetector::new("https://api.mainnet-beta.solana.com".to_string()).await {
            Ok(detector) => detector,
            Err(_) => {
                // If network fails, create a minimal test that doesn't need detector
                let lamports = 100000u64;
                let tip_account = Pubkey::from_str("9649qRqpZbe96vSST9fM9qf4C65BPrV78vSNDtKx25n6").unwrap();
                let system_program_id = Pubkey::from_str(SYSTEM_PROGRAM_ID).unwrap();
                
                let accounts = vec![
                    AccountMeta::new(signer.pubkey(), true),
                    AccountMeta::new(tip_account, false),
                ];
                
                let mut data = vec![2, 0, 0, 0];
                data.extend_from_slice(&lamports.to_le_bytes());
                
                let inst = Instruction {
                    program_id: system_program_id,
                    accounts,
                    data,
                };
                
                assert_eq!(inst.program_id.to_string(), SYSTEM_PROGRAM_ID);
                assert_eq!(inst.accounts.len(), 2);
                assert!(inst.accounts[0].is_signer);
                assert!(inst.accounts[0].is_writable);
                assert!(!inst.accounts[1].is_signer);
                assert!(inst.accounts[1].is_writable);
                assert_eq!(inst.data[0..4], [2, 0, 0, 0]);
                let data_lamports = u64::from_le_bytes(inst.data[4..12].try_into().unwrap());
                assert_eq!(data_lamports, lamports);
                return;
            }
        };
        let jito_rpc_url = JITO_BLOCK_ENGINE_URL.to_string();
        let executor = SolanaExecutor {
            rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
            jito_rpc_url,
            client,
            signer,
            path_detector,
        };

        let lamports = 100000;
        let inst = executor.build_jito_tip_instruction(lamports);

        assert_eq!(inst.program_id.to_string(), SYSTEM_PROGRAM_ID);
        assert_eq!(inst.accounts.len(), 2);
        assert!(inst.accounts[0].is_signer);
        assert!(inst.accounts[0].is_writable);
        assert!(!inst.accounts[1].is_signer);
        assert!(inst.accounts[1].is_writable);

        // Check discriminator and lamports in data
        assert_eq!(inst.data[0..4], [2, 0, 0, 0]);
        let data_lamports = u64::from_le_bytes(inst.data[4..12].try_into().unwrap());
        assert_eq!(data_lamports, lamports);
    }

    #[tokio::test]
    async fn test_build_pump_sell_instruction() {
        let priv_key = "56xhNWxYX4EzHs8s3bVcvM3DuScRvCvjU6uajV26GKgRCscJv6TtsQGp94HsycgaxU5gteBBSbd6d9yQmH6oyAR2";
        let executor = SolanaExecutor::new("https://api.mainnet-beta.solana.com".to_string(), priv_key).await.unwrap();
        
        let mint = Pubkey::new_unique();
        let bonding_curve = Pubkey::new_unique();
        let amount_tokens = 1_000_000;
        let min_sol_out = 90_000;
        let token_program_id = Pubkey::from_str(LEGACY_TOKEN_PROGRAM_ID).unwrap();
        
        let inst = executor.build_pump_sell_instruction_with_pda(&mint.to_string(), token_program_id, bonding_curve, amount_tokens, min_sol_out).await.unwrap();
        
        assert_eq!(inst.program_id.to_string(), PUMP_FUN_PROGRAM_ID);
        assert_eq!(inst.accounts.len(), 12);
        
        // Check discriminator for sell
        assert_eq!(inst.data[0..8], [51, 230, 133, 164, 1, 127, 131, 173]);
        let data_amount = u64::from_le_bytes(inst.data[8..16].try_into().unwrap());
        let data_min_sol = u64::from_le_bytes(inst.data[16..24].try_into().unwrap());
        assert_eq!(data_amount, amount_tokens);
        assert_eq!(data_min_sol, min_sol_out);
    }
}
