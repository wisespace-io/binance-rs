use hex::encode as hex_encode;
use errors::*;
use reqwest;
use reqwest::{Response, StatusCode};
use reqwest::header::{ContentType, Headers, UserAgent};
use std::io::Read;
use ring::{digest, hmac};

static API1_HOST: &'static str = "https://www.binance.com";

#[derive(Clone)]
pub struct Client {
    api_key: String,
    secret_key: String,
}

impl Client {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Client {
            api_key: api_key.unwrap_or_else(|| "".into()),
            secret_key: secret_key.unwrap_or_else(|| "".into()),
        }
    }

    pub fn get_signed(&self, endpoint: &str, request: &str) -> Result<(String)> {
        let url = self.sign_request(endpoint, request);
        let client = reqwest::Client::new();
        let response = client
            .get(url.as_str())
            .headers(self.build_headers(true))
            .send()?;

        self.handler(response)
    }

    pub fn post_signed(&self, endpoint: &str, request: &str) -> Result<(String)> {
        let url = self.sign_request(endpoint, request);
        let client = reqwest::Client::new();
        let response = client
            .post(url.as_str())
            .headers(self.build_headers(true))
            .send()?;

        self.handler(response)
    }

    pub fn delete_signed(&self, endpoint: &str, request: &str) -> Result<(String)> {
        let url = self.sign_request(endpoint, request);
        let client = reqwest::Client::new();
        let response = client
            .delete(url.as_str())
            .headers(self.build_headers(true))
            .send()?;

        self.handler(response)
    }

    pub fn get(&self, endpoint: &str, request: &str) -> Result<(String)> {
        let mut url: String = format!("{}{}", API1_HOST, endpoint);
        if !request.is_empty() {
            url.push_str(format!("?{}", request).as_str());
        }

        let response = reqwest::get(url.as_str())?;

        self.handler(response)
    }

    pub fn post(&self, endpoint: &str) -> Result<(String)> {
        let url: String = format!("{}{}", API1_HOST, endpoint);

        let client = reqwest::Client::new();
        let response = client
            .post(url.as_str())
            .headers(self.build_headers(false))
            .send()?;

        self.handler(response)
    }

    pub fn put(&self, endpoint: &str, listen_key: &str) -> Result<(String)> {
        let url: String = format!("{}{}", API1_HOST, endpoint);
        let data: String = format!("listenKey={}", listen_key);

        let client = reqwest::Client::new();
        let response = client
            .put(url.as_str())
            .headers(self.build_headers(false))
            .body(data)
            .send()?;

        self.handler(response)
    }

    pub fn delete(&self, endpoint: &str, listen_key: &str) -> Result<(String)> {
        let url: String = format!("{}{}", API1_HOST, endpoint);
        let data: String = format!("listenKey={}", listen_key);

        let client = reqwest::Client::new();
        let response = client
            .delete(url.as_str())
            .headers(self.build_headers(false))
            .body(data)
            .send()?;

        self.handler(response)
    }

    // Request must be signed
    fn sign_request(&self, endpoint: &str, request: &str) -> String {
        let signed_key = hmac::SigningKey::new(&digest::SHA256, self.secret_key.as_bytes());
        let signature = hex_encode(hmac::sign(&signed_key, request.as_bytes()).as_ref());

        let request_body: String = format!("{}&signature={}", request, signature);
        let url: String = format!("{}{}?{}", API1_HOST, endpoint, request_body);

        url
    }

    fn build_headers(&self, content_type: bool) -> Headers {
        let mut custon_headers = Headers::new();

        custon_headers.set(UserAgent::new("binance-rs"));
        if content_type {
            custon_headers.set(ContentType::form_url_encoded());
        }
        custon_headers.set_raw("X-MBX-APIKEY", self.api_key.as_str());

        custon_headers
    }

    fn handler(&self, mut response: Response) -> Result<(String)> {
        match response.status() {
            StatusCode::Ok => {
                let mut body = String::new();
                response.read_to_string(&mut body)?;
                Ok(body)
            }
            StatusCode::InternalServerError => {
                bail!("Internal Server Error");
            }
            StatusCode::ServiceUnavailable => {
                bail!("Service Unavailable");
            }
            StatusCode::Unauthorized => {
                bail!("Unauthorized");
            }
            StatusCode::BadRequest => {
                bail!(format!("Bad Request: {:?}", response));
            }
            s => {
                bail!(format!("Received response: {:?}", s));
            }
        }
    }
}
