use actix_web::{web, App, HttpServer, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest::Client;
use actix_cors::Cors;

use actix_files::Files;

// Serve frontend
async fn index() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../../frontend/index.html")))
}


// Ethereum
use ethers::{
    providers::{Provider, Http},
    types::Address,
};
use ethers::providers::Middleware; 

// Solana
use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, native_token::LAMPORTS_PER_SOL};

#[derive(Debug, Serialize, Deserialize)]
struct TradeRequest {
    account_size: f64,
    risk_dollars: f64,
    entry_price: f64,
    stop_loss: f64,
    target_price: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TradeResponse {
    units_to_buy: f64,
    total_position_size: f64,
    risk_per_share: f64,
    risk_percentage: f64,
    risk_reward_ratio: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct WalletInfo {
    address: String,
    chain: String,
    balances: HashMap<String, f64>,
    total_value_usd: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct WalletConnectRequest {
    address: String,
    chain: String,
}

// ETHEREUM RPC
async fn get_eth_balance(address: &str) -> Result<f64, Box<dyn std::error::Error>> {
    println!("🔍 Querying ETH balance for: {}", address);
    
    let provider = Provider::<Http>::try_from("https://eth.llamarpc.com")?;
    let eth_address: Address = address.parse()?;
    
    let balance_wei = provider.get_balance(eth_address, None).await?;
    let balance_eth = balance_wei.as_u128() as f64 / 1e18;
    
    println!("✅ ETH Balance: {} ETH", balance_eth);
    Ok(balance_eth)
}

// BITCOIN RPC - Using blockchain.com API
async fn get_btc_balance(address: &str) -> Result<f64, Box<dyn std::error::Error>> {
    println!("🔍 Querying BTC balance for: {}", address);
    
    let client = Client::new();
    
    // Using blockchain.com API
    let url = format!("https://blockchain.info/q/addressbalance/{}", address);
    let response_text = client.get(&url).send().await?.text().await?;
    
    // Parse the balance in satoshis
    let balance_satoshis: u64 = response_text.trim().parse()?;
    let balance_btc = balance_satoshis as f64 / 100_000_000.0;
    
    println!("✅ BTC Balance: {} BTC", balance_btc);
    Ok(balance_btc)
}

// SOLANA RPC
async fn get_sol_balance(address: &str) -> Result<f64, Box<dyn std::error::Error>> {
    println!("🔍 Querying SOL balance for: {}", address);
    
    let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
    let address = address.to_string();
    
    // Use tokio::task::spawn_blocking with thread-safe errors
    let balance = tokio::task::spawn_blocking(move || {
        let client = RpcClient::new(rpc_url);
        let pubkey: Pubkey = match address.parse() {
            Ok(pk) => pk,
            Err(e) => return Err(e.to_string()),
        };
        
        let balance_lamports = match client.get_balance(&pubkey) {
            Ok(bal) => bal,
            Err(e) => return Err(e.to_string()),
        };
        
        let balance_sol = balance_lamports as f64 / LAMPORTS_PER_SOL as f64;
        Ok::<f64, String>(balance_sol)
    })
    .await
    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)? // Handle JoinError
    .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)) as Box<dyn std::error::Error>)?; // Convert String to Error
    
    println!("✅ SOL Balance: {} SOL", balance);
    Ok(balance)
}

// PRICE API
async fn get_crypto_price(coin_id: &str) -> Result<f64, Box<dyn std::error::Error>> {
    println!("🔍 Querying price for: {}", coin_id);
    
    let client = Client::new();
    let url = format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd", coin_id);
    
    let response: HashMap<String, HashMap<String, f64>> = client
        .get(&url)
        .send()
        .await?
        .json()
        .await?;
    
    if let Some(coin_data) = response.get(coin_id) {
        if let Some(price) = coin_data.get("usd") {
            println!("✅ {} Price: ${}", coin_id, price);
            return Ok(*price);
        }
    }
    
    Err("Price not found".into())
}

async fn calculate_position(trade_req: web::Json<TradeRequest>) -> Result<HttpResponse> {
    println!("Calculating position for: {:?}", trade_req);
    
    if trade_req.account_size <= 0.0 {
        return Ok(HttpResponse::BadRequest().body("Account size must be positive"));
    }
    if trade_req.risk_dollars <= 0.0 {
        return Ok(HttpResponse::BadRequest().body("Risk amount must be positive"));
    }
    if trade_req.entry_price <= 0.0 {
        return Ok(HttpResponse::BadRequest().body("Entry price must be positive"));
    }
    if trade_req.stop_loss <= 0.0 {
        return Ok(HttpResponse::BadRequest().body("Stop loss must be positive"));
    }
    if trade_req.risk_dollars > trade_req.account_size {
        return Ok(HttpResponse::BadRequest().body("Risk amount cannot exceed account size"));
    }
    
    let risk_percentage = (trade_req.risk_dollars / trade_req.account_size) * 100.0;
    let risk_per_share = (trade_req.entry_price - trade_req.stop_loss).abs();
    
    if risk_per_share == 0.0 {
        return Ok(HttpResponse::BadRequest().body("Entry and stop loss prices are too close"));
    }
    
    let units_to_buy = trade_req.risk_dollars / risk_per_share;
    let total_position_size = units_to_buy * trade_req.entry_price;
    
    let risk_reward_ratio = match trade_req.target_price {
        Some(target) if target > 0.0 => {
            let reward = (target - trade_req.entry_price).abs();
            format!("1:{:.2}", reward / risk_per_share)
        }
        _ => "N/A".to_string()
    };
    
    let response = TradeResponse {
        units_to_buy,
        total_position_size,
        risk_per_share,
        risk_percentage,
        risk_reward_ratio,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

async fn connect_wallet(wallet_req: web::Json<WalletConnectRequest>) -> Result<HttpResponse> {
    println!("🚀 Connecting wallet: {} on chain: {}", wallet_req.address, wallet_req.chain);
    
    let mut balances = HashMap::new();
    let total_value_usd;

    match wallet_req.chain.as_str() {
        "ethereum" => {
            match get_eth_balance(&wallet_req.address).await {
                Ok(eth_balance) => {
                    match get_crypto_price("ethereum").await {
                        Ok(eth_price) => {
                            balances.insert("ETH".to_string(), eth_balance);
                            total_value_usd = eth_balance * eth_price;
                            println!("💰 Final: {} ETH = ${}", eth_balance, total_value_usd);
                        }
                        Err(e) => {
                            eprintln!("❌ ETH price error: {}", e);
                            return Ok(HttpResponse::InternalServerError().body("Failed to get ETH price"));
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ ETH balance error: {}", e);
                    return Ok(HttpResponse::BadRequest().body(format!("Invalid Ethereum address or RPC error: {}", e)));
                }
            }
        }
        "bitcoin" => {
            match get_btc_balance(&wallet_req.address).await {
                Ok(btc_balance) => {
                    match get_crypto_price("bitcoin").await {
                        Ok(btc_price) => {
                            balances.insert("BTC".to_string(), btc_balance);
                            total_value_usd = btc_balance * btc_price;
                            println!("💰 Final: {} BTC = ${}", btc_balance, total_value_usd);
                        }
                        Err(e) => {
                            eprintln!("❌ BTC price error: {}", e);
                            return Ok(HttpResponse::InternalServerError().body("Failed to get BTC price"));
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ BTC balance error: {}", e);
                    return Ok(HttpResponse::BadRequest().body(format!("Invalid Bitcoin address or RPC error: {}", e)));
                }
            }
        }
        "solana" => {
            match get_sol_balance(&wallet_req.address).await {
                Ok(sol_balance) => {
                    match get_crypto_price("solana").await {
                        Ok(sol_price) => {
                            balances.insert("SOL".to_string(), sol_balance);
                            total_value_usd = sol_balance * sol_price;
                            println!("💰 Final: {} SOL = ${}", sol_balance, total_value_usd);
                        }
                        Err(e) => {
                            eprintln!("❌ SOL price error: {}", e);
                            return Ok(HttpResponse::InternalServerError().body("Failed to get SOL price"));
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ SOL balance error: {}", e);
                    return Ok(HttpResponse::BadRequest().body(format!("Invalid Solana address or RPC error: {}", e)));
                }
            }
        }
        _ => {
            return Ok(HttpResponse::BadRequest().body("Unsupported chain. Use: ethereum, bitcoin, solana"));
        }
    }

    let wallet_info = WalletInfo {
        address: wallet_req.address.clone(),
        chain: wallet_req.chain.clone(),
        balances,
        total_value_usd,
    };
    
    Ok(HttpResponse::Ok().json(wallet_info))
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("Trading Calculator API is running!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get port from environment variable (Cloud Run provides this)
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("0.0.0.0:{}", port);

    println!("🔥 Starting Trading Calculator Web3 Backend on http://{}", bind_address);

    HttpServer::new(|| {
        // CORS CONFIGURATION
        let cors = Cors::default()  // Remove underscore to use the variable
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)  // Use the variable here
            // Serve the main page
            .route("/", web::get().to(index))
            // Serve static files (CSS, JS, etc.)
            .service(Files::new("/css", "frontend/css").show_files_listing())
            .service(Files::new("/js", "frontend/js").show_files_listing())
            // Existing API routes
            .route("/health", web::get().to(health_check))
            .route("/calculate", web::post().to(calculate_position))
            .route("/wallet/connect", web::post().to(connect_wallet))
    })
        .bind(&bind_address)?
        .run()
        .await
}