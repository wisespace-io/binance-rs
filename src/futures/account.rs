use std::collections::BTreeMap;

use crate::util::*;
use crate::errors::*;
use crate::client::Client;
use crate::api::{API, Futures};
use crate::model::Empty;
use crate::account::{OrderSide, TimeInForce};
use super::model::{ChangeLeverageResponse, Transaction, CanceledOrder, Position, AccountBalance};


#[derive(Clone)]
pub struct FuturesAccount {
    pub client: Client,
    pub recv_window: u64,
}

pub enum ContractType {
    Perpetual,
    CurrentMonth,
    NextMonth,
    CurrentQuarter,
    NextQuarter,
}

impl From<ContractType> for String {
    fn from(item: ContractType) -> Self {
        match item {
            ContractType::Perpetual => String::from("PERPETUAL"),
            ContractType::CurrentMonth => String::from("CURRENT_MONTH"),
            ContractType::NextMonth => String::from("NEXT_MONTH"),
            ContractType::CurrentQuarter => String::from("CURRENT_QUARTER"),
            ContractType::NextQuarter => String::from("NEXT_QUARTER"),
        }
    }
}

pub enum PositionSide {
    Both,
    Long,
    Short,
}

impl From<PositionSide> for String {
    fn from(item: PositionSide) -> Self {
        match item {
            PositionSide::Both => String::from("BOTH"),
            PositionSide::Long => String::from("LONG"),
            PositionSide::Short => String::from("SHORT"),
        }
    }
}

pub enum OrderType {
    Limit,
    Market,
    Stop,
    StopMarket,
    TakeProfit,
    TakeProfitMarket,
    TrailingStopMarket,

}

impl From<OrderType> for String {
    fn from(item: OrderType) -> Self {
        match item {
            OrderType::Limit => String::from("LIMIT"),
            OrderType::Market => String::from("MARKET"),
            OrderType::Stop => String::from("STOP"),
            OrderType::StopMarket => String::from("STOP_MARKET"),
            OrderType::TakeProfit => String::from("TAKE_PROFIT"),
            OrderType::TakeProfitMarket => String::from("TAKE_PROFIT_MARKET"),
            OrderType::TrailingStopMarket => String::from("TRAILING_STOP_MARKET"),
        }
    }
}

pub enum WorkingType {
    MarkPrice,
    ContractPrice,
}

impl From<WorkingType> for String {
    fn from(item: WorkingType) -> Self {
        match item {
            WorkingType::MarkPrice => String::from("MARK_PRICE"),
            WorkingType::ContractPrice => String::from("CONTRACT_PRICE"),
        }
    }
}

struct OrderRequest {
    pub symbol: String,
    pub side: OrderSide,
    pub position_side: Option<PositionSide>,
    pub order_type: OrderType,
    pub time_in_force: Option<TimeInForce>,
    pub qty: Option<f64>,
    pub reduce_only: Option<bool>,
    pub price: Option<f64>,
    pub stop_price: Option<f64>,
    pub close_position: Option<bool>,
    pub activation_price: Option<f64>,
    pub callback_rate: Option<f64>,
    pub working_type: Option<WorkingType>,
    pub price_protect: Option<f64>,
}


impl FuturesAccount {
    pub fn limit_buy(&self,
                     symbol: impl Into<String>,
                     qty: impl Into<f64>,
                     price: f64,
                     time_in_force: TimeInForce,
    ) -> Result<Transaction> {
        let buy = OrderRequest {
            symbol: symbol.into(),
            side: OrderSide::Buy,
            position_side: None,
            order_type: OrderType::Limit,
            time_in_force: Some(time_in_force),
            qty: Some(qty.into()),
            reduce_only: None,
            price: Some(price),
            stop_price: None,
            close_position: None,
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
        };
        let order = self.build_order(buy);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Futures(Futures::Order), request)
    }

    pub fn limit_sell(&self,
                      symbol: impl Into<String>,
                      qty: impl Into<f64>,
                      price: f64,
                      time_in_force: TimeInForce,
    ) -> Result<Transaction> {
        let sell = OrderRequest {
            symbol: symbol.into(),
            side: OrderSide::Sell,
            position_side: None,
            order_type: OrderType::Limit,
            time_in_force: Some(time_in_force),
            qty: Some(qty.into()),
            reduce_only: None,
            price: Some(price),
            stop_price: None,
            close_position: None,
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Futures(Futures::Order), request)
    }

    // Place a MARKET order - BUY
    pub fn market_buy<S, F>(&self, symbol: S, qty: F) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy = OrderRequest {
            symbol: symbol.into(),
            side: OrderSide::Buy,
            position_side: None,
            order_type: OrderType::Market,
            time_in_force: None,
            qty: Some(qty.into()),
            reduce_only: None,
            price: None,
            stop_price: None,
            close_position: None,
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
        };
        let order = self.build_order(buy);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Futures(Futures::Order), request)
    }

    // Place a MARKET order - SELL
    pub fn market_sell<S, F>(&self, symbol: S, qty: F) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            side: OrderSide::Sell,
            position_side: None,
            order_type: OrderType::Market,
            time_in_force: None,
            qty: Some(qty.into()),
            reduce_only: None,
            price: None,
            stop_price: None,
            close_position: None,
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Futures(Futures::Order), request)
    }

    pub fn cancel_order<S>(&self, symbol: S, order_id: u64) -> Result<CanceledOrder> 
    where 
        S: Into<String>
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("orderId".into(), order_id.to_string());
        
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.delete_signed(API::Futures(Futures::Order), Some(request))
    }

    // Place a STOP_MARKET close - BUY
    pub fn stop_market_close_buy<S, F>(&self, symbol: S, stop_price: F) -> Result<Transaction>
        where
            S: Into<String>,
            F: Into<f64>,
    {
        let sell: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            side: OrderSide::Buy,
            position_side: None,
            order_type: OrderType::StopMarket,
            time_in_force: None,
            qty: None,
            reduce_only: None,
            price: None,
            stop_price: Some(stop_price.into()),
            close_position: Some(true),
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Futures(Futures::Order), request)
    }

    // Place a STOP_MARKET close - SELL
    pub fn stop_market_close_sell<S, F>(&self, symbol: S, stop_price: F) -> Result<Transaction>
        where
            S: Into<String>,
            F: Into<f64>,
    {
        let sell: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            side: OrderSide::Sell,
            position_side: None,
            order_type: OrderType::StopMarket,
            time_in_force: None,
            qty: None,
            reduce_only: None,
            price: None,
            stop_price: Some(stop_price.into()),
            close_position: Some(true),
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client.post_signed(API::Futures(Futures::Order), request)
    }

    fn build_order(&self, order: OrderRequest) -> BTreeMap<String, String> {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), order.symbol);
        parameters.insert("side".into(), order.side.into());
        parameters.insert("type".into(), order.order_type.into());

        if let Some(position_side) = order.position_side {
            parameters.insert("positionSide".into(), position_side.into());
        }
        if let Some(time_in_force) = order.time_in_force {
            parameters.insert("timeInForce".into(), time_in_force.into());
        }
        if let Some(qty) = order.qty {
            parameters.insert("quantity".into(), qty.to_string());
        }
        if let Some(reduce_only) = order.reduce_only {
            parameters.insert("reduceOnly".into(), reduce_only.to_string().to_uppercase());
        }
        if let Some(price) = order.price {
            parameters.insert("price".into(), price.to_string());
        }
        if let Some(stop_price) = order.stop_price {
            parameters.insert("stopPrice".into(), stop_price.to_string());
        }
        if let Some(close_position) = order.close_position {
            parameters.insert("closePosition".into(), close_position.to_string().to_uppercase());
        }
        if let Some(activation_price) = order.activation_price {
            parameters.insert("activationPrice".into(), activation_price.to_string());
        }
        if let Some(callback_rate) = order.callback_rate {
            parameters.insert("callbackRate".into(), callback_rate.to_string());
        }
        if let Some(working_type) = order.working_type {
            parameters.insert("workingType".into(), working_type.into());
        }
        if let Some(price_protect) = order.price_protect {
            parameters.insert("priceProtect".into(), price_protect.to_string().to_uppercase());
        }

        parameters
    }

    pub fn position_information<S>(&self, symbol: S) -> Result<Vec<Position>>
        where S: Into<String>
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.get_signed(API::Futures(Futures::PositionRisk), Some(request))
    }

    pub fn account_balance(&self) -> Result<Vec<AccountBalance>> {
        let parameters = BTreeMap::new();

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.get_signed(API::Futures(Futures::Balance), Some(request))
    }

    pub fn change_initial_leverage<S>(&self, symbol: S, leverage: u8) -> Result<ChangeLeverageResponse>
    where
        S: Into<String>
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("leverage".into(), leverage.to_string());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.post_signed(API::Futures(Futures::ChangeInitialLeverage), request)
    }

    pub fn change_position_mode(&self, dual_side_position: bool) -> Result<()> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        let dual_side = if dual_side_position { "true" } else { "false" };
        parameters.insert("dualSidePosition".into(), dual_side.into());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Futures(Futures::PositionSide), request)
            .map(|_| ())
    }

    pub fn cancel_all_open_orders<S>(&self, symbol: S) -> Result<()> where S: Into<String> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .delete_signed::<Empty>(API::Futures(Futures::AllOpenOrders), Some(request))
            .map(|_| ())
    }

    pub fn get_all_open_orders<S>(&self, symbol: S) -> Result<Vec<crate::futures::model::Order>> where S: Into<String> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.get_signed(API::Futures(Futures::OpenOrders), Some(request))
    }
}