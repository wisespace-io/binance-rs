use binance::websockets::*;
use binance::futures::websockets::*;
use tungstenite::stream;
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
    let stream_examples_USD = vec![
        // works
        String::from("btcusdt@aggTrade"),               // <symbol>@aggTrade
        
        // does not work
        // String::from("btcusdt@indexPrice@1s"),                     //<pair>@indexPrice OR <pair>@indexPrice@1s
        
        // works
        String::from("btcusdt@markPrice"), // <symbol>@markPrice OR <symbol>@markPrice@1s

        // String::from("btcusd_210924@markPrice"), // <pair>@markPrice OR <pair>@markPrice@1s

        // String::from("btcusdt@kline_1m"), // <symbol>@kline_<interval>

        // String::from("btcusd_210924@continuousKline_1m"), // <pair>_<contractType>@continuousKline_<interval> e.g. "btcusd_next_quarter@continuousKline_1m"
        // String::from("btcusd_210924@indexPriceKline_1m"), // <pair>@indexPriceKline_<interval> e.g. "btcusd@indexPriceKline_1m"
        // String::from("btcusd_210924@markPriceKline_1m"), // <symbol>@markPriceKline_<interval> e.g. e.g. "btcusd_200626@markPriceKline_1m"
        // String::from("btcusdt@miniTicker"), // <symbol>@miniTicker
        // String::from("!miniTicker@arr"),
        // String::from("btcusdt@ticker"), // <symbol>@ticker
        // String::from("!ticker@arr"),
        // String::from("btcusdt@bookTicker"), // <symbol>@bookTicker
        // String::from("!bookTicker"),
        // String::from("btcusd_210924@forceOrder"), // <symbol>@forceOrder
        // String::from("!forceOrder@arr"),
        // String::from("btcusd_210924@depth20@100ms"), // <symbol>@depth<levels> OR <symbol>@depth<levels>@500ms OR <symbol>@depth<levels>@100ms. 
        // String::from("btcusd_210924@depth@100ms"), // <symbol>@depth OR <symbol>@depth@500ms OR <symbol>@depth@100ms
    ];
    let stream_examples_COIN = vec![
            String::from("btcusdt@aggTrade"),               // <symbol>@aggTrade
            // String::from("btcusd@indexPrice@1s"),                     //<pair>@indexPrice OR <pair>@indexPrice@1s
            // String::from("btcusdt@markPrice"),
            String::from("btcusd@kline_1m"),
            String::from("btcusd@continuousKline_1m"),
            String::from("btcusd@indexPriceKline_1m"),
            String::from("btcusd@markPriceKline_1m"),
            String::from("btcusd@miniTicker"),
            String::from("!miniTicker@arr"),
            String::from("btcusd_210924@ticker"),
            String::from("!ticker@arr"),
            String::from("btcusd_210924@bookTicker"),
            String::from("!bookTicker"),
            String::from("btcusd_210924@forceOrder"),
            String::from("!forceOrder@arr"),
            String::from("btcusd_210924@depth20@100ms"),
            String::from("btcusd_210924@depth@100ms")];
    

    let callback_fn = |event: WebsocketEvent| {
        println!("{:?}\n", event);
        keep_running.swap(false, Ordering::Relaxed);
        match event {
            WebsocketEvent::Trade(trade) => {
                println!(
                    "Symbol: {}, price: {}, qty: {}",
                    trade.symbol, trade.price, trade.qty
                );
                keep_running.swap(false, Ordering::Relaxed);
            }
            WebsocketEvent::DepthOrderBook(depth_order_book) => {
                println!(
                    "Symbol: {}, Bids: {:?}, Ask: {:?}",
                    depth_order_book.symbol, depth_order_book.bids, depth_order_book.asks
                );
                keep_running.swap(false, Ordering::Relaxed);
            }
            WebsocketEvent::OrderBook(order_book) => {
                println!(
                    "last_update_id: {}, Bids: {:?}, Ask: {:?}",
                    order_book.last_update_id, order_book.bids, order_book.asks
                );
                keep_running.swap(false, Ordering::Relaxed);
            }
            _ => (),
        };

        Ok(())
    };

    for stream_example in stream_examples_USD {
        keep_running.swap(true, Ordering::Relaxed);

        let mut web_socket: FuturesWebSockets<'_> = FuturesWebSockets::new(callback_fn);
        web_socket
            .connect(FuturesMarket::USD, &stream_example)
            .unwrap();
        web_socket.event_loop(&keep_running).unwrap();
        web_socket.disconnect().unwrap();
    }
}
