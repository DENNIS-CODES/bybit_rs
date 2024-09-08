#![allow(unused)]
use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use bybit_rs::bybit::{
    asset::{self, Asset},
    http_manager::{HttpManager, Manager},
    market::{self, Market},
    trade::{self, Trade},
};

use bybit_rs::errors::app_error::AppError;
use serde_json::Value;

fn main() {
    dotenv::dotenv().ok();
    let rt = tokio::runtime::Builder::new_current_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();
    if let Err(err) = rt.block_on(start_app()) {
        println!("Error: {}", err);
    }
}

async fn start_app() -> Result<(), AppError> {
    let http_api_key =
        std::env::var("API_KEY").map_err(|_| AppError::EnvVarMissing("API_KEY".to_string()))?;
    let http_api_secret = std::env::var("API_SECRET")
        .map_err(|_| AppError::EnvVarMissing("API_SECRET".to_string()))?;

    let testnet_str =
        std::env::var("API_TEST").map_err(|_| AppError::EnvVarMissing("API_TEST".to_string()))?;
    let testnet = testnet_str == "true";

    // Create a manager
    let manager = Arc::new(HttpManager::new(http_api_key, http_api_secret, testnet));

    println!("============ GET KLINE  =========== ");
    let mut query: HashMap<String, String> = HashMap::new();
    query.insert("category".to_string(), "inverse".to_string());
    query.insert("symbol".to_string(), "BTCUSD".to_string());
    query.insert("interval".to_string(), "60".to_string());
    query.insert("start".to_string(), "1670601600000".to_string());
    query.insert("end".to_string(), "1670608800000".to_string());

    let market: market::MarketHTTP = market::MarketHTTP::new(manager.clone());

    match market.get_kline(query).await {
        Ok(result) => println!("{:?}", result),
        Err(e) => println!("{:?}", e),
    }

    println!("============== PLACE AN ACTIVE ORDER  =============== ");
    let mut query: HashMap<String, String> = HashMap::new();
    query.insert("category".to_owned(), "linear".to_owned());
    query.insert("symbol".to_owned(), "BTCUSDT".to_owned());
    query.insert("orderType".to_owned(), "Limit".to_owned());
    query.insert("qty".to_owned(), "0.06".to_owned());
    query.insert("price".to_owned(), "25000".to_owned());
    query.insert("side".to_owned(), "Buy".to_owned());
    query.insert("timeInForce".to_owned(), "GTC".to_owned());

    let trade: trade::TradeHTTP = trade::TradeHTTP::new(manager.clone());

    match trade.place_order(query).await {
        Ok(result) => println!("{:?}", result),
        Err(e) => println!("{:?}", e),
    }

    println!("============ GET A SINGLE ORDER =========== ");
    let mut query: HashMap<String, String> = HashMap::new();
    query.insert("category".to_owned(), "linear".to_owned());
    query.insert("limit".to_owned(), "1".to_owned());
    query.insert("symbol".to_owned(), "BTCUSDT".to_owned());
    query.insert("openOnly".to_owned(), "0".to_owned());

    match trade.get_open_orders(query).await {
        Ok(result) => println!("{:?}", result),
        Err(e) => println!("{:?}", e),
    }
    println!("============ GET ORDER HISTORY =========== ");
    let mut query: HashMap<String, String> = HashMap::new();
    query.insert("category".to_owned(), "linear".to_owned());
    query.insert("limit".to_owned(), "1".to_owned());

    match trade.get_order_history(query).await {
        Ok(result) => println!("{:?}", result),
        Err(e) => println!("{:?}", e),
    }

    println!("============ CANCEL A SINGLE ORDER  ================");
    let mut query: HashMap<String, String> = HashMap::new();
    query.insert("category".to_owned(), "linear".to_owned());
    query.insert(
        "orderId".to_owned(),
        "18451585-03d7-4853-897b-8e11738f3495".to_owned(),
    );
    query.insert("symbol".to_owned(), "BTCUSDT".to_owned());
    query.insert("settleCoin".to_owned(), "USDT".to_owned());

    match trade.cancel_order(query).await {
        Ok(result) => println!("{:?}", result),
        Err(e) => println!("{:?}", e),
    }

    println!("============= CANCEL ALL ORDERS =================");
    let mut query: HashMap<String, String> = HashMap::new();
    query.insert("category".to_owned(), "linear".to_owned());
    query.insert("symbol".to_owned(), "".to_owned());
    query.insert("settleCoin".to_owned(), "USDT".to_owned());

    match trade.cancel_all_orders(query).await {
        Ok(result) => println!("{:?}", result),
        Err(e) => println!("{:?}", e),
    }

    Ok(())
}
