use binance::api::*;
use binance::futures::general::*;
use binance::futures::market::*;
use binance::errors::ErrorKind as BinanceLibErrorKind;

fn main() {
    general();
    //account();
    market_data();
}

fn general() {
    let general: FuturesGeneral = Binance::new(None, None);

    let ping = general.ping();
    match ping {
        Ok(answer) => println!("{:?}", answer),
        Err(err) => {
            match err.0 {
                BinanceLibErrorKind::BinanceError(response) => match response.code {
                    -1000_i16 => println!("An unknown error occured while processing the request"),
                    _ => println!("Non-catched code {}: {}", response.code, response.msg),
                },
                BinanceLibErrorKind::Msg(msg) => println!("Binancelib error msg: {}", msg),
                _ => println!("Other errors: {}.", err.0),
            };
        }
    }

    let result = general.get_server_time();
    match result {
        Ok(answer) => println!("Server Time: {}", answer.server_time),
        Err(e) => println!("Error: {}", e),
    }

    let result = general.exchange_info();
    match result {
        Ok(answer) => println!("Exchange information: {:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    let result = general.get_symbol_info("btcusdt");
    match result {
        Ok(answer) => println!("Symbol information: {:?}", answer),
        Err(e) => println!("Error: {}", e),
    }
}

fn market_data() {
    let _market: FuturesMarket = Binance::new(None, None);
}
