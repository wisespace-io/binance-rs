use crate::{
    util::build_signed_request,
    model::{TradeFees, AssetDetails, DepositAddress, WithdrawResponse},
    client::Client,
    errors::Result,
    api::{API, Wapi},
};
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct Withdraw {
    pub client: Client,
    pub recv_window: u64,
}

impl Withdraw {
    // Maker and Taker trade fees for each asset pair
    pub fn get_trade_fees(&self) -> Result<TradeFees> {
        let parameters: BTreeMap<String, String> = BTreeMap::new();
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.get_signed(API::Withdraw(Wapi::TradeFee), Some(request))
    }

    // Fetch asset details: min_withdraw_amount, deposit_status, withdraw_fee, withdraw_status, Option<deposit_tip>
    pub fn get_asset_details(&self) -> Result<AssetDetails> {
        let parameters: BTreeMap<String, String> = BTreeMap::new();
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.get_signed(API::Withdraw(Wapi::AssetDetail), Some(request))
    }

    // Depoist Address to given Asset 
    pub fn get_deposit_address<S>(&self, asset: S) -> Result<DepositAddress>
    where
        S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("asset".into(), asset.into());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.get_signed(API::Withdraw(Wapi::DepositAddress), Some(request))
    }

    // Withdraw currency 
    pub fn withdraw_currency<S>(&self, asset: S, address: S, address_tag: Option<u64>, amount: f64) -> Result<WithdrawResponse>
    where
        S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("asset".into(), asset.into());
        parameters.insert("address".into(), address.into());
        if address_tag.is_some() {
            parameters.insert("addressTag".into(), address_tag.unwrap().to_string());
        }
        parameters.insert("amount".into(), amount.to_string());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.get_signed(API::Withdraw(Wapi::Withdraw), Some(request))
    }    
}