use binance::api::*;
use binance::config::*;
use binance::general::*;
// use crate::account::*;
// use crate::market::*;

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;
    // use binance::config::Config;
    // use binance::general::General;

    #[test]
    fn exchange_info() {
        let mock_exchange_info = mock("GET", "/api/v3/exchangeInfo")
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body_from_file("tests/mocks/general/exchangeInfo.json")
            .create();

        let config = Config::default().set_rest_api_endpoint(mockito::server_url());
        let general: General = Binance::new_with_config(None, None, &config);

        let exchange_info = general.exchange_info().unwrap();
        mock_exchange_info.assert();

        assert!(exchange_info.symbols.len() > 1);
    }
}
