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

enum WebsocketAPI {
    Default,
    MultiStream,
    Custom(String),
}

impl WebsocketAPI {

    fn params<'a>(self, subscription: &'a str) -> String {
        match self {
            WebsocketAPI::Default => String::from(format!("wss://stream.binance.com:9443/ws/{}", subscription)),
            WebsocketAPI::MultiStream => String::from(format!("wss://stream.binance.com:9443/stream?streams={}", subscription)),
            WebsocketAPI::Custom(url) => url,
        }
    }

}

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

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Events {
    BookTickerEvent(BookTickerEvent),
    AccountUpdateEvent(AccountUpdateEvent),
    OrderTradeEvent(OrderTradeEvent),
    AggrTradesEvent(AggrTradesEvent),
    TradeEvent(TradeEvent),
    DayTickerEvent(DayTickerEvent),
    KlineEvent(KlineEvent),
    OrderBook(OrderBook),
    DepthOrderBookEvent(DepthOrderBookEvent),
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
        self.connect_wss(WebsocketAPI::Default.params(subscription))
    }

    pub fn connect_with_config(&mut self, subscription: &'a str, config: &'a Config) -> Result<()> {
        self.subscription = subscription;
        self.connect_wss(WebsocketAPI::Custom(config.ws_endpoint.clone()).params(subscription))
    }

    pub fn connect_multiple_streams(&mut self, endpoints: &[String]) -> Result<()> {
        self.connect_wss(WebsocketAPI::MultiStream.params(&endpoints.join("/")))
    }

    fn connect_wss(&mut self, wss: String) -> Result<()> {
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
            return Ok(());
        }
        bail!("Not able to close the connection");
    }

    fn handle_msg(&mut self, msg: &str) -> Result<()> {

        let value: serde_json::Value = serde_json::from_str(msg)?;

        if let Some(data) = value.get("data") {
            self.handle_msg(&data.to_string())?;
            return Ok(());
        }

        // Special case to handle Vec<DayTickerEvent>.
        if self.subscription == "!ticker@arr" {
            if let Ok(trades) = serde_json::from_value::<Vec<DayTickerEvent>>(value) {
                (self.handler)(WebsocketEvent::DayTickerAll(trades))?;
            }
            return Ok(());
        }

        if let Ok(events) = serde_json::from_value::<Events>(value) {
            let action = match events {
                Events::BookTickerEvent(v) => WebsocketEvent::BookTicker(v),
                Events::AccountUpdateEvent(v) => WebsocketEvent::AccountUpdate(v),
                Events::OrderTradeEvent(v) => WebsocketEvent::OrderTrade(v),
                Events::AggrTradesEvent(v) => WebsocketEvent::AggrTrades(v),
                Events::TradeEvent(v) => WebsocketEvent::Trade(v),
                Events::DayTickerEvent(v) => WebsocketEvent::DayTicker(v),
                Events::KlineEvent(v) => WebsocketEvent::Kline(v),
                Events::OrderBook(v) => WebsocketEvent::OrderBook(v),
                Events::DepthOrderBookEvent(v) => WebsocketEvent::DepthOrderBook(v),
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
                    Message::Close(e) => bail!(format!("Disconnected {:?}", e));
                }
            }
        }
        Ok(())
    }
}
