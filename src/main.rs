#[allow(dead_code)]
use std::thread::sleep;
use rand::Rng;
use rand::thread_rng;
use std::time::Duration;
use binance::api::*;
use binance::market::*;

const API_KEY: &str = "your_api_key";
const API_SECRET: &str = "your_api_secret";

// fn get_price(market: &Market, symbol: &str) -> f64 {
//     let price = market.get_price(symbol).unwrap();
//     price.price.parse::<f64>().unwrap()
// }
// use rand::Rng;  // Add this to use the random number generator

fn get_price(market: &Market, symbol: &str) -> f64 {
    // Generate a random price between 10,000 and 60,000 for the mock-up
    // market and symbol is added to make competibility.
    thread_rng().gen_range(10000.0..60000.0)
}


fn arbitrage(market: &Market, symbol1: &str, symbol2: &str) {
    let buy_price = get_price(market, symbol1);
    let sell_price = get_price(market, symbol2);

    if buy_price < sell_price {
        let profit = sell_price - buy_price;
        println!(
            "Buy {} on Binance at {}, sell on another exchange at {}, profit: {}",
            symbol1, buy_price, sell_price, profit
        );
    } else {
        println!("No arbitrage opportunity found.");
    }
}

fn main() {
    let market: Market = Binance::new(Some(API_KEY.into()), Some(API_SECRET.into()));

    let symbol1 = "BTCUSDT";
    let symbol2 = "ETHUSDT";

    loop {
        arbitrage(&market, symbol1, symbol2);
        sleep(Duration::from_secs(60)); // Check every minute
    }
}
