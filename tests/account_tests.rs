use std::path::Path;
use std::vec;

use binance::api::Binance;
use binance::config::Config;
use binance::account::{Account, OrderSide, OrderType, TimeInForce};
use binance::model::{OrderCanceled, Transaction};

use mockito::{self, Matcher, Mock};
use float_cmp::assert_approx_eq;

mod common;

type TestBuilder = common::Builder<Account>;

const CONTENT_TYPE: &str = "application/json;charset=UTF-8";
const RECV_WINDOW: u64 = 1234;

fn setup_mock_from_file<P>(
    method: &str, path: P, extra_query_matchers: Vec<Matcher>, body: impl AsRef<Path>,
) -> (Mock, Account)
where
    P: Into<Matcher>,
{
    common::setup_mock_from_file(method, path, extra_query_matchers, body)
}

#[test]
fn get_account() {
    let (mock, client) = TestBuilder::new(
        "GET",
        "/api/v3/account",
        vec![
            Matcher::Regex("timestamp=\\d+".to_string()),
            Matcher::Regex("signature=.*".to_string()),
        ],
    )
    .with_body_from_file("tests/mocks/account/get_account.json");

    let account = client.get_account().unwrap();

    mock.assert();

    assert_approx_eq!(f32, account.maker_commission, 15.0, ulps = 2);
    assert_approx_eq!(f32, account.taker_commission, 15.0, ulps = 2);
    assert_approx_eq!(f32, account.buyer_commission, 0.0, ulps = 2);
    assert_approx_eq!(f32, account.seller_commission, 0.0, ulps = 2);
    assert!(account.can_trade);
    assert!(account.can_withdraw);
    assert!(account.can_deposit);

    assert!(!account.balances.is_empty());

    let first_balance = &account.balances[0];
    assert_eq!(first_balance.asset, "BTC");
    assert_eq!(first_balance.free, "4723846.89208129");
    assert_eq!(first_balance.locked, "0.00000000");

    let second_balance = &account.balances[1];
    assert_eq!(second_balance.asset, "LTC");
    assert_eq!(second_balance.free, "4763368.68006011");
    assert_eq!(second_balance.locked, "0.00000000");
}

#[test]
fn get_balance() {
    let (mock, client) = TestBuilder::new(
        "GET",
        "/api/v3/account",
        vec![
            Matcher::Regex("timestamp=\\d+".to_string()),
            Matcher::Regex("signature=.*".to_string()),
        ],
    )
    .with_body_from_file("tests/mocks/account/get_account.json");

    let balance = client.get_balance("BTC").unwrap();

    mock.assert();

    assert_eq!(balance.asset, "BTC");
    assert_eq!(balance.free, "4723846.89208129");
    assert_eq!(balance.locked, "0.00000000");
}

#[test]
fn get_open_orders() {
    let (mock, client) = TestBuilder::new(
        "GET",
        "/api/v3/openOrders",
        vec![
            Matcher::UrlEncoded("symbol".to_string(), "LTCBTC".to_string()),
            Matcher::Regex("timestamp=\\d+".to_string()),
        ],
    )
    .with_body_from_file("tests/mocks/account/get_open_orders.json");

    let open_orders = client.get_open_orders("LTCBTC").unwrap();

    mock.assert();

    assert!(open_orders.len() == 1);
    let open_order = &open_orders[0];

    assert_eq!(open_order.symbol, "LTCBTC");
    assert_eq!(open_order.order_id, 1);
    assert_eq!(open_order.order_list_id, -1);
    assert_eq!(open_order.client_order_id, "myOrder1");
    assert_approx_eq!(f64, open_order.price, 0.1, ulps = 2);
    assert_eq!(open_order.orig_qty, "1.0");
    assert_eq!(open_order.executed_qty, "0.0");
    assert_eq!(open_order.cummulative_quote_qty, "0.0");
    assert_eq!(open_order.status, "NEW");
    assert_eq!(open_order.time_in_force, "GTC"); //Migrate to TimeInForce enum
    assert_eq!(open_order.type_name, "LIMIT");
    assert_eq!(open_order.side, "BUY");
    assert_approx_eq!(f64, open_order.stop_price, 0.0, ulps = 2);
    assert_eq!(open_order.iceberg_qty, "0.0");
    assert_eq!(open_order.time, 1499827319559);
    assert_eq!(open_order.update_time, 1499827319559);
    assert!(open_order.is_working);
    assert_eq!(open_order.orig_quote_order_qty, "0.000000");
}

#[test]
fn get_all_open_orders() {
    let (mock, client) = TestBuilder::new(
        "GET",
        "/api/v3/openOrders",
        vec![Matcher::Regex("timestamp=\\d+".to_string())],
    )
    .with_body_from_file("tests/mocks/account/get_open_orders.json");

    let open_orders = client.get_all_open_orders().unwrap();

    mock.assert();

    assert!(open_orders.len() == 1);
    let open_order = &open_orders[0];

    assert_eq!(open_order.symbol, "LTCBTC");
    assert_eq!(open_order.order_id, 1);
    assert_eq!(open_order.order_list_id, -1);
    assert_eq!(open_order.client_order_id, "myOrder1");
    assert_approx_eq!(f64, open_order.price, 0.1, ulps = 2);
    assert_eq!(open_order.orig_qty, "1.0");
    assert_eq!(open_order.executed_qty, "0.0");
    assert_eq!(open_order.cummulative_quote_qty, "0.0");
    assert_eq!(open_order.status, "NEW");
    assert_eq!(open_order.time_in_force, "GTC"); //Migrate to TimeInForce enum
    assert_eq!(open_order.type_name, "LIMIT");
    assert_eq!(open_order.side, "BUY");
    assert_approx_eq!(f64, open_order.stop_price, 0.0, ulps = 2);
    assert_eq!(open_order.iceberg_qty, "0.0");
    assert_eq!(open_order.time, 1499827319559);
    assert_eq!(open_order.update_time, 1499827319559);
    assert!(open_order.is_working);
    assert_eq!(open_order.orig_quote_order_qty, "0.000000");
}

#[test]
fn cancel_all_open_orders() {
    let symbol = "BTCUSDT";
    let (mock, client) = TestBuilder::new(
        "DELETE",
        "/api/v3/openOrders",
        vec![
            Matcher::UrlEncoded("symbol".to_string(), symbol.to_string()),
            Matcher::Regex("timestamp=\\d+".to_string()),
        ],
    )
    .with_body_from_file("tests/mocks/account/cancel_all_open_orders.json");

    let cancel_all_open_orders = client.cancel_all_open_orders(symbol).unwrap();

    mock.assert();

    assert!(cancel_all_open_orders.len() == 3);

    let first_order_cancelled = cancel_all_open_orders[0].clone();
    assert_eq!(first_order_cancelled.symbol, symbol);
    assert_eq!(
        first_order_cancelled.orig_client_order_id.unwrap(),
        "E6APeyTJvkMvLMYMqu1KQ4"
    );
    assert_eq!(first_order_cancelled.order_id.unwrap(), 11);
    assert_eq!(
        first_order_cancelled.client_order_id.unwrap(),
        "pXLV6Hz6mprAcVYpVMTGgx"
    );

    let second_order_cancelled: OrderCanceled = cancel_all_open_orders[1].clone();
    assert_eq!(second_order_cancelled.symbol, symbol);
    assert_eq!(
        second_order_cancelled.orig_client_order_id.unwrap(),
        "A3EF2HCwxgZPFMrfwbgrhv"
    );
    assert_eq!(second_order_cancelled.order_id.unwrap(), 13);
    assert_eq!(
        second_order_cancelled.client_order_id.unwrap(),
        "pXLV6Hz6mprAcVYpVMTGgx"
    );
}

#[test]
fn order_status() {
    let symbol = "LTCBTC";
    let (mock, client) = TestBuilder::new(
        "GET",
        "/api/v3/order",
        vec![
            Matcher::UrlEncoded("orderId".to_string(), 1.to_string()),
            Matcher::UrlEncoded("symbol".to_string(), symbol.to_string()),
            Matcher::Regex("timestamp=\\d+".to_string()),
        ],
    )
    .with_body_from_file("tests/mocks/account/order_status.json");

    let order_status = client.order_status(symbol, 1).unwrap();

    mock.assert();

    assert_eq!(order_status.symbol, symbol);
    assert_eq!(order_status.order_id, 1);
    assert_eq!(order_status.order_list_id, -1);
    assert_eq!(order_status.client_order_id, "myOrder1");
    assert_approx_eq!(f64, order_status.price, 0.1, ulps = 2);
    assert_eq!(order_status.orig_qty, "1.0");
    assert_eq!(order_status.executed_qty, "0.0");
    assert_eq!(order_status.cummulative_quote_qty, "0.0");
    assert_eq!(order_status.status, "NEW");
    assert_eq!(order_status.time_in_force, "GTC"); //Migrate to TimeInForce enum
    assert_eq!(order_status.type_name, "LIMIT");
    assert_eq!(order_status.side, "BUY");
    assert_approx_eq!(f64, order_status.stop_price, 0.0, ulps = 2);
    assert_eq!(order_status.iceberg_qty, "0.0");
    assert_eq!(order_status.time, 1499827319559);
    assert_eq!(order_status.update_time, 1499827319559);
    assert!(order_status.is_working);
    assert_eq!(order_status.orig_quote_order_qty, "0.000000");
}

#[test]
fn test_order_status() {
    let symbol = "LTCBTC";
    let (mock, client) = TestBuilder::new(
        "GET",
        "/api/v3/order/test",
        vec![
            Matcher::UrlEncoded("orderId".to_string(), 1.to_string()),
            Matcher::UrlEncoded("symbol".to_string(), symbol.to_string()),
            Matcher::Regex("timestamp=\\d+".to_string()),
        ],
    )
    .with_empty_body();

    client.test_order_status(symbol, 1).unwrap();

    mock.assert();
}

#[test]
fn limit_buy() {
    let symbol = "LTCBTC";
    let (mock, client) = TestBuilder::new(
        "POST",
        "/api/v3/order",
        vec![
            Matcher::UrlEncoded("price".to_string(), "0.1".to_string()),
            Matcher::UrlEncoded("quantity".to_string(), 1.to_string()),
            Matcher::UrlEncoded("side".to_string(), "BUY".to_string()),
            Matcher::UrlEncoded("symbol".to_string(), symbol.to_string()),
            Matcher::UrlEncoded("timeInForce".to_string(), "GTC".to_string()),
            Matcher::UrlEncoded("type".to_string(), "LIMIT".to_string()),
            Matcher::Regex("timestamp=\\d+".to_string()),
        ],
    )
    .with_body_from_file("tests/mocks/account/limit_buy.json");

    let transaction = client.limit_buy(symbol, 1, 0.1).unwrap();

    mock.assert();

    assert_eq!(transaction.symbol, symbol);
    assert_eq!(transaction.order_id, 1);
    assert_eq!(transaction.order_list_id.unwrap(), -1);
    assert_eq!(transaction.client_order_id, "6gCrw2kRUAF9CvJDGP16IP");
    assert_eq!(transaction.transact_time, 1507725176595);
    assert_approx_eq!(f64, transaction.price, 0.1, ulps = 2);
    assert_approx_eq!(f64, transaction.orig_qty, 1.0, ulps = 2);
    assert_approx_eq!(f64, transaction.executed_qty, 1.0, ulps = 2);
    assert_approx_eq!(f64, transaction.cummulative_quote_qty, 0.0, ulps = 2);
    assert_eq!(transaction.status, "NEW");
    assert_eq!(transaction.time_in_force, "GTC"); //Migrate to TimeInForce enum
    assert_eq!(transaction.type_name, "LIMIT");
    assert_eq!(transaction.side, "BUY");
}

#[test]
fn test_limit_buy() {
    let symbol = "LTCBTC";
    let (mock, client) = TestBuilder::new(
        "POST",
        "/api/v3/order/test",
        vec![
            Matcher::UrlEncoded("price".to_string(), "0.1".to_string()),
            Matcher::UrlEncoded("quantity".to_string(), 1.to_string()),
            Matcher::UrlEncoded("side".to_string(), "BUY".to_string()),
            Matcher::UrlEncoded("symbol".to_string(), symbol.to_string()),
            Matcher::UrlEncoded("timeInForce".to_string(), "GTC".to_string()),
            Matcher::Regex("timestamp=\\d+".to_string()),
            Matcher::UrlEncoded("type".to_string(), "LIMIT".to_string()),
        ],
    )
    .with_empty_body();

    client.test_limit_buy(symbol, 1, 0.1).unwrap();

    mock.assert();
}

#[test]
fn limit_sell() {
    let symbol = "LTCBTC";
    let (mock, client) = TestBuilder::new(
        "POST",
        "/api/v3/order",
        vec![
            Matcher::UrlEncoded("price".to_string(), "0.1".to_string()),
            Matcher::UrlEncoded("quantity".to_string(), 1.to_string()),
            Matcher::UrlEncoded("side".to_string(), "SELL".to_string()),
            Matcher::UrlEncoded("symbol".to_string(), symbol.to_string()),
            Matcher::UrlEncoded("timeInForce".to_string(), "GTC".to_string()),
            Matcher::UrlEncoded("type".to_string(), "LIMIT".to_string()),
            Matcher::Regex("timestamp=\\d+".to_string()),
        ],
    )
    .with_body_from_file("tests/mocks/account/limit_sell.json");

    let transaction = client.limit_sell(symbol, 1, 0.1).unwrap();

    mock.assert();

    assert_eq!(transaction.symbol, symbol);
    assert_eq!(transaction.order_id, 1);
    assert_eq!(transaction.order_list_id.unwrap(), -1);
    assert_eq!(transaction.client_order_id, "6gCrw2kRUAF9CvJDGP16IP");
    assert_eq!(transaction.transact_time, 1507725176595);
    assert_approx_eq!(f64, transaction.price, 0.1, ulps = 2);
    assert_approx_eq!(f64, transaction.orig_qty, 1.0, ulps = 2);
    assert_approx_eq!(f64, transaction.executed_qty, 1.0, ulps = 2);
    assert_approx_eq!(f64, transaction.cummulative_quote_qty, 0.0, ulps = 2);
    assert_eq!(transaction.status, "NEW");
    assert_eq!(transaction.time_in_force, "GTC"); //Migrate to TimeInForce enum
    assert_eq!(transaction.type_name, "LIMIT");
    assert_eq!(transaction.side, "SELL");
}

#[test]
fn test_limit_sell() {
    let symbol = "LTCBTC";
    let (mock, client) = TestBuilder::new(
        "POST",
        "/api/v3/order/test",
        vec![
            Matcher::UrlEncoded("price".to_string(), "0.1".to_string()),
            Matcher::UrlEncoded("quantity".to_string(), 1.to_string()),
            Matcher::UrlEncoded("side".to_string(), "SELL".to_string()),
            Matcher::UrlEncoded("symbol".to_string(), symbol.to_string()),
            Matcher::UrlEncoded("timeInForce".to_string(), "GTC".to_string()),
            Matcher::UrlEncoded("type".to_string(), "LIMIT".to_string()),
            Matcher::Regex("timestamp=\\d+".to_string()),
        ],
    )
    .with_empty_body();

    client.test_limit_sell(symbol, 1, 0.1).unwrap();

    mock.assert();
}

#[test]
fn market_buy() {
    let symbol = "LTCBTC";
    let (mock, client) = TestBuilder::new(
        "POST",
        "/api/v3/order",
        vec![
            Matcher::UrlEncoded("quantity".to_string(), 1.to_string()),
            Matcher::UrlEncoded("side".to_string(), "BUY".to_string()),
            Matcher::UrlEncoded("symbol".to_string(), symbol.to_string()),
            Matcher::Regex("timestamp=\\d+".to_string()),
            Matcher::UrlEncoded("type".to_string(), "MARKET".to_string()),
        ],
    )
    .with_body_from_file("tests/mocks/account/market_buy.json");

    let transaction = client.market_buy(symbol, 1).unwrap();

    mock.assert();

    assert_eq!(transaction.symbol, symbol);
    assert_eq!(transaction.order_id, 1);
    assert_eq!(transaction.order_list_id.unwrap(), -1);
    assert_eq!(transaction.client_order_id, "6gCrw2kRUAF9CvJDGP16IP");
    assert_eq!(transaction.transact_time, 1507725176595);
    assert_approx_eq!(f64, transaction.price, 0.1, ulps = 2);
    assert_approx_eq!(f64, transaction.orig_qty, 1.0, ulps = 2);
    assert_approx_eq!(f64, transaction.executed_qty, 1.0, ulps = 2);
    assert_approx_eq!(f64, transaction.cummulative_quote_qty, 0.0, ulps = 2);
    assert_eq!(transaction.status, "NEW");
    assert_eq!(transaction.time_in_force, "GTC"); //Migrate to TimeInForce enum
    assert_eq!(transaction.type_name, "MARKET");
    assert_eq!(transaction.side, "BUY");
}

#[test]
fn test_market_buy() {
    let symbol = "LTCBTC";
    let (mock, client) = TestBuilder::new(
        "POST",
        "/api/v3/order/test",
        vec![
            Matcher::UrlEncoded("quantity".to_string(), 1.to_string()),
            Matcher::UrlEncoded("side".to_string(), "BUY".to_string()),
            Matcher::UrlEncoded("symbol".to_string(), symbol.to_string()),
            Matcher::Regex("timestamp=\\d+".to_string()),
            Matcher::UrlEncoded("type".to_string(), "MARKET".to_string()),
        ],
    )
    .with_empty_body();

    client.test_market_buy("LTCBTC", 1).unwrap();

    mock.assert();
}

#[test]
fn market_buy_using_quote_quantity() {
    let symbol = "BNBBTC";
    let (mock, client) = setup_mock_from_file(
        "POST",
        "/api/v3/order",
        vec![
            Matcher::UrlEncoded("quoteOrderQty".to_string(), 0.002.to_string()),
            Matcher::UrlEncoded("recvWindow".to_string(), RECV_WINDOW.to_string()),
            Matcher::UrlEncoded("side".to_string(), "BUY".to_string()),
            Matcher::UrlEncoded("symbol".to_string(), symbol.to_string()),
            Matcher::Regex("timestamp=\\d+".into()),
            Matcher::UrlEncoded("type".to_string(), "MARKET".to_string()),
            Matcher::Regex("signature=.*".into()),
        ],
        "tests/mocks/account/market_buy_using_quote_quantity.json",
    );

    let transaction = client
        .market_buy_using_quote_quantity(symbol, 0.002)
        .unwrap();

    mock.assert();
    assert!(transaction.order_id == 1);
}

#[test]
fn test_market_buy_using_quote_quantity() {
    let symbol = "BNBBTC";
    let mock_test_market_buy_using_quote_quantity = mockito::mock("POST", "/api/v3/order/test")
        .with_header("content-type", CONTENT_TYPE)
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("recvWindow".to_string(), 1234.to_string()),
            Matcher::UrlEncoded("quoteOrderQty".to_string(), 0.002.to_string()),
            Matcher::UrlEncoded("recvWindow".to_string(), RECV_WINDOW.to_string()),
            Matcher::UrlEncoded("side".to_string(), "BUY".to_string()),
            Matcher::UrlEncoded("symbol".to_string(), symbol.to_string()),
            Matcher::Regex("timestamp=\\d+".into()),
            Matcher::UrlEncoded("type".to_string(), "MARKET".to_string()),
            Matcher::Regex("signature=.*".into()),
        ]))
        .with_body("{}")
        .create();

    let config = Config::default()
        .set_rest_api_endpoint(mockito::server_url())
        .set_recv_window(RECV_WINDOW);
    let account: Account = Binance::new_with_config(None, None, &config);
    let _ = env_logger::try_init();
    account
        .test_market_buy_using_quote_quantity(symbol, 0.002)
        .unwrap();

    mock_test_market_buy_using_quote_quantity.assert();
}

#[test]
fn market_sell() {
    let symbol = "LTCBTC";
    let (mock, client) = setup_mock_from_file(
        "POST",
        "/api/v3/order",
        vec![
            Matcher::UrlEncoded("quantity".to_string(), 1.to_string()),
            Matcher::UrlEncoded("side".to_string(), "SELL".to_string()),
            Matcher::UrlEncoded("symbol".to_string(), symbol.to_string()),
            Matcher::Regex("timestamp=\\d+".into()),
            Matcher::UrlEncoded("type".to_string(), "MARKET".to_string()),
        ],
        "tests/mocks/account/market_sell.json",
    );

    let transaction = client.market_sell(symbol, 1).unwrap();

    mock.assert();

    assert_eq!(transaction.symbol, symbol);
    assert_eq!(transaction.order_id, 1);
    assert_eq!(transaction.order_list_id.unwrap(), -1);
    assert_eq!(transaction.client_order_id, "6gCrw2kRUAF9CvJDGP16IP");
    assert_eq!(transaction.transact_time, 1507725176595);
    assert_approx_eq!(f64, transaction.price, 0.1, ulps = 2);
    assert_approx_eq!(f64, transaction.orig_qty, 1.0, ulps = 2);
    assert_approx_eq!(f64, transaction.executed_qty, 1.0, ulps = 2);
    assert_approx_eq!(f64, transaction.cummulative_quote_qty, 0.0, ulps = 2);
    assert_eq!(transaction.status, "NEW");
    assert_eq!(transaction.time_in_force, "GTC"); //Migrate to TimeInForce enum
    assert_eq!(transaction.type_name, "MARKET");
    assert_eq!(transaction.side, "SELL");
}

#[test]
fn test_market_sell() {
    let mock_test_market_sell = mockito::mock("POST", "/api/v3/order/test")
        .with_header("content-type", CONTENT_TYPE)
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("quantity".to_string(), 1.to_string()),
            Matcher::UrlEncoded("recvWindow".to_string(), RECV_WINDOW.to_string()),
            Matcher::UrlEncoded("side".to_string(), "SELL".to_string()),
            Matcher::UrlEncoded("symbol".to_string(), "LTCBTC".to_string()),
            Matcher::Regex("timestamp=\\d+".to_string()),
            Matcher::UrlEncoded("type".to_string(), "MARKET".to_string()),
        ]))
        .with_body("{}")
        .create();

    let config = Config::default()
        .set_rest_api_endpoint(mockito::server_url())
        .set_recv_window(RECV_WINDOW);
    let account: Account = Binance::new_with_config(None, None, &config);
    let _ = env_logger::try_init();
    account.test_market_sell("LTCBTC", 1).unwrap();

    mock_test_market_sell.assert();
}

#[test]
fn market_sell_using_quote_quantity() {
    let mock_market_sell_using_quote_quantity = mockito::mock("POST", "/api/v3/order")
        .with_header("content-type", CONTENT_TYPE)
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("quoteOrderQty".to_string(), 0.002.to_string()),
            Matcher::UrlEncoded("recvWindow".to_string(), RECV_WINDOW.to_string()),
            Matcher::UrlEncoded("side".to_string(), "SELL".to_string()),
            Matcher::UrlEncoded("symbol".to_string(), "BNBBTC".to_string()),
            Matcher::Regex("timestamp=\\d+".to_string()),
            Matcher::UrlEncoded("type".to_string(), "MARKET".to_string()),
            Matcher::Regex("signature=.*".to_string()),
        ]))
        .with_body_from_file("tests/mocks/account/market_sell_using_quote_quantity.json")
        .create();

    let config = Config::default()
        .set_rest_api_endpoint(mockito::server_url())
        .set_recv_window(RECV_WINDOW);
    let account: Account = Binance::new_with_config(None, None, &config);
    let _ = env_logger::try_init();
    match account.market_sell_using_quote_quantity("BNBBTC", 0.002) {
        Ok(answer) => {
            assert!(answer.order_id == 1);
        }
        Err(e) => panic!("Error: {}", e),
    }

    mock_market_sell_using_quote_quantity.assert();
}

#[test]
fn test_market_sell_using_quote_quantity() {
    let mock_test_market_sell_using_quote_quantity = mockito::mock("POST", "/api/v3/order/test")
        .with_header("content-type", CONTENT_TYPE)
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("quoteOrderQty".to_string(), 0.002.to_string()),
            Matcher::UrlEncoded("recvWindow".to_string(), RECV_WINDOW.to_string()),
            Matcher::UrlEncoded("side".to_string(), "SELL".to_string()),
            Matcher::UrlEncoded("symbol".to_string(), "BNBBTC".to_string()),
            Matcher::Regex("timestamp=\\d+".to_string()),
            Matcher::UrlEncoded("type".to_string(), "MARKET".to_string()),
            Matcher::Regex("signature=.*".to_string()),
        ]))
        .with_body("{}")
        .create();

    let config = Config::default()
        .set_rest_api_endpoint(mockito::server_url())
        .set_recv_window(RECV_WINDOW);
    let account: Account = Binance::new_with_config(None, None, &config);
    let _ = env_logger::try_init();
    account
        .test_market_sell_using_quote_quantity("BNBBTC", 0.002)
        .unwrap();

    mock_test_market_sell_using_quote_quantity.assert();
}

#[test]
fn stop_limit_buy_order() {
    let mock_stop_limit_buy_order = mockito::mock("POST", "/api/v3/order")
        .with_header("content-type", CONTENT_TYPE)
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("price".to_string(), 0.1.to_string()),
            Matcher::UrlEncoded("quantity".to_string(), 1.to_string()),
            Matcher::UrlEncoded("recvWindow".to_string(), RECV_WINDOW.to_string()),
            Matcher::UrlEncoded("side".to_string(), "BUY".to_string()),
            Matcher::UrlEncoded("stopPrice".to_string(), 0.09.to_string()),
            Matcher::UrlEncoded("symbol".to_string(), "LTCBTC".to_string()),
            Matcher::UrlEncoded("timeInForce".to_string(), "GTC".to_string()),
            Matcher::Regex("timestamp=\\d+".to_string()),
            Matcher::UrlEncoded("type".to_string(), "STOP_LOSS_LIMIT".to_string()),
        ]))
        .with_body_from_file("tests/mocks/account/stop_limit_buy.json")
        .create();

    let config = Config::default()
        .set_rest_api_endpoint(mockito::server_url())
        .set_recv_window(RECV_WINDOW);
    let account: Account = Binance::new_with_config(None, None, &config);
    let _ = env_logger::try_init();
    let transaction: Transaction = account
        .stop_limit_buy_order("LTCBTC", 1, 0.1, 0.09, TimeInForce::GTC)
        .unwrap();

    mock_stop_limit_buy_order.assert();

    assert_eq!(transaction.symbol, "LTCBTC");
    assert_eq!(transaction.order_id, 1);
    assert_eq!(transaction.order_list_id.unwrap(), -1);
    assert_eq!(transaction.client_order_id, "6gCrw2kRUAF9CvJDGP16IP");
    assert_eq!(transaction.transact_time, 1507725176595);
    assert_approx_eq!(f64, transaction.price, 0.1, ulps = 2);
    assert_approx_eq!(f64, transaction.orig_qty, 1.0, ulps = 2);
    assert_approx_eq!(f64, transaction.executed_qty, 1.0, ulps = 2);
    assert_approx_eq!(f64, transaction.cummulative_quote_qty, 0.0, ulps = 2);
    assert_approx_eq!(f64, transaction.stop_price, 0.09, ulps = 2);
    assert_eq!(transaction.status, "NEW");
    assert_eq!(transaction.time_in_force, "GTC"); //Migrate to TimeInForce enum
    assert_eq!(transaction.type_name, "STOP_LOSS_LIMIT");
    assert_eq!(transaction.side, "BUY");
}

#[test]
fn test_stop_limit_buy_order() {
    let mock_test_stop_limit_buy_order = mockito::mock("POST", "/api/v3/order/test")
        .with_header("content-type", CONTENT_TYPE)
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("price".to_string(), 0.1.to_string()),
            Matcher::UrlEncoded("quantity".to_string(), 1.to_string()),
            Matcher::UrlEncoded("recvWindow".to_string(), RECV_WINDOW.to_string()),
            Matcher::UrlEncoded("side".to_string(), "BUY".to_string()),
            Matcher::UrlEncoded("stopPrice".to_string(), 0.09.to_string()),
            Matcher::UrlEncoded("symbol".to_string(), "LTCBTC".to_string()),
            Matcher::UrlEncoded("timeInForce".to_string(), "GTC".to_string()),
            Matcher::Regex("timestamp=\\d+".to_string()),
            Matcher::UrlEncoded("type".to_string(), "STOP_LOSS_LIMIT".to_string()),
        ]))
        .with_body("{}")
        .create();

    let config = Config::default()
        .set_rest_api_endpoint(mockito::server_url())
        .set_recv_window(RECV_WINDOW);
    let account: Account = Binance::new_with_config(None, None, &config);
    let _ = env_logger::try_init();
    account
        .test_stop_limit_buy_order("LTCBTC", 1, 0.1, 0.09, TimeInForce::GTC)
        .unwrap();

    mock_test_stop_limit_buy_order.assert();
}

#[test]
fn stop_limit_sell_order() {
    let symbol = "LTCBTC";
    let (mock, client) = setup_mock_from_file(
        "POST",
        "/api/v3/order",
        vec![
            Matcher::UrlEncoded("price".to_string(), 0.1.to_string()),
            Matcher::UrlEncoded("side".to_string(), "SELL".to_string()),
            Matcher::UrlEncoded("stopPrice".to_string(), 0.to_string()),
            Matcher::UrlEncoded("symbol".to_string(), symbol.to_string()),
            Matcher::UrlEncoded("timeInForce".to_string(), "GTC".to_string()),
            Matcher::Regex("timestamp=\\d+".to_string()),
            Matcher::UrlEncoded("type".to_string(), "STOP_LOSS_LIMIT".to_string()),
        ],
        "tests/mocks/account/stop_limit_sell.json",
    );

    let transaction = client
        .stop_limit_sell_order(symbol, 1, 0.1, 0.09, TimeInForce::GTC)
        .unwrap();

    mock.assert();

    assert_eq!(transaction.symbol, symbol);
    assert_eq!(transaction.order_id, 1);
    assert_eq!(transaction.order_list_id.unwrap(), -1);
    assert_eq!(transaction.client_order_id, "6gCrw2kRUAF9CvJDGP16IP");
    assert_eq!(transaction.transact_time, 1507725176595);
    assert_approx_eq!(f64, transaction.price, 0.1, ulps = 2);
    assert_approx_eq!(f64, transaction.orig_qty, 1.0, ulps = 2);
    assert_approx_eq!(f64, transaction.executed_qty, 1.0, ulps = 2);
    assert_approx_eq!(f64, transaction.cummulative_quote_qty, 0.0, ulps = 2);
    assert_approx_eq!(f64, transaction.stop_price, 0.09, ulps = 2);
    assert_eq!(transaction.status, "NEW");
    assert_eq!(transaction.time_in_force, "GTC"); //Migrate to TimeInForce enum
    assert_eq!(transaction.type_name, "STOP_LOSS_LIMIT");
    assert_eq!(transaction.side, "SELL");
}

#[test]
fn test_stop_limit_sell_order() {
    let symbol = "LTCBTC";
    let stop_price = 0.09;
    let mock = mockito::mock("POST", "/api/v3/order/test")
        .with_header("content-type", CONTENT_TYPE)
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("recvWindow".to_string(), 1234.to_string()),
            Matcher::UrlEncoded("price".to_string(), 0.1.to_string()),
            Matcher::UrlEncoded("side".to_string(), "SELL".to_string()),
            Matcher::UrlEncoded("stopPrice".to_string(), stop_price.to_string()),
            Matcher::UrlEncoded("symbol".to_string(), symbol.to_string()),
            Matcher::UrlEncoded("timeInForce".to_string(), "GTC".to_string()),
            Matcher::Regex("timestamp=\\d+".to_string()),
            Matcher::UrlEncoded("type".to_string(), "STOP_LOSS_LIMIT".to_string()),
        ]))
        .with_body("{}")
        .create();

    let config = Config::default()
        .set_rest_api_endpoint(mockito::server_url())
        .set_recv_window(RECV_WINDOW);
    let client: Account = Binance::new_with_config(None, None, &config);
    client
        .test_stop_limit_sell_order(symbol, 1, 0.1, stop_price, TimeInForce::GTC)
        .unwrap();

    mock.assert();
}

#[test]
fn custom_order() {
    let symbol = "LTCBTC";
    let (mock, client) = TestBuilder::new(
        "POST",
        "/api/v3/order",
        vec![
            Matcher::UrlEncoded(
                "newClientOrderId".to_string(),
                "6gCrw2kRUAF9CvJDGP16IP".to_string(),
            ),
            Matcher::UrlEncoded("price".to_string(), 0.1.to_string()),
            Matcher::UrlEncoded("quantity".to_string(), 1.to_string()),
            Matcher::UrlEncoded("side".to_string(), "BUY".to_string()),
            Matcher::UrlEncoded("symbol".to_string(), symbol.to_string()),
            Matcher::UrlEncoded("timeInForce".to_string(), "GTC".to_string()),
            Matcher::Regex("timestamp=\\d+".to_string()),
            Matcher::UrlEncoded("type".to_string(), "MARKET".to_string()),
        ],
    )
    .with_body_from_file("tests/mocks/account/stop_limit_sell.json");

    let transaction = client
        .custom_order(
            symbol,
            1,
            0.1,
            None,
            OrderSide::Buy,
            OrderType::Market,
            TimeInForce::GTC,
            Some("6gCrw2kRUAF9CvJDGP16IP".into()),
        )
        .unwrap();

    mock.assert();

    assert_eq!(transaction.symbol, symbol);
    assert_eq!(transaction.order_id, 1);
    assert_eq!(transaction.order_list_id.unwrap(), -1);
    assert_eq!(transaction.client_order_id, "6gCrw2kRUAF9CvJDGP16IP");
    assert_eq!(transaction.transact_time, 1507725176595);
    assert_approx_eq!(f64, transaction.price, 0.1, ulps = 2);
    assert_approx_eq!(f64, transaction.orig_qty, 1.0, ulps = 2);
    assert_approx_eq!(f64, transaction.executed_qty, 1.0, ulps = 2);
    assert_approx_eq!(f64, transaction.cummulative_quote_qty, 0.0, ulps = 2);
    assert_approx_eq!(f64, transaction.stop_price, 0.09, ulps = 2);
    assert_eq!(transaction.status, "NEW");
    assert_eq!(transaction.time_in_force, "GTC"); //Migrate to TimeInForce enum
    assert_eq!(transaction.type_name, "STOP_LOSS_LIMIT");
    assert_eq!(transaction.side, "SELL");
}

#[test]
fn test_custom_order() {
    let symbol = "LTCBTC";
    let (mock, client) = TestBuilder::new(
        "POST",
        "/api/v3/order/test",
        vec![
            Matcher::UrlEncoded("price".to_string(), 0.1.to_string()),
            Matcher::UrlEncoded("quantity".to_string(), 1.to_string()),
            Matcher::UrlEncoded("side".to_string(), "BUY".to_string()),
            Matcher::UrlEncoded("symbol".to_string(), symbol.to_string()),
            Matcher::UrlEncoded("timeInForce".to_string(), "GTC".to_string()),
            Matcher::Regex("timestamp=\\d+".to_string()),
            Matcher::UrlEncoded("type".to_string(), "MARKET".to_string()),
        ],
    )
    .with_empty_body();

    client
        .test_custom_order(
            symbol,
            1,
            0.1,
            None,
            OrderSide::Buy,
            OrderType::Market,
            TimeInForce::GTC,
            None,
        )
        .unwrap();

    mock.assert();
}

#[test]
fn cancel_order() {
    let symbol = "BTCUSDT";
    let (mock, client) = TestBuilder::new(
        "DELETE",
        "/api/v3/order",
        vec![
            Matcher::UrlEncoded("orderId".to_string(), 1.to_string()),
            Matcher::UrlEncoded("symbol".to_string(), symbol.to_string()),
            Matcher::Regex("timestamp=\\d+".to_string()),
        ],
    )
    .with_body_from_file("tests/mocks/account/cancel_order.json");

    let cancelled_order = client.cancel_order(symbol, 1).unwrap();

    mock.assert();

    assert_eq!(cancelled_order.symbol, symbol);
    assert_eq!(cancelled_order.orig_client_order_id.unwrap(), "myOrder1");
    assert_eq!(cancelled_order.order_id.unwrap(), 4);
    assert_eq!(cancelled_order.client_order_id.unwrap(), "cancelMyOrder1");
}

#[test]
fn test_cancel_order() {
    let symbol = "BTCUSDT";
    let (mock, client) = TestBuilder::new(
        "DELETE",
        "/api/v3/order/test",
        vec![
            Matcher::UrlEncoded("orderId".to_string(), 1.to_string()),
            Matcher::UrlEncoded("symbol".to_string(), symbol.to_string()),
            Matcher::Regex("timestamp=\\d+".to_string()),
        ],
    )
    .with_body_from_file("tests/mocks/account/cancel_order.json");

    client.test_cancel_order(symbol, 1).unwrap();

    mock.assert();
}

#[test]
fn trade_history() {
    let symbol = "BTCUSDT";
    let (mock, client) = setup_mock_from_file(
        "GET",
        "/api/v3/myTrades",
        vec![
            Matcher::UrlEncoded("symbol".to_string(), symbol.to_string()),
            Matcher::Regex("timestamp=\\d+".to_string()),
        ],
        "tests/mocks/account/trade_history.json",
    );

    let histories = client.trade_history(symbol).unwrap();

    mock.assert();

    assert!(histories.len() == 1);

    let history = histories[0].clone();

    assert_eq!(history.id, 28457);
    assert_approx_eq!(f64, history.price, 4.00000100, ulps = 2);
    assert_approx_eq!(f64, history.qty, 12.00000000, ulps = 2);
    assert_eq!(history.commission, "10.10000000");
    assert_eq!(history.commission_asset, "BNB");
    assert_eq!(history.time, 1499865549590);
    assert!(history.is_buyer);
    assert!(!history.is_maker);
    assert!(history.is_best_match);
}
