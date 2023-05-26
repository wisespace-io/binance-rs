use error_chain::bail;

use crate::api::{Convert, Sapi, Spot, API};
use crate::client::Client;
use crate::errors::Result;
use crate::model::{
    AccountInformation, AccountSnapshot, Balance, Empty, Order,
    OrderCanceled, Quote, QuoteResponse, TradeHistory, Transaction,
};
use crate::util::build_signed_request;
use std::collections::BTreeMap;
use std::fmt::Display;

#[derive(Clone)]
pub struct Account {
    pub client: Client,
    pub recv_window: u64,
}

struct OrderRequest {
    pub symbol: String,
    pub qty: f64,
    pub price: f64,
    pub stop_price: Option<f64>,
    pub order_side: OrderSide,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    pub new_client_order_id: Option<String>,
}

struct OrderQuoteQuantityRequest {
    pub symbol: String,
    pub quote_order_qty: f64,
    pub price: f64,
    pub order_side: OrderSide,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    pub new_client_order_id: Option<String>,
}
pub enum WalletType {
    SPOT,
    FUNDING,
}

pub enum ValidTime {
    TenSeconds,
    ThirtySeconds,
    OneMinutes,
    TwoMinutes,
}

///* "From" When specified, it is the amount you will be debited after the conversion
///* "To" When specified, it is the amount you will be credited after the conversion
pub enum QtyType<T: Into<f64>> {
    From(T),
    To(T),
}

struct OrderQuoteRequest<T: Into<f64>> {
    pub from_asset: String,
    pub to_asset: String,
    pub from_or_to_amount: QtyType<T>,
    pub wallet_type: Option<WalletType>,
    // default 10s
    pub valid_time: Option<ValidTime>,
}

struct AccountSnapshotRequest {
    // "SPOT", "MARGIN", "FUTURES"
    pub type_: String,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    // min 7, max 30, default 7
    pub limit: Option<u16>,
}

pub enum OrderType {
    Limit,
    Market,
    StopLossLimit,
}

impl Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Limit => write!(f, "LIMIT"),
            Self::Market => write!(f, "MARKET"),
            Self::StopLossLimit => write!(f, "STOP_LOSS_LIMIT"),
        }
    }
}

pub enum OrderSide {
    Buy,
    Sell,
}

impl Display for OrderSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Buy => write!(f, "BUY"),
            Self::Sell => write!(f, "SELL"),
        }
    }
}

#[allow(clippy::all)]
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
}

impl Display for TimeInForce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GTC => write!(f, "GTC"),
            Self::IOC => write!(f, "IOC"),
            Self::FOK => write!(f, "FOK"),
        }
    }
}

impl Account {
    // Account Information
    pub fn get_account(&self) -> Result<AccountInformation> {
        let request =
            build_signed_request(BTreeMap::new(), self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::Account), Some(request))
    }

    // Balance for a single Asset
    pub fn get_balance<S>(&self, asset: S) -> Result<Balance>
    where
        S: Into<String>,
    {
        match self.get_account() {
            Ok(account) => {
                let cmp_asset = asset.into();
                for balance in account.balances {
                    if balance.asset == cmp_asset {
                        return Ok(balance);
                    }
                }
                bail!("Asset not found");
            }
            Err(e) => Err(e),
        }
    }

    // Current open orders for ONE symbol
    pub fn get_open_orders<S>(&self, symbol: S) -> Result<Vec<Order>>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());

        let request =
            build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::OpenOrders), Some(request))
    }

    // All current open orders
    pub fn get_all_open_orders(&self) -> Result<Vec<Order>> {
        let parameters: BTreeMap<String, String> = BTreeMap::new();

        let request =
            build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::OpenOrders), Some(request))
    }

    // Cancel all open orders for a single symbol
    pub fn cancel_all_open_orders<S>(
        &self,
        symbol: S,
    ) -> Result<Vec<OrderCanceled>>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request =
            build_signed_request(parameters, self.recv_window)?;
        self.client
            .delete_signed(API::Spot(Spot::OpenOrders), Some(request))
    }

    // Check an order's status
    pub fn order_status<S>(
        &self,
        symbol: S,
        order_id: u64,
    ) -> Result<Order>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("orderId".into(), order_id.to_string());

        let request =
            build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::Order), Some(request))
    }

    /// Place a test status order
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub fn test_order_status<S>(
        &self,
        symbol: S,
        order_id: u64,
    ) -> Result<()>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("orderId".into(), order_id.to_string());

        let request =
            build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed::<Empty>(
                API::Spot(Spot::OrderTest),
                Some(request),
            )
            .map(|_| ())
    }

    // Place a LIMIT order - BUY
    pub fn limit_buy<S, F>(
        &self,
        symbol: S,
        qty: F,
        price: f64,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: None,
            order_side: OrderSide::Buy,
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_order(buy);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Spot(Spot::Order), request)
    }

    /// Place a test limit order - BUY
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub fn test_limit_buy<S, F>(
        &self,
        symbol: S,
        qty: F,
        price: f64,
    ) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: None,
            order_side: OrderSide::Buy,
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_order(buy);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .map(|_| ())
    }

    // Place a LIMIT order - SELL
    pub fn limit_sell<S, F>(
        &self,
        symbol: S,
        qty: F,
        price: f64,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: None,
            order_side: OrderSide::Sell,
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Spot(Spot::Order), request)
    }

    /// Place a test LIMIT order - SELL
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub fn test_limit_sell<S, F>(
        &self,
        symbol: S,
        qty: F,
        price: f64,
    ) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: None,
            order_side: OrderSide::Sell,
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .map(|_| ())
    }

    // Place a MARKET order - BUY
    pub fn market_buy<S, F>(
        &self,
        symbol: S,
        qty: F,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price: 0.0,
            stop_price: None,
            order_side: OrderSide::Buy,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_order(buy);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Spot(Spot::Order), request)
    }

    /// Place a test MARKET order - BUY
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub fn test_market_buy<S, F>(&self, symbol: S, qty: F) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price: 0.0,
            stop_price: None,
            order_side: OrderSide::Buy,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_order(buy);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .map(|_| ())
    }

    // Place a MARKET order with quote quantity - BUY
    pub fn market_buy_using_quote_quantity<S, F>(
        &self,
        symbol: S,
        quote_order_qty: F,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy = OrderQuoteQuantityRequest {
            symbol: symbol.into(),
            quote_order_qty: quote_order_qty.into(),
            price: 0.0,
            order_side: OrderSide::Buy,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_quote_quantity_order(buy);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Spot(Spot::Order), request)
    }

    /// Place a test MARKET order with quote quantity - BUY
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub fn test_market_buy_using_quote_quantity<S, F>(
        &self,
        symbol: S,
        quote_order_qty: F,
    ) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy = OrderQuoteQuantityRequest {
            symbol: symbol.into(),
            quote_order_qty: quote_order_qty.into(),
            price: 0.0,
            order_side: OrderSide::Buy,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_quote_quantity_order(buy);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .map(|_| ())
    }

    // Place a MARKET order - SELL
    pub fn market_sell<S, F>(
        &self,
        symbol: S,
        qty: F,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price: 0.0,
            stop_price: None,
            order_side: OrderSide::Sell,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Spot(Spot::Order), request)
    }

    /// Place a test MARKET order - SELL
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub fn test_market_sell<S, F>(
        &self,
        symbol: S,
        qty: F,
    ) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price: 0.0,
            stop_price: None,
            order_side: OrderSide::Sell,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .map(|_| ())
    }

    // Place a MARKET order with quote quantity - SELL
    pub fn market_sell_using_quote_quantity<S, F>(
        &self,
        symbol: S,
        quote_order_qty: F,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderQuoteQuantityRequest {
            symbol: symbol.into(),
            quote_order_qty: quote_order_qty.into(),
            price: 0.0,
            order_side: OrderSide::Sell,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_quote_quantity_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Spot(Spot::Order), request)
    }

    /// Place a test MARKET order with quote quantity - SELL
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub fn test_market_sell_using_quote_quantity<S, F>(
        &self,
        symbol: S,
        quote_order_qty: F,
    ) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderQuoteQuantityRequest {
            symbol: symbol.into(),
            quote_order_qty: quote_order_qty.into(),
            price: 0.0,
            order_side: OrderSide::Sell,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_quote_quantity_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .map(|_| ())
    }

    /// Create a stop limit buy order for the given symbol, price and stop price.
    /// Returning a `Transaction` value with the same parameters sent on the order.
    ///
    ///```no_run
    /// use binance::api::Binance;
    /// use binance::account::*;
    ///
    /// fn main() {
    ///     let api_key = Some("api_key".into());
    ///     let secret_key = Some("secret_key".into());
    ///     let account: Account = Binance::new(api_key, secret_key);
    ///     let result = account.stop_limit_buy_order("LTCBTC", 1, 0.1, 0.09, TimeInForce::GTC);
    /// }
    /// ```
    pub fn stop_limit_buy_order<S, F>(
        &self,
        symbol: S,
        qty: F,
        price: f64,
        stop_price: f64,
        time_in_force: TimeInForce,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: Some(stop_price),
            order_side: OrderSide::Buy,
            order_type: OrderType::StopLossLimit,
            time_in_force,
            new_client_order_id: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Spot(Spot::Order), request)
    }

    /// Create a stop limit buy test order for the given symbol, price and stop price.
    /// Returning a `Transaction` value with the same parameters sent on the order.
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    ///
    ///```no_run
    /// use binance::api::Binance;
    /// use binance::account::*;
    ///
    /// fn main() {
    ///     let api_key = Some("api_key".into());
    ///     let secret_key = Some("secret_key".into());
    ///     let account: Account = Binance::new(api_key, secret_key);
    ///     let result = account.test_stop_limit_buy_order("LTCBTC", 1, 0.1, 0.09, TimeInForce::GTC);
    /// }
    /// ```
    pub fn test_stop_limit_buy_order<S, F>(
        &self,
        symbol: S,
        qty: F,
        price: f64,
        stop_price: f64,
        time_in_force: TimeInForce,
    ) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: Some(stop_price),
            order_side: OrderSide::Buy,
            order_type: OrderType::StopLossLimit,
            time_in_force,
            new_client_order_id: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .map(|_| ())
    }

    /// Create a stop limit sell order for the given symbol, price and stop price.
    /// Returning a `Transaction` value with the same parameters sent on the order.
    ///
    ///```no_run
    /// use binance::api::Binance;
    /// use binance::account::*;
    ///
    /// fn main() {
    ///     let api_key = Some("api_key".into());
    ///     let secret_key = Some("secret_key".into());
    ///     let account: Account = Binance::new(api_key, secret_key);
    ///     let result = account.stop_limit_sell_order("LTCBTC", 1, 0.1, 0.09, TimeInForce::GTC);
    /// }
    /// ```
    pub fn stop_limit_sell_order<S, F>(
        &self,
        symbol: S,
        qty: F,
        price: f64,
        stop_price: f64,
        time_in_force: TimeInForce,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: Some(stop_price),
            order_side: OrderSide::Sell,
            order_type: OrderType::StopLossLimit,
            time_in_force,
            new_client_order_id: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Spot(Spot::Order), request)
    }

    /// Create a stop limit sell order for the given symbol, price and stop price.
    /// Returning a `Transaction` value with the same parameters sent on the order.
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    ///
    ///```no_run
    /// use binance::api::Binance;
    /// use binance::account::*;
    ///
    /// fn main() {
    ///     let api_key = Some("api_key".into());
    ///     let secret_key = Some("secret_key".into());
    ///     let account: Account = Binance::new(api_key, secret_key);
    ///     let result = account.test_stop_limit_sell_order("LTCBTC", 1, 0.1, 0.09, TimeInForce::GTC);
    /// }
    /// ```
    pub fn test_stop_limit_sell_order<S, F>(
        &self,
        symbol: S,
        qty: F,
        price: f64,
        stop_price: f64,
        time_in_force: TimeInForce,
    ) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: Some(stop_price),
            order_side: OrderSide::Sell,
            order_type: OrderType::StopLossLimit,
            time_in_force,
            new_client_order_id: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .map(|_| ())
    }

    /// Place a custom order
    #[allow(clippy::too_many_arguments)]
    pub fn custom_order<S, F>(
        &self,
        symbol: S,
        qty: F,
        price: f64,
        stop_price: Option<f64>,
        order_side: OrderSide,
        order_type: OrderType,
        time_in_force: TimeInForce,
        new_client_order_id: Option<String>,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price,
            order_side,
            order_type,
            time_in_force,
            new_client_order_id,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Spot(Spot::Order), request)
    }

    /// Place a test custom order
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    #[allow(clippy::too_many_arguments)]
    pub fn test_custom_order<S, F>(
        &self,
        symbol: S,
        qty: F,
        price: f64,
        stop_price: Option<f64>,
        order_side: OrderSide,
        order_type: OrderType,
        time_in_force: TimeInForce,
        new_client_order_id: Option<String>,
    ) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price,
            order_side,
            order_type,
            time_in_force,
            new_client_order_id,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .map(|_| ())
    }

    // Check an order's status
    pub fn cancel_order<S>(
        &self,
        symbol: S,
        order_id: u64,
    ) -> Result<OrderCanceled>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("orderId".into(), order_id.to_string());

        let request =
            build_signed_request(parameters, self.recv_window)?;
        self.client
            .delete_signed(API::Spot(Spot::Order), Some(request))
    }

    pub fn cancel_order_with_client_id<S>(
        &self,
        symbol: S,
        orig_client_order_id: String,
    ) -> Result<OrderCanceled>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters
            .insert("origClientOrderId".into(), orig_client_order_id);

        let request =
            build_signed_request(parameters, self.recv_window)?;
        self.client
            .delete_signed(API::Spot(Spot::Order), Some(request))
    }
    /// Place a test cancel order
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub fn test_cancel_order<S>(
        &self,
        symbol: S,
        order_id: u64,
    ) -> Result<()>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("orderId".into(), order_id.to_string());
        let request =
            build_signed_request(parameters, self.recv_window)?;
        self.client
            .delete_signed::<Empty>(
                API::Spot(Spot::OrderTest),
                Some(request),
            )
            .map(|_| ())
    }

    // Trade history
    pub fn trade_history<S>(
        &self,
        symbol: S,
    ) -> Result<Vec<TradeHistory>>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());

        let request =
            build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::MyTrades), Some(request))
    }

    fn build_order(
        &self,
        order: OrderRequest,
    ) -> BTreeMap<String, String> {
        let mut order_parameters: BTreeMap<String, String> =
            BTreeMap::new();

        order_parameters.insert("symbol".into(), order.symbol);
        order_parameters
            .insert("side".into(), order.order_side.to_string());
        order_parameters
            .insert("type".into(), order.order_type.to_string());
        order_parameters
            .insert("quantity".into(), order.qty.to_string());

        if let Some(stop_price) = order.stop_price {
            order_parameters
                .insert("stopPrice".into(), stop_price.to_string());
        }

        if order.price != 0.0 {
            order_parameters
                .insert("price".into(), order.price.to_string());
            order_parameters.insert(
                "timeInForce".into(),
                order.time_in_force.to_string(),
            );
        }

        if let Some(client_order_id) = order.new_client_order_id {
            order_parameters
                .insert("newClientOrderId".into(), client_order_id);
        }

        order_parameters
    }

    fn build_quote_quantity_order(
        &self,
        order: OrderQuoteQuantityRequest,
    ) -> BTreeMap<String, String> {
        let mut order_parameters: BTreeMap<String, String> =
            BTreeMap::new();

        order_parameters.insert("symbol".into(), order.symbol);
        order_parameters
            .insert("side".into(), order.order_side.to_string());
        order_parameters
            .insert("type".into(), order.order_type.to_string());
        order_parameters.insert(
            "quoteOrderQty".into(),
            order.quote_order_qty.to_string(),
        );

        if order.price != 0.0 {
            order_parameters
                .insert("price".into(), order.price.to_string());
            order_parameters.insert(
                "timeInForce".into(),
                order.time_in_force.to_string(),
            );
        }

        if let Some(client_order_id) = order.new_client_order_id {
            order_parameters
                .insert("newClientOrderId".into(), client_order_id);
        }

        order_parameters
    }

    fn converter_order_to_btree_map<T: Into<f64>>(
        &self,
        order: OrderQuoteRequest<T>,
    ) -> BTreeMap<String, String> {
        let mut order_parameters: BTreeMap<String, String> =
            BTreeMap::new();

        order_parameters
            .insert("fromAsset".into(), order.from_asset.to_string());
        order_parameters
            .insert("toAsset".into(), order.to_asset.to_string());

        match order.from_or_to_amount {
            QtyType::From(v) => {
                let qty: f64 = v.into();
                order_parameters
                    .insert("fromAmount".into(), qty.to_string());
            }
            QtyType::To(v) => {
                let qty: f64 = v.into();
                order_parameters
                    .insert("toAmount".into(), qty.to_string());
            }
        };

        if let Some(wallet_type) = order.wallet_type {
            match wallet_type {
                WalletType::SPOT => {
                    order_parameters.insert(
                        "walletType".into(),
                        "SPOT".to_string(),
                    );
                }
                WalletType::FUNDING => {
                    order_parameters.insert(
                        "walletType".into(),
                        "FUNDING".to_string(),
                    );
                }
            }
        }

        if let Some(time) = order.valid_time {
            match time {
                ValidTime::TenSeconds => {
                    order_parameters
                        .insert("validTime".into(), "10s".to_string());
                }
                ValidTime::ThirtySeconds => {
                    order_parameters
                        .insert("validTime".into(), "30s".to_string());
                }
                ValidTime::OneMinutes => {
                    order_parameters
                        .insert("validTime".into(), "1m".to_string());
                }
                ValidTime::TwoMinutes => {
                    order_parameters
                        .insert("validTime".into(), "2m".to_string());
                }
            }
        }

        order_parameters
    }

    // função que faz o request pra converter
    fn send_quote_request<S, F>(
        &self,
        symbol_from: S,
        symbol_to: S,
        qty: QtyType<F>,
        wallet_type: Option<WalletType>,
        valid_time: Option<ValidTime>,
    ) -> Result<Quote>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let params = OrderQuoteRequest {
            from_asset: symbol_from.into(),
            to_asset: symbol_to.into(),
            from_or_to_amount: qty,
            wallet_type,
            valid_time,
        };

        let order = self.converter_order_to_btree_map(params);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed(API::Convert(Convert::QuoteRequest), request)
    }

    // method que aceita a negociação do convert
    fn accept_quote(
        &self,
        quote: Result<Quote>,
    ) -> Result<QuoteResponse> {
        let quote = quote?;

        //let quote = quote?;
        let mut params: BTreeMap<String, String> = BTreeMap::new();

        if let Some(quote_id) = quote.quote_id {
            params.insert("quoteId".into(), quote_id);
        } else {
            bail!("Not enough funds")
        }

        let request: String =
            build_signed_request(params, self.recv_window)?;
        self.client
            .post_signed(API::Convert(Convert::AcceptQuote), request)
    }

    /// # Example
    /// Convert a currency to another.
    ///
    ///
    /// let account: Account = Binance::new_with_config("API_KEY", "SECRET_KEY");
    ///
    /// // QtyType::From reduces the value of the first symbol in this case "BTC"
    /// // QtyType::To reduces the value of the second symbol in this case "USDT"
    /// let answer = account.convert("BTC", "USDT", QtyType::From(0.0001)).unwrap();
    ///
    pub fn convert<S, F>(
        &self,
        symbol_from: S,
        symbol_to: S,
        qty: QtyType<F>,
    ) -> Result<QuoteResponse>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let quote = self.send_quote_request(
            symbol_from,
            symbol_to,
            qty,
            None,
            Some(ValidTime::TenSeconds),
        );

        self.accept_quote(quote)
    }

    fn daily_account_snapshot_to_btree_map(
        &self,
        params: AccountSnapshotRequest,
    ) -> BTreeMap<String, String> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("type".into(), params.type_);

        if let Some(start_time) = params.start_time {
            parameters
                .insert("startTime".into(), start_time.to_string());
        }

        if let Some(end_time) = params.end_time {
            parameters.insert("endTime".into(), end_time.to_string());
        }

        if let Some(limit) = params.limit {
            parameters.insert("limit".into(), limit.to_string());
        }

        parameters
    }

    /// # Example
    /// Get the daily account snapshot.
    ///
    ///
    /// let account: Account = Binance::new_with_config("API_KEY", "SECRET_KEY");
    /// let answer = account.daily_account_snapshot().unwrap();
    ///
    pub fn daily_account_snapshot(&self) -> Result<AccountSnapshot> {
        let params = AccountSnapshotRequest {
            type_: "SPOT".to_string(),
            start_time: None,
            end_time: None,
            limit: None,
        };
        let btree_params =
            self.daily_account_snapshot_to_btree_map(params);

        // this gets the timestamp and recv_windows to the btreemap
        let request =
            build_signed_request(btree_params, self.recv_window)?;

        eprintln!("{:#?}", request);

        self.client.get_signed(
            API::Savings(Sapi::AccountSnapshot),
            Some(request),
        )
    }
}
