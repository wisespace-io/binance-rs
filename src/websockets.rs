use model::*;
use errors::*;
use url::{Url};
use serde_json::{from_str};

use tungstenite::{connect};
use tungstenite::protocol::WebSocket;
use tungstenite::client::AutoStream;
use tungstenite::handshake::client::{Response};

static WEBSOCKET_URL : &'static str = "wss://stream.binance.com:9443/ws/";

static OUTBOUND_ACCOUNT_INFO : &'static str = "outboundAccountInfo";
static EXECUTION_REPORT : &'static str = "executionReport";

static KLINE : &'static str = "kline";
static AGGREGATED_TRADE : &'static str = "aggTrade";

pub trait UserStreamEventHandler {
    fn account_update_handler(&self, event: &AccountUpdateEvent);
    fn order_trade_handler(&self, event: &OrderTradeEvent);
}

pub trait MarketEventHandler {
    fn aggregated_trades_handler(&self, event: &TradesEvent);
}

pub trait KlineEventHandler {
    fn kline_handler(&self, event: &KlineEvent);
}

pub struct WebSockets {
    socket: Option<(WebSocket<AutoStream>, Response)>, 
    user_stream_handler: Option<Box<UserStreamEventHandler>>,
    market_handler: Option<Box<MarketEventHandler>>,
    kline_handler: Option<Box<KlineEventHandler>>,
}

impl WebSockets {

    pub fn new() -> WebSockets {
        WebSockets {
            socket: None,
            user_stream_handler: None, 
            market_handler: None,     
            kline_handler: None, 
        }
    }

    pub fn connect(&mut self, endpoint: String) -> Result<()> {        
        let wss: String = format!("{}{}", WEBSOCKET_URL, endpoint);
        let url = Url::parse(&wss)?;

        match connect(url) {
            Ok(answer) => {
                self.socket = Some(answer);
                return Ok(());
            },
            Err(e) => {
                bail!(format!("Error during handshake {}", e));
            },
        } 
    }

    pub fn add_user_stream_handler<H>(&mut self, handler: H)
    where
        H: UserStreamEventHandler + 'static,
    {
        self.user_stream_handler = Some(Box::new(handler));
    }

    pub fn add_market_handler<H>(&mut self, handler: H)
    where
        H: MarketEventHandler + 'static,
    {
        self.market_handler = Some(Box::new(handler));
    }    

    pub fn add_kline_handler<H>(&mut self, handler: H)
    where
        H: KlineEventHandler + 'static,
    {
        self.kline_handler = Some(Box::new(handler));
    }  

    pub fn event_loop(&mut self) {
        loop {
            if let Some(ref mut socket) = self.socket {
                let msg: String = socket.0.read_message().unwrap().into_text().unwrap();
                
                if msg.find(OUTBOUND_ACCOUNT_INFO) != None {
                    let account_update: AccountUpdateEvent = from_str(msg.as_str()).unwrap();

                    if let Some(ref h) = self.user_stream_handler {
                        h.account_update_handler(&account_update);
                    }
                } else if msg.find(EXECUTION_REPORT) != None {
                    let order_trade: OrderTradeEvent = from_str(msg.as_str()).unwrap();

                    if let Some(ref h) = self.user_stream_handler {
                        h.order_trade_handler(&order_trade);
                    }
                } else if msg.find(AGGREGATED_TRADE) != None {
                    let trades: TradesEvent = from_str(msg.as_str()).unwrap();

                    if let Some(ref h) = self.market_handler {
                        h.aggregated_trades_handler(&trades);
                    }
                } else if msg.find(KLINE) != None {
                    let kline: KlineEvent = from_str(msg.as_str()).unwrap();

                    if let Some(ref h) = self.kline_handler {
                        h.kline_handler(&kline);
                    }
                }                
            }
        }
    }
}