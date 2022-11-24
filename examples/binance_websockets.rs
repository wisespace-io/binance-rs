#![allow(dead_code)]

use binance::api::*;
use binance::userstream::*;
use binance::websockets::*;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    //user_stream();
    //user_stream_websocket();
    //market_websocket();
    //kline_websocket();
    //all_trades_websocket();
    //last_price_for_one_symbol();
    multiple_streams();
}

fn user_stream() {
    let api_key_user = Some("YOUR_API_KEY".into());
    let user_stream: UserStream = Binance::new(api_key_user, None);

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

fn user_stream_websocket() {
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let api_key_user = Some("YOUR_KEY".into());
    let user_stream: UserStream = Binance::new(api_key_user, None);

    if let Ok(answer) = user_stream.start() {
        let listen_key = answer.listen_key;

        let mut web_socket: WebSockets<'_> = WebSockets::new(|event: WebsocketEvent| {
            match event {
                WebsocketEvent::AccountUpdate(account_update) => {
                    for balance in &account_update.data.balances {
                        println!(
                            "Asset: {}, wallet_balance: {}, cross_wallet_balance: {}, balance: {}",
                            balance.asset,
                            balance.wallet_balance,
                            balance.cross_wallet_balance,
                            balance.balance_change
                        );
                    }
                }
                WebsocketEvent::OrderTrade(trade) => {
                    println!(
                        "Symbol: {}, Side: {}, Price: {}, Execution Type: {}",
                        trade.symbol, trade.side, trade.price, trade.execution_type
                    );
                }
                _ => (),
            };

            Ok(())
        });

        web_socket.connect(&listen_key).unwrap(); // check error
        if let Err(e) = web_socket.event_loop(&keep_running) {
            println!("Error: {}", e);
        }
        user_stream.close(&listen_key).unwrap();
        web_socket.disconnect().unwrap();
        println!("Userstrem closed and disconnected");
    } else {
        println!("Not able to start an User Stream (Check your API_KEY)");
    }
}

fn market_websocket() {
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let agg_trade = String::from("ethbtc@aggTrade");
    let mut web_socket: WebSockets<'_> = WebSockets::new(|event: WebsocketEvent| {
        match event {
            WebsocketEvent::Trade(trade) => {
                println!(
                    "Symbol: {}, price: {}, qty: {}",
                    trade.symbol, trade.price, trade.qty
                );
            }
            WebsocketEvent::DepthOrderBook(depth_order_book) => {
                println!(
                    "Symbol: {}, Bids: {:?}, Ask: {:?}",
                    depth_order_book.symbol, depth_order_book.bids, depth_order_book.asks
                );
            }
            WebsocketEvent::OrderBook(order_book) => {
                println!(
                    "last_update_id: {}, Bids: {:?}, Ask: {:?}",
                    order_book.last_update_id, order_book.bids, order_book.asks
                );
            }
            _ => (),
        };

        Ok(())
    });

    web_socket.connect(&agg_trade).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Error: {}", e);
    }
    web_socket.disconnect().unwrap();
    println!("disconnected");
}

fn all_trades_websocket() {
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let agg_trade = String::from("!ticker@arr");
    let mut web_socket = WebSockets::new(|event: WebsocketEvent| {
        if let WebsocketEvent::DayTickerAll(ticker_events) = event {
            for tick_event in ticker_events {
                println!(
                    "Symbol: {}, price: {}, qty: {}",
                    tick_event.symbol, tick_event.best_bid, tick_event.best_bid_qty
                );
            }
        }

        Ok(())
    });

    web_socket.connect(&agg_trade).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Error: {}", e);
    }
    web_socket.disconnect().unwrap();
    println!("disconnected");
}

fn kline_websocket() {
    let keep_running = AtomicBool::new(true);
    let kline = String::from("ethbtc@kline_1m");
    let mut web_socket = WebSockets::new(|event: WebsocketEvent| {
        if let WebsocketEvent::Kline(kline_event) = event {
            println!(
                "Symbol: {}, high: {}, low: {}",
                kline_event.kline.symbol, kline_event.kline.low, kline_event.kline.high
            );
        }

        Ok(())
    });

    web_socket.connect(&kline).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Error: {}", e);
    }
    web_socket.disconnect().unwrap();
    println!("disconnected");
}

fn last_price_for_one_symbol() {
    let keep_running = AtomicBool::new(true);
    let agg_trade = String::from("btcusdt@ticker");
    let mut btcusdt: f32 = "0".parse().unwrap();

    let mut web_socket = WebSockets::new(|event: WebsocketEvent| {
        if let WebsocketEvent::DayTicker(ticker_event) = event {
            btcusdt = ticker_event.average_price.parse().unwrap();
            let btcusdt_close: f32 = ticker_event.current_close.parse().unwrap();
            println!("{} - {}", btcusdt, btcusdt_close);

            if btcusdt_close as i32 == 7000 {
                // Break the event loop
                keep_running.store(false, Ordering::Relaxed);
            }
        }

        Ok(())
    });

    web_socket.connect(&agg_trade).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Error: {}", e);
    }
    web_socket.disconnect().unwrap();
    println!("disconnected");
}

fn multiple_streams() {
    let endpoints =
        ["ETHBTC", "BNBETH"].map(|symbol| format!("{}@depth@100ms", symbol.to_lowercase()));

    let keep_running = AtomicBool::new(true);
    let mut web_socket = WebSockets::new(|event: WebsocketEvent| {
        if let WebsocketEvent::DepthOrderBook(depth_order_book) = event {
            println!("{:?}", depth_order_book);
        }

        Ok(())
    });

    web_socket.connect_multiple_streams(&endpoints).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Error: {}", e);
    }
    web_socket.disconnect().unwrap();
}
