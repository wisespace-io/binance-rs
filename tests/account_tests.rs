use binance::api::*;
use binance::config::*;
use binance::account::*;

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, Matcher};

    #[test]
    fn test_order_market_buy_using_quote_quantity() {
        let mock_exchange_info = mock("POST", "/api/v3/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("quoteOrderQty=0.002&recvWindow=1234&side=BUY&symbol=BNBBTC&timestamp=\\d+&type=MARKET&signature=.*".into()))
            .with_body_from_file("tests/mocks/account/order_market_buy_using_quote_quantity.json")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(mockito::server_url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        match account.market_buy_using_quote_quantity("BNBBTC", 0.002) {
            Ok(answer) => {
                assert!(answer.order_id == 1);
            }
            Err(e) => panic!("Error: {}", e),
        }

        mock_exchange_info.assert();
    }
}
