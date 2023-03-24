use crate::{
    error::BinanceError::*,
    model::websocket::{AccountUpdate, BinanceWebsocketMessage, Subscription, UserOrderUpdate},
    rest::Binance,
};
use anyhow::Error;
use fehler::{throw, throws};
use futures::{
    stream::{SplitStream, Stream},
    StreamExt,
};
use log::trace;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::{
    collections::HashMap,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::Message;
use url::Url;

const WS_URL: &'static str = "wss://stream.binance.com:9443/ws";

impl Binance {
    pub fn websocket(&self) -> BinanceWebsocket {
        BinanceWebsocket {
            subscriptions: HashMap::new(),
        }
    }
}

#[allow(dead_code)]
type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct BinanceWebsocket {
    subscriptions: HashMap<Subscription, SplitStream<WSStream>>,
}

impl BinanceWebsocket {
    #[throws(Error)]
    pub async fn subscribe(mut self, subscription: Subscription) -> Self {
        let sub = match subscription {
            Subscription::AggregateTrade(ref symbol) => format!("{}@aggTrade", symbol),
            Subscription::Candlestick(ref symbol, ref interval) => {
                format!("{}@kline_{}", symbol, interval)
            }
            Subscription::Depth(ref symbol) => format!("{}@depth", symbol),
            Subscription::MiniTicker(ref symbol) => format!("{}@miniTicker", symbol),
            Subscription::MiniTickerAll => "!miniTicker@arr".to_string(),
            Subscription::OrderBook(ref symbol, depth) => format!("{}@depth{}", symbol, depth),
            Subscription::Ticker(ref symbol) => format!("{}@ticker", symbol),
            Subscription::TickerAll => "!ticker@arr".to_string(),
            Subscription::Trade(ref symbol) => format!("{}@trade", symbol),
            Subscription::UserData(ref key) => key.clone(),
        };

        trace!("[Websocket] Subscribing to '{:?}'", subscription);

        let endpoint = Url::parse(&format!("{}/{}", WS_URL, sub)).unwrap();
        let (stream, _) = connect_async(endpoint).await?;
        let stream = stream.split().1;
        self.subscriptions.insert(subscription, stream);
        self
    }

    pub fn unsubscribe(&mut self, subscription: &Subscription) -> Option<SplitStream<WSStream>> {
        self.subscriptions.remove(subscription)
    }
}

impl Stream for BinanceWebsocket {
    type Item = Result<BinanceWebsocketMessage, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        if self.subscriptions.is_empty() {
            return Poll::Ready(None);
        }

        for (sub, stream) in &mut self.subscriptions {
            let c = match stream.poll_next_unpin(cx) {
                Poll::Ready(Some(Ok(c))) => c,
                Poll::Ready(Some(Err(e))) => return Poll::Ready(Some(Err(e.into()))),
                Poll::Pending => continue,
                Poll::Ready(None) => continue,
            };
            if let Some(msg) = parse_message(sub.clone(), c)? {
                return Poll::Ready(Some(Ok(msg)));
            }
        }

        return Poll::Pending;
    }
}

#[throws(Error)]
fn parse_message(sub: Subscription, msg: Message) -> Option<BinanceWebsocketMessage> {
    let msg = match msg {
        Message::Text(msg) => msg,
        Message::Binary(_) => return None,
        Message::Pong(..) => return Some(BinanceWebsocketMessage::Pong),
        Message::Ping(..) => return Some(BinanceWebsocketMessage::Ping),
        Message::Close(_) => throw!(WebsocketClosed),
        Message::Frame(_) => return None,
    };

    trace!("Incoming websocket message {}", msg);
    let message = match sub {
        Subscription::AggregateTrade(..) => {
            BinanceWebsocketMessage::AggregateTrade(from_str(&msg)?)
        }
        Subscription::Candlestick(..) => BinanceWebsocketMessage::Candlestick(from_str(&msg)?),
        Subscription::Depth(..) => BinanceWebsocketMessage::Depth(from_str(&msg)?),
        Subscription::MiniTicker(..) => BinanceWebsocketMessage::MiniTicker(from_str(&msg)?),
        Subscription::MiniTickerAll => BinanceWebsocketMessage::MiniTickerAll(from_str(&msg)?),
        Subscription::OrderBook(..) => BinanceWebsocketMessage::OrderBook(from_str(&msg)?),
        Subscription::Ticker(..) => BinanceWebsocketMessage::Ticker(from_str(&msg)?),
        Subscription::TickerAll => BinanceWebsocketMessage::TickerAll(from_str(&msg)?),
        Subscription::Trade(..) => BinanceWebsocketMessage::Trade(from_str(&msg)?),
        Subscription::UserData(..) => {
            let msg: Either<AccountUpdate, UserOrderUpdate> = from_str(&msg)?;
            match msg {
                Either::Left(m) => BinanceWebsocketMessage::UserAccountUpdate(m),
                Either::Right(m) => BinanceWebsocketMessage::UserOrderUpdate(m),
            }
        }
    };
    Some(message)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum Either<L, R> {
    Left(L),
    Right(R),
}
