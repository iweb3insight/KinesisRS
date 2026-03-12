// src/solana/detector.rs

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

// --- Constants ---
pub const LEGACY_TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
pub const TOKEN_2022_PROGRAM_ID: &str = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";
const PUMP_CREATOR_PDA: &str = "TSLp3kgYv9zLfS96Rv1ScAcDAAfKtcYnRW9tuKMvRjn";

// --- Data Structures ---

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum Path {
    PumpFun,
    Raydium,
    RaydiumGraduated,
    Unknown(String),
}

#[derive(Debug, Clone, Serialize)]
pub struct TokenPathInfo {
    pub path: Path,
    #[serde(serialize_with = "serialize_pubkey")]
    pub token_program_id: Pubkey,
}

fn serialize_pubkey<S>(pubkey: &Pubkey, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&pubkey.to_string())
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
    data: (String, String), // [base64_data, encoding]
    owner: String,
}


// --- Detector ---

pub struct SolanaPathDetector {
    rpc_url: String,
    client: reqwest::Client,
}

impl SolanaPathDetector {
    pub async fn new(rpc_url: String) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?;
        Ok(Self { rpc_url, client })
    }

    async fn call_rpc<T: for<'de> Deserialize<'de>>(&self, method: &str, params: serde_json::Value) -> Result<T> {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: method.to_string(),
            params,
        };

        let response = self.client.post(&self.rpc_url)
            .json(&request)
            .send()
            .await?;

        let rpc_res: JsonRpcResponse<T> = response.json().await?;
        
        if let Some(error) = rpc_res.error {
            return Err(anyhow!("RPC Error {}: {}", error.code, error.message));
        }

        rpc_res.result.ok_or_else(|| anyhow!("Empty result from RPC"))
    }

    pub async fn detect_path(&self, token_address: &str) -> Result<TokenPathInfo> {
        tracing::debug!(token_address, "Detecting Solana path");

        let params = serde_json::json!([
            token_address,
            { "encoding": "base64", "outputDataSlice": { "offset": 0, "length": 44 } }
        ]);

        let account_info: GetAccountInfoResponse = self.call_rpc("getAccountInfo", params).await?;
        
        let value = account_info.value.ok_or_else(|| anyhow!("Token account not found: {}", token_address))?;
        let token_program_id = Pubkey::from_str(&value.owner)?;
        
        // Dispatch based on token program
        if value.owner == LEGACY_TOKEN_PROGRAM_ID {
            let path = self.detect_legacy_path(&value.data.0, token_address).await?;
            return Ok(TokenPathInfo { path, token_program_id });
        } else if value.owner == TOKEN_2022_PROGRAM_ID {
            let path = self.detect_token2022_path(token_address).await?;
            return Ok(TokenPathInfo { path, token_program_id });
        }

        Ok(TokenPathInfo {
            path: Path::Unknown(format!("Address is not an SPL Token (Owner: {})", value.owner)),
            token_program_id,
        })
    }

    async fn detect_legacy_path(&self, base64_data: &str, token_address: &str) -> Result<Path> {
        let data = general_purpose::STANDARD.decode(base64_data)?;
        if data.len() < 44 {
            return Ok(Path::Unknown("Invalid mint data length".to_string()));
        }

        let mint_authority_option = &data[0..4];
        let mint_authority = &data[4..36];

        let has_mint_authority = mint_authority_option != [0, 0, 0, 0];
        
        let pump_creator_bytes = bs58::decode(PUMP_CREATOR_PDA).into_vec()?;

        if has_mint_authority && mint_authority == pump_creator_bytes {
            return Ok(Path::PumpFun);
        }

        if !has_mint_authority {
            if token_address.ends_with("pump") {
                return Ok(Path::RaydiumGraduated);
            }
            return Ok(Path::Raydium);
        }

        Ok(Path::Unknown(format!("Standard SPL Token with mint authority: {}", bs58::encode(mint_authority).into_string())))
    }

    async fn detect_token2022_path(&self, token_address: &str) -> Result<Path> {
        // Method: Use PumpPortal API to verify if this Token-2022 mint is from Pump.fun
        let api_url = format!("https://pumpportal.funapi.com/v1/token/{}", token_address);
        
        match self.client.get(&api_url).send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    let token_info: serde_json::Value = resp.json().await?;
                    if token_info.get("program").and_then(|p| p.as_str()) == Some("pump") {
                        let is_complete = token_info.get("complete").and_then(|c| c.as_bool()).unwrap_or(false);
                        if is_complete {
                            return Ok(Path::RaydiumGraduated);
                        }
                        return Ok(Path::PumpFun);
                    }
                }
            }
            Err(e) => {
                tracing::warn!(error = %e, "Failed to call PumpPortal API for Token-2022 detection");
            }
        }

        // Fallback for non-pump Token-2022 (assuming DEX if ends with pump suffix, else unknown)
        if token_address.ends_with("pump") {
            Ok(Path::RaydiumGraduated)
        } else {
            Ok(Path::Unknown("Standard Token-2022 without pump metadata".to_string()))
        }
    }
}
