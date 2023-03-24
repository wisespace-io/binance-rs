use super::Request;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize)]
pub struct PingRequest {}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub struct PingResponse {}

impl Request for PingRequest {
    const ENDPOINT: &'static str = "/api/v3/ping";
    const METHOD: Method = Method::GET;
    type Response = PingResponse;
}
