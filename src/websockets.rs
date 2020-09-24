use crate::model::*;
use crate::errors::*;
use url::Url;
use serde_json::from_str;

use std::sync::atomic::{AtomicBool, Ordering};
use tungstenite::{connect, Message};
use tungstenite::protocol::WebSocket;
use tungstenite::client::AutoStream;
use tungstenite::handshake::client::Response;

static WEBSOCKET_URL: &str = "wss://stream.binance.com:9443/ws/";

static OUTBOUND_ACCOUNT_INFO: &str = "outboundAccountInfo";
static EXECUTION_REPORT: &str = "executionReport";

static KLINE: &str = "kline";
static AGGREGATED_TRADE: &str = "aggTrade";
static DEPTH_ORDERBOOK: &str = "depthUpdate";
static PARTIAL_ORDERBOOK: &str = "lastUpdateId";

static DAYTICKER: &str = "24hrTicker";

#[allow(clippy::large_enum_variant)]
pub enum WebsocketEvent {
    AccountUpdate(AccountUpdateEvent),
    OrderTrade(OrderTradeEvent),
    Trade(TradesEvent),
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
    subscription: &'a str
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

    pub fn disconnect(&mut self) -> Result<()> {
        if let Some(ref mut socket) = self.socket {
            socket.0.close(None)?;
            Ok(())
        } else {
            bail!("Not able to close the connection");
        }
    }

    pub fn event_loop(&mut self, running: &AtomicBool) -> Result<()> {
        while running.load(Ordering::Relaxed) {
            if let Some(ref mut socket) = self.socket {
                let message = socket.0.read_message()?;
                let value: serde_json::Value = serde_json::from_str(message.to_text()?)?;

                match message {
                    Message::Text(msg) => {
                        if value["u"] != serde_json::Value::Null &&
                            value["s"] != serde_json::Value::Null &&
                            value["b"] != serde_json::Value::Null &&
                            value["B"] != serde_json::Value::Null &&
                            value["a"] != serde_json::Value::Null &&
                            value["A"] != serde_json::Value::Null
                        {
                            let book_ticker: BookTickerEvent = from_str(msg.as_str())?;
                            (self.handler)(WebsocketEvent::BookTicker(book_ticker))?;
                        } else if msg.find(OUTBOUND_ACCOUNT_INFO) != None {
                            let account_update: AccountUpdateEvent = from_str(msg.as_str())?;
                            (self.handler)(WebsocketEvent::AccountUpdate(account_update))?;
                        } else if msg.find(EXECUTION_REPORT) != None {
                            let order_trade: OrderTradeEvent = from_str(msg.as_str())?;
                            (self.handler)(WebsocketEvent::OrderTrade(order_trade))?;
                        } else if msg.find(AGGREGATED_TRADE) != None {
                            let trade: TradesEvent = from_str(msg.as_str())?;
                            (self.handler)(WebsocketEvent::Trade(trade))?;
                        } else if msg.find(DAYTICKER) != None {
                            if self.subscription == "!ticker@arr" {
                                let trades: Vec<DayTickerEvent> = from_str(msg.as_str())?;
                                (self.handler)(WebsocketEvent::DayTickerAll(trades))?;
                            } else {
                                let trades: DayTickerEvent = from_str(msg.as_str())?;
                                (self.handler)(WebsocketEvent::DayTicker(trades))?;
                            }
                        } else if msg.find(KLINE) != None {
                            let kline: KlineEvent = from_str(msg.as_str())?;
                            (self.handler)(WebsocketEvent::Kline(kline))?;
                        } else if msg.find(PARTIAL_ORDERBOOK) != None {
                            let partial_orderbook: OrderBook = from_str(msg.as_str())?;
                            (self.handler)(WebsocketEvent::OrderBook(partial_orderbook))?;
                        } else if msg.find(DEPTH_ORDERBOOK) != None {
                            let depth_orderbook: DepthOrderBookEvent = from_str(msg.as_str())?;
                            (self.handler)(WebsocketEvent::DepthOrderBook(depth_orderbook))?;
                        }
                    }
                    Message::Ping(_) | Message::Pong(_) | Message::Binary(_) => {}
                    Message::Close(e) => {
                        bail!(format!("Disconnected {:?}", e));
                    }
                }
            }
        }
        Ok(())
    }
}
