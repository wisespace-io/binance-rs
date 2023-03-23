use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Deserialize, Serialize, Debug, Clone, Error)]
#[error("Binance returns error: {.msg}")]
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

#[derive(Debug, Error, Serialize, Deserialize, Clone)]
pub enum BinanceError {
    #[error("Assets not found")]
    AssetsNotFound,
    #[error("Symbol not found")]
    SymbolNotFound,
    #[error("No Api key set for private api")]
    NoApiKeySet,
    #[error("No stream is subscribed")]
    NoStreamSubscribed,
}
