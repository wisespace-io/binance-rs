use binance::api::*;
use binance::config::*;
use binance::futures::market::FuturesMarket;
use binance::futures::model::OpenInterestHist;

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, Matcher};

    #[test]
    fn open_interest_statistics() {
        let mock_open_interest_statistics = mock("GET", "/futures/data/openInterestHist")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("limit=10&period=5m&symbol=BTCUSDT".into()))
            .with_body_from_file("tests/mocks/futures/market/open_interest_statistics.json")
            .create();

        let config = Config::default().set_futures_rest_api_endpoint(mockito::server_url());
        let market: FuturesMarket = Binance::new_with_config(None, None, &config);

        let open_interest_hists = market
            .open_interest_statistics("BTCUSDT", "5m", 10, None, None)
            .unwrap();
        mock_open_interest_statistics.assert();

        let expectation = vec![
            OpenInterestHist {
                symbol: "BTCUSDT".into(),
                sum_open_interest: "20403.63700000".into(),
                sum_open_interest_value: "150570784.07809979".into(),
                timestamp: 1583127900000,
            },
            OpenInterestHist {
                symbol: "BTCUSDT".into(),
                sum_open_interest: "20401.36700000".into(),
                sum_open_interest_value: "149940752.14464448".into(),
                timestamp: 1583128200000,
            },
        ];

        assert_eq!(open_interest_hists, expectation);
    }
}
