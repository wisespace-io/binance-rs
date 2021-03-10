use crate::account::*;
use crate::config::*;
use crate::market::*;
use crate::general::*;
use crate::futures::general::*;
use crate::futures::market::*;
use crate::userstream::*;
use crate::client::*;

pub trait Binance {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Self;
    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> Self;
}

impl Binance for General {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> General {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> General {
        General {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
        }
    }
}

impl Binance for Account {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Account {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> Account {
        Account {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
            recv_window: 5000,
        }
    }
}

impl Binance for Market {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Market {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> Market {
        Market {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
            recv_window: 5000,
        }
    }
}

impl Binance for UserStream {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> UserStream {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> UserStream {
        UserStream {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
            recv_window: 5000,
        }
    }
}

// *****************************************************
//              Binance Futures API
// *****************************************************

impl Binance for FuturesGeneral {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> FuturesGeneral {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> FuturesGeneral {
        FuturesGeneral {
            client: Client::new(api_key, secret_key, config.futures_rest_api_endpoint.clone()),
        }
    }
}

impl Binance for FuturesMarket {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> FuturesMarket {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, secret_key: Option<String>, config: &Config,
    ) -> FuturesMarket {
        FuturesMarket {
            client: Client::new(api_key, secret_key, config.futures_rest_api_endpoint.clone()),
            recv_window: 5000,
        }
    }
}
