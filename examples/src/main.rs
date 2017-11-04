extern crate binance;

use binance::api::*;
use binance::general::*;
use binance::account::*;
use binance::market::*;
use binance::userstream::*;

fn main() {

    general();
    account();
    market_data();
    user_stream();

}

fn general() {
    let general: General = Binance::new(None, None);

    let ping = general.ping();
    match ping {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }   

    let result = general.get_server_time();
    match result {
        Ok(answer) => println!("Server Time: {}", answer.server_time),
        Err(e) => println!("Error: {}", e),
    }
}


fn account() {
    // ACCOUNT
    let api_key = Some("YOUR_API_KEY".into());
    let secret_key = Some("YOUR_SECRET_KEY".into());

    let account: Account = Binance::new(api_key.clone(), secret_key);
   
    match account.get_account() {
        Ok(answer) => println!("{:?}", answer.balances),
        Err(e) => println!("Error: {}", e),
    }
    
    //match account.get_open_orders("WTCETH".into()) {
    //    Ok(answer) => println!("{:?}", answer),
    //    Err(e) => println!("Error: {}", e),
    //} 

    //match account.limit_buy("WTCETH".into(), 10, 0.014000) {
    //    Ok(answer) => println!("{:?}", answer),
    //    Err(e) => println!("Error: {}", e),
    //}

    //match account.market_buy("WTCETH".into(), 5) {
    //    Ok(answer) => println!("{:?}", answer),
    //    Err(e) => println!("Error: {}", e),
    //}

    //match account.limit_sell("WTCETH".into(), 10, 0.035000) {
    //    Ok(answer) => println!("{:?}", answer),
    //    Err(e) => println!("Error: {}", e),
    //}

    //let order_id = 1957528;
    //match account.order_status("WTCETH".into(), order_id) {
    //    Ok(answer) => println!("{:?}", answer),
    //    Err(e) => println!("Error: {}", e),
    //}

    //match account.cancel_order("WTCETH".into(), order_id) {
    //    Ok(answer) => println!("{:?}", answer),
    //    Err(e) => println!("Error: {}", e),
    //}   

    //match account.get_balance("KNC".into()) {
    //    Ok(answer) => println!("{:?}", answer),
    //    Err(e) => println!("Error: {}", e),
    //} 

    //match account.trade_history("WTCETH".into()) {
    //    Ok(answer) => println!("{:?}", answer),
    //    Err(e) => println!("Error: {}", e),
    //}         
}

fn market_data() {
    let market: Market = Binance::new(None, None);        

    match market.get_price("BNBETH".into()) {
        Ok(answer) => println!("PRICE: {:?}", answer),
        Err(e) => println!("Error: {}", e),
    } 

    match market.get_book_ticker("BNBETH".into()) {
        Ok(answer) => println!("Bid Price: {}, Ask Price: {}", answer.bid_price, answer.ask_price),
        Err(e) => println!("Error: {}", e),
    }     

    match market.get_24h_price_stats("BNBETH".into()) {
        Ok(answer) => println!("Open Price: {}, Higher Price: {}, Lower Price: {:?}",
                                answer.open_price, answer.high_price, answer.low_price),
        Err(e) => println!("Error: {}", e),
    } 

    //match market.get_depth("BNBETH".into()) {
    //    Ok(answer) => println!("{:?}", answer),
    //    Err(e) => println!("Error: {}", e),
    //}     
}

fn user_stream() {
    let api_key_user = Some("YOUR_API_KEY".into());
    let user_stream: UserStream = Binance::new(api_key_user.clone(), None);
    
    if let Ok(answer) = user_stream.start() {
        println!("Data Stream Started ...");
        let listen_key = answer.listen_key;

        match user_stream.keep_alive(listen_key.clone()) {
            Ok(msg) => println!("Keepalive user data stream: {:?}", msg),
            Err(e) => println!("Error: {}", e),
        }

        match user_stream.close(listen_key.clone()) {
            Ok(msg) => println!("Close user data stream: {:?}", msg),
            Err(e) => println!("Error: {}", e),
        }       
    } else {
        println!("Not able to start an User Stream (Check your API_KEY)");
    };       
}