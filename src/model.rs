#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime 
{
    pub server_time: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformation 
{
    pub maker_commission: f32,
    pub taker_commission: f32,
	pub buyer_commission: f32,
	pub seller_commission: f32,
	pub can_trade: bool,
	pub can_withdraw: bool,
	pub can_deposit: bool,
	pub balances: Vec<Balance>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance    
{
	pub asset: String,
	pub free: String,
	pub locked: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order    
{
    pub symbol: String,
    pub order_id: u32,
    pub client_order_id: String,
    pub price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub status: String,
    pub time_in_force: String,
	#[serde(rename = "type")] 
    pub type_name: String,
    pub side: String,
    pub stop_price: String,
    pub iceberg_qty: String,
    pub time: u64
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderCanceled  
{
    pub symbol: String,
    pub orig_client_order_id: String,
    pub order_id: u32,
    pub client_order_id: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction 
{
    pub symbol: String,
    pub order_id: u32,
    pub client_order_id: String,
    pub transact_time: u32
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook    
{
    pub last_update_id: u64,
    pub bids: Vec<Bids>,
    pub asks: Vec<Asks>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bids
{
    price: String,
    qty: String,

    // Never serialized.
    #[serde(skip_serializing)]    
    ignore: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asks
{
    price: String,
    qty: String,
    
    // Never serialized.
    #[serde(skip_serializing)]
    ignore: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDataStream
{
    pub listen_key: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Success { }

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Prices
{
    AllPrices(Vec<SymbolPrice>)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolPrice { 
    pub symbol: String,
    pub price: String    
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum BookTickers
{
    AllBookTickers(Vec<Tickers>)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tickers { 
    pub symbol: String,
    pub bid_price: String,
    pub bid_qty: String,
    pub ask_price: String, 
    pub ask_qty: String     
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeHistory { 
    pub id: u32,
    pub price: String,
    pub qty: String,
    pub commission: String,
    pub commission_asset: String, 
    pub time: u64,
    pub is_buyer: bool,
    pub is_maker: bool,
    pub is_best_match: bool   
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceStats { 
    pub price_change: String,
    pub price_change_percent: String,
    pub weighted_avg_price: String,
    pub prev_close_price: String,
    pub last_price: String, 
    pub bid_price: String,
    pub ask_price: String,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub volume: String, 
    pub open_time: u64,
    pub close_time: u64,
    pub first_id: u32,
    pub last_id: u32,
    pub count: u32
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountUpdateEvent { 
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    m: u32,
    t: u32,
    b: u32,
    s: u32,

    #[serde(rename = "T")]
    t_ignore: bool,
    #[serde(rename = "W")]
    w_ignore: bool,
    #[serde(rename = "D")]    
    d_ignore: bool,

    #[serde(rename = "B")]
    pub balance: Vec<EventBalance>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventBalance    
{
    #[serde(rename = "a")] 
	pub asset: String,
    #[serde(rename = "f")] 
	pub free: String,
    #[serde(rename = "l")] 
	pub locked: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderTradeEvent { 
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "c")]
    pub new_client_order_id: String,

    #[serde(rename = "S")]
    pub side: String,

    #[serde(rename = "o")]
    pub order_type: String,    

    #[serde(rename = "f")]
    pub time_in_force: String,

    #[serde(rename = "q")]
    pub qty: String,

    #[serde(rename = "p")]
    pub price: String,

    #[serde(skip_serializing, rename = "P")]
    pub p_ignore: String,

    #[serde(skip_serializing, rename = "F")]
    pub f_ignore: String,

    #[serde(skip_serializing)]
    pub g: i32,

    #[serde(skip_serializing, rename = "C")]
    pub c_ignore: Option<String>,    

    #[serde(rename = "x")]
    pub execution_type: String,

    #[serde(rename = "X")]
    pub order_status: String,

    #[serde(rename = "r")]
    pub order_reject_reason: String,

    #[serde(rename = "i")]
    pub order_id: u32,

    #[serde(rename = "l")]
    pub qty_last_filled_trade: String,  

    #[serde(rename = "z")]
    pub accumulated_qty_filled_trades: String,  

    #[serde(rename = "L")]
    pub price_last_filled_trade: String,  

    #[serde(rename = "n")]
    pub commission: String,  

    #[serde(skip_serializing, rename = "N")]
    pub asset_commisioned: Option<String>,  

    #[serde(rename = "T")]
    pub trade_order_time: u64,  

    #[serde(rename = "t")]
    pub trade_id: i32,  

    #[serde(skip_serializing, rename = "I")]
    pub i_ignore: u32,  

    #[serde(skip_serializing)]
    pub w: bool,  

    #[serde(rename = "m")]
    pub is_buyer_maker: bool,  

    #[serde(skip_serializing, rename = "M")]
    pub m_ignore: bool,  
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradesEvent { 
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "a")]
    pub aggregated_trade_id: u32,

    #[serde(rename = "p")]
    pub price: String,

    #[serde(rename = "q")]
    pub qty: String,

    #[serde(rename = "f")]
    pub first_break_trade_id: u32,

    #[serde(rename = "l")]
    pub last_break_trade_id: u32,

    #[serde(rename = "T")]
    pub trade_order_time: u64,  

    #[serde(rename = "m")]
    pub is_buyer_maker: bool,  

    #[serde(skip_serializing, rename = "M")]
    pub m_ignore: bool
}
	
						