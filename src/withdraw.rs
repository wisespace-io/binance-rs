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
}
