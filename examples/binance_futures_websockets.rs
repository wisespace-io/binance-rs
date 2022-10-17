use binance::futures::websockets::*;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    market_websocket();
}

fn market_websocket() {
    // Example to show the future market websockets. It will print one event for each
    // endpoint and continue to the next.

    let keep_running = AtomicBool::new(true);
    let stream_examples_usd_m = vec![
        // taken from https://binance-docs.github.io/apidocs/futures/en/#websocket-market-streams
        "btcusdt@aggTrade",                     // <symbol>@aggTrade
        "btcusdt@markPrice",                    // <symbol>@markPrice OR <symbol>@markPrice@1s
        "btcusdt@kline_1m",                     // <symbol>@kline_<interval>
        "btcusdt_perpetual@continuousKline_1m", // <pair>_<contractType>@continuousKline_<interval> e.g. "btcusd_next_quarter@continuousKline_1m"
        "btcusdt@miniTicker",                   // <symbol>@miniTicker
        "!miniTicker@arr",
        "btcusdt@ticker", // <symbol>@ticker
        "!ticker@arr",
        "btcusdt@bookTicker", // <symbol>@bookTicker
        "!bookTicker",
        // forceOrder can take a while before a message comes in, since
        // it depends on when a position is liquidated
        "btcusdt@forceOrder", // <symbol>@forceOrder
        "!forceOrder@arr",
        "btcusdt@depth20@100ms", // <symbol>@depth<levels> OR <symbol>@depth<levels>@500ms OR <symbol>@depth<levels>@100ms.
        "btcusdt@depth@100ms",   // <symbol>@depth OR <symbol>@depth@500ms OR <symbol>@depth@100ms
    ];

    let stream_examples_coin_m = vec![
        // taken from https://binance-docs.github.io/apidocs/delivery/en/#websocket-market-streams

        // A possible symbol is btcusd_210924. This needs updates if the current date
        // is greater than 2021-09-24. It'd be nice to make this symbol automatically
        // generated, or find a <symbol> that always works.
        "btcusd_210924@aggTrade",                 // <symbol>@aggTrade
        "btcusd@indexPrice@1s",                   //<pair>@indexPrice OR <pair>@indexPrice@1s
        "btcusd_210924@markPrice",                // <symbol>@markPrice OR <symbol>@markPrice@1s
        "btcusd@markPrice",                       // <pair>@markPrice OR <pair>@markPrice@1s
        "btcusd_210924@kline_1m",                 // <symbol>@kline_<interval>
        "btcusd_next_quarter@continuousKline_1m", // <pair>_<contractType>@continuousKline_<interval>
        "btcusd@indexPriceKline_1m",              // <pair>@indexPriceKline_<interval>
        "btcusd_210924@markPriceKline_1m",        // <symbol>@markPriceKline_<interval>
        "btcusd_210924@miniTicker",               // <symbol>@miniTicker
        "!miniTicker@arr",
        "btcusd_210924@ticker", // <symbol>@ticker
        "!ticker@arr",
        "btcusd_210924@bookTicker", // <symbol>@bookTicker
        "!bookTicker",
        // forceOrder can take a while before a message comes in, since
        // it depends on when a position is liquidated
        "btcusd_210924@forceOrder", // <symbol>@forceOrder
        "!forceOrder@arr",
        "btcusd_210924@depth20@100ms", // <symbol>@depth<levels> OR <symbol>@depth<levels>@500ms OR <symbol>@depth<levels>@100ms.
        "btcusd_210924@depth@100ms", // <symbol>@depth OR <symbol>@depth@500ms OR <symbol>@depth@100ms
    ];

    let callback_fn = |event: FuturesWebsocketEvent| {
        // once a FuturesWebsocketEvent is recevied, we print it
        // and stop this socket, so the example will continue to the next one
        //
        // in case an event comes in that doesn't properly serialize to
        // a FuturesWebsocketEvent, the web socket loop will keep running
        println!("{:?}\n", event);
        keep_running.swap(false, Ordering::Relaxed);

        Ok(())
    };

    // USD-M futures examples
    for stream_example in stream_examples_usd_m {
        println!("Starting with USD_M {:?}", stream_example);
        keep_running.swap(true, Ordering::Relaxed);

        let mut web_socket: FuturesWebSockets<'_> = FuturesWebSockets::new(callback_fn);
        web_socket
            .connect(&FuturesMarket::USDM, stream_example)
            .unwrap();
        web_socket.event_loop(&keep_running).unwrap();
        web_socket.disconnect().unwrap();
    }

    // COIN-M futures examples
    for stream_example in stream_examples_coin_m {
        println!("Starting with COIN_M {:?}", stream_example);
        keep_running.swap(true, Ordering::Relaxed);

        let mut web_socket: FuturesWebSockets<'_> = FuturesWebSockets::new(callback_fn);
        web_socket
            .connect(&FuturesMarket::COINM, stream_example)
            .unwrap();
        web_socket.event_loop(&keep_running).unwrap();
        web_socket.disconnect().unwrap();
    }
}
