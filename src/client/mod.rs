mod account;
mod general;
mod market;
mod userstream;
mod websocket;

use transport::Transport;

#[derive(Clone)]
pub struct Binance {
    pub transport: Transport,
}

impl Binance {
    pub fn new() -> Self {
        Binance {
            transport: Transport::new(),
        }
    }

    pub fn with_credential(api_key: &str, api_secret: &str) -> Self {
        Binance {
            transport: Transport::with_credential(api_key, api_secret),
        }
    }
}
