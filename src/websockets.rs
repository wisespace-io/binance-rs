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

pub trait UserStreamEventHandler {
    fn account_update_handler(&mut self, event: &AccountUpdateEvent);
    fn order_trade_handler(&mut self, event: &OrderTradeEvent);
}

pub trait MarketEventHandler {
    fn aggregated_trades_handler(&mut self, event: &TradesEvent);
    fn depth_orderbook_handler(&mut self, event: &DepthOrderBookEvent);
    fn partial_orderbook_handler(&mut self, order_book: &OrderBook);
}

pub trait DayTickerEventHandler {
    fn day_ticker_handler(&mut self, event: &[DayTickerEvent]);
}

pub trait KlineEventHandler {
    fn kline_handler(&mut self, event: &KlineEvent);
}

#[derive(Default)]
pub struct WebSockets<'a> {
    socket: Option<(WebSocket<AutoStream>, Response)>,
    user_stream_handler: Option<&'a mut UserStreamEventHandler>,
    market_handler: Option<&'a mut MarketEventHandler>,
    ticker_handler: Option<&'a mut DayTickerEventHandler>,
    kline_handler: Option<&'a mut KlineEventHandler>,
}

impl<'a> WebSockets<'a> {
    pub fn new() -> WebSockets<'a> {
        WebSockets {
            socket: None,
            user_stream_handler: None,
            market_handler: None,
            ticker_handler: None,
            kline_handler: None,
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

    pub fn add_user_stream_handler(&mut self, handler: &'a mut UserStreamEventHandler) {
        self.user_stream_handler = Some(handler);
    }

    pub fn add_market_handler(&mut self, handler: &'a mut MarketEventHandler) {
        self.market_handler = Some(handler);
    }

    pub fn add_day_ticker_handler(&mut self, handler: &'a mut DayTickerEventHandler) {
        self.ticker_handler = Some(handler);
    }

    pub fn add_kline_handler(&mut self, handler: &'a mut KlineEventHandler) {
        self.kline_handler = Some(handler);
    }

    pub fn event_loop(&mut self) {
        loop {
            if let Some(ref mut socket) = self.socket {
                let msg: String = socket.0.read_message().unwrap().into_text().unwrap();

                if msg.find(OUTBOUND_ACCOUNT_INFO) != None {
                    let account_update: AccountUpdateEvent = from_str(msg.as_str()).unwrap();

                    if let Some(ref mut h) = self.user_stream_handler {
                        h.account_update_handler(&account_update);
                    }
                } else if msg.find(EXECUTION_REPORT) != None {
                    let order_trade: OrderTradeEvent = from_str(msg.as_str()).unwrap();

                    if let Some(ref mut h) = self.user_stream_handler {
                        h.order_trade_handler(&order_trade);
                    }
                } else if msg.find(AGGREGATED_TRADE) != None {
                    let trades: TradesEvent = from_str(msg.as_str()).unwrap();

                    if let Some(ref mut h) = self.market_handler {
                        h.aggregated_trades_handler(&trades);
                    }
                } else if msg.find(DAYTICKER) != None {
                    let trades: Vec<DayTickerEvent> = from_str(msg.as_str()).unwrap();

                    if let Some(ref mut h) = self.ticker_handler {
                        h.day_ticker_handler(&trades);
                    }
                } else if msg.find(KLINE) != None {
                    let kline: KlineEvent = from_str(msg.as_str()).unwrap();

                    if let Some(ref mut h) = self.kline_handler {
                        h.kline_handler(&kline);
                    }
                } else if msg.find(PARTIAL_ORDERBOOK) != None {
                    let partial_orderbook: OrderBook = from_str(msg.as_str()).unwrap();

                    if let Some(ref mut h) = self.market_handler {
                        h.partial_orderbook_handler(&partial_orderbook);
                    }
                } else if msg.find(DEPTH_ORDERBOOK) != None {
                    let depth_orderbook: DepthOrderBookEvent = from_str(msg.as_str()).unwrap();

                    if let Some(ref mut h) = self.market_handler {
                        h.depth_orderbook_handler(&depth_orderbook);
                    }
                }
            }
        }
    }
}
