[![Build Status](https://travis-ci.org/wisespace-io/binance-rs.png?branch=master)](https://travis-ci.org/wisespace-io/binance-rs)

# binance-rs
Rust Library for the [Binance API](https://www.binance.com/restapipub.html)

# Usage

Add this to your Cargo.toml

```toml
[dependencies]
binance = "git = "https://github.com/wisespace-io/binance-rs.git""
```

### GENERAL
```
extern crate binance;

use binance::api::*;
use binance::general::*;

fn main() {
    let general: General = Binance::new(None, None);

    let ping = general.ping();
    match ping {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }   

    let server = general.get_server_time();
    match server {
        Ok(answer) => println!("{}", answer.server_time),
        Err(e) => println!("Error: {}", e),
    }
}
```

### MARKET DATA
```
extern crate binance;

use binance::api::*;
use binance::market::*;

fn main() {
    let market: Market = Binance::new(None, None);

    // Order book
    match market.get_depth("BNBETH".into()) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }         

    // Latest price for ALL symbols
    match market.get_all_prices() {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    } 

    // Latest price for ONE symbol
    match market.get_price("KNCETH".into()) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    } 

    // Best price/qty on the order book for ALL symbols
    match market.get_all_book_tickers("BNBETH".into()) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Best price/qty on the order book for ONE symbol
    match market.get_book_ticker("BNBETH".into()) {
        Ok(answer) => println!("Bid Price: {}, Ask Price: {}", answer.bid_price, answer.ask_price),
        Err(e) => println!("Error: {}", e),
    }     

    // 24hr ticker price change statistics
    match market.get_24h_price_stats("BNBETH".into()) {
        Ok(answer) => println!("Open Price: {}, Higher Price: {}, Lower Price: {:?}",
                                answer.open_price, answer.high_price, answer.low_price),
        Err(e) => println!("Error: {}", e),
    } 
}
```

### ACCOUNT DATA
```
extern crate binance;

use binance::api::*;
use binance::account::*;

fn main() {
    let api_key = Some("YOUR_API_KEY".into());
    let secret_key = Some("YOUR_SECRET_KEY".into());

    let account: Account = Binance::new(api_key, secret_key);
   
    match account.get_account() {
        Ok(answer) => println!("{:?}", answer.balances),
        Err(e) => println!("Error: {}", e),
    }
    
    match account.get_open_orders("WTCETH".into()) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    } 

    match account.limit_buy("WTCETH".into(), 10, 0.014000) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.market_buy("WTCETH".into(), 5) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.limit_sell("WTCETH".into(), 10, 0.035000) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.market_sell("WTCETH".into(), 5) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    let order_id = 1957528;
    match account.order_status("WTCETH".into(), order_id) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.cancel_order("WTCETH".into(), order_id) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }   

    match account.get_balance("KNC".into()) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.trade_history("WTCETH".into()) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }    
}
```

### USER STREAM
```
extern crate binance;

use binance::api::*;
use binance::userstream::*;

fn main() {
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
```

### WEB SOCKETS
```
extern crate binance;

use binance::api::*;
use binance::userstream::*;
use binance::websockets::*;
use binance::model::{AccountUpdateEvent, OrderTradeEvent};

struct WebScoketHandler;

impl EventHandler for WebScoketHandler {
    fn account_update_handler(&self, event: &AccountUpdateEvent) {
        for balance in &event.balance {
            println!("Asset: {}, free: {}, locked: {}", balance.asset, balance.free, balance.locked);
        }
    }

    fn order_trade_handler(&self, event: &OrderTradeEvent) {
        println!("Symbol: {}, Side: {}, Price: {}, Execution Type: {}", 
                 event.symbol, event.side, event.price, event.execution_type);
    }        
}

fn main() {
    let api_key_user = Some("YOUR_KEY".into());
    let user_stream: UserStream = Binance::new(api_key_user, None);
    
    if let Ok(answer) = user_stream.start() {
        let listen_key = answer.listen_key;
       
        let mut web_socket: WebSockets = WebSockets::new();
        web_socket.handler(WebScoketHandler);
        web_socket.connect_stream(listen_key).unwrap(); // check error

    } else {
        println!("Not able to start an User Stream (Check your API_KEY)");
    };    
}
```