use crate::model::*;
use crate::client::*;
use crate::errors::*;
use serde_json::from_str;
use crate::api::API;
use crate::api::Spot;

#[derive(Clone)]
pub struct UserStream {
    pub client: Client,
    pub recv_window: u64,
}

impl UserStream {
    // User Stream
    pub fn start(&self) -> Result<UserDataStream> {
        let data = self.client.post(API::Spot(Spot::UserDataStream))?;
        let user_data_stream: UserDataStream = from_str(data.as_str())?;
        Ok(user_data_stream)
    }

    // Current open orders on a symbol
    pub fn keep_alive(&self, listen_key: &str) -> Result<Success> {
        let data = self.client.put(API::Spot(Spot::UserDataStream), listen_key)?;
        let success: Success = from_str(data.as_str())?;
        Ok(success)
    }

    pub fn close(&self, listen_key: &str) -> Result<Success> {
        let data = self.client.delete(API::Spot(Spot::UserDataStream), listen_key)?;
        let success: Success = from_str(data.as_str())?;
        Ok(success)
    }
}
