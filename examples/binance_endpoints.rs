use binance::api::*;
use binance::config::*;
use binance::general::*;
use binance::account::*;
use binance::market::*;
use binance::errors::ErrorKind as BinanceLibErrorKind;

fn main() {
    general(false);
    general(true);

    // those examples need an API key. change those lines locally
    // and uncomment those (and do not commit your api key :))
    //account();
    //market_data();
}

fn general(use_testnet: bool) {
    let general: General = if use_testnet {
        let config = Config::default().set_rest_api_endpoint("https://testnet.binance.vision".to_string());
        Binance::new_with_config(None, None, &config)
    } else {
        Binance::new(None, None)
    };

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

    let result = general.get_symbol_info("ethbtc");
    match result {
        Ok(answer) => println!("Symbol information: {:?}", answer),
        Err(e) => println!("Error: {}", e),
    }
}

#[allow(dead_code)]
fn account() {
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

    match account.market_buy_using_quote_quantity("WTCETH", 5) {
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

    match account.market_sell_using_quote_quantity("WTCETH", 5) {
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

#[allow(dead_code)]
fn market_data() {
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

    // Current average price for ONE symbol
    match market.get_average_price("KNCETH") {
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
