use std::{path::Path, marker::PhantomData};

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
    client_type: PhantomData<B>,
}

impl<B> Builder<B>
where
    B: Binance,
{
    pub fn new<P>(method: &str, path: P, extra_query_matchers: Vec<Matcher>) -> Self
    where
        P: Into<Matcher>,
    {
        let mut query_matchers = vec![Matcher::UrlEncoded(
            "recvWindow".to_string(),
            RECV_WINDOW.to_string(),
        )];
        query_matchers.extend(extra_query_matchers);

        let mock = mockito::mock(method, path)
            .with_header("content-type", CONTENT_TYPE)
            .match_query(Matcher::AllOf(query_matchers));

        Self {
            mock,
            client_type: PhantomData::default(),
        }
    }

    fn setup(&mut self) -> B {
        let _ = env_logger::try_init();
        let config = Config::default()
            .set_rest_api_endpoint(mockito::server_url())
            .set_recv_window(RECV_WINDOW);

        Binance::new_with_config(None, None, &config)
    }

    pub fn with_body_from_file(mut self, path: impl AsRef<Path>) -> (Mock, B)
    where
        B: Binance,
    {
        let client = self.setup();
        let mock = self.mock.with_body_from_file(path).create();

        (mock, client)
    }

    pub fn with_empty_body(mut self) -> (Mock, B) {
        let client = self.setup();
        let mock = self.mock.with_body("{}").create();

        (mock, client)
    }
}
