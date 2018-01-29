extern crate binance_async as binance;
extern crate dotenv;
extern crate env_logger;
extern crate failure;
extern crate futures;
extern crate tokio;

use std::env::var;

use failure::Fallible;
use futures::{Future, Stream};
use tokio::runtime::current_thread::Runtime;

use binance::model::websocket::Subscription;
use binance::Binance;

fn main() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    ::env_logger::init();
    let mut rt = Runtime::new()?;

    let api_key_user = var("BINANCE_KEY")?;
    let api_secret_user = var("BINANCE_SECRET")?;

    let bn = Binance::with_credential(&api_key_user, &api_secret_user);
    match rt.block_on(bn.user_stream_start()?) {
        Ok(answer) => {
            println!("Data Stream Started ...");
            let listen_key = answer.listen_key;

            let job = bn
                .websocket()
                .subscribe(Subscription::Ticker("ethbtc".to_string()))
                .and_then(|ws| ws.subscribe(Subscription::AggregateTrade("eosbtc".to_string())))
                .and_then(|ws| {
                    ws.subscribe(Subscription::Candlestick(
                        "ethbtc".to_string(),
                        "1m".to_string(),
                    ))
                })
                .and_then(|ws| ws.subscribe(Subscription::Depth("xrpbtc".to_string())))
                .and_then(|ws| ws.subscribe(Subscription::MiniTicker("zrxbtc".to_string())))
                .and_then(|ws| ws.subscribe(Subscription::OrderBook("trxbtc".to_string(), 5)))
                .and_then(|ws| ws.subscribe(Subscription::Trade("adabtc".to_string())))
                .and_then(|ws| ws.subscribe(Subscription::UserData(listen_key)))
                .and_then(|ws| ws.subscribe(Subscription::MiniTickerAll))
                .and_then(|ws| ws.subscribe(Subscription::TickerAll))
                .and_then(|ws| ws.map(|msg| println!("{:?}", msg)).collect());
            let _ = rt.block_on(job).unwrap();
        }
        Err(e) => println!("Error obtaining userstream: {}", e),
    }

    Ok(())
}
