use super::{
    string_or_float, Asks, Bids, Kline, OrderBook, OrderExecType, OrderRejectReason, OrderStatus,
    OrderType, Side, TimeInForce,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Subscription {
    UserData(String),            // listen key
    AggregateTrade(String),      //symbol
    Trade(String),               //symbol
    Candlestick(String, String), //symbol, interval
    MiniTicker(String),          //symbol
    MiniTickerAll,
    Ticker(String), // symbol
    TickerAll,
    OrderBook(String, i64), //symbol, depth
    Depth(String),          //symbol
}

#[derive(Debug, Clone, Serialize)]
pub enum BinanceWebsocketMessage {
    UserOrderUpdate(UserOrderUpdate),
    UserAccountUpdate(AccountUpdate),
    AggregateTrade(AggregateTrade),
    Trade(TradeMessage),
    Candlestick(CandelStickMessage),
    MiniTicker(MiniTicker),
    MiniTickerAll(Vec<MiniTicker>),
    Ticker(Ticker),
    TickerAll(Vec<Ticker>),
    OrderBook(OrderBook),
    Depth(Depth),
    Ping,
    Pong,
    Binary(Vec<u8>), // Unexpected, unparsed
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradeMessage {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "t")]
    pub trade_id: u64,
    #[serde(rename = "p", with = "string_or_float")]
    pub price: f64,
    #[serde(rename = "q", with = "string_or_float")]
    pub qty: f64,
    #[serde(rename = "b")]
    pub buyer_order_id: u64,
    #[serde(rename = "a")]
    pub seller_order_id: u64,
    #[serde(rename = "T")]
    pub trade_order_time: u64,
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
    #[serde(skip_serializing, rename = "M")]
    pub m_ignore: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AggregateTrade {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "a")]
    pub aggregated_trade_id: u64,
    #[serde(rename = "p", with = "string_or_float")]
    pub price: f64,
    #[serde(rename = "q", with = "string_or_float")]
    pub qty: f64,
    #[serde(rename = "f")]
    pub first_break_trade_id: u64,
    #[serde(rename = "l")]
    pub last_break_trade_id: u64,
    #[serde(rename = "T")]
    pub trade_order_time: u64,
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
    #[serde(skip_serializing, rename = "M")]
    pub m_ignore: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserOrderUpdate {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c")]
    pub new_client_order_id: String,
    #[serde(rename = "S")]
    pub side: Side,
    #[serde(rename = "o")]
    pub order_type: OrderType,
    #[serde(rename = "f")]
    pub time_in_force: TimeInForce,
    #[serde(rename = "q", with = "string_or_float")]
    pub qty: f64,
    #[serde(rename = "p", with = "string_or_float")]
    pub price: f64,
    #[serde(rename = "P", with = "string_or_float")]
    pub stop_price: f64,
    #[serde(rename = "F", with = "string_or_float")]
    pub iceberg_qty: f64,
    #[serde(skip_serializing)]
    pub g: i32,
    #[serde(skip_serializing, rename = "C")]
    pub c_ignore: Option<String>,
    #[serde(rename = "x")]
    pub execution_type: OrderExecType,
    #[serde(rename = "X")]
    pub order_status: OrderStatus,
    #[serde(rename = "r")]
    pub order_reject_reason: OrderRejectReason,
    #[serde(rename = "i")]
    pub order_id: u64,
    #[serde(rename = "l", with = "string_or_float")]
    pub qty_last_filled_trade: f64,
    #[serde(rename = "z", with = "string_or_float")]
    pub accumulated_qty_filled_trades: f64,
    #[serde(rename = "L", with = "string_or_float")]
    pub price_last_filled_trade: f64,
    #[serde(rename = "n", with = "string_or_float")]
    pub commission: f64,
    #[serde(skip_serializing, rename = "N")]
    pub asset_commisioned: Option<String>,
    #[serde(rename = "T")]
    pub trade_order_time: u64,
    #[serde(rename = "t")]
    pub trade_id: i64,
    #[serde(skip_serializing, rename = "I")]
    pub i_ignore: u64,
    #[serde(skip_serializing)]
    pub w: bool,
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
    #[serde(skip_serializing, rename = "M")]
    pub m_ignore: bool,
    #[serde(skip_serializing, rename = "O")]
    pub order_creation_time: u64,
    #[serde(skip_serializing, rename = "Z", with = "string_or_float")]
    pub cumulative_quote_asset_transacted_qty: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Depth {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "U")]
    pub first_update_id: u64,
    #[serde(rename = "u")]
    pub final_update_id: u64,
    #[serde(rename = "b")]
    pub bids: Vec<Bids>,
    #[serde(rename = "a")]
    pub asks: Vec<Asks>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "p", with = "string_or_float")]
    pub price_change: f64,
    #[serde(rename = "P", with = "string_or_float")]
    pub price_change_percent: f64,
    #[serde(rename = "w", with = "string_or_float")]
    pub average_price: f64,
    #[serde(rename = "x", with = "string_or_float")]
    pub prev_close: f64,
    #[serde(rename = "c", with = "string_or_float")]
    pub current_close: f64,
    #[serde(rename = "Q", with = "string_or_float")]
    pub current_close_qty: f64,
    #[serde(rename = "b", with = "string_or_float")]
    pub best_bid: f64,
    #[serde(rename = "B", with = "string_or_float")]
    pub best_bid_qty: f64,
    #[serde(rename = "a", with = "string_or_float")]
    pub best_ask: f64,
    #[serde(rename = "A", with = "string_or_float")]
    pub best_ask_qty: f64,
    #[serde(rename = "o", with = "string_or_float")]
    pub open: f64,
    #[serde(rename = "h", with = "string_or_float")]
    pub high: f64,
    #[serde(rename = "l", with = "string_or_float")]
    pub low: f64,
    #[serde(rename = "v", with = "string_or_float")]
    pub volume: f64,
    #[serde(rename = "q", with = "string_or_float")]
    pub quote_volume: f64,
    #[serde(rename = "O")]
    pub open_time: u64,
    #[serde(rename = "C")]
    pub close_time: u64,
    #[serde(rename = "F")]
    pub first_trade_id: u64,
    #[serde(rename = "L")]
    pub last_trade_id: u64,
    #[serde(rename = "n")]
    pub num_trades: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CandelStickMessage {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "k")]
    pub kline: Kline,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountUpdate {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "m")]
    pub maker_commision_rate: u64,
    #[serde(rename = "t")]
    pub taker_commision_rate: u64,
    #[serde(rename = "b")]
    pub buyer_commision_rate: u64,
    #[serde(rename = "s")]
    pub seller_commision_rate: u64,
    #[serde(rename = "T")]
    pub can_trade: bool,
    #[serde(rename = "W")]
    pub can_withdraw: bool,
    #[serde(rename = "D")]
    pub can_deposit: bool,
    #[serde(rename = "u")]
    pub last_account_update: u64,
    #[serde(rename = "B")]
    pub balance: Vec<AccountUpdateBalance>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountUpdateBalance {
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "f", with = "string_or_float")]
    pub free: f64,
    #[serde(rename = "l", with = "string_or_float")]
    pub locked: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MiniTicker {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c", with = "string_or_float")]
    pub close: f64,
    #[serde(rename = "o", with = "string_or_float")]
    pub open: f64,
    #[serde(rename = "l", with = "string_or_float")]
    pub low: f64,
    #[serde(rename = "h", with = "string_or_float")]
    pub high: f64,
    #[serde(rename = "v", with = "string_or_float")]
    pub volume: f64,
    #[serde(rename = "q", with = "string_or_float")]
    pub quote_volume: f64,
}
