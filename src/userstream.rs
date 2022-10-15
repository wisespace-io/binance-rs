use crate::model::{Success, UserDataStream};
use crate::client::Client;
use crate::errors::Result;
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
        self.client.post(API::Spot(Spot::UserDataStream))
    }

    // Current open orders on a symbol
    pub fn keep_alive(&self, listen_key: &str) -> Result<Success> {
        self.client.put(API::Spot(Spot::UserDataStream), listen_key)
    }

    pub fn close(&self, listen_key: &str) -> Result<Success> {
        self.client
            .delete(API::Spot(Spot::UserDataStream), listen_key)
    }
}
