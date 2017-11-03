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
    listen_key: String
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