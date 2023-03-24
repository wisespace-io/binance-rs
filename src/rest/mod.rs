mod account;
mod general;
mod market;
mod userstream;
mod websocket;

pub use self::{
    account::{GetAccountRequest, OrderRequest},
    market::PingRequest,
};
use crate::{
    config::Config,
    error::{BinanceError, BinanceResponse},
};
use anyhow::Error;
use chrono::Utc;
use fehler::{throw, throws};
use hex::encode as hexify;
use hmac::{Hmac, Mac};
use log::trace;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT},
    Client, Method, Response,
};
use serde::{de::DeserializeOwned, Serialize};
use sha2::Sha256;

static RECV_WINDOW: usize = 5000;

pub trait Request: Serialize {
    const ENDPOINT: &'static str;
    const METHOD: Method;
    const SIGNED: bool = false;
    type Response: DeserializeOwned;
}

#[derive(Clone)]
pub struct Binance {
    credential: Option<(String, String)>,
    client: Client,
    config: Config,
    pub recv_window: usize,
}

impl Binance {
    pub fn new() -> Self {
        Binance {
            credential: None,
            client: Client::new(),
            config: Config::default(),
            recv_window: RECV_WINDOW,
        }
    }

    pub fn with_credential(api_key: &str, api_secret: &str) -> Self {
        Binance {
            client: Client::new(),
            credential: Some((api_key.into(), api_secret.into())),
            config: Config::default(),
            recv_window: RECV_WINDOW,
        }
    }

    #[throws(Error)]
    pub async fn request<R>(&self, req: R) -> R::Response
    where
        R: Request,
    {
        let mut params = if matches!(R::METHOD, Method::GET) {
            serde_qs::to_string(&req)?
        } else {
            String::new()
        };

        let body = if !matches!(R::METHOD, Method::GET) {
            serde_qs::to_string(&req)?
        } else {
            String::new()
        };

        if R::SIGNED {
            if !params.is_empty() {
                params.push('&');
            }
            params.push_str(&format!("timestamp={}", Utc::now().timestamp_millis()));
            params.push_str(&format!("&recvWindow={}", self.recv_window));

            let signature = self.signature(&params, &body)?;
            params.push_str(&format!("&signature={}", signature));
        }

        let path = R::ENDPOINT.to_string();

        let base = &self.config.rest_api_endpoint;
        let url = format!("{base}{path}?{params}");

        let mut custom_headers = HeaderMap::new();
        custom_headers.insert(USER_AGENT, HeaderValue::from_static("binance-async-rs"));
        if !body.is_empty() {
            custom_headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            );
        }
        if let Ok((key, _)) = self.check_key() {
            // This is for user stream: user stream requests need api key in the header but no signature. WEIRD
            custom_headers.insert(
                HeaderName::from_static("x-mbx-apikey"),
                HeaderValue::from_str(key)?,
            );
        }

        let resp = self
            .client
            .request(R::METHOD, url.as_str())
            .headers(custom_headers)
            .body(body)
            .send()
            .await?;

        self.handle_response(resp).await?
    }

    #[throws(Error)]
    fn check_key(&self) -> (&str, &str) {
        match self.credential.as_ref() {
            None => throw!(BinanceError::NoApiKeySet),
            Some((k, s)) => (&**k, &**s),
        }
    }

    #[throws(Error)]
    fn signature(&self, params: &str, body: &str) -> String {
        let (_, secret) = self.check_key()?;
        // Signature: hex(HMAC_SHA256(queries + data))
        let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
        let sign_message = format!("{}{}", params, body);
        trace!("Sign message: {}", sign_message);
        mac.update(sign_message.as_bytes());
        let signature = hexify(mac.finalize().into_bytes());
        signature
    }

    #[throws(Error)]
    async fn handle_response<O: DeserializeOwned>(&self, resp: Response) -> O {
        // use serde_json::from_str;
        // let body = resp.text().await?;
        // println!("{body}");
        // let resp: BinanceResponse<O> = from_str(&body)?;
        let resp: BinanceResponse<O> = resp.json().await?;
        resp.to_result()?
    }
}

#[cfg(test)]
mod test {
    use super::Binance;
    use anyhow::Error;
    use fehler::throws;
    use url::{form_urlencoded::Serializer, Url};

    #[throws(Error)]
    #[test]
    fn signature_query() {
        let tr = Binance::with_credential(
            "vmPUZE6mv9SD5VNHk4HlWFsOr6aKE2zvsw0MuIgwCIPy6utIco14y7Ju91duEh8A",
            "NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j",
        );
        let sig = tr.signature(
            &Url::parse_with_params(
                "http://a.com/api/v1/test",
                &[
                    ("symbol", "LTCBTC"),
                    ("side", "BUY"),
                    ("type", "LIMIT"),
                    ("timeInForce", "GTC"),
                    ("quantity", "1"),
                    ("price", "0.1"),
                    ("recvWindow", "5000"),
                    ("timestamp", "1499827319559"),
                ],
            )?
            .query()
            .unwrap_or_default(),
            "",
        )?;
        assert_eq!(
            sig,
            "c8db56825ae71d6d79447849e617115f4a920fa2acdcab2b053c4b2838bd6b71"
        );
    }

    #[throws(Error)]
    #[test]
    fn signature_body() {
        let tr = Binance::with_credential(
            "vmPUZE6mv9SD5VNHk4HlWFsOr6aKE2zvsw0MuIgwCIPy6utIco14y7Ju91duEh8A",
            "NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j",
        );
        let mut s = Serializer::new(String::new());
        s.extend_pairs(&[
            ("symbol", "LTCBTC"),
            ("side", "BUY"),
            ("type", "LIMIT"),
            ("timeInForce", "GTC"),
            ("quantity", "1"),
            ("price", "0.1"),
            ("recvWindow", "5000"),
            ("timestamp", "1499827319559"),
        ]);

        let sig = tr.signature(
            &Url::parse("http://a.com/api/v1/test")?
                .query()
                .unwrap_or_default(),
            &s.finish(),
        )?;
        assert_eq!(
            sig,
            "c8db56825ae71d6d79447849e617115f4a920fa2acdcab2b053c4b2838bd6b71"
        );
    }

    #[throws(Error)]
    #[test]
    fn signature_query_body() {
        let tr = Binance::with_credential(
            "vmPUZE6mv9SD5VNHk4HlWFsOr6aKE2zvsw0MuIgwCIPy6utIco14y7Ju91duEh8A",
            "NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j",
        );

        let mut s = Serializer::new(String::new());
        s.extend_pairs(&[
            ("quantity", "1"),
            ("price", "0.1"),
            ("recvWindow", "5000"),
            ("timestamp", "1499827319559"),
        ]);

        let sig = tr.signature(
            &Url::parse_with_params(
                "http://a.com/api/v1/order",
                &[
                    ("symbol", "LTCBTC"),
                    ("side", "BUY"),
                    ("type", "LIMIT"),
                    ("timeInForce", "GTC"),
                ],
            )?
            .query()
            .unwrap_or_default(),
            &s.finish(),
        )?;
        assert_eq!(
            sig,
            "0fd168b8ddb4876a0358a8d14d0c9f3da0e9b20c5d52b2a00fcf7d1c602f9a77"
        );
    }

    #[throws(Error)]
    #[test]
    fn signature_body2() {
        let tr = Binance::with_credential(
            "vj1e6h50pFN9CsXT5nsL25JkTuBHkKw3zJhsA6OPtruIRalm20vTuXqF3htCZeWW",
            "5Cjj09rLKWNVe7fSalqgpilh5I3y6pPplhOukZChkusLqqi9mQyFk34kJJBTdlEJ",
        );

        let q = &mut [
            ("symbol", "ETHBTC"),
            ("side", "BUY"),
            ("type", "LIMIT"),
            ("timeInForce", "GTC"),
            ("quantity", "1"),
            ("price", "0.1"),
            ("recvWindow", "5000"),
            ("timestamp", "1540687064555"),
        ];
        q.sort();
        let q: Vec<_> = q.into_iter().map(|(k, v)| format!("{}={}", k, v)).collect();
        let q = q.join("&");
        let sig = tr.signature(
            &Url::parse("http://a.com/api/v1/test")?
                .query()
                .unwrap_or_default(),
            &q,
        )?;
        assert_eq!(
            sig,
            "1ee5a75760b9496a2144a22116e02bc0b7fdcf828781fa87ca273540dfcf2cb0"
        );
    }
}
