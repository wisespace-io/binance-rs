use chrono::Utc;
use error::{BinanceError, BinanceResponse};
use failure::{Error, Fallible};
use futures::{Future, Stream};
use hex::encode as hexify;
use hmac::{Hmac, Mac};
use hyper::client::{HttpConnector, ResponseFuture};
use hyper::{Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{from_slice, to_string, to_value};
use sha2::Sha256;
use url::Url;

static BASE: &'static str = "https://www.binance.com";
// static BASE: &'static str = "http://requestbin.fullcontact.com/199a3mf1";
static RECV_WINDOW: usize = 5000;

#[derive(Clone)]
pub struct Transport {
    credential: Option<(String, String)>,
    client: Client<HttpsConnector<HttpConnector>>,
    pub recv_window: usize,
}

impl Transport {
    pub fn new() -> Self {
        let https = HttpsConnector::new(4).unwrap();
        let client = Client::builder().build::<_, Body>(https);

        Transport {
            credential: None,
            client: client,
            recv_window: RECV_WINDOW,
        }
    }

    pub fn with_credential(api_key: &str, api_secret: &str) -> Self {
        let https = HttpsConnector::new(4).unwrap();
        let client = Client::builder().build::<_, Body>(https);

        Transport {
            client: client,
            credential: Some((api_key.into(), api_secret.into())),
            recv_window: RECV_WINDOW,
        }
    }

    pub fn get<O, Q>(
        &self,
        endpoint: &str,
        params: Option<Q>,
    ) -> Fallible<impl Future<Item = O, Error = Error>>
    where
        O: DeserializeOwned,
        Q: Serialize,
    {
        self.request::<_, _, ()>(Method::GET, endpoint, params, None)
    }

    pub fn post<O, D>(
        &self,
        endpoint: &str,
        data: Option<D>,
    ) -> Fallible<impl Future<Item = O, Error = Error>>
    where
        O: DeserializeOwned,
        D: Serialize,
    {
        self.request::<_, (), _>(Method::POST, endpoint, None, data)
    }

    pub fn put<O, D>(
        &self,
        endpoint: &str,
        data: Option<D>,
    ) -> Fallible<impl Future<Item = O, Error = Error>>
    where
        O: DeserializeOwned,
        D: Serialize,
    {
        self.request::<_, (), _>(Method::PUT, endpoint, None, data)
    }

    pub fn delete<O, Q>(
        &self,
        endpoint: &str,
        params: Option<Q>,
    ) -> Fallible<impl Future<Item = O, Error = Error>>
    where
        O: DeserializeOwned,
        Q: Serialize,
    {
        self.request::<_, _, ()>(Method::DELETE, endpoint, params, None)
    }

    pub fn signed_get<O, Q>(
        &self,
        endpoint: &str,
        params: Option<Q>,
    ) -> Fallible<impl Future<Item = O, Error = Error>>
    where
        O: DeserializeOwned,
        Q: Serialize,
    {
        self.signed_request::<_, _, ()>(Method::GET, endpoint, params, None)
    }

    pub fn signed_post<O, D>(
        &self,
        endpoint: &str,
        data: Option<D>,
    ) -> Fallible<impl Future<Item = O, Error = Error>>
    where
        O: DeserializeOwned,
        D: Serialize,
    {
        self.signed_request::<_, (), _>(Method::POST, endpoint, None, data)
    }

    pub fn signed_put<O, Q>(
        &self,
        endpoint: &str,
        params: Option<Q>,
    ) -> Fallible<impl Future<Item = O, Error = Error>>
    where
        O: DeserializeOwned,
        Q: Serialize,
    {
        self.signed_request::<_, _, ()>(Method::PUT, endpoint, params, None)
    }

    pub fn signed_delete<O, Q>(
        &self,
        endpoint: &str,
        params: Option<Q>,
    ) -> Fallible<impl Future<Item = O, Error = Error>>
    where
        O: DeserializeOwned,
        Q: Serialize,
    {
        self.signed_request::<_, _, ()>(Method::DELETE, endpoint, params, None)
    }

    pub fn request<O, Q, D>(
        &self,
        method: Method,
        endpoint: &str,
        params: Option<Q>,
        data: Option<D>,
    ) -> Fallible<impl Future<Item = O, Error = Error>>
    where
        O: DeserializeOwned,
        Q: Serialize,
        D: Serialize,
    {
        let url = format!("{}{}", BASE, endpoint);
        let url = match params {
            Some(p) => Url::parse_with_params(&url, p.to_url_query())?,
            None => Url::parse(&url)?,
        };

        let body = match data {
            Some(data) => data.to_url_query_string(),
            None => "".to_string(),
        };

        let mut req = Request::builder();
        req.method(method)
            .uri(url.as_str())
            .header("user-agent", "binance-rs")
            .header("content-type", "application/x-www-form-urlencoded");

        if let Ok((key, _)) = self.check_key() {
            // This is for user stream: user stream requests need api key in the header but no signature. WEIRD
            req.header("X-MBX-APIKEY", key);
        }

        let req = req.body(Body::from(body))?;

        // let req = Request::builder()
        //     .method(method)
        //     .uri(url.as_str())
        //     .header("user-agent", "binance-rs")
        //     .header("content-type", "application/x-www-form-urlencoded")
        //     .body(Body::from(body))?;
        Ok(self.handle_response(self.client.request(req)))
    }

    pub fn signed_request<O, Q, D>(
        &self,
        method: Method,
        endpoint: &str,
        params: Option<Q>,
        data: Option<D>,
    ) -> Fallible<impl Future<Item = O, Error = Error>>
    where
        O: DeserializeOwned,
        Q: Serialize,
        D: Serialize,
    {
        let query = params.map(|q| q.to_url_query()).unwrap_or_else(|| vec![]);
        let url = format!("{}{}", BASE, endpoint);
        let mut url = Url::parse_with_params(&url, &query)?;
        url.query_pairs_mut()
            .append_pair("timestamp", &Utc::now().timestamp_millis().to_string());
        url.query_pairs_mut()
            .append_pair("recvWindow", &self.recv_window.to_string());

        let body = data
            .map(|data| data.to_url_query_string())
            .unwrap_or_else(|| "".to_string());

        let (key, signature) = self.signature(&url, &body)?;
        url.query_pairs_mut().append_pair("signature", &signature);

        let req = Request::builder()
            .method(method)
            .uri(url.as_str())
            .header("user-agent", "binance-rs")
            .header("X-MBX-APIKEY", key)
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from(body))?;

        Ok(self.handle_response(self.client.request(req)))
    }

    fn check_key(&self) -> Fallible<(&str, &str)> {
        match self.credential.as_ref() {
            None => Err(BinanceError::NoApiKeySet)?,
            Some((k, s)) => Ok((k, s)),
        }
    }

    pub(self) fn signature(&self, url: &Url, body: &str) -> Fallible<(&str, String)> {
        let (key, secret) = self.check_key()?;
        // Signature: hex(HMAC_SHA256(queries + data))
        let mut mac = Hmac::<Sha256>::new_varkey(secret.as_bytes()).unwrap();
        let sign_message = match url.query() {
            Some(query) => format!("{}{}", query, body),
            None => format!("{}", body),
        };
        trace!("Sign message: {}", sign_message);
        mac.input(sign_message.as_bytes());
        let signature = hexify(mac.result().code());
        Ok((key, signature))
    }

    fn handle_response<O: DeserializeOwned>(
        &self,
        fut: ResponseFuture,
    ) -> impl Future<Item = O, Error = Error> {
        fut.from_err::<Error>()
            .and_then(|resp| resp.into_body().concat2().from_err::<Error>())
            .map(|chunk| {
                trace!("{}", String::from_utf8_lossy(&*chunk));
                chunk
            })
            .and_then(|chunk| Ok(from_slice(&chunk)?))
            .and_then(|resp: BinanceResponse<O>| Ok(resp.to_result()?))
    }
}

trait ToUrlQuery: Serialize {
    fn to_url_query_string(&self) -> String {
        let vec = self.to_url_query();

        let s = vec
            .into_iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");
        s
    }

    fn to_url_query(&self) -> Vec<(String, String)> {
        let v = to_value(self).unwrap();
        let v = v.as_object().unwrap();
        let mut vec = vec![];

        for (key, value) in v.into_iter() {
            if value.is_null() {
                continue;
            } else if value.is_string() {
                vec.push((key.clone(), value.as_str().unwrap().to_string()))
            } else {
                vec.push((key.clone(), to_string(value).unwrap()))
            }
        }

        vec
    }
}

impl<S: Serialize> ToUrlQuery for S {}

#[cfg(test)]
mod test {
    use super::Transport;
    use failure::Fallible;
    use url::form_urlencoded::Serializer;
    use url::Url;

    #[test]
    fn signature_query() -> Fallible<()> {
        let tr = Transport::with_credential(
            "vmPUZE6mv9SD5VNHk4HlWFsOr6aKE2zvsw0MuIgwCIPy6utIco14y7Ju91duEh8A",
            "NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j",
        );
        let (_, sig) = tr.signature(
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
            )?,
            "",
        )?;
        assert_eq!(
            sig,
            "c8db56825ae71d6d79447849e617115f4a920fa2acdcab2b053c4b2838bd6b71"
        );
        Ok(())
    }

    #[test]
    fn signature_body() -> Fallible<()> {
        let tr = Transport::with_credential(
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

        let (_, sig) = tr.signature(&Url::parse("http://a.com/api/v1/test")?, &s.finish())?;
        assert_eq!(
            sig,
            "c8db56825ae71d6d79447849e617115f4a920fa2acdcab2b053c4b2838bd6b71"
        );
        Ok(())
    }

    #[test]
    fn signature_query_body() -> Fallible<()> {
        let tr = Transport::with_credential(
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

        let (_, sig) = tr.signature(
            &Url::parse_with_params(
                "http://a.com/api/v1/order",
                &[
                    ("symbol", "LTCBTC"),
                    ("side", "BUY"),
                    ("type", "LIMIT"),
                    ("timeInForce", "GTC"),
                ],
            )?,
            &s.finish(),
        )?;
        assert_eq!(
            sig,
            "0fd168b8ddb4876a0358a8d14d0c9f3da0e9b20c5d52b2a00fcf7d1c602f9a77"
        );
        Ok(())
    }

    #[test]
    fn signature_body2() -> Fallible<()> {
        let tr = Transport::with_credential(
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
        let (_, sig) = tr.signature(&Url::parse("http://a.com/api/v1/test")?, &q)?;
        assert_eq!(
            sig,
            "1ee5a75760b9496a2144a22116e02bc0b7fdcf828781fa87ca273540dfcf2cb0"
        );
        Ok(())
    }
}
