use crate::util::*;
use crate::model::*;
use crate::client::*;
use crate::errors::*;
use crate::api::API;
use crate::api::Spot;
use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

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
}

struct OrderQuoteQuantityRequest {
    pub symbol: String,
    pub quote_order_qty: f64,
    pub price: f64,
    pub order_side: OrderSide,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderType {
    Limit,
    Market,
    StopLossLimit,
}

impl From<OrderType> for String {
    fn from(item: OrderType) -> Self {
        match item {
            OrderType::Limit => String::from("LIMIT"),
            OrderType::Market => String::from("MARKET"),
            OrderType::StopLossLimit => String::from("STOP_LOSS_LIMIT"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

impl From<OrderSide> for String {
    fn from(item: OrderSide) -> Self {
        match item {
            OrderSide::Buy => String::from("BUY"),
            OrderSide::Sell => String::from("SELL"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
}

impl From<TimeInForce> for String {
    fn from(item: TimeInForce) -> Self {
        match item {
            TimeInForce::GTC => String::from("GTC"),
            TimeInForce::IOC => String::from("IOC"),
            TimeInForce::FOK => String::from("FOK"),
        }
    }
}

impl Account {
    // Account Information
    pub fn get_account(&self) -> Result<AccountInformation> {
        let request = build_signed_request(BTreeMap::new(), self.recv_window)?;
        self.client.get_signed(API::Spot(Spot::Account), Some(request))
    }

    // Balance for ONE Asset
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
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.get_signed(API::Spot(Spot::OpenOrders), Some(request))
    }

    // All current open orders
    pub fn get_all_open_orders(&self) -> Result<Vec<Order>> {
        let request = build_signed_request(BTreeMap::new(), self.recv_window)?;
        self.client.get_signed(API::Spot(Spot::OpenOrders), Some(request))
    }

    // Cancel all open orders for ONE symbol
    pub fn cancel_all_open_orders<S>(&self, symbol: S) -> Result<Vec<OrderCanceled>>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.delete_signed(API::Spot(Spot::OpenOrders), Some(request))
    }

    // Check an order's status
    pub fn order_status<S>(&self, symbol: S, order_id: u64) -> Result<Order>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("orderId".into(), order_id.to_string());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.get_signed(API::Spot(Spot::Order), Some(request))
    }

    /// Place a test status order
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub fn test_order_status<S>(&self, symbol: S, order_id: u64) -> Result<()>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("orderId".into(), order_id.to_string());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.get_signed::<()>(API::Spot(Spot::OrderTest), Some(request))
    }

    // Place a LIMIT order - BUY
    pub fn limit_buy<S, F>(&self, symbol: S, qty: F, price: f64) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let request: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: None,
            order_side: OrderSide::Buy,
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::GTC,
        };
        let request = build_signed_request(self.build_order(request), self.recv_window)?;
        self.client.post_signed(API::Spot(Spot::Order), request)
    }

    /// Place a test limit order - BUY
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub fn test_limit_buy<S, F>(&self, symbol: S, qty: F, price: f64) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let request: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: None,
            order_side: OrderSide::Buy,
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::GTC,
        };
        let request = build_signed_request(self.build_order(request), self.recv_window)?;
        self.client.post_signed::<()>(API::Spot(Spot::OrderTest), request)
    }

    // Place a LIMIT order - SELL
    pub fn limit_sell<S, F>(&self, symbol: S, qty: F, price: f64) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let request: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: None,
            order_side: OrderSide::Sell,
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::GTC,
        };
        let request = build_signed_request(self.build_order(request), self.recv_window)?;
        self.client.post_signed(API::Spot(Spot::Order), request)
    }

    /// Place a test LIMIT order - SELL
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub fn test_limit_sell<S, F>(&self, symbol: S, qty: F, price: f64) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let request: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: None,
            order_side: OrderSide::Sell,
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::GTC,
        };
        let request = build_signed_request(self.build_order(request), self.recv_window)?;
        self.client.post_signed::<()>(API::Spot(Spot::OrderTest), request)
    }

    // Place a MARKET order - BUY
    pub fn market_buy<S, F>(&self, symbol: S, qty: F) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let request: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price: 0.0,
            stop_price: None,
            order_side: OrderSide::Buy,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
        };
        let request = build_signed_request(self.build_order(request), self.recv_window)?;
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
        let request: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price: 0.0,
            stop_price: None,
            order_side: OrderSide::Buy,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
        };
        let request = build_signed_request(self.build_order(request), self.recv_window)?;
        self.client.post_signed::<()>(API::Spot(Spot::OrderTest), request)
    }

    // Place a MARKET order with quote quantity - BUY
    pub fn market_buy_using_quote_quantity<S, F>(
        &self, symbol: S, quote_order_qty: F,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let request: OrderQuoteQuantityRequest = OrderQuoteQuantityRequest {
            symbol: symbol.into(),
            quote_order_qty: quote_order_qty.into(),
            price: 0.0,
            order_side: OrderSide::Buy,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
        };
        let order = self.build_quote_quantity_order(request);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Spot(Spot::Order), request)
    }

    /// Place a test MARKET order with quote quantity - BUY
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub fn test_market_buy_using_quote_quantity<S, F>(
        &self, symbol: S, quote_order_qty: F,
    ) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let request: OrderQuoteQuantityRequest = OrderQuoteQuantityRequest {
            symbol: symbol.into(),
            quote_order_qty: quote_order_qty.into(),
            price: 0.0,
            order_side: OrderSide::Buy,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
        };
        let order = self.build_quote_quantity_order(request);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed::<()>(API::Spot(Spot::OrderTest), request)
    }

    // Place a MARKET order - SELL
    pub fn market_sell<S, F>(&self, symbol: S, qty: F) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let request: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price: 0.0,
            stop_price: None,
            order_side: OrderSide::Sell,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
        };
        let request = build_signed_request(self.build_order(request), self.recv_window)?;
        self.client.post_signed(API::Spot(Spot::Order), request)
    }

    /// Place a test MARKET order - SELL
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub fn test_market_sell<S, F>(&self, symbol: S, qty: F) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let request: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price: 0.0,
            stop_price: None,
            order_side: OrderSide::Sell,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
        };
        let request = build_signed_request(self.build_order(request), self.recv_window)?;
        self.client.post_signed::<()>(API::Spot(Spot::OrderTest), request)
    }

    // Place a MARKET order with quote quantity - SELL
    pub fn market_sell_using_quote_quantity<S, F>(
        &self, symbol: S, quote_order_qty: F,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let request: OrderQuoteQuantityRequest = OrderQuoteQuantityRequest {
            symbol: symbol.into(),
            quote_order_qty: quote_order_qty.into(),
            price: 0.0,
            order_side: OrderSide::Sell,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
        };
        let order = self.build_quote_quantity_order(request);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Spot(Spot::Order), request)
    }

    /// Place a test MARKET order with quote quantity - SELL
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub fn test_market_sell_using_quote_quantity<S, F>(
        &self, symbol: S, quote_order_qty: F,
    ) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let request: OrderQuoteQuantityRequest = OrderQuoteQuantityRequest {
            symbol: symbol.into(),
            quote_order_qty: quote_order_qty.into(),
            price: 0.0,
            order_side: OrderSide::Sell,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
        };
        let order = self.build_quote_quantity_order(request);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed::<()>(API::Spot(Spot::OrderTest), request)
    }

    /// Place a stop limit sell order
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
        let request: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: Some(stop_price),
            order_side: OrderSide::Sell,
            order_type: OrderType::StopLossLimit,
            time_in_force,
        };
        let request = build_signed_request(self.build_order(request), self.recv_window)?;
        self.client.post_signed(API::Spot(Spot::Order), request)
    }

    /// Place a test stop limit sell order
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
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
        let request: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: Some(stop_price),
            order_side: OrderSide::Sell,
            order_type: OrderType::StopLossLimit,
            time_in_force,
        };
        let request = build_signed_request(self.build_order(request), self.recv_window)?;
        self.client.post_signed::<()>(API::Spot(Spot::OrderTest), request)
    }

    /// Place a stop limit buy order
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
        let request: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: Some(stop_price),
            order_side: OrderSide::Buy,
            order_type: OrderType::StopLossLimit,
            time_in_force,
        };
        let request = build_signed_request(self.build_order(request), self.recv_window)?;
        self.client.post_signed(API::Spot(Spot::Order), request)
    }

    /// Place a test stop limit buy order
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
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
        let request: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: Some(stop_price),
            order_side: OrderSide::Buy,
            order_type: OrderType::StopLossLimit,
            time_in_force,
        };
        let request = build_signed_request(self.build_order(request), self.recv_window)?;
        self.client.post_signed::<()>(API::Spot(Spot::OrderTest), request)
    }

    /// Place a stop limit sell order
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
        let sell: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: Some(stop_price),
            order_side: OrderSide::Sell,
            order_type: OrderType::StopLossLimit,
            time_in_force: time_in_force,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        let data = self.client.post_signed(API_V3_ORDER, &request)?;
        let transaction: Transaction = from_str(data.as_str())?;

        Ok(transaction)
    }

    /// Place a test stop limit sell order
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
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
        let sell: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: Some(stop_price),
            order_side: OrderSide::Sell,
            order_type: OrderType::StopLossLimit,
            time_in_force: time_in_force,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        let data = self.client.post_signed(API_V3_ORDER_TEST, &request)?;
        let _: TestResponse = from_str(data.as_str())?;

        Ok(())
    }

    /// Place a stop limit buy order
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
        let sell: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: Some(stop_price),
            order_side: OrderSide::Buy,
            order_type: OrderType::StopLossLimit,
            time_in_force: time_in_force,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        let data = self.client.post_signed(API_V3_ORDER, &request)?;
        let transaction: Transaction = from_str(data.as_str())?;

        Ok(transaction)
    }

    /// Place a test stop limit buy order
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
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
        let sell: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: Some(stop_price),
            order_side: OrderSide::Buy,
            order_type: OrderType::StopLossLimit,
            time_in_force: time_in_force,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        let data = self.client.post_signed(API_V3_ORDER_TEST, &request)?;
        let _: TestResponse = from_str(data.as_str())?;

        Ok(())
    }

    /// Place a custom order
    pub fn custom_order<S, F>(
        &self,
        symbol: S,
        qty: F,
        price: f64,
        stop_price: f64,
        order_side: OrderSide,
        order_type: OrderType,
        time_in_force: TimeInForce,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let request: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: Some(stop_price),
            order_side: order_side,
            order_type: order_type,
            time_in_force: time_in_force,
        };
        let request = build_signed_request(self.build_order(request), self.recv_window)?;
        self.client.post_signed(API::Spot(Spot::Order), request)
    }

    /// Place a test custom order
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub fn test_custom_order<S, F>(
        &self,
        symbol: S,
        qty: F,
        price: f64,
        order_side: OrderSide,
        order_type: OrderType,
        time_in_force: TimeInForce,
    ) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let request: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: None,
            order_side: order_side,
            order_type: order_type,
            time_in_force: time_in_force,
        };
        let request = build_signed_request(self.build_order(request), self.recv_window)?;
        self.client.post_signed::<()>(API::Spot(Spot::OrderTest), request)
    }

    // Check an order's status
    pub fn cancel_order<S>(&self, symbol: S, order_id: u64) -> Result<OrderCanceled>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("orderId".into(), order_id.to_string());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.delete_signed(API::Spot(Spot::Order), Some(request))
    }

    /// Place a test cancel order
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub fn test_cancel_order<S>(&self, symbol: S, order_id: u64) -> Result<()>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("orderId".into(), order_id.to_string());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.delete_signed::<()>(API::Spot(Spot::OrderTest), Some(request))
    }

    // Trade history
    pub fn trade_history<S>(&self, symbol: S) -> Result<Vec<TradeHistory>>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.get_signed(API::Spot(Spot::MyTrades), Some(request))
    }

    fn build_order(&self, order: OrderRequest) -> BTreeMap<String, String> {
        let mut order_parameters: BTreeMap<String, String> = BTreeMap::new();

        order_parameters.insert("symbol".into(), order.symbol);
        order_parameters.insert("side".into(), order.order_side.into());
        order_parameters.insert("type".into(), order.order_type.into());
        order_parameters.insert("quantity".into(), order.qty.to_string());

        if let Some(stop_price) = order.stop_price {
            order_parameters.insert("stopPrice".into(), stop_price.to_string());
        }

        if order.price != 0.0 {
            order_parameters.insert("price".into(), order.price.to_string());
            order_parameters.insert("timeInForce".into(), order.time_in_force.into());
        }

        order_parameters
    }

    fn build_quote_quantity_order(
        &self, order: OrderQuoteQuantityRequest,
    ) -> BTreeMap<String, String> {
        let mut order_parameters: BTreeMap<String, String> = BTreeMap::new();

        order_parameters.insert("symbol".into(), order.symbol);
        order_parameters.insert("side".into(), order.order_side.into());
        order_parameters.insert("type".into(), order.order_type.into());
        order_parameters.insert("quoteOrderQty".into(), order.quote_order_qty.to_string());

        if order.price != 0.0 {
            order_parameters.insert("price".into(), order.price.to_string());
            order_parameters.insert("timeInForce".into(), order.time_in_force.into());
        }

        order_parameters
    }
}
