use crate::errors::*;
use crate::config::*;
use crate::model::*;
use url::Url;
use serde_json::from_str;
use serde::{Deserialize, Serialize};

use std::sync::atomic::{AtomicBool, Ordering};
use tungstenite::{connect, Message};
use tungstenite::protocol::WebSocket;
use tungstenite::client::AutoStream;
use tungstenite::handshake::client::Response;

static WEBSOCKET_URL: &str = "wss://stream.binance.com:9443/ws/";
static WEBSOCKET_MULTI_STREAM: &str = "wss://stream.binance.com:9443/stream?streams="; // <streamName1>/<streamName2>/<streamName3>

static OUTBOUND_ACCOUNT_INFO: &str = "outboundAccountInfo";
static EXECUTION_REPORT: &str = "executionReport";

static KLINE: &str = "kline";
static AGGREGATED_TRADE: &str = "aggTrade";
static TRADE: &str = "trade";
static DEPTH_ORDERBOOK: &str = "depthUpdate";
static PARTIAL_ORDERBOOK: &str = "lastUpdateId";
static STREAM: &str = "stream";

static DAYTICKER: &str = "24hrTicker";

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WebsocketEvent {
    AccountUpdate(AccountUpdateEvent),
    OrderTrade(OrderTradeEvent),
    AggrTrades(AggrTradesEvent),
    Trade(TradeEvent),
    OrderBook(OrderBook),
    DayTicker(DayTickerEvent),
    DayTickerAll(Vec<DayTickerEvent>),
    Kline(KlineEvent),
    DepthOrderBook(DepthOrderBookEvent),
    BookTicker(BookTickerEvent),
}

pub struct WebSockets<'a> {
    pub socket: Option<(WebSocket<AutoStream>, Response)>,
    handler: Box<dyn FnMut(WebsocketEvent) -> Result<()> + 'a>,
    subscription: &'a str,
}

impl<'a> WebSockets<'a> {
    pub fn new<Callback>(handler: Callback) -> WebSockets<'a>
    where
        Callback: FnMut(WebsocketEvent) -> Result<()> + 'a,
    {
        WebSockets {
            socket: None,
            handler: Box::new(handler),
            subscription: "",
        }
    }

    pub fn connect(&mut self, subscription: &'a str) -> Result<()> {
        self.subscription = subscription;
        let wss: String = format!("{}{}", WEBSOCKET_URL, subscription);
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

    pub fn connect_with_config(&mut self, subscription: &'a str, config: &'a Config) -> Result<()> {
        self.subscription = subscription;
        let wss: String = format!("{}{}", &config.ws_endpoint, subscription);
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

    pub fn connect_multiple_streams(&mut self, endpoints: &[String]) -> Result<()> {
        let wss: String = format!("{}{}", WEBSOCKET_MULTI_STREAM, endpoints.join("/"));
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

    pub fn disconnect(&mut self) -> Result<()> {
        if let Some(ref mut socket) = self.socket {
            socket.0.close(None)?;
            Ok(())
        } else {
            bail!("Not able to close the connection");
        }
    }

    fn handle_msg(&mut self, msg: &str) -> Result<()> {
        let value: serde_json::Value = serde_json::from_str(msg)?;
        if msg.find(STREAM) != None {
            if value["data"] != serde_json::Value::Null {
                let data = format!("{}", value["data"]);
                self.handle_msg(&data)?;
            }
        } else if value["u"] != serde_json::Value::Null
            && value["s"] != serde_json::Value::Null
            && value["b"] != serde_json::Value::Null
            && value["B"] != serde_json::Value::Null
            && value["a"] != serde_json::Value::Null
            && value["A"] != serde_json::Value::Null
        {
            let book_ticker: BookTickerEvent = from_str(msg)?;
            (self.handler)(WebsocketEvent::BookTicker(book_ticker))?;
        } else if msg.find(OUTBOUND_ACCOUNT_INFO) != None {
            let account_update: AccountUpdateEvent = from_str(msg)?;
            (self.handler)(WebsocketEvent::AccountUpdate(account_update))?;
        } else if msg.find(EXECUTION_REPORT) != None {
            let order_trade: OrderTradeEvent = from_str(msg)?;
            (self.handler)(WebsocketEvent::OrderTrade(order_trade))?;
        } else if msg.find(AGGREGATED_TRADE) != None {
            let trade: AggrTradesEvent = from_str(msg)?;
            (self.handler)(WebsocketEvent::AggrTrades(trade))?;
        } else if msg.find(TRADE) != None {
            let trade: TradeEvent = from_str(msg)?;
            (self.handler)(WebsocketEvent::Trade(trade))?;
        } else if msg.find(DAYTICKER) != None {
            if self.subscription == "!ticker@arr" {
                let trades: Vec<DayTickerEvent> = from_str(msg)?;
                (self.handler)(WebsocketEvent::DayTickerAll(trades))?;
            } else {
                let trades: DayTickerEvent = from_str(msg)?;
                (self.handler)(WebsocketEvent::DayTicker(trades))?;
            }
        } else if msg.find(KLINE) != None {
            let kline: KlineEvent = from_str(msg)?;
            (self.handler)(WebsocketEvent::Kline(kline))?;
        } else if msg.find(PARTIAL_ORDERBOOK) != None {
            let partial_orderbook: OrderBook = from_str(msg)?;
            (self.handler)(WebsocketEvent::OrderBook(partial_orderbook))?;
        } else if msg.find(DEPTH_ORDERBOOK) != None {
            let depth_orderbook: DepthOrderBookEvent = from_str(msg)?;
            (self.handler)(WebsocketEvent::DepthOrderBook(depth_orderbook))?;
        }
        Ok(())
    }

    pub fn event_loop(&mut self, running: &AtomicBool) -> Result<()> {
        while running.load(Ordering::Relaxed) {
            if let Some(ref mut socket) = self.socket {
                let message = socket.0.read_message()?;
                match message {
                    Message::Text(msg) => {
                        if let Err(e) = self.handle_msg(&msg) {
                            bail!(format!("Error on handling stream message: {}", e));
                        }
                    }
                    Message::Ping(_) | Message::Pong(_) | Message::Binary(_) => (),
                    Message::Close(e) => {
                        bail!(format!("Disconnected {:?}", e));
                    }
                }
            }
        }
        Ok(())
    }
}
