use crate::errors::*;
use crate::config::*;
use crate::model::*;
use url::Url;
use serde::{Deserialize, Serialize};

use std::sync::atomic::{AtomicBool, Ordering};
use tungstenite::{connect, Message};
use tungstenite::protocol::WebSocket;
use tungstenite::client::AutoStream;
use tungstenite::handshake::client::Response;

#[allow(clippy::all)]
enum FuturesWebsocketAPI {
    Default,
    MultiStream,
    Custom(String),
}

impl FuturesWebsocketAPI {

    fn params(self, subscription: &str) -> String {
        match self {
            FutureWebsocketAPI::Default => format!("wss://fstream.binance.com:9443/ws/{}", subscription),
            FutureWebsocketAPI::MultiStream => format!("wss://fstream.binance.com:9443/stream?streams={}", subscription),
            FutureWebsocketAPI::Custom(url) => url,
        }
    }

}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FuturesWebsocketEvent {
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

pub struct FutureWebSockets<'a> {
    pub socket: Option<(WebSocket<AutoStream>, Response)>,
    handler: Box<dyn FnMut(WebsocketEvent) -> Result<()> + 'a>,
    subscription: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum FuturesEvents {
    Vec(Vec<DayTickerEvent>),
    DayTickerEvent(DayTickerEvent),
    BookTickerEvent(BookTickerEvent),
    AccountUpdateEvent(AccountUpdateEvent),
    OrderTradeEvent(OrderTradeEvent),
    AggrTradesEvent(AggrTradesEvent),
    TradeEvent(TradeEvent),
    KlineEvent(KlineEvent),
    OrderBook(OrderBook),
    DepthOrderBookEvent(DepthOrderBookEvent),
}

impl<'a> FuturesWebSockets<'a> {

    pub fn new<Callback>(handler: Callback) -> FuturesWebSockets<'a>
    where
        Callback: FnMut(WebsocketEvent) -> Result<()> + 'a,
    {
        FuturesWebSockets {
            socket: None,
            handler: Box::new(handler),
            subscription: "",
        }
    }

    pub fn new_with_subscription<Callback>(subscription: &'a str, handler: Callback) -> FuturesWebSockets<'a>
    where
        Callback: FnMut(FuturesWebsocketEvent) -> Result<()> + 'a,
    {
        FuturesWebSockets {
            socket: None,
            handler: Box::new(handler),
            subscription,
        }
    }

    pub fn connect(&mut self, subscription: &'a str) -> Result<()> {
        self.subscription = subscription;
        self.connect_wss(FuturesWebsocketAPI::Default.params(subscription))
    }

    pub fn connect_with_config(&mut self, subscription: &'a str, config: &'a Config) -> Result<()> {
        self.subscription = subscription;
        self.connect_wss(FuturesWebsocketAPI::Custom(config.ws_endpoint.clone()).params(subscription))
    }

    pub fn connect_multiple_streams(&mut self, endpoints: &[String]) -> Result<()> {
        self.connect_wss(FuturesWebsocketAPI::MultiStream.params(&endpoints.join("/")))
    }

    fn connect_wss(&mut self, wss: String) -> Result<()> {
        let url = Url::parse(&wss)?;
        match connect(url) {
            Ok(answer) => {
                self.socket = Some(answer);
                Ok(())
            }
            Err(e) => bail!(format!("Error during handshake {}", e))
        }
    }

    pub fn disconnect(&mut self) -> Result<()> {
        if let Some(ref mut socket) = self.socket {
            socket.0.close(None)?;
            return Ok(());
        }
        bail!("Not able to close the connection");
    }
    
    pub fn test_handle_msg(&mut self, msg: &str) -> Result<()> {
        self.handle_msg(msg)
    }

    fn handle_msg(&mut self, msg: &str) -> Result<()> {

        let value: serde_json::Value = serde_json::from_str(msg)?;

        if let Some(data) = value.get("data") {
            self.handle_msg(&data.to_string())?;
            return Ok(());
        }

        if let Ok(events) = serde_json::from_value::<FuturesEvents>(value) {
            let action = match events {
                FuturesEvents::Vec(v) => WebsocketEvent::DayTickerAll(v),
                FuturesEvents::BookTickerEvent(v) => WebsocketEvent::BookTicker(v),
                FuturesEvents::AccountUpdateEvent(v) => WebsocketEvent::AccountUpdate(v),
                FuturesEvents::OrderTradeEvent(v) => WebsocketEvent::OrderTrade(v),
                FuturesEvents::AggrTradesEvent(v) => WebsocketEvent::AggrTrades(v),
                FuturesEvents::TradeEvent(v) => WebsocketEvent::Trade(v),
                FuturesEvents::DayTickerEvent(v) => WebsocketEvent::DayTicker(v),
                FuturesEvents::KlineEvent(v) => WebsocketEvent::Kline(v),
                FuturesEvents::OrderBook(v) => WebsocketEvent::OrderBook(v),
                FuturesEvents::DepthOrderBookEvent(v) => WebsocketEvent::DepthOrderBook(v),
            };
            (self.handler)(action)?;
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
                    Message::Close(e) => bail!(format!("Disconnected {:?}", e))
                }
            }
        }
        Ok(())
    }

}
