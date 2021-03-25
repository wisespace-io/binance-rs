use crate::account::*;
use crate::config::*;
use crate::market::*;
use crate::general::*;
use crate::futures::general::*;
use crate::futures::market::*;
use crate::userstream::*;
use crate::client::*;

pub enum API {
    Spot(Spot),
    Futures(Futures),
}

/// Endpoint for production and test orders.
///
/// Orders issued to test are validated, but not sent into the matching engine.
pub enum Spot {
    Ping,
    Time,
    ExchangeInfo,
    Order,
    OrderTest,
    Account,
    OpenOrders,
    MyTrades,
    Depth,
    Price,
    AvgPrice,
    BookTicker,
    Ticker24hr,
    Klines,
    UserDataStream,
}

pub enum Futures {
    Ping,
    Time,
    ExchangeInfo,
    Trades,
    HistoricalTrades,
    AggTrades,
    Depth,
    BookTicker,
    Ticker24hr,
    TickerPrice,
    Klines,
    PremiumIndex,
    AllForceOrders,
    OpenInterest,
}

impl From<API> for String {
    fn from(item: API) -> Self {
        match item {
            API::Spot(route) => {
                match route {
                    Spot::Ping => String::from("/api/v3/ping"),
                    Spot::Time => String::from("/api/v3/time"),
                    Spot::ExchangeInfo => String::from("/api/v3/exchangeInfo"),
                    Spot::Order => String::from("/api/v3/order"),
                    Spot::OrderTest => String::from("/api/v3/order/test"),
                    Spot::Account => String::from("/api/v3/account"),
                    Spot::OpenOrders => String::from("/api/v3/openOrders"),
                    Spot::MyTrades => String::from("/api/v3/myTrades"),
                    Spot::Depth => String::from("/api/v3/depth"),
                    Spot::Price => String::from("/api/v3/price"),
                    Spot::AvgPrice => String::from("/api/v3/avgPrice"),
                    Spot::BookTicker => String::from("/api/v3/ticker/bookTicker"),
                    Spot::Ticker24hr => String::from("/api/v3/ticker/24h"),
                    Spot::Klines => String::from("/api/v3/klines"),
                    Spot::UserDataStream => String::from("/api/v3/userDataStream"),
                }
            },
            API::Futures(route) => {
                match route {
                    Futures::Ping => String::from("/fapi/v1/ping"),
                    Futures::Time => String::from("/fapi/v1/time"),
                    Futures::ExchangeInfo => String::from("/fapi/v1/exchangeInfo"),
                    Futures::Trades => String::from("/fapi/v1/trades"),
                    Futures::HistoricalTrades => String::from("/fapi/v1/historicalTrades"),
                    Futures::AggTrades => String::from("/fapi/v1/aggTrades"),
                    Futures::Depth => String::from("/fapi/v1/depth"),
                    Futures::BookTicker => String::from("/fapi/v1/ticker/bookTicker"),
                    Futures::Ticker24hr => String::from("/fapi/v1/ticker/24hr"),
                    Futures::TickerPrice => String::from("/fapi/v1/ticker/price"),
                    Futures::Klines => String::from("/fapi/v1/klines"),
                    Futures::PremiumIndex => String::from("/fapi/v1/premiumIndex"),
                    Futures::AllForceOrders => String::from("/fapi/v1/allForceOrders"),
                    Futures::OpenInterest => String::from("/fapi/v1/openInterest"),
                }
            }
        }
    }
}

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
            recv_window: config.recv_window,
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
            recv_window: config.recv_window,
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
            recv_window: config.recv_window,
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
            recv_window: config.recv_window,
        }
    }
}
