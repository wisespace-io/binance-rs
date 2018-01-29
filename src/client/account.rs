use failure::{Error, Fallible};
use futures::Future;
use serde_json::json;
use std::collections::HashMap;
use sugar::{convert_args, hashmap};

use client::Binance;
use error::BinanceError;
use model::{AccountInformation, Balance, Order, OrderCanceled, TradeHistory, Transaction};

static ORDER_TYPE_LIMIT: &'static str = "LIMIT";
static ORDER_TYPE_MARKET: &'static str = "MARKET";
static ORDER_SIDE_BUY: &'static str = "BUY";
static ORDER_SIDE_SELL: &'static str = "SELL";
static TIME_IN_FORCE_GTC: &'static str = "GTC";

static API_V3_ORDER: &'static str = "/api/v3/order";

struct OrderRequest {
    pub symbol: String,
    pub qty: f64,
    pub price: f64,
    pub order_side: String,
    pub order_type: String,
    pub time_in_force: String,
}

impl Binance {
    // Account Information
    pub fn get_account(&self) -> Fallible<impl Future<Item = AccountInformation, Error = Error>> {
        let account_info = self
            .transport
            .signed_get::<_, ()>("/api/v3/account", None)?;
        Ok(account_info)
    }

    // Balance for ONE Asset
    pub fn get_balance(&self, asset: &str) -> Fallible<impl Future<Item = Balance, Error = Error>> {
        let asset = asset.to_string();
        let search = move |account: AccountInformation| -> Fallible<Balance> {
            let balance = account
                .balances
                .into_iter()
                .find(|balance| balance.asset == asset);
            Ok(balance.ok_or(BinanceError::AssetsNotFound)?)
        };

        let balance = self.get_account()?.and_then(search);
        Ok(balance)
    }

    // Current open orders for ONE symbol
    pub fn get_open_orders(
        &self,
        symbol: &str,
    ) -> Fallible<impl Future<Item = Vec<Order>, Error = Error>> {
        let params = json! {{"symbol": symbol}};
        let orders = self
            .transport
            .signed_get("/api/v3/openOrders", Some(params))?;
        Ok(orders)
    }

    // All current open orders
    pub fn get_all_open_orders(&self) -> Fallible<impl Future<Item = Vec<Order>, Error = Error>> {
        let orders = self
            .transport
            .signed_get::<_, ()>("/api/v3/openOrders", None)?;
        Ok(orders)
    }

    // Check an order's status
    pub fn order_status(
        &self,
        symbol: &str,
        order_id: u64,
    ) -> Fallible<impl Future<Item = Order, Error = Error>> {
        let params = json! {{"symbol": symbol, "orderId": order_id}};

        let order = self.transport.signed_get(API_V3_ORDER, Some(params))?;
        Ok(order)
    }

    // Place a LIMIT order - BUY
    pub fn limit_buy(
        &self,
        symbol: &str,
        qty: f64,
        price: f64,
    ) -> Fallible<impl Future<Item = Transaction, Error = Error>> {
        let buy: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price: price,
            order_side: ORDER_SIDE_BUY.to_string(),
            order_type: ORDER_TYPE_LIMIT.to_string(),
            time_in_force: TIME_IN_FORCE_GTC.to_string(),
        };
        let params = self.build_order(buy);

        let transaction = self.transport.signed_post(API_V3_ORDER, Some(params))?;

        Ok(transaction)
    }

    // Place a LIMIT order - SELL
    pub fn limit_sell(
        &self,
        symbol: &str,
        qty: f64,
        price: f64,
    ) -> Fallible<impl Future<Item = Transaction, Error = Error>> {
        let sell: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price: price,
            order_side: ORDER_SIDE_SELL.to_string(),
            order_type: ORDER_TYPE_LIMIT.to_string(),
            time_in_force: TIME_IN_FORCE_GTC.to_string(),
        };
        let params = self.build_order(sell);
        let transaction = self.transport.signed_post(API_V3_ORDER, Some(params))?;

        Ok(transaction)
    }

    // Place a MARKET order - BUY
    pub fn market_buy(
        &self,
        symbol: &str,
        qty: f64,
    ) -> Fallible<impl Future<Item = Transaction, Error = Error>> {
        let buy: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price: 0.0,
            order_side: ORDER_SIDE_BUY.to_string(),
            order_type: ORDER_TYPE_MARKET.to_string(),
            time_in_force: TIME_IN_FORCE_GTC.to_string(),
        };
        let params = self.build_order(buy);
        let transaction = self.transport.signed_post(API_V3_ORDER, Some(params))?;

        Ok(transaction)
    }

    // Place a MARKET order - SELL
    pub fn market_sell(
        &self,
        symbol: &str,
        qty: f64,
    ) -> Fallible<impl Future<Item = Transaction, Error = Error>> {
        let sell: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price: 0.0,
            order_side: ORDER_SIDE_SELL.to_string(),
            order_type: ORDER_TYPE_MARKET.to_string(),
            time_in_force: TIME_IN_FORCE_GTC.to_string(),
        };
        let params = self.build_order(sell);
        let transaction = self.transport.signed_post(API_V3_ORDER, Some(params))?;
        Ok(transaction)
    }

    // Check an order's status
    pub fn cancel_order(
        &self,
        symbol: &str,
        order_id: u64,
    ) -> Fallible<impl Future<Item = OrderCanceled, Error = Error>> {
        let params = json! {{"symbol":symbol, "orderId":order_id}};
        let order_canceled = self.transport.signed_delete(API_V3_ORDER, Some(params))?;
        Ok(order_canceled)
    }

    // Trade history
    pub fn trade_history(
        &self,
        symbol: &str,
    ) -> Fallible<impl Future<Item = Vec<TradeHistory>, Error = Error>> {
        let params = json! {{"symbol":symbol}};
        let trade_history = self
            .transport
            .signed_get("/api/v3/myTrades", Some(params))?;

        Ok(trade_history)
    }

    fn build_order(&self, order: OrderRequest) -> HashMap<&'static str, String> {
        let mut params: HashMap<&str, String> = convert_args!(hashmap!(
            "symbol" => order.symbol,
            "side" => order.order_side,
            "type" => order.order_type,
            "quantity" => order.qty.to_string(),
        ));

        if order.price != 0.0 {
            params.insert("price", order.price.to_string());
            params.insert("timeInForce", order.time_in_force.to_string());
        }

        params
    }
}
