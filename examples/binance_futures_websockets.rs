use binance::api::*;
use binance::websockets::*;
// use binance::userstream::*;
use binance::futures::websockets::*;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    // user_stream();
    // user_stream_websocket();
    market_websocket();
    //kline_websocket();
    //all_trades_websocket();
    //last_price_for_one_symbol();
    // multiple_streams();
}


fn market_websocket() {
    let keep_running = AtomicBool::new(true);
    let agg_trade: String = String::from("btcusd_210924@bookTicker");
    // let agg_trade: String = String::from("btcusd_200925@aggTrade");

    let mut web_socket: FuturesWebSockets<'_> = FuturesWebSockets::new(|event: WebsocketEvent| {
        println!("{:?}", event);
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

    let streams = vec![String::from("btcusd_210924@bookTicker")];

    web_socket.connect(&agg_trade).unwrap(); // check error
    // web_socket.connect_multiple_streams(&streams).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Error: {}", e);
    }
    web_socket.disconnect().unwrap();
    println!("disconnected");
}
