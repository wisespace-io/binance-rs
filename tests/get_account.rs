use anyhow::Error;
use binance_async::{rest::GetAccountRequest, Binance};
use fehler::throws;
use std::env::var;

#[throws(Error)]
#[tokio::test]
async fn get_account() {
    env_logger::init();

    let binance = Binance::with_credential(&var("BINANCE_KEY")?, &var("BINANCE_SECRET")?);
    let ai = binance.request(GetAccountRequest {}).await?;
    println!("{ai:?}");
}
