use binance::api::*;
use binance::config::*;
use binance::account::*;

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, Matcher};

    #[test]
    fn get_account() {

        let mock_get_account = mock("GET", "/api/v3/account")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("recvWindow=1234&timestamp=\\d+&signature=.*".into()))
            .with_body_from_file("tests/mocks/account/get_account.json")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(mockito::server_url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let account = account.get_account().unwrap();
        mock_get_account.assert();

        assert_eq!(account.maker_commission, 15.0);
        assert_eq!(account.taker_commission, 15.0);
        assert_eq!(account.buyer_commission, 0.0);
        assert_eq!(account.seller_commission, 0.0);
        assert_eq!(account.can_trade, true);
        assert_eq!(account.can_withdraw, true);
        assert_eq!(account.can_deposit, true);

        assert!(!account.balances.is_empty());

        let first_balance = account.balances[0].clone();
        assert_eq!(first_balance.asset, "BTC");
        assert_eq!(first_balance.free, "4723846.89208129");
        assert_eq!(first_balance.locked, "0.00000000");

        let second_balance = account.balances[1].clone();
        assert_eq!(second_balance.asset, "LTC");
        assert_eq!(second_balance.free, "4763368.68006011");
        assert_eq!(second_balance.locked, "0.00000000");

    }

    #[test]
    fn get_balance() {

        let mock_get_account = mock("GET", "/api/v3/account")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("recvWindow=1234&timestamp=\\d+&signature=.*".into()))
            .with_body_from_file("tests/mocks/account/get_account.json")
            .create();

        let config = Config::default()
            .set_rest_api_endpoint(mockito::server_url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let balance = account.get_balance("BTC").unwrap();
        mock_get_account.assert();

        assert_eq!(balance.asset, "BTC");
        assert_eq!(balance.free, "4723846.89208129");
        assert_eq!(balance.locked, "0.00000000");

    }

    #[test]
    fn order_market_buy_using_quote_quantity() {

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
