use util::*;
use model::*;
use client::*;
use errors::*;
use std::collections::BTreeMap;
use serde_json::from_str;

static ORDER_TYPE_LIMIT: &'static str = "LIMIT";
static ORDER_TYPE_MARKET: &'static str = "MARKET";
static ORDER_SIDE_BUY: &'static str = "BUY";
static ORDER_SIDE_SELL: &'static str = "SELL";
static TIME_IN_FORCE_GTC: &'static str = "GTC";

static API_V3_ORDER: &'static str = "/api/v3/order";

#[derive(Clone)]
pub struct Account {
    pub client: Client,
    pub recv_window: u64,
}

impl Account {
    // Account Information
    pub fn get_account(&self) -> Result<(AccountInformation)> {
        let parameters: BTreeMap<String, String> = BTreeMap::new();

        let request = build_signed_request(parameters, self.recv_window);
        let data = self.client.get_signed("/api/v3/account", &request)?;
        let account_info: AccountInformation = from_str(data.as_str()).unwrap();

        Ok(account_info)
    }

    // Balance for ONE Asset
    pub fn get_balance(&self, asset: &str) -> Result<(Balance)> {
        match self.get_account() {
            Ok(account) => {
                for balance in account.balances {
                    if balance.asset == asset {
                        return Ok(balance);
                    }
                }
                bail!("Asset not found");
            }
            Err(e) => Err(e),
        }
    }

    // Current open orders for ONE symbol
    pub fn get_open_orders(&self, symbol: String) -> Result<(Vec<Order>)> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol);

        let request = build_signed_request(parameters, self.recv_window);
        let data = self.client.get_signed("/api/v3/openOrders", &request)?;
        let order: Vec<Order> = from_str(data.as_str()).unwrap();

        Ok(order)
    }

    // Check an order's status
    pub fn order_status(&self, symbol: String, order_id: u32) -> Result<(Order)> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol);
        parameters.insert("orderId".into(), order_id.to_string());

        let request = build_signed_request(parameters, self.recv_window);
        let data = self.client.get_signed(API_V3_ORDER, &request)?;
        let order: Order = from_str(data.as_str()).unwrap();

        Ok(order)
    }

    // Place a LIMIT order - BUY
    pub fn limit_buy(&self, symbol: String, qty: u32, price: f64) -> Result<(Transaction)> {
        let order = self.build_order(
            symbol,
            qty,
            price,
            ORDER_SIDE_BUY,
            ORDER_TYPE_LIMIT,
            TIME_IN_FORCE_GTC,
        );
        let request = build_signed_request(order, self.recv_window);
        let data = self.client.post_signed(API_V3_ORDER, &request)?;
        let transaction: Transaction = from_str(data.as_str()).unwrap();

        Ok(transaction)
    }

    // Place a LIMIT order - SELL
    pub fn limit_sell(&self, symbol: String, qty: u32, price: f64) -> Result<(Transaction)> {
        let order = self.build_order(
            symbol,
            qty,
            price,
            ORDER_SIDE_SELL,
            ORDER_TYPE_LIMIT,
            TIME_IN_FORCE_GTC,
        );
        let request = build_signed_request(order, self.recv_window);
        let data = self.client.post_signed(API_V3_ORDER, &request)?;
        let transaction: Transaction = from_str(data.as_str()).unwrap();

        Ok(transaction)
    }

    // Place a MARKET order - BUY
    pub fn market_buy(&self, symbol: String, qty: u32) -> Result<(Transaction)> {
        let order = self.build_order(
            symbol,
            qty,
            0.0,
            ORDER_SIDE_BUY,
            ORDER_TYPE_MARKET,
            TIME_IN_FORCE_GTC,
        );
        let request = build_signed_request(order, self.recv_window);
        let data = self.client.post_signed(API_V3_ORDER, &request)?;
        let transaction: Transaction = from_str(data.as_str()).unwrap();

        Ok(transaction)
    }

    // Place a MARKET order - SELL
    pub fn market_sell(&self, symbol: String, qty: u32) -> Result<(Transaction)> {
        let order = self.build_order(
            symbol,
            qty,
            0.0,
            ORDER_SIDE_SELL,
            ORDER_TYPE_MARKET,
            TIME_IN_FORCE_GTC,
        );
        let request = build_signed_request(order, self.recv_window);
        let data = self.client.post_signed(API_V3_ORDER, &request)?;
        let transaction: Transaction = from_str(data.as_str()).unwrap();

        Ok(transaction)
    }

    // Check an order's status
    pub fn cancel_order(&self, symbol: String, order_id: u32) -> Result<(OrderCanceled)> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol);
        parameters.insert("orderId".into(), order_id.to_string());

        let request = build_signed_request(parameters, self.recv_window);
        let data = self.client.delete_signed(API_V3_ORDER, &request)?;
        let order_canceled: OrderCanceled = from_str(data.as_str()).unwrap();

        Ok(order_canceled)
    }

    // Trade history
    pub fn trade_history(&self, symbol: String) -> Result<(Vec<TradeHistory>)> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol);

        let request = build_signed_request(parameters, self.recv_window);
        let data = self.client.get_signed("/api/v3/myTrades", &request)?;
        let trade_history: Vec<TradeHistory> = from_str(data.as_str()).unwrap();

        Ok(trade_history)
    }

    fn build_order(
        &self,
        symbol: String,
        qty: u32,
        price: f64,
        order_side: &str,
        order_type: &str,
        time_in_force: &str,
    ) -> BTreeMap<String, String> {
        let mut order: BTreeMap<String, String> = BTreeMap::new();

        order.insert("symbol".into(), symbol);
        order.insert("side".into(), order_side.to_string());
        order.insert("type".into(), order_type.to_string());
        order.insert("quantity".into(), qty.to_string());

        if price != 0.0 {
            order.insert("price".into(), price.to_string());
            order.insert("timeInForce".into(), time_in_force.to_string());
        }

        order
    }
}
