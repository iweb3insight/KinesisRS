// src/main.rs

use clap::Parser;
use alloy_primitives::U256;
use std::str::FromStr;
use solana_sdk::pubkey::Pubkey;
use solana_claw_coin_cli::{
    cli::{Cli, Commands},
    config::Config,
    types::{Stage, TradeResult, TradeError, Chain},
    bsc::executor::{BscExecutor, ExecutorError},
    solana::executor::SolanaExecutor,
    // solana::detector::Path, // Removed as it is unused.
};

const SOL_MINT_ADDRESS: &str = "So11111111111111111111111111111111111111112";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = match Config::load() {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("Failed to load configuration: {}", err);
            std::process::exit(1);
        }
    };

    let cli = Cli::parse();
    let dry_run = cli.dry_run && !cli.no_dry_run;

    match &cli.command {
        Commands::Buy(args) => {
            if let Err(e) = solana_claw_coin_cli::cli::validate_args(args.slippage, args.tip_rate) {
                eprintln!("Invalid arguments: {}", e);
                std::process::exit(1);
            }
            let start_time = std::time::Instant::now();
            let mut stages = Vec::new();

            stages.push(Stage {
                name: "cli_parse".to_string(),
                duration_ms: start_time.elapsed().as_millis() as u64,
                input: Some(serde_json::json!(args)),
                output: None,
            });

            let executor_init_start = std::time::Instant::now();
            stages.push(Stage {
                name: "executor_init".to_string(),
                duration_ms: executor_init_start.elapsed().as_millis() as u64,
                input: None,
                output: None,
            });

            if args.chain == Chain::Solana {
                let sol_key = config.get_sol_private_key(cli.wallet).expect("Failed to load private key for selected wallet");
                let executor = SolanaExecutor::new(config.sol_rpc_url.clone(), &sol_key)
                    .await
                    .expect("Failed to create Solana executor");

                let mut result = TradeResult {
                    success: false,
                    chain: args.chain,
                    stages,
                    tx_hash: None,
                    amount_out: None,
                    gas_used: None,
                    gas_estimate: None,
                    price_impact_percent: None,
                    route_info: None,
                    error: None,
                };

                let buy_start = std::time::Instant::now();
                let jito_tip_lamports = args.jito_tip.map(|sol| (sol * 1_000_000_000.0) as u64);
                match executor.buy(&args.token_address, args.amount, (args.slippage * 100.0) as u16, dry_run, jito_tip_lamports).await {
                    Ok(sig) => {
                        result.success = true;
                        result.tx_hash = Some(sig);
                        result.stages.push(Stage {
                            name: "buy".to_string(),
                            duration_ms: buy_start.elapsed().as_millis() as u64,
                            input: Some(serde_json::json!({ "amount": args.amount, "slippage_bps": (args.slippage * 100.0) as u16, "jito_tip": args.jito_tip })),
                            output: Some(serde_json::json!({ "status": "success" })),
                        });
                    }
                    Err(e) => {
                        result.success = false;
                        result.error = Some(TradeError::ContractError { message: e.to_string() });
                    }
                }

                if cli.json {
                    println!("{}", serde_json::to_string(&result).unwrap());
                } else {
                    println!("{}", serde_json::to_string_pretty(&result).unwrap());
                }
                return;
            }

            let bsc_key = config.get_bsc_private_key(cli.wallet).expect("Failed to load private key for selected wallet");
            let executor = BscExecutor::new(config.clone(), bsc_key).await.expect("Failed to create executor");
            let token_addr = args.token_address.parse().expect("Invalid BSC token address");
            let amount_in = alloy::primitives::utils::parse_ether(&args.amount.to_string()).expect("Invalid amount");

            let quote_start = std::time::Instant::now();
            let quote_result = executor.quote_buy(token_addr, amount_in).await;
            let quote_duration = quote_start.elapsed().as_millis() as u64;

            let mut result = TradeResult {
                success: false,
                chain: args.chain,
                stages,
                tx_hash: None,
                amount_out: None,
                gas_used: None,
                gas_estimate: None,
                price_impact_percent: None,
                route_info: None,
                error: None,
            };

            match quote_result {
                Ok(amount_out) => {
                    result.amount_out = Some(amount_out.to_string());
                    result.stages.push(Stage {
                        name: "quote".to_string(),
                        duration_ms: quote_duration,
                        input: None,
                        output: Some(serde_json::json!({ "expected_out": amount_out.to_string() })),
                    });

                    let slippage_factor = 1.0 - (args.slippage / 100.0);
                    let amount_out_min = amount_out * U256::from((slippage_factor * 1000.0) as u64) / U256::from(1000);
                    let tip_rate = U256::from(args.tip_rate as u64);

                    let simulate_start = std::time::Instant::now();
                    let simulate_result = executor.buy(token_addr, amount_in, amount_out_min, tip_rate, dry_run).await;
                    let simulate_duration = simulate_start.elapsed().as_millis() as u64;

                    match simulate_result {
                        Ok(exec_res) => {
                            result.success = true;
                            result.gas_estimate = Some(exec_res.gas_used);
                            if !dry_run {
                                result.gas_used = Some(exec_res.gas_used);
                            }
                            result.stages.push(Stage {
                                name: "simulate_execution".to_string(),
                                duration_ms: simulate_duration,
                                input: Some(serde_json::json!({ "amount_out_min": amount_out_min.to_string(), "tip_rate": tip_rate.to_string() })),
                                output: Some(serde_json::json!({ "status": "success", "message": exec_res.message, "gas_used": exec_res.gas_used })),
                            });
                        }
                        Err(e) => {
                            result.success = false;
                            match e {
                                ExecutorError::SimulationFailed { revert_reason, raw_revert_data } => {
                                    result.error = Some(TradeError::SimulationFailed {
                                        revert_reason: Some(revert_reason),
                                        raw_revert_data,
                                        decoded_custom: None,
                                    });
                                }
                                ExecutorError::RpcError(msg) => {
                                    result.error = Some(TradeError::RpcError { message: msg });
                                }
                                _ => {
                                    result.error = Some(TradeError::ContractError { message: e.to_string() });
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    result.success = false;
                    match e {
                        ExecutorError::RpcError(msg) => {
                            result.error = Some(TradeError::RpcError { message: msg });
                        }
                        _ => {
                            result.error = Some(TradeError::ContractError { message: e.to_string() });
                        }
                    }
                }
            }

            if cli.json {
                println!("{}", serde_json::to_string(&result).unwrap());
            } else {
                println!("{}", serde_json::to_string_pretty(&result).unwrap());
            }
        }
        Commands::Sell(args) => {
            if let Err(e) = solana_claw_coin_cli::cli::validate_args(args.slippage, args.tip_rate) {
                eprintln!("Invalid arguments: {}", e);
                std::process::exit(1);
            }
            let start_time = std::time::Instant::now();
            let mut stages = Vec::new();

            stages.push(Stage {
                name: "cli_parse".to_string(),
                duration_ms: start_time.elapsed().as_millis() as u64,
                input: Some(serde_json::json!(args)),
                output: None,
            });

            if args.chain == Chain::Solana {
                let sol_key = config.get_sol_private_key(cli.wallet).expect("Failed to load private key for selected wallet");
                let executor = SolanaExecutor::new(config.sol_rpc_url.clone(), &sol_key)
                    .await
                    .expect("Failed to create Solana executor");

                let mut result = TradeResult {
                    success: false,
                    chain: args.chain,
                    stages,
                    tx_hash: None,
                    amount_out: None,
                    gas_used: None,
                    gas_estimate: None,
                    price_impact_percent: None,
                    route_info: None,
                    error: None,
                };

                let sell_start = std::time::Instant::now();
                let jito_tip_lamports = args.jito_tip.map(|sol| (sol * 1_000_000_000.0) as u64);
                // For Solana, sell amount is usually in base units (e.g., 1000000 for 1 token if 6 decimals)
                // However, our CLI takes f64. Assuming 6 decimals for now for SPL tokens.
                // TODO: Better decimal handling.
                let amount_base = (args.amount * 1_000_000.0) as u64; 

                match executor.sell(&args.token_address, amount_base, (args.slippage * 100.0) as u16, dry_run, jito_tip_lamports).await {
                    Ok(sig) => {
                        result.success = true;
                        result.tx_hash = Some(sig);
                        result.stages.push(Stage {
                            name: "sell".to_string(),
                            duration_ms: sell_start.elapsed().as_millis() as u64,
                            input: Some(serde_json::json!({ "amount": args.amount, "slippage_bps": (args.slippage * 100.0) as u16, "jito_tip": args.jito_tip })),
                            output: Some(serde_json::json!({ "status": "success" })),
                        });
                    }
                    Err(e) => {
                        result.success = false;
                        result.error = Some(TradeError::ContractError { message: e.to_string() });
                    }
                }

                if cli.json {
                    println!("{}", serde_json::to_string(&result).unwrap());
                } else {
                    println!("{}", serde_json::to_string_pretty(&result).unwrap());
                }
                return;
            }

            let executor_init_start = std::time::Instant::now();
            let bsc_key = config.get_bsc_private_key(cli.wallet).expect("Failed to load private key for selected wallet");
            let executor = BscExecutor::new(config.clone(), bsc_key).await.expect("Failed to create executor");
            stages.push(Stage {
                name: "executor_init".to_string(),
                duration_ms: executor_init_start.elapsed().as_millis() as u64,
                input: None,
                output: None,
            });

            let token_addr = args.token_address.parse().expect("Invalid BSC token address");
            let amount_in = alloy::primitives::utils::parse_ether(&args.amount.to_string()).expect("Invalid amount");

            let approve_start = std::time::Instant::now();
            let approve_result = executor.approve_if_needed(token_addr, executor.wallet_address(), amount_in, dry_run).await;
            let approve_duration = approve_start.elapsed().as_millis() as u64;

            let mut result = TradeResult {
                success: false,
                chain: args.chain,
                stages,
                tx_hash: None,
                amount_out: None,
                gas_used: None,
                gas_estimate: None,
                price_impact_percent: None,
                route_info: None,
                error: None,
            };

            match approve_result {
                Ok(Some(exec_res)) => {
                    result.stages.push(Stage {
                        name: "approve".to_string(),
                        duration_ms: approve_duration,
                        input: None,
                        output: Some(serde_json::json!({ "status": "approved", "message": exec_res.message, "gas_used": exec_res.gas_used })),
                    });
                }
                Ok(None) => {
                    result.stages.push(Stage {
                        name: "approve".to_string(),
                        duration_ms: approve_duration,
                        input: None,
                        output: Some(serde_json::json!({ "status": "skipped", "reason": "allowance sufficient" })),
                    });
                }
                Err(e) => {
                    result.success = false;
                    result.error = Some(TradeError::ContractError { message: format!("Approval failed: {}", e) });
                    if cli.json {
                        println!("{}", serde_json::to_string(&result).unwrap());
                    } else {
                        println!("{}", serde_json::to_string_pretty(&result).unwrap());
                    }
                    std::process::exit(1);
                }
            }

            let quote_start = std::time::Instant::now();
            let quote_result = executor.quote_sell(token_addr, amount_in).await;
            let quote_duration = quote_start.elapsed().as_millis() as u64;

            match quote_result {
                Ok(amount_out) => {
                    result.amount_out = Some(amount_out.to_string());
                    result.stages.push(Stage {
                        name: "quote".to_string(),
                        duration_ms: quote_duration,
                        input: None,
                        output: Some(serde_json::json!({ "expected_out": amount_out.to_string() })),
                    });

                    let slippage_factor = 1.0 - (args.slippage / 100.0);
                    let amount_out_min = amount_out * U256::from((slippage_factor * 1000.0) as u64) / U256::from(1000);
                    let tip_rate = U256::from(args.tip_rate as u64);

                    let simulate_start = std::time::Instant::now();
                    let simulate_result = executor.sell(token_addr, amount_in, amount_out_min, tip_rate, dry_run).await;
                    let simulate_duration = simulate_start.elapsed().as_millis() as u64;

                    match simulate_result {
                        Ok(exec_res) => {
                            result.success = true;
                            result.gas_estimate = Some(exec_res.gas_used);
                            result.stages.push(Stage {
                                name: "simulate_execution".to_string(),
                                duration_ms: simulate_duration,
                                input: Some(serde_json::json!({ "amount_out_min": amount_out_min.to_string() })),
                                output: Some(serde_json::json!({ "status": "success", "message": exec_res.message, "gas_used": exec_res.gas_used })),
                            });
                        }
                        Err(e) => {
                            result.success = false;
                            result.error = Some(TradeError::ContractError { message: e.to_string() });
                        }
                    }
                }
                Err(e) => {
                    result.success = false;
                    result.error = Some(TradeError::ContractError { message: e.to_string() });
                }
            }

            if cli.json {
                println!("{}", serde_json::to_string(&result).unwrap());
            } else {
                println!("{}", serde_json::to_string_pretty(&result).unwrap());
            }
        }
        Commands::Quote(args) => {
            if args.chain == Chain::Bsc {
                let bsc_key = config.get_bsc_private_key(cli.wallet).expect("Failed to load private key for selected wallet");
                let executor = BscExecutor::new(config.clone(), bsc_key).await.expect("Failed to create executor");

                let token_addr = args.token_address.parse().expect("Invalid BSC token address");
                let amount_in = alloy::primitives::utils::parse_ether(&args.amount.to_string()).expect("Invalid amount");

                let quote_result = if args.action == "buy" {
                    executor.quote_buy(token_addr, amount_in).await
                } else {
                    executor.quote_sell(token_addr, amount_in).await
                };

                match quote_result {
                    Ok(amount_out) => {
                        if cli.json {
                            println!("{}", serde_json::json!({ "success": true, "amount_out": amount_out.to_string() }));
                        } else {
                            println!("Quote ({}): Expected to receive {} units", args.action, amount_out);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error fetching quote: {}", e);
                        std::process::exit(1);
                    }
                }
            } else if args.chain == Chain::Solana {
                let sol_key = config.get_sol_private_key(cli.wallet).expect("Failed to load private key for selected wallet");
                let executor = SolanaExecutor::new(config.sol_rpc_url.clone(), &sol_key)
                    .await
                    .expect("Failed to create Solana executor");

                // For buys, the input is SOL. For sells, the input is the token.
                let (input_mint, output_mint, amount_lamports) = if args.action == "buy" {
                    (SOL_MINT_ADDRESS, args.token_address.as_str(), (args.amount * 1_000_000_000.0) as u64)
                } else {
                    // Assuming 6 decimals for most SPL tokens when selling
                    (args.token_address.as_str(), SOL_MINT_ADDRESS, (args.amount * 1_000_000.0) as u64)
                };

                match executor.quote(input_mint, output_mint, amount_lamports).await {
                    Ok((amount_out, path)) => {
                        if cli.json {
                            println!("{}", serde_json::json!({ "success": true, "amount_out": amount_out, "path": path }));
                        } else {
                            println!("Quote ({}): Expected to receive {} units (Path: {:?})", args.action, amount_out, path);
                        }
                    }
                    Err(e) => {
                        eprintln!("Solana Quote Error: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        }
        Commands::Balance(args) => {
            if args.chain == Chain::Bsc {
                let bsc_key = config.get_bsc_private_key(cli.wallet).expect("Failed to load private key");
                let executor = BscExecutor::new(config.clone(), bsc_key).await.expect("Failed to create executor");

                let token_addr = args.token_address.as_ref().map(|s| s.parse().expect("Invalid BSC token address"));
                let owner = executor.wallet_address();

                let balance_result = executor.get_balance(owner, token_addr).await;

                match balance_result {
                    Ok(balance) => {
                        let formatted_balance = alloy::primitives::utils::format_ether(balance);
                        let asset_name = token_addr.map(|a| format!("Token ({})", a)).unwrap_or_else(|| "Native BNB".to_string());

                        if cli.json {
                            println!("{}", serde_json::json!({ 
                                "success": true, 
                                "balance_raw": balance.to_string(),
                                "balance_formatted": formatted_balance,
                                "asset": asset_name,
                                "owner": owner.to_string() 
                            }));
                        } else {
                            println!("Balance for {} [{}]: {} BNB", owner, asset_name, formatted_balance);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error fetching balance: {}", e);
                        std::process::exit(1);
                    }
                }
            } else if args.chain == Chain::Solana {
                let sol_key = config.get_sol_private_key(cli.wallet).expect("Failed to load private key");
                let executor = SolanaExecutor::new(config.sol_rpc_url.clone(), &sol_key).await.expect("Failed to create Solana executor");

                let token_addr = args.token_address.as_ref().map(|s| Pubkey::from_str(s).expect("Invalid Solana token address"));
                let owner = executor.wallet_address();

                let balance_result = executor.get_balance(owner, token_addr).await;

                match balance_result {
                    Ok(balance) => {
                        // For SOL, balance is in lamports. For tokens, we assumed base units earlier.
                        let (formatted_balance, asset_name) = if let Some(token) = token_addr {
                            // Assuming 6 decimals for most SPL tokens for display
                            ((balance as f64 / 1_000_000.0).to_string(), format!("Token ({})", token))
                        } else {
                            ((balance as f64 / 1_000_000_000.0).to_string(), "Native SOL".to_string())
                        };

                        if cli.json {
                            println!("{}", serde_json::json!({ 
                                "success": true, 
                                "balance_raw": balance.to_string(),
                                "balance_formatted": formatted_balance,
                                "asset": asset_name,
                                "owner": owner.to_string() 
                            }));
                        } else {
                            println!("Balance for {} [{}]: {} units", owner, asset_name, formatted_balance);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error fetching Solana balance: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        }
        Commands::Approve(args) => {
            if args.chain != Chain::Bsc {
                eprintln!("Error: Approve command currently only supports BSC.");
                std::process::exit(1);
            }
            let bsc_key = config.get_bsc_private_key(cli.wallet).expect("Failed to load private key");
            let executor = BscExecutor::new(config.clone(), bsc_key).await.expect("Failed to create executor");
            let token_addr = args.token_address.parse().expect("Invalid BSC token address");
            
            let amount = match args.amount {
                Some(a) => alloy::primitives::utils::parse_ether(&a.to_string()).expect("Invalid amount"),
                None => U256::MAX,
            };

            let approve_result = executor.approve(token_addr, amount, dry_run).await;

            match approve_result {
                Ok(exec_res) => {
                    if cli.json {
                        println!("{}", serde_json::json!({ 
                            "success": true, 
                            "message": exec_res.message,
                            "gas_used": exec_res.gas_used,
                            "token": token_addr.to_string(),
                            "amount": amount.to_string()
                        }));
                    } else {
                        println!("Approval Success: {} (Gas Used: {})", exec_res.message, exec_res.gas_used);
                    }
                }
                Err(e) => {
                    eprintln!("Error during approval: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Config => {
            if cli.json {
                println!("{}", serde_json::to_string(&config).unwrap());
            } else {
                println!("Configuration:");
                println!("  BSC RPCs: {:?}", config.bsc_rpc_urls);
                println!("  SOL RPC: {}", config.sol_rpc_url);
            }
        }
        Commands::Wallet => {
            let mut addresses = serde_json::json!({});
            
            // Get BSC Address
            if let Ok(bsc_key) = config.get_bsc_private_key(cli.wallet) {
                if let Ok(executor) = BscExecutor::new(config.clone(), bsc_key).await {
                    let addr = executor.wallet_address();
                    addresses["bsc"] = serde_json::json!(addr.to_string());
                }
            }

            // Get Solana Address
            if let Ok(sol_key) = config.get_sol_private_key(cli.wallet) {
                if let Ok(executor) = SolanaExecutor::new(config.sol_rpc_url.clone(), &sol_key).await {
                    let addr = executor.wallet_address();
                    addresses["solana"] = serde_json::json!(addr.to_string());
                }
            }

            if cli.json {
                println!("{}", serde_json::to_string(&addresses).unwrap());
            } else {
                println!("Wallet Addresses (Index {}):", cli.wallet);
                if let Some(bsc) = addresses["bsc"].as_str() {
                    println!("  BSC:    {}", bsc);
                }
                if let Some(sol) = addresses["solana"].as_str() {
                    println!("  Solana: {}", sol);
                }
            }
        }
        Commands::Detect(args) => {
            if args.chain != Chain::Solana {
                eprintln!("Error: Detect command only supports Solana chain for now.");
                std::process::exit(1);
            }

            let detector = solana_claw_coin_cli::solana::detector::SolanaPathDetector::new(config.sol_rpc_url.clone())
                .await
                .expect("Failed to create SolanaPathDetector");
            
            match detector.detect_path(&args.token_address).await {
                Ok(path_info) => {
                    if cli.json {
                        println!("{}", serde_json::json!({ "success": true, "token_address": args.token_address, "path": path_info }));
                    } else {
                        println!("Solana Token Path Detected: {:?}", path_info);
                    }
                },
                Err(e) => {
                    eprintln!("Error detecting Solana path: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_claw_coin_cli::cli::{Cli, Commands};
    use serde_json::json;
    use solana_claw_coin_cli::types::{Chain, TradeError};

    #[test]
    fn test_trade_result_success_serialization() {
        let result = TradeResult {
            success: true,
            chain: Chain::Bsc,
            stages: vec![Stage {
                name: "execute".to_string(),
                duration_ms: 2500,
                input: None,
                output: None,
            }],
            tx_hash: Some("0x...".to_string()),
            amount_out: None,
            gas_used: None,
            gas_estimate: None,
            price_impact_percent: None,
            route_info: None,
            error: None,
        };

        let actual = serde_json::to_value(&result).unwrap();
        let expected = json!({
            "success": true,
            "chain": "bsc",
            "stages": [
                {
                    "name": "execute",
                    "duration_ms": 2500
                }
            ],
            "tx_hash": "0x..."
        });

        assert_eq!(actual["success"], expected["success"]);
        assert_eq!(actual["chain"], expected["chain"]);
        assert_eq!(actual["stages"][0]["name"], expected["stages"][0]["name"]);
        assert!(actual["tx_hash"].is_string());
        assert!(actual["error"].is_null());
    }

    #[test]
    fn test_trade_result_failure_serialization() {
        let result = TradeResult {
            success: false,
            chain: Chain::Solana,
            stages: vec![],
            tx_hash: None,
            amount_out: None,
            gas_used: None,
            gas_estimate: None,
            price_impact_percent: None,
            route_info: None,
            error: Some(TradeError::SimulationFailed {
                revert_reason: Some("Insufficient Liquidity".to_string()),
                raw_revert_data: None,
                decoded_custom: None,
            }),
        };

        let actual = serde_json::to_value(&result).unwrap();
        let expected = json!({
            "success": false,
            "chain": "solana",
            "stages": [],
            "error": {
                "type": "simulation_failed",
                "revert_reason": "Insufficient Liquidity",
                "raw_revert_data": null
            }
        });
        
        assert_eq!(actual["success"], expected["success"]);
        assert_eq!(actual["chain"], expected["chain"]);
        assert_eq!(actual["error"]["type"], "simulation_failed");
        assert_eq!(actual["error"]["revert_reason"], expected["error"]["revert_reason"]);
        assert!(actual["tx_hash"].is_null());
    }

    #[test]
    fn test_cli_parsing_buy_basic() {
        let cli = Cli::parse_from([
            "freedom-agent-rs",
            "--wallet",
            "2",
            "--json",
            "buy",
            "0xToken",
            "0.1",
            "--chain",
            "bsc",
        ]);

        assert!(cli.json);
        assert_eq!(cli.wallet, 2);
        assert!(dry_run);

        match cli.command {
            Commands::Buy(args) => {
                assert_eq!(args.token_address, "0xToken");
                assert_eq!(args.amount, 0.1);
                assert!(matches!(args.chain, Chain::Bsc));
                assert_eq!(args.slippage, 15.0);
            }
            _ => panic!("Incorrect command parsed"),
        }
    }
}
