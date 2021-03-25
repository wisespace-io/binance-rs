use hex::encode as hex_encode;
use hmac::{Hmac, Mac, NewMac};
use crate::errors::*;
use reqwest::StatusCode;
use reqwest::blocking::Response;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, USER_AGENT, CONTENT_TYPE};
use sha2::Sha256;
use crate::api::API;

#[derive(Clone)]
pub struct Client {
    api_key: String,
    secret_key: String,
    host: String,
    inner_client: reqwest::blocking::Client,
}

impl Client {
    pub fn new(api_key: Option<String>, secret_key: Option<String>, host: String) -> Self {
        Client {
            api_key: api_key.unwrap_or_else(|| "".into()),
            secret_key: secret_key.unwrap_or_else(|| "".into()),
            host,
            inner_client: reqwest::blocking::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .unwrap(),
        }
    }

    pub fn get_signed(&self, endpoint: API, request: &str) -> Result<String> {
        let url = self.sign_request(endpoint, request);
        let client = &self.inner_client;
        let response = client
            .get(url.as_str())
            .headers(self.build_headers(true)?)
            .send()?;

        self.handler(response)
    }

    pub fn post_signed(&self, endpoint: API, request: &str) -> Result<String> {
        let url = self.sign_request(endpoint, request);
        let client = &self.inner_client;
        let response = client
            .post(url.as_str())
            .headers(self.build_headers(true)?)
            .send()?;

        self.handler(response)
    }

    pub fn delete_signed(&self, endpoint: API, request: &str) -> Result<String> {
        let url = self.sign_request(endpoint, request);
        let client = &self.inner_client;
        let response = client
            .delete(url.as_str())
            .headers(self.build_headers(true)?)
            .send()?;

        self.handler(response)
    }

    pub fn get(&self, endpoint: API, request: &str) -> Result<String> {
        let mut url: String = format!("{}{}", self.host, String::from(endpoint));
        if !request.is_empty() {
            url.push_str(format!("?{}", request).as_str());
        }

        let client = &self.inner_client;
        let response = client.get(url.as_str()).send()?;

        self.handler(response)
    }

    pub fn post(&self, endpoint: API) -> Result<String> {
        let url: String = format!("{}{}", self.host, String::from(endpoint));

        let client = &self.inner_client;
        let response = client
            .post(url.as_str())
            .headers(self.build_headers(false)?)
            .send()?;

        self.handler(response)
    }

    pub fn put(&self, endpoint: API, listen_key: &str) -> Result<String> {
        let url: String = format!("{}{}", self.host, String::from(endpoint));
        let data: String = format!("listenKey={}", listen_key);

        let client = &self.inner_client;
        let response = client
            .put(url.as_str())
            .headers(self.build_headers(false)?)
            .body(data)
            .send()?;

        self.handler(response)
    }

    pub fn delete(&self, endpoint: API, listen_key: &str) -> Result<String> {
        let url: String = format!("{}{}", self.host, String::from(endpoint));
        let data: String = format!("listenKey={}", listen_key);

        let client = &self.inner_client;
        let response = client
            .delete(url.as_str())
            .headers(self.build_headers(false)?)
            .body(data)
            .send()?;

        self.handler(response)
    }

    // Request must be signed
    fn sign_request(&self, endpoint: API, request: &str) -> String {
        let mut signed_key = Hmac::<Sha256>::new_varkey(self.secret_key.as_bytes()).unwrap();
        signed_key.update(request.as_bytes());
        let signature = hex_encode(signed_key.finalize().into_bytes());
        let request_body: String = format!("{}&signature={}", request, signature);
        let url: String = format!("{}{}?{}", self.host, String::from(endpoint), request_body);

        url
    }

    fn build_headers(&self, content_type: bool) -> Result<HeaderMap> {
        let mut custom_headers = HeaderMap::new();

        custom_headers.insert(USER_AGENT, HeaderValue::from_static("binance-rs"));
        if content_type {
            custom_headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            );
        }
        custom_headers.insert(
            HeaderName::from_static("x-mbx-apikey"),
            HeaderValue::from_str(self.api_key.as_str())?,
        );

        Ok(custom_headers)
    }

    fn handler(&self, response: Response) -> Result<String> {
        match response.status() {
            StatusCode::OK => {
                Ok(response.text()?)
            }
            StatusCode::INTERNAL_SERVER_ERROR => {
                bail!("Internal Server Error");
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                bail!("Service Unavailable");
            }
            StatusCode::UNAUTHORIZED => {
                bail!("Unauthorized");
            }
            StatusCode::BAD_REQUEST => {
                let error: BinanceContentError = response.json()?;

                Err(ErrorKind::BinanceError(error).into())
            }
            s => {
                bail!(format!("Received response: {:?}", s));
            }
        }
    }
}
