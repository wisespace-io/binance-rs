use model::*;
use client::*;
use errors::*;

use serde_json::from_str;

#[derive(Clone)]
pub struct General {
    pub client: Client,
}

impl General {
    // Test connectivity
    pub fn ping(&self) -> Result<(String)> {
        self.client.get("/api/v1/ping", "")?;

        Ok("pong".into())
    }

    // Check server time
    pub fn get_server_time(&self) -> Result<(ServerTime)> {
        let data: String = self.client.get("/api/v1/time", "")?;

        let server_time: ServerTime = from_str(data.as_str())?;

        Ok(server_time)
    }

    // Obtain exchange information (rate limits, symbol metadata etc)
    pub fn exchange_info(&self) -> Result<(ExchangeInformation)> {
        let data: String = self.client.get("/api/v1/exchangeInfo", "")?;

        let info: ExchangeInformation = from_str(data.as_str())?;

        Ok(info)
    }
}
