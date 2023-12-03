use binance::api::*;
use binance::config::*;
use binance::futures::general::*;

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[test]
    fn ping() {
        let mut server = Server::new();
        let mock_ping = server.mock("GET", "/fapi/v1/ping")
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body("{}")
            .create();

        let config = Config::default().set_futures_rest_api_endpoint(server.url());
        println!("{}", server.url());
        let general: FuturesGeneral = Binance::new_with_config(None, None, &config);

        let pong = general.ping().unwrap();
        mock_ping.assert();

        assert_eq!(pong, "pong");
    }
}