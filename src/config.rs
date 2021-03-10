#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub rest_api_endpoint: String,
    pub ws_endpoint: String,

    pub futures_rest_api_endpoint: String,
    pub futures_ws_endpoint: String,
}

impl Config {
    pub fn default() -> Config {
        Config {
            rest_api_endpoint: "https://api.binance.com".into(),
            ws_endpoint: "wss://stream.binance.com:9443/ws/".into(),

            futures_rest_api_endpoint: "https://fapi.binance.com".into(),
            futures_ws_endpoint: "wss://fstream.binance.com".into(),
        }
    }

    pub fn set_rest_api_endpoint(mut self, rest_api_endpoint: String) -> Self {
        self.rest_api_endpoint = rest_api_endpoint;
        self
    }

    pub fn set_ws_endpoint(mut self, ws_endpoint: String) -> Self {
        self.ws_endpoint = ws_endpoint;
        self
    }
    pub fn set_futures_rest_api_endpoint(mut self, futures_rest_api_endpoint: String) -> Self {
        self.futures_rest_api_endpoint = futures_rest_api_endpoint;
        self
    }

    pub fn set_futures_ws_endpoint(mut self, futures_ws_endpoint: String) -> Self {
        self.futures_ws_endpoint = futures_ws_endpoint;
        self
    }
}
