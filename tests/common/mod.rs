use std::path::Path;

use binance::api::Binance;
use binance::config::Config;

use mockito::{self, Matcher, Mock};

const CONTENT_TYPE: &str = "application/json;charset=UTF-8";
const RECV_WINDOW: u64 = 1234;

pub struct Builder<B>
where
    B: Binance,
{
    mock: Mock,
    client: B,
}

impl<B> Builder<B>
where
    B: Binance,
{
    pub fn new<P>(method: &str, path: P, extra_query_matchers: Vec<Matcher>) -> Self
    where
        P: Into<Matcher>,
    {
        let _ = env_logger::try_init();

        let mut query_matchers = vec![Matcher::UrlEncoded(
            "recvWindow".to_string(),
            RECV_WINDOW.to_string(),
        )];
        query_matchers.extend(extra_query_matchers);

        let mock = mockito::mock(method, path)
            .with_header("content-type", CONTENT_TYPE)
            .match_query(Matcher::AllOf(query_matchers));

        let config = Config::default()
            .set_rest_api_endpoint(mockito::server_url())
            .set_recv_window(RECV_WINDOW);

        let client = Binance::new_with_config(None, None, &config);

        Self { mock, client }
    }

    pub fn with_body_from_file(self, path: impl AsRef<Path>) -> (Mock, B)
    where
        B: Binance,
    {
        let mock = self.mock.with_body_from_file(path).create();

        (mock, self.client)
    }

    pub fn with_empty_body(self) -> (Mock, B) {
        let mock = self.mock.with_body("{}").create();

        (mock, self.client)
    }
}
