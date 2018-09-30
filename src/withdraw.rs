use util::*;
use model::*;
use client::*;
use errors::*;
use std::collections::BTreeMap;
use serde_json::from_str;

#[derive(Clone)]
pub struct Withdraw {
    pub client: Client,
    pub recv_window: u64,
}

impl Withdraw {
    // Maker and Taker trade fees for each asset pair
    pub fn get_trade_fees(&self) -> Result<(TradeFees)> {
        let parameters: BTreeMap<String, String> = BTreeMap::new();

        let request = build_signed_request(parameters, self.recv_window)?;
        let data = self.client.get_signed("/wapi/v3/tradeFee.html", &request)?;
        let trade_fees: TradeFees = from_str(data.as_str())?;

        Ok(trade_fees)
    }

    // Fetch asset details: min_withdraw_amount, deposit_status, withdraw_fee, withdraw_status, Option<deposit_tip>
    pub fn get_asset_details(&self) -> Result<(AssetDetails)> {
        let parameters: BTreeMap<String, String> = BTreeMap::new();

        let request = build_signed_request(parameters, self.recv_window)?;
        let data = self.client.get_signed("/wapi/v3/assetDetail.html", &request)?;
        let asset_details: AssetDetails = from_str(data.as_str())?;

        Ok(asset_details)
    }

    // Depoist Address to given Asset 
    pub fn get_deposit_address(&self, asset: String) -> Result<(DepositAddress)> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("asset".into(), asset);

        let request = build_signed_request(parameters, self.recv_window)?;
        let data = self.client.get_signed("/wapi/v3/depositAddress.html", &request)?;
        let deposit_address: DepositAddress = from_str(data.as_str())?;

        Ok(deposit_address)
    }

    // Withdraw currency 
    pub fn withdraw_currency(&self, asset: String, address: String, address_tag: Option<u64>, amount: f64) -> Result<(WithdrawResponse)> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("asset".into(), asset);
        parameters.insert("address".into(), address);
        if address_tag.is_some() {
            parameters.insert("addressTag".into(), address_tag.unwrap().to_string());
        }
        parameters.insert("amount".into(), amount.to_string());

        let request = build_signed_request(parameters, self.recv_window)?;
        let data = self.client.get_signed("/wapi/v3/withdraw.html", &request)?;
        let withdraw: WithdrawResponse = from_str(data.as_str())?;

        Ok(withdraw)
    }    
}
