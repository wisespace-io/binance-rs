use crate::model::*;
use crate::client::*;
use crate::errors::*;
use crate::api::API;
use crate::api::Spot;

use serde_json::from_str;

#[derive(Clone)]
pub struct General {
    pub client: Client,
}

impl General {
    // Test connectivity
    pub fn ping(&self) -> Result<String> {
        self.client.get(API::Spot(Spot::Ping), "")?;

        Ok("pong".into())
    }

    // Check server time
    pub fn get_server_time(&self) -> Result<ServerTime> {
        let data: String = self.client.get(API::Spot(Spot::Time), "")?;

        let server_time: ServerTime = from_str(data.as_str())?;

        Ok(server_time)
    }

    // Obtain exchange information
    // - Current exchange trading rules and symbol information
    pub fn exchange_info(&self) -> Result<ExchangeInformation> {
        let data: String = self.client.get(API::Spot(Spot::ExchangeInfo), "")?;

        let info: ExchangeInformation = from_str(data.as_str())?;

        Ok(info)
    }

    // Get Symbol information
    pub fn get_symbol_info<S>(&self, symbol: S) -> Result<Symbol>
    where
        S: Into<String>,
    {
        let upper_symbol = symbol.into().to_uppercase();
        match self.exchange_info() {
            Ok(info) => {
                for item in info.symbols {
                    if item.symbol == upper_symbol {
                        return Ok(item);
                    }
                }
                bail!("Symbol not found")
            }
            Err(e) => Err(e),
        }
    }
}
