use model::*;
use errors::*;
use url::{Url};
use serde_json::{from_str};

use futures::{Future, Stream};
use tokio_core::reactor::Core;

use tokio_tungstenite::connect_async;

static WEBSOCKET_URL : &'static str = "wss://stream.binance.com:9443/ws/";
static OUTBOUND_ACCOUNT_INFO : &'static str = "outboundAccountInfo";
static EXECUTION_REPORT : &'static str = "executionReport";

pub trait EventHandler {
    fn account_update_handler(&self, event: &AccountUpdateEvent);
    fn order_trade_handler(&self, event: &OrderTradeEvent);
}

pub struct WebSockets {
    handler: Option<Box<EventHandler>>,
}

impl WebSockets {

    pub fn new() -> WebSockets {
        WebSockets {
            handler: None,        
        }
    }

    pub fn connect_stream(&mut self, endpoint: String) -> Result<()> {
        
        let wss: String = format!("{}{}", WEBSOCKET_URL, endpoint);
        let url = Url::parse(&wss).unwrap();

        let mut event = Core::new().unwrap();
        let handle = event.handle();
        let client = connect_async(url, handle.remote().clone()).and_then(|(ws_stream, _)| {

            let (_sink, stream) = ws_stream.split();

            let result = stream.for_each(|message| {
                let msg: String = message.into_text().unwrap();

                if msg.find(OUTBOUND_ACCOUNT_INFO) != None {
                    let account_update: AccountUpdateEvent = from_str(msg.as_str()).unwrap();

                    if let Some(ref h) = self.handler {
                        h.account_update_handler(&account_update);
                    }
                } else if msg.find(EXECUTION_REPORT) != None {
                    let order_trade: OrderTradeEvent = from_str(msg.as_str()).unwrap();

                    if let Some(ref h) = self.handler {
                        h.order_trade_handler(&order_trade);
                    }
                }

                Ok(())
            });

            result.map(|_| ()).then(|_| Ok(()))
        }).map_err(|e| Error::with_chain(e, "Error during the websocket handshake"));        

        event.run(client).unwrap();
        Ok(())
    }

    pub fn handler<H>(&mut self, handler: H)
    where
        H: EventHandler + 'static,
    {
        self.handler = Some(Box::new(handler));
    }

    pub fn event_loop() {
        //self.event_loop.run(client).unwrap();
    }
}