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
    let stream_examples_usd = vec![
        // taken from https://binance-docs.github.io/apidocs/futures/en/#websocket-market-streams
        String::from("btcusdt@aggTrade"),  // <symbol>@aggTrade
        String::from("btcusdt@markPrice"), // <symbol>@markPrice OR <symbol>@markPrice@1s
        String::from("btcusdt@kline_1m"),  // <symbol>@kline_<interval>
        String::from("btcusdt_perpetual@continuousKline_1m"), // <pair>_<contractType>@continuousKline_<interval> e.g. "btcusd_next_quarter@continuousKline_1m"
        String::from("btcusdt@miniTicker"),                   // <symbol>@miniTicker
        String::from("!miniTicker@arr"),
        String::from("btcusdt@ticker"), // <symbol>@ticker
        String::from("!ticker@arr"),
        String::from("btcusdt@bookTicker"), // <symbol>@bookTicker
        String::from("!bookTicker"),
        // forceOrder can take a while before a message comes in, since
        // it depends on when a position is liquidated
        String::from("btcusdt@forceOrder"), // <symbol>@forceOrder
        String::from("!forceOrder@arr"),
        String::from("btcusdt@depth20@100ms"), // <symbol>@depth<levels> OR <symbol>@depth<levels>@500ms OR <symbol>@depth<levels>@100ms.
        String::from("btcusdt@depth@100ms"), // <symbol>@depth OR <symbol>@depth@500ms OR <symbol>@depth@100ms
    ];

    let _stream_examples_coin = vec![
        // taken from https://binance-docs.github.io/apidocs/delivery/en/#websocket-market-streams
        //ok
        // String::from("btcusd_210924@aggTrade"), // <symbol>@aggTrade
        //ok
        // String::from("btcusd@indexPrice@1s"),                     //<pair>@indexPrice OR <pair>@indexPrice@1s

        // onbekend
        String::from("btcusd_210924@markPrice"), // <symbol>@markPrice OR <symbol>@markPrice@1s
        String::from("btcusd@markPrice"), // <pair>@markPrice OR <pair>@markPrice@1s

        // ok
        String::from("btcusd_210924@kline_1m"), // <symbol>@kline_<interval>
        String::from("btcusd_210924@continuousKline_1m"), // <pair>_<contractType>@continuousKline_<interval>
        String::from("btcusd_210924@indexPriceKline_1m"), // <pair>@indexPriceKline_<interval>
        String::from("btcusd_210924@markPriceKline_1m"),  // <symbol>@markPriceKline_<interval>
        String::from("btcusd_210924@miniTicker"),         // <symbol>@miniTicker
        String::from("!miniTicker@arr"),
        String::from("btcusd_210924@ticker"), // <symbol>@ticker
        String::from("!ticker@arr"),
        String::from("btcusd_210924@bookTicker"), // <symbol>@bookTicker
        String::from("!bookTicker"),
        String::from("btcusd_210924@forceOrder"), // <symbol>@forceOrder
        String::from("!forceOrder@arr"),
        String::from("btcusd_210924@depth20@100ms"), // <symbol>@depth<levels> OR <symbol>@depth<levels>@500ms OR <symbol>@depth<levels>@100ms.
        String::from("btcusd_210924@depth@100ms"), // <symbol>@depth OR <symbol>@depth@500ms OR <symbol>@depth@100ms
    ];

    let callback_fn = |event: FuturesWebsocketEvent| {
        println!("{:?}\n", event);
        keep_running.swap(false, Ordering::Relaxed);
        match event {
            FuturesWebsocketEvent::Trade(trade) => {
                println!(
                    "Symbol: {}, price: {}, qty: {}",
                    trade.symbol, trade.price, trade.qty
                );
                keep_running.swap(false, Ordering::Relaxed);
            }
            FuturesWebsocketEvent::DepthOrderBook(depth_order_book) => {
                println!(
                    "Symbol: {}, Bids: {:?}, Ask: {:?}",
                    depth_order_book.symbol, depth_order_book.bids, depth_order_book.asks
                );
                keep_running.swap(false, Ordering::Relaxed);
            }
            FuturesWebsocketEvent::OrderBook(order_book) => {
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
    for stream_example in stream_examples_usd {
        keep_running.swap(true, Ordering::Relaxed);

        let mut web_socket: FuturesWebSockets<'_> = FuturesWebSockets::new(callback_fn);
        web_socket
            .connect(FuturesMarket::USD, &stream_example)
            .unwrap();
        web_socket.event_loop(&keep_running).unwrap();
        web_socket.disconnect().unwrap();
    }

    // for stream_example in stream_examples_coin {
    //     keep_running.swap(true, Ordering::Relaxed);

    //     let mut web_socket: FuturesWebSockets<'_> = FuturesWebSockets::new(callback_fn);
    //     web_socket
    //         .connect(FuturesMarket::COINM, &stream_example)
    //         .unwrap();
    //     web_socket.event_loop(&keep_running).unwrap();
    //     web_socket.disconnect().unwrap();
    // }
}
