use serde::Deserialize;

#[derive(Deserialize, Serialize, Debug, Clone, Fail)]
#[fail(display = "Binance returns error: {}", msg)]
pub struct BinanceResponseError {
    pub code: i64,
    pub msg: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum BinanceResponse<T> {
    Success(T),
    Error(BinanceResponseError),
}

impl<T: for<'a> Deserialize<'a>> BinanceResponse<T> {
    pub fn to_result(self) -> Result<T, BinanceResponseError> {
        match self {
            BinanceResponse::Success(t) => Result::Ok(t),
            BinanceResponse::Error(e) => Result::Err(e),
        }
    }
}

#[derive(Debug, Fail, Serialize, Deserialize, Clone)]
pub enum BinanceError {
    #[fail(display = "Assets not found")]
    AssetsNotFound,
    #[fail(display = "Symbol not found")]
    SymbolNotFound,
    #[fail(display = "No Api key set for private api")]
    NoApiKeySet,
    #[fail(display = "No stream is subscribed")]
    NoStreamSubscribed,
}
