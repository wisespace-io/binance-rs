use model::*;
use errors::*;
use url::Url;
use serde_json::from_str;

use tungstenite::connect;
use tungstenite::protocol::WebSocket;
use tungstenite::client::AutoStream;
use tungstenite::handshake::client::Response;

static WEBSOCKET_URL: &'static str = "wss://stream.binance.com:9443/ws/";

static OUTBOUND_ACCOUNT_INFO: &'static str = "outboundAccountInfo";
static EXECUTION_REPORT: &'static str = "executionReport";

static KLINE: &'static str = "kline";
static AGGREGATED_TRADE: &'static str = "aggTrade";
static DEPTH_ORDERBOOK : &'static str = "depthUpdate";
static PARTIAL_ORDERBOOK : &'static str = "lastUpdateId";

static DAYTICKER: &'static str = "24hrTicker";

pub enum WebsocketEvent {
    AccountUpdate(AccountUpdateEvent),
    OrderTrade(OrderTradeEvent),
    Trade(TradesEvent),
    OrderBook(OrderBook),
    DayTicker(Vec<DayTickerEvent>),
    Kline(KlineEvent),
    DepthOrderBook(DepthOrderBookEvent),
}

pub struct WebSockets<'a> {
    socket: Option<(WebSocket<AutoStream>, Response)>,
    handler: Box<FnMut(WebsocketEvent) + 'a>,
}

impl<'a> WebSockets<'a> {
    pub fn new<Callback>(handler: Callback) -> WebSockets<'a>
    where
        Callback: FnMut(WebsocketEvent) + 'a
    {
        WebSockets {
            socket: None,
            handler: Box::new(handler),
        }
    }

    pub fn connect(&mut self, endpoint: &str) -> Result<()> {
        let wss: String = format!("{}{}", WEBSOCKET_URL, endpoint);
        let url = Url::parse(&wss)?;

        match connect(url) {
            Ok(answer) => {
                self.socket = Some(answer);
                Ok(())
            }
            Err(e) => {
                bail!(format!("Error during handshake {}", e));
            }
        }
    }

    pub fn event_loop(&mut self) {
        loop {
            if let Some(ref mut socket) = self.socket {
                let msg: String = socket.0.read_message().unwrap().into_text().unwrap();

                if msg.find(OUTBOUND_ACCOUNT_INFO) != None {
                    let account_update: AccountUpdateEvent = from_str(msg.as_str()).unwrap();

                    (self.handler)(WebsocketEvent::AccountUpdate(account_update));
                } else if msg.find(EXECUTION_REPORT) != None {
                    let order_trade: OrderTradeEvent = from_str(msg.as_str()).unwrap();

                    (self.handler)(WebsocketEvent::OrderTrade(order_trade));
                } else if msg.find(AGGREGATED_TRADE) != None {
                    let trade: TradesEvent = from_str(msg.as_str()).unwrap();

                    (self.handler)(WebsocketEvent::Trade(trade));
                } else if msg.find(DAYTICKER) != None {
                    let trades: Vec<DayTickerEvent> = from_str(msg.as_str()).unwrap();

                    (self.handler)(WebsocketEvent::DayTicker(trades));
                } else if msg.find(KLINE) != None {
                    let kline: KlineEvent = from_str(msg.as_str()).unwrap();

                    (self.handler)(WebsocketEvent::Kline(kline));
                } else if msg.find(PARTIAL_ORDERBOOK) != None {
                    let partial_orderbook: OrderBook = from_str(msg.as_str()).unwrap();

                    (self.handler)(WebsocketEvent::OrderBook(partial_orderbook));
                } else if msg.find(DEPTH_ORDERBOOK) != None {
                    let depth_orderbook: DepthOrderBookEvent = from_str(msg.as_str()).unwrap();

                    (self.handler)(WebsocketEvent::DepthOrderBook(depth_orderbook));
                }
            }
        }
    }
}
