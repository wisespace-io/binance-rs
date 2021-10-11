use binance::api::*;
use binance::config::*;
use binance::futures::account::*;

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, Matcher};
    use float_cmp::*;
    use binance::futures::model::Transaction;

    #[test]
    fn change_initial_leverage() {
        let mock_change_leverage = mock("POST", "/fapi/v1/leverage")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("leverage=2&recvWindow=1234&symbol=LTCUSDT&timestamp=\\d+&signature=.*".into()))
            .with_body_from_file("tests/mocks/futures/account/change_initial_leverage.json")
            .create();
        
        let config = Config::default()
            .set_futures_rest_api_endpoint(mockito::server_url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let response = account.change_initial_leverage("LTCUSDT", 2).unwrap();

        mock_change_leverage.assert();

        assert_eq!(response.leverage, 2);
        assert_eq!(response.symbol, "LTCUSDT");
        assert!(approx_eq!(f64, response.max_notional_value, 9223372036854776000.0, ulps = 2));
    }

    #[test]
    fn cancel_all_open_orders() {
        let mock = mock("DELETE", "/fapi/v1/allOpenOrders")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("recvWindow=1234&symbol=BTCUSDT&timestamp=\\d+&signature=.*".into()))
            .with_body_from_file("tests/mocks/futures/account/cancel_all_open_orders.json")
            .create();
        
        let config = Config::default()
            .set_futures_rest_api_endpoint(mockito::server_url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        account.cancel_all_open_orders("BTCUSDT").unwrap();

        mock.assert();
    }

    #[test]
    fn change_position_mode() {
        let mock = mock("POST", "/fapi/v1/positionSide/dual")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("dualSidePosition=true&recvWindow=1234&timestamp=\\d+&signature=.*".into()))
            .with_body_from_file("tests/mocks/futures/account/change_position_mode.json")
            .create();
        
        let config = Config::default()
            .set_futures_rest_api_endpoint(mockito::server_url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        account.change_position_mode(true).unwrap();

        mock.assert();
    }

    #[test]
    fn stop_market_close_buy() {
        let mock_stop_market_close_sell = mock("POST", "/fapi/v1/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("closePosition=TRUE&recvWindow=1234&side=BUY&stopPrice=10.5&symbol=SRMUSDT&timestamp=\\d+&type=STOP_MARKET".into()))
            .with_body_from_file("tests/mocks/futures/account/stop_market_close_position_buy.json")
            .create();

        let config = Config::default()
            .set_futures_rest_api_endpoint(mockito::server_url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let transaction: Transaction = account.stop_market_close_buy("SRMUSDT", 10.5).unwrap();

        mock_stop_market_close_sell.assert();

        assert_eq!(transaction.symbol, "SRMUSDT");
        assert_eq!(transaction.side, "BUY");
        assert_eq!(transaction.orig_type, "STOP_MARKET");
        assert_eq!(transaction.close_position, true);
        assert!(approx_eq!(f64, transaction.stop_price, 10.5, ulps = 2));

    }

    #[test]
    fn stop_market_close_sell() {
        let mock_stop_market_close_sell = mock("POST", "/fapi/v1/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("closePosition=TRUE&recvWindow=1234&side=SELL&stopPrice=7.4&symbol=SRMUSDT&timestamp=\\d+&type=STOP_MARKET".into()))
            .with_body_from_file("tests/mocks/futures/account/stop_market_close_position_sell.json")
            .create();

        let config = Config::default()
            .set_futures_rest_api_endpoint(mockito::server_url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let transaction: Transaction = account.stop_market_close_sell("SRMUSDT", 7.4).unwrap();

        mock_stop_market_close_sell.assert();

        assert_eq!(transaction.symbol, "SRMUSDT");
        assert_eq!(transaction.side, "SELL");
        assert_eq!(transaction.orig_type, "STOP_MARKET");
        assert_eq!(transaction.close_position, true);
        assert!(approx_eq!(f64, transaction.stop_price, 7.4, ulps = 2));

    }

}