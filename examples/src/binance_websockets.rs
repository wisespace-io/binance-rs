extern crate binance;

use binance::api::*;
use binance::userstream::*;
use binance::websockets::*;

fn main() {
    user_stream();
    user_stream_websocket();
    market_websocket();
    kline_websocket();
    all_trades_websocket();
}

fn user_stream() {
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

fn user_stream_websocket() {
    let api_key_user = Some("YOUR_KEY".into());
    let user_stream: UserStream = Binance::new(api_key_user, None);

    if let Ok(answer) = user_stream.start() {
        let listen_key = answer.listen_key;

        let mut web_socket: WebSockets = WebSockets::new(|event: WebsocketEvent| {
            match event {
                WebsocketEvent::AccountUpdate(account_update) => {
                    for balance in &account_update.balance {
                        println!(
                            "Asset: {}, free: {}, locked: {}",
                            balance.asset, balance.free, balance.locked
                        );
                    }
                },
                WebsocketEvent::OrderTrade(trade) => {
                    println!(
                        "Symbol: {}, Side: {}, Price: {}, Execution Type: {}",
                        trade.symbol, trade.side, trade.price, trade.execution_type
                    );
                },
                _ => return,
            }
        });

        web_socket.connect(&listen_key).unwrap(); // check error
        web_socket.event_loop();
    } else {
        println!("Not able to start an User Stream (Check your API_KEY)");
    }
}

fn market_websocket() {
    let agg_trade: String = format!("{}@aggTrade", "ethbtc");
    let mut web_socket: WebSockets = WebSockets::new(|event: WebsocketEvent| {
        match event {
            WebsocketEvent::Trade(trade) => {
                println!(
                    "Symbol: {}, price: {}, qty: {}",
                    trade.symbol, trade.price, trade.qty
                );
            },
            WebsocketEvent::DepthOrderBook(depth_order_book) => {
                println!(
                    "Symbol: {}, Bids: {:?}, Ask: {:?}",
                    depth_order_book.symbol, depth_order_book.bids, depth_order_book.asks
                );
            },
            WebsocketEvent::OrderBook(order_book) => {
                println!(
                    "last_update_id: {}, Bids: {:?}, Ask: {:?}",
                    order_book.last_update_id, order_book.bids, order_book.asks
                );
            },
            _ => return,
        }
    });

    web_socket.connect(&agg_trade).unwrap(); // check error
    web_socket.event_loop();
}

fn all_trades_websocket() {
    let agg_trade: String = format!("!ticker@arr");
    let mut web_socket: WebSockets = WebSockets::new(|event: WebsocketEvent| {
        match event {
            WebsocketEvent::DayTicker(ticker_events) => {
                for tick_event in ticker_events {
                    println!(
                        "Symbol: {}, price: {}, qty: {}",
                        tick_event.symbol, tick_event.best_bid, tick_event.best_bid_qty
                    );
                }
            },
            _ => return,
        }
    });

    web_socket.connect(&agg_trade).unwrap(); // check error
    web_socket.event_loop();
}

fn kline_websocket() {
    let kline: String = format!("{}", "ethbtc@kline_1m");
    let mut web_socket: WebSockets = WebSockets::new(|event: WebsocketEvent| {
        match event {
            WebsocketEvent::Kline(kline_event) => {
                println!(
                    "Symbol: {}, high: {}, low: {}",
                    kline_event.kline.symbol, kline_event.kline.low, kline_event.kline.high
                );
            },
            _ => return,
        }
    });

    web_socket.connect(&kline).unwrap(); // check error
    web_socket.event_loop();
}
