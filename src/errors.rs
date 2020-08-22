use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
pub struct BinanceContentError {
    pub code: i16,
    pub msg: String,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

error_chain! {
    errors {
        BinanceError(response: BinanceContentError)
     }

    foreign_links {
        ReqError(reqwest::Error);
        InvalidHeaderError(reqwest::header::InvalidHeaderValue);
        IoError(std::io::Error);
        ParseFloatError(std::num::ParseFloatError);
        UrlParserError(url::ParseError);
        Json(serde_json::Error);
        Tungstenite(tungstenite::Error);
        TimestampError(std::time::SystemTimeError);
    }
}
