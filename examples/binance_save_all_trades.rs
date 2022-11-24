use std::error::Error;
use std::fs::File;
use csv::Writer;
use std::sync::atomic::{AtomicBool};

use binance::websockets::*;
use binance::model::{DayTickerEvent};

fn main() {
    save_all_trades_websocket();
}

fn save_all_trades_websocket() {
    struct WebSocketHandler {
        wrt: Writer<File>,
    }

    impl WebSocketHandler {
        pub fn new(local_wrt: Writer<File>) -> Self {
            WebSocketHandler { wrt: local_wrt }
        }

        // serialize DayTickerEvent as CSV records
        pub fn write_to_file(&mut self, events: Vec<DayTickerEvent>) -> Result<(), Box<dyn Error>> {
            for event in events {
                self.wrt.serialize(event)?;
            }
            Ok(())
        }
    }

    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let file_path = std::path::Path::new("test.csv");
    let local_wrt = csv::Writer::from_path(file_path).unwrap();

    let mut web_socket_handler = WebSocketHandler::new(local_wrt);
    let agg_trade = String::from("!ticker@arr");
    let mut web_socket = WebSockets::new(move |event: WebsocketEvent| {
        if let WebsocketEvent::DayTickerAll(events) = event {
            // You can break the event_loop if some condition is met be setting keep_running to false
            // keep_running.store(false, Ordering::Relaxed);
            if let Err(error) = web_socket_handler.write_to_file(events) {
                println!("{}", error);
            }
        }

        Ok(())
    });

    web_socket.connect(&agg_trade).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Error: {}", e);
    }
}
