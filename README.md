# binance-rs

Unofficial Rust Library for the [Binance API](https://github.com/binance-exchange/binance-official-api-docs) and [Binance Futures API](https://binance-docs.github.io/apidocs/futures/en/#general-info)

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

## Rust >= 1.37

```rust
rustup install stable
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
    match market.get_price("BNBETH") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Current average price for ONE symbol
    match market.get_average_price("BNBETH") {
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
    match market.get_klines("BNBETH", "5m", 10, None, None) {
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

### ERROR HANDLING - More detailed error information

You can check out the [Binance Error Codes](https://github.com/binance-exchange/binance-official-api-docs/blob/master/errors.md)

```rust
use binance::errors::ErrorKind as BinanceLibErrorKind;

[...]

Err(err) => {
    println!("Can't put an order!");

    match err.0 {
        BinanceLibErrorKind::BinanceError(response) => match response.code {
            -1013_i16 => println!("Filter failure: LOT_SIZE!"),
            -2010_i16 => println!("Funds insufficient! {}", response.msg),
            _ => println!("Non-catched code {}: {}", response.code, response.msg),
        },
        BinanceLibErrorKind::Msg(msg) => {
            println!("Binancelib error msg: {}", msg)
        }
        _ => println!("Other errors: {}.", err.0),
    };
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
use std::sync::atomic::{AtomicBool};

fn main() {
    let api_key_user = Some("YOUR_KEY".into());
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let user_stream: UserStream = Binance::new(api_key_user, None);

    if let Ok(answer) = user_stream.start() {
	let listen_key = answer.listen_key;

	let mut web_socket: WebSockets = WebSockets::new(|event: WebsocketEvent| {
	    match event {
		WebsocketEvent::AccountUpdate(account_update) => {
		    for balance in &account_update.balance {
			println!("Asset: {}, free: {}, locked: {}", balance.asset, balance.free, balance.locked);
		    }
		},
		WebsocketEvent::OrderTrade(trade) => {
		    println!("Symbol: {}, Side: {}, Price: {}, Execution Type: {}", trade.symbol, trade.side, trade.price, trade.execution_type);
		},
		_ => (),
	    };
	    Ok(())
	});

	web_socket.connect(&listen_key).unwrap(); // check error
	    if let Err(e) = web_socket.event_loop(&keep_running) {
		match e {
		    err => {
		        println!("Error: {}", err);
		    }
		}
	     }
	} else {
	    println!("Not able to start an User Stream (Check your API_KEY)");
	}
}
```

### WEBSOCKETS - TRADES

```rust
extern crate binance;

use binance::websockets::*;
use std::sync::atomic::{AtomicBool};

fn main() {
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let agg_trade: String = format!("!ticker@arr");
    let mut web_socket: WebSockets = WebSockets::new(|event: WebsocketEvent| {
	match event {
	    WebsocketEvent::DayTicker(ticker_events) => {
	        for tick_event in ticker_events {
		    if tick_event.symbol == "BTCUSDT" {
			let btcusdt: f32 = tick_event.average_price.parse().unwrap();
			let btcusdt_close: f32 = tick_event.current_close.parse().unwrap();
			println!("{} - {}", btcusdt, btcusdt_close);
		    }
		}
	    },
	    _ => (),
        };

        Ok(())
    });

    web_socket.connect(&agg_trade).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
	match e {
	    err => {
	        println!("Error: {}", err);
	    }
	}
     }
}
```

### WEBSOCKETS - KLINE

```rust
extern crate binance;

use binance::websockets::*;
use std::sync::atomic::{AtomicBool};

fn main() {
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let kline: String = format!("{}", "ethbtc@kline_1m");
    let mut web_socket: WebSockets = WebSockets::new(|event: WebsocketEvent| {
        match event {
            WebsocketEvent::Kline(kline_event) => {
                println!("Symbol: {}, high: {}, low: {}", kline_event.kline.symbol, kline_event.kline.low, kline_event.kline.high);
            },
            _ => (),
        };
        Ok(())
    });
 
    web_socket.connect(&kline).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        match e {
          err => {
             println!("Error: {}", err);
          }
        }
     }
     web_socket.disconnect().unwrap();
}

```

## Other Exchanges

If you use [Bitfinex](https://www.bitfinex.com/) check out my [Rust library for bitfinex API](https://github.com/wisespace-io/bitfinex-rs)
