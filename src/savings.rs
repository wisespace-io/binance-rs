use crate::util::*;
use crate::model::*;
use crate::client::*;
use crate::errors::*;
use std::collections::BTreeMap;
use crate::api::API;
use crate::api::Sapi;

#[derive(Clone)]
pub struct Savings {
    pub client: Client,
    pub recv_window: u64,
}

impl Savings {
    /// Get all coins available for deposit and withdrawal
    pub fn get_all_coins(&self) -> Result<Vec<CoinInfo>> {
        let request = build_signed_request(BTreeMap::new(), self.recv_window)?;
        self.client.get_signed(API::Savings(Sapi::AllCoins), Some(request))
    }

    /// Fetch details of assets supported on Binance.
    pub fn asset_detail(&self, asset: Option<String>) -> Result<BTreeMap<String, AssetDetail>> {
        let mut parameters = BTreeMap::new();
        if let Some(asset) = asset {
            parameters.insert("asset", asset);
        }
        let request = build_signed_request(BTreeMap::new(), self.recv_window)?;
        self.client.get_signed(API::Savings(Sapi::AssetDetail), Some(request))
    }
}
