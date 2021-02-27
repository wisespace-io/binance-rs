use crate::account::*;
use crate::market::*;
use crate::general::*;
use crate::futures::general::*;
use crate::futures::market::*;
use crate::userstream::*;
use crate::client::*;

static API_HOST: &str = "https://api.binance.com";
static FAPI_HOST: &str = "https://fapi.binance.com";

//#[derive(Clone)]
pub trait Binance {
    fn new(
        inner_client: Option<reqwest::blocking::Client>, 
        api_key: Option<String>,
        secret_key: Option<String>,
    ) -> Self;
}

impl Binance for General {
    fn new(
        inner_client: Option<reqwest::blocking::Client>, 
        api_key: Option<String>,
        secret_key: Option<String>,
    ) -> General {
        General {
            client: Client::new(inner_client, api_key, secret_key, API_HOST.to_string()),
        }
    }
}

impl Binance for Account {
    fn new(
        inner_client: Option<reqwest::blocking::Client>, 
        api_key: Option<String>,
        secret_key: Option<String>,
    ) -> Account {
        Account {
            client: Client::new(inner_client, api_key, secret_key, API_HOST.to_string()),
            recv_window: 5000,
        }
    }
}

impl Binance for Market {
    fn new(
        inner_client: Option<reqwest::blocking::Client>, 
        api_key: Option<String>,
        secret_key: Option<String>,
    ) -> Market {
        Market {
            client: Client::new(inner_client, api_key, secret_key, API_HOST.to_string()),
            recv_window: 5000,
        }
    }
}

impl Binance for UserStream {
    fn new(
        inner_client: Option<reqwest::blocking::Client>, 
        api_key: Option<String>,
        secret_key: Option<String>,
    ) -> UserStream {
        UserStream {
            client: Client::new(inner_client, api_key, secret_key, API_HOST.to_string()),
            recv_window: 5000,
        }
    }
}

// *****************************************************
//              Binance Futures API
// *****************************************************

impl Binance for FuturesGeneral {
    fn new(
        inner_client: Option<reqwest::blocking::Client>, 
        api_key: Option<String>,
        secret_key: Option<String>,
    ) -> FuturesGeneral {
        FuturesGeneral {
            client: Client::new(inner_client, api_key, secret_key, FAPI_HOST.to_string()),
        }
    }
}

impl Binance for FuturesMarket {
    fn new(
        inner_client: Option<reqwest::blocking::Client>, 
        api_key: Option<String>,
        secret_key: Option<String>,
    ) -> FuturesMarket {
        FuturesMarket {
            client: Client::new(inner_client, api_key, secret_key, FAPI_HOST.to_string()),
            recv_window: 5000,
        }
    }
}
