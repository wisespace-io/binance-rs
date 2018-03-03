# binance-rs

Unofficial Rust Library for the [Binance API](https://github.com/binance-exchange/binance-official-api-docs)

[![Crates.io](https://img.shields.io/crates/v/binance.svg)](https://crates.io/crates/binance)
[![Build Status](https://travis-ci.org/wisespace-io/binance-rs.png?branch=master)](https://travis-ci.org/wisespace-io/binance-rs)
[![MIT licensed](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE-MIT)
[![Apache-2.0 licensed](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](./LICENSE-APACHE)

[Documentation](https://docs.rs/crate/binance/)

## Binance API Telegram

  <https://t.me/binance_api_english>

## Risk Warning

It is a personal project, use at your own risk. I will not be responsible for your investment losses.
Cryptocurrency investment is subject to high market risk.

## Usage

Add this to your Cargo.toml

```toml
[dependencies]
binance = { git = "https://github.com/wisespace-io/binance-rs.git" }
```

### MARKET DATA

```rust
extern crate binance;

use binance::api::*;
use binance::market::*;

fn main() {
    let market: Market = Binance::new(None, None);

    // Order book
    match market.get_depth("BNBETH") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Latest price for ALL symbols
    match market.get_all_prices() {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Latest price for ONE symbol
    match market.get_price("KNCETH") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Best price/qty on the order book for ALL symbols
    match market.get_all_book_tickers() {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Best price/qty on the order book for ONE symbol
    match market.get_book_ticker("BNBETH") {
        Ok(answer) => println!(
            "Bid Price: {}, Ask Price: {}",
            answer.bid_price, answer.ask_price
        ),
        Err(e) => println!("Error: {}", e),
    }

    // 24hr ticker price change statistics
    match market.get_24h_price_stats("BNBETH") {
        Ok(answer) => println!(
            "Open Price: {}, Higher Price: {}, Lower Price: {:?}",
            answer.open_price, answer.high_price, answer.low_price
        ),
        Err(e) => println!("Error: {}", e),
    }

    // last 10 5min klines (candlesticks) for a symbol:
    match market.get_klines("BNBETH", "5m", 10) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }
}
```

### ACCOUNT DATA

```rust
extern crate binance;

use binance::api::*;
use binance::account::*;

fn main() {
    let api_key = Some("YOUR_API_KEY".into());
    let secret_key = Some("YOUR_SECRET_KEY".into());

    let account: Account = Binance::new(api_key, secret_key);

    match account.get_account() {
        Ok(answer) => println!("{:?}", answer.balances),
        Err(e) => println!("Error: {}", e),
    }

    match account.get_open_orders("WTCETH") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.limit_buy("WTCETH", 10, 0.014000) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.market_buy("WTCETH", 5) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.limit_sell("WTCETH", 10, 0.035000) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.market_sell("WTCETH", 5) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    let order_id = 1_957_528;
    match account.order_status("WTCETH", order_id) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.cancel_order("WTCETH", order_id) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.get_balance("KNC") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.trade_history("WTCETH") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }
}
```

### USER STREAM

```rust
extern crate binance;

use binance::api::*;
use binance::userstream::*;

fn main() {
    let api_key_user = Some("YOUR_API_KEY".into());
    let user_stream: UserStream = Binance::new(api_key_user.clone(), None);

    if let Ok(answer) = user_stream.start() {
        println!("Data Stream Started ...");
        let listen_key = answer.listen_key;

        match user_stream.keep_alive(&listen_key) {
            Ok(msg) => println!("Keepalive user data stream: {:?}", msg),
            Err(e) => println!("Error: {}", e),
        }

        match user_stream.close(&listen_key) {
            Ok(msg) => println!("Close user data stream: {:?}", msg),
            Err(e) => println!("Error: {}", e),
        }
    } else {
        println!("Not able to start an User Stream (Check your API_KEY)");
    }
}
```

### WEBSOCKETS - USER STREAM

```rust
extern crate binance;

use binance::api::*;
use binance::userstream::*;
use binance::websockets::*;
use binance::model::{AccountUpdateEvent, OrderTradeEvent};

struct WebSocketHandler;

impl UserStreamEventHandler for WebSocketHandler {
    fn account_update_handler(&self, event: &AccountUpdateEvent) {
        for balance in &event.balance {
            println!(
                "Asset: {}, free: {}, locked: {}",
                balance.asset, balance.free, balance.locked
            );
        }
    }

    fn order_trade_handler(&self, event: &OrderTradeEvent) {
        println!(
            "Symbol: {}, Side: {}, Price: {}, Execution Type: {}",
            event.symbol, event.side, event.price, event.execution_type
        );
    }
}

fn main() {
    let api_key_user = Some("YOUR_KEY".into());
    let user_stream: UserStream = Binance::new(api_key_user, None);

    if let Ok(answer) = user_stream.start() {
        let listen_key = answer.listen_key;

        let mut web_socket: WebSockets = WebSockets::new();
        web_socket.add_user_stream_handler(WebSocketHandler);
        web_socket.connect(&listen_key).unwrap(); // check error
        web_socket.event_loop();
    } else {
        println!("Not able to start an User Stream (Check your API_KEY)");
    }
}
```

### WEBSOCKETS - TRADES

```rust
extern crate binance;

use binance::api::*;
use binance::websockets::*;
use binance::model::TradesEvent;

struct WebSocketHandler;

impl MarketEventHandler for WebSocketHandler {
    fn aggregated_trades_handler(&self, event: &TradesEvent) {
        println!(
            "Symbol: {}, price: {}, qty: {}",
            event.symbol, event.price, event.qty
        );
    }
}

fn main() {
    let agg_trade: String = format!("{}@aggTrade", "ethbtc");
    let mut web_socket: WebSockets = WebSockets::new();

    web_socket.add_market_handler(WebSocketHandler);
    web_socket.connect(&agg_trade).unwrap(); // check error
    web_socket.event_loop();
}
```

### WEBSOCKETS - KLINE

```rust
extern crate binance;

use binance::api::*;
use binance::websockets::*;
use binance::model::KlineEvent;

struct WebSocketHandler;

impl KlineEventHandler for WebSocketHandler {
    fn kline_handler(&self, event: &KlineEvent) {
        println!(
            "Symbol: {}, high: {}, low: {}",
            event.kline.symbol, event.kline.low, event.kline.high
        );
    }
}

fn main() {
    let kline: String = format!("{}", "ethbtc@kline_1m");
    let mut web_socket: WebSockets = WebSockets::new();

    web_socket.add_kline_handler(WebSocketHandler);
    web_socket.connect(&kline).unwrap(); // check error
    web_socket.event_loop();
}
```

## Other Exchanges

If you use [Bitfinex](https://www.bitfinex.com/) check out my [Rust library for bitfinex API](https://github.com/wisespace-io/bitfinex-rs)
