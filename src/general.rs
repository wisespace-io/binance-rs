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
        self.client.get("/api/v1/ping", String::new())?;

        Ok("pong".into())
    }

    // Check server time
    pub fn get_server_time(&self) -> Result<(ServerTime)> {
        let data: String = self.client.get("/api/v1/time", String::new())?;

        let server_time: ServerTime = from_str(data.as_str()).unwrap();

        Ok(server_time)
    }
}
