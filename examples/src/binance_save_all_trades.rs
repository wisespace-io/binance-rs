extern crate csv;
extern crate binance;

use std::error::Error;
use std::cell::RefCell;
use std::fs::File;
use csv::Writer;

use binance::websockets::*;
use binance::model::{DayTickerEvent};

fn main() {
    save_all_trades_websocket();
}

fn save_all_trades_websocket() { 

    struct WebSocketHandler {
        wrt: RefCell<Writer<File>>
    };

    impl WebSocketHandler {
        pub fn new(local_wrt: Writer<File>) -> Self {
            WebSocketHandler {
                wrt: RefCell::new(local_wrt)
            }
        }

        // serialize DayTickerEvent as CSV records
        pub fn write_to_file(&self, event: DayTickerEvent) -> Result<(), Box<Error>> {
            let mut local_wrt = self.wrt.borrow_mut();
            local_wrt.serialize(event)?;
            Ok(())
        }
    }

    impl DayTickerEventHandler for WebSocketHandler {
        fn day_ticker_handler(&self, events: &[DayTickerEvent]) {
            for event in events {
                if let Err(error) = self.write_to_file(event.clone()) {
                    println!("{}", error);
                }
            }
        }
    }

    let file_path = std::path::Path::new("test.csv");
    let local_wrt = csv::Writer::from_path(file_path).unwrap();

    let web_socket_handler = WebSocketHandler::new(local_wrt);
    let agg_trade: String = format!("!ticker@arr");
    let mut web_socket: WebSockets = WebSockets::new();

    web_socket.add_day_ticker_handler(web_socket_handler);
    web_socket.connect(&agg_trade).unwrap(); // check error
    web_socket.event_loop();
}