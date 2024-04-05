use std::collections::BTreeMap;
use std::fmt::Display;
use crate::util::{build_list_of_json, build_signed_request};
use crate::errors::Result;
use crate::client::Client;
use crate::api::{API, Futures};
use crate::model::Empty;
use crate::account::OrderSide;
use crate::futures::model::{Order, TradeHistory};

use super::model::{
    AccountBalance, AccountInformation, CanceledOrder, ChangeLeverageResponse, PositionRisk,
    Transaction, TransactionOrError,
};

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

impl Display for PositionSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Both => write!(f, "BOTH"),
            Self::Long => write!(f, "LONG"),
            Self::Short => write!(f, "SHORT"),
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

impl Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Limit => write!(f, "LIMIT"),
            Self::Market => write!(f, "MARKET"),
            Self::Stop => write!(f, "STOP"),
            Self::StopMarket => write!(f, "STOP_MARKET"),
            Self::TakeProfit => write!(f, "TAKE_PROFIT"),
            Self::TakeProfitMarket => write!(f, "TAKE_PROFIT_MARKET"),
            Self::TrailingStopMarket => write!(f, "TRAILING_STOP_MARKET"),
        }
    }
}

pub enum WorkingType {
    MarkPrice,
    ContractPrice,
}

impl Display for WorkingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MarkPrice => write!(f, "MARK_PRICE"),
            Self::ContractPrice => write!(f, "CONTRACT_PRICE"),
        }
    }
}

#[allow(clippy::all)]
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
    GTX,
}

impl Display for TimeInForce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GTC => write!(f, "GTC"),
            Self::IOC => write!(f, "IOC"),
            Self::FOK => write!(f, "FOK"),
            Self::GTX => write!(f, "GTX"),
        }
    }
}

pub struct OrderRequest {
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

impl OrderRequest {
    pub fn limit_buy(
        symbol: impl Into<String>, qty: impl Into<f64>, price: f64, time_in_force: TimeInForce,
    ) -> Self {
        Self {
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
        }
    }
    pub fn limit_sell(
        symbol: impl Into<String>, qty: impl Into<f64>, price: f64, time_in_force: TimeInForce,
    ) -> Self {
        Self {
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
        }
    }
    pub fn market_buy(symbol: impl Into<String>, qty: impl Into<f64>) -> Self {
        Self {
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
        }
    }
    pub fn market_sell(symbol: impl Into<String>, qty: impl Into<f64>) -> Self {
        Self {
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
        }
    }
    pub fn stop_market_close_buy(symbol: impl Into<String>, stop_price: impl Into<f64>) -> Self {
        Self {
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
        }
    }
    pub fn stop_market_close_sell(symbol: impl Into<String>, stop_price: impl Into<f64>) -> Self {
        Self {
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
        }
    }
}

pub struct IncomeRequest {
    pub symbol: Option<String>,
    pub income_type: Option<IncomeType>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: Option<u32>,
}

#[allow(non_camel_case_types)]
pub enum IncomeType {
    TRANSFER,
    WELCOME_BONUS,
    REALIZED_PNL,
    FUNDING_FEE,
    COMMISSION,
    INSURANCE_CLEAR,
    REFERRAL_KICKBACK,
    COMMISSION_REBATE,
    API_REBATE,
    CONTEST_REWARD,
    CROSS_COLLATERAL_TRANSFER,
    OPTIONS_PREMIUM_FEE,
    OPTIONS_SETTLE_PROFIT,
    INTERNAL_TRANSFER,
    AUTO_EXCHANGE,
    DELIVERED_SETTELMENT,
    COIN_SWAP_DEPOSIT,
    COIN_SWAP_WITHDRAW,
    POSITION_LIMIT_INCREASE_FEE,
}

impl Display for IncomeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TRANSFER => write!(f, "TRANSFER"),
            Self::WELCOME_BONUS => write!(f, "WELCOME_BONUS"),
            Self::REALIZED_PNL => write!(f, "REALIZED_PNL"),
            Self::FUNDING_FEE => write!(f, "FUNDING_FEE"),
            Self::COMMISSION => write!(f, "COMMISSION"),
            Self::INSURANCE_CLEAR => write!(f, "INSURANCE_CLEAR"),
            Self::REFERRAL_KICKBACK => write!(f, "REFERRAL_KICKBACK"),
            Self::COMMISSION_REBATE => write!(f, "COMMISSION_REBATE"),
            Self::API_REBATE => write!(f, "API_REBATE"),
            Self::CONTEST_REWARD => write!(f, "CONTEST_REWARD"),
            Self::CROSS_COLLATERAL_TRANSFER => write!(f, "CROSS_COLLATERAL_TRANSFER"),
            Self::OPTIONS_PREMIUM_FEE => write!(f, "OPTIONS_PREMIUM_FEE"),
            Self::OPTIONS_SETTLE_PROFIT => write!(f, "OPTIONS_SETTLE_PROFIT"),
            Self::INTERNAL_TRANSFER => write!(f, "INTERNAL_TRANSFER"),
            Self::AUTO_EXCHANGE => write!(f, "AUTO_EXCHANGE"),
            Self::DELIVERED_SETTELMENT => write!(f, "DELIVERED_SETTELMENT"),
            Self::COIN_SWAP_DEPOSIT => write!(f, "COIN_SWAP_DEPOSIT"),
            Self::COIN_SWAP_WITHDRAW => write!(f, "COIN_SWAP_WITHDRAW"),
            Self::POSITION_LIMIT_INCREASE_FEE => write!(f, "POSITION_LIMIT_INCREASE_FEE"),
        }
    }
}

impl FuturesAccount {
    pub fn limit_buy(
        &self, symbol: impl Into<String>, qty: impl Into<f64>, price: f64,
        time_in_force: TimeInForce,
    ) -> Result<Transaction> {
        let buy = OrderRequest::limit_buy(symbol, qty, price, time_in_force);
        let order = self.build_order(buy);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed(API::Futures(Futures::Order), request)
    }

    pub fn limit_sell(
        &self, symbol: impl Into<String>, qty: impl Into<f64>, price: f64,
        time_in_force: TimeInForce,
    ) -> Result<Transaction> {
        let sell = OrderRequest::limit_sell(symbol, qty, price, time_in_force);
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed(API::Futures(Futures::Order), request)
    }

    // Place a MARKET order - BUY
    pub fn market_buy<S, F>(&self, symbol: S, qty: F) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy = OrderRequest::market_buy(symbol, qty);
        let order = self.build_order(buy);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed(API::Futures(Futures::Order), request)
    }

    // Place a MARKET order - SELL
    pub fn market_sell<S, F>(&self, symbol: S, qty: F) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest::market_sell(symbol, qty);
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed(API::Futures(Futures::Order), request)
    }

    pub fn cancel_order<S>(&self, symbol: S, order_id: u64) -> Result<CanceledOrder>
    where
        S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("orderId".into(), order_id.to_string());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .delete_signed(API::Futures(Futures::Order), Some(request))
    }

    pub fn cancel_order_with_client_id<S>(
        &self, symbol: S, orig_client_order_id: String,
    ) -> Result<CanceledOrder>
    where
        S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("origClientOrderId".into(), orig_client_order_id);

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .delete_signed(API::Futures(Futures::Order), Some(request))
    }

    // Place a STOP_MARKET close - BUY
    pub fn stop_market_close_buy<S, F>(&self, symbol: S, stop_price: F) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest::stop_market_close_buy(symbol, stop_price);
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed(API::Futures(Futures::Order), request)
    }

    // Place a STOP_MARKET close - SELL
    pub fn stop_market_close_sell<S, F>(&self, symbol: S, stop_price: F) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest::stop_market_close_sell(symbol, stop_price);
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed(API::Futures(Futures::Order), request)
    }

    // Custom order for professional traders
    pub fn custom_order(&self, order: OrderRequest) -> Result<Transaction> {
        let order = self.build_order(order);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed(API::Futures(Futures::Order), request)
    }

    // Custom batch orders for professional traders
    pub fn custom_batch_orders(
        &self, order_requests: Vec<OrderRequest>,
    ) -> Result<Vec<TransactionOrError>> {
        if order_requests.is_empty() {
            return Ok(Vec::new());
        }
        let mut parameters = BTreeMap::new();
        let mut orders = Vec::new();
        for order in order_requests {
            let order = self.build_order(order);
            orders.push(order);
        }
        parameters.insert("batchOrders".into(), build_list_of_json(orders));
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .post_signed(API::Futures(Futures::BatchOrders), request)
    }

    pub fn get_all_orders<S, F, N>(
        &self, symbol: S, order_id: F, start_time: F, end_time: F, limit: N,
    ) -> Result<Vec<Order>>
    where
        S: Into<String>,
        F: Into<Option<u64>>,
        N: Into<Option<u16>>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        if let Some(order_id) = order_id.into() {
            parameters.insert("orderId".into(), order_id.to_string());
        }
        if let Some(start_time) = start_time.into() {
            parameters.insert("startTime".into(), start_time.to_string());
        }
        if let Some(end_time) = end_time.into() {
            parameters.insert("endTime".into(), end_time.to_string());
        }
        if let Some(limit) = limit.into() {
            parameters.insert("limit".into(), limit.to_string());
        }

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::AllOrders), Some(request))
    }

    pub fn get_user_trades<S, F, N>(
        &self, symbol: S, from_id: F, start_time: F, end_time: F, limit: N,
    ) -> Result<Vec<TradeHistory>>
    where
        S: Into<String>,
        F: Into<Option<u64>>,
        N: Into<Option<u16>>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        if let Some(order_id) = from_id.into() {
            parameters.insert("fromId".into(), order_id.to_string());
        }
        if let Some(start_time) = start_time.into() {
            parameters.insert("startTime".into(), start_time.to_string());
        }
        if let Some(end_time) = end_time.into() {
            parameters.insert("endTime".into(), end_time.to_string());
        }
        if let Some(limit) = limit.into() {
            parameters.insert("limit".into(), limit.to_string());
        }

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::UserTrades), Some(request))
    }
    fn build_order(&self, order: OrderRequest) -> BTreeMap<String, String> {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), order.symbol);
        parameters.insert("side".into(), order.side.to_string());
        parameters.insert("type".into(), order.order_type.to_string());

        if let Some(position_side) = order.position_side {
            parameters.insert("positionSide".into(), position_side.to_string());
        }
        if let Some(time_in_force) = order.time_in_force {
            parameters.insert("timeInForce".into(), time_in_force.to_string());
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
            parameters.insert(
                "closePosition".into(),
                close_position.to_string().to_uppercase(),
            );
        }
        if let Some(activation_price) = order.activation_price {
            parameters.insert("activationPrice".into(), activation_price.to_string());
        }
        if let Some(callback_rate) = order.callback_rate {
            parameters.insert("callbackRate".into(), callback_rate.to_string());
        }
        if let Some(working_type) = order.working_type {
            parameters.insert("workingType".into(), working_type.to_string());
        }
        if let Some(price_protect) = order.price_protect {
            parameters.insert(
                "priceProtect".into(),
                price_protect.to_string().to_uppercase(),
            );
        }

        parameters
    }

    pub fn position_information<S>(&self, symbol: S) -> Result<Vec<PositionRisk>>
    where
        S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::PositionRisk), Some(request))
    }

    pub fn account_information(&self) -> Result<AccountInformation> {
        let parameters = BTreeMap::new();

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::Account), Some(request))
    }

    pub fn account_balance(&self) -> Result<Vec<AccountBalance>> {
        let parameters = BTreeMap::new();

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::Balance), Some(request))
    }

    pub fn change_initial_leverage<S>(
        &self, symbol: S, leverage: u8,
    ) -> Result<ChangeLeverageResponse>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("leverage".into(), leverage.to_string());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .post_signed(API::Futures(Futures::ChangeInitialLeverage), request)
    }

    pub fn change_margin_type<S>(&self, symbol: S, isolated: bool) -> Result<()>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        let margin_type = if isolated { "ISOLATED" } else { "CROSSED" };
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("marginType".into(), margin_type.into());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Futures(Futures::MarginType), request)
            .map(|_| ())
    }

    pub fn change_position_margin<S>(
        &self, symbol: S, amount: f64, is_adding_margin: bool,
    ) -> Result<()>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        let margin = if is_adding_margin { "1" } else { "2" };
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("amount".into(), amount.to_string());
        parameters.insert("type".into(), margin.into());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Futures(Futures::PositionMargin), request)
            .map(|_| ())
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

    pub fn cancel_all_open_orders<S>(&self, symbol: S) -> Result<()>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .delete_signed::<Empty>(API::Futures(Futures::AllOpenOrders), Some(request))
            .map(|_| ())
    }

    pub fn get_all_open_orders<S>(&self, symbol: S) -> Result<Vec<crate::futures::model::Order>>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::OpenOrders), Some(request))
    }

    pub fn get_income(
        &self, income_request: IncomeRequest,
    ) -> Result<Vec<crate::futures::model::Income>> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        if let Some(symbol) = income_request.symbol {
            parameters.insert("symbol".into(), symbol);
        }
        if let Some(income_type) = income_request.income_type {
            parameters.insert("incomeType".into(), income_type.to_string());
        }
        if let Some(start_time) = income_request.start_time {
            parameters.insert("startTime".into(), start_time.to_string());
        }
        if let Some(end_time) = income_request.end_time {
            parameters.insert("endTime".into(), end_time.to_string());
        }
        if let Some(limit) = income_request.limit {
            parameters.insert("limit".into(), limit.to_string());
        }

        let request = build_signed_request(parameters, self.recv_window)?;
        println!("{}", request);
        self.client
            .get_signed(API::Futures(Futures::Income), Some(request))
    }

    pub fn leverage_brackets<S>(
        &self, symbol: S,
    ) -> Result<Vec<crate::futures::model::LeverageBrackets>>
    where
        S: Into<Option<String>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        if let Some(symbol) = symbol.into() {
            parameters.insert("symbol".into(), symbol);
        }
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::LeverageBracket), Some(request))
    }
}
