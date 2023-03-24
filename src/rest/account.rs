use super::Request;
use crate::model::{AccountInformation, OrderInfo, OrderSide, OrderType, TimeInForce};
use reqwest::Method;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Default, Serialize)]
pub struct GetAccountRequest {}

impl Request for GetAccountRequest {
    const ENDPOINT: &'static str = "/api/v3/account";
    const METHOD: Method = Method::GET;
    const SIGNED: bool = true;
    type Response = AccountInformation;
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct OrderRequest {
    pub symbol: String,
    pub qty: f64,
    pub price: f64,
    pub stop_price: Option<f64>,
    pub order_side: OrderSide,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    pub new_client_order_id: Option<String>,
}

impl Request for OrderRequest {
    const ENDPOINT: &'static str = "/api/v3/order";
    const METHOD: Method = Method::POST;
    const SIGNED: bool = true;
    type Response = OrderInfo;
}
