extern crate binance_async as binance;
extern crate dotenv;
extern crate env_logger;
extern crate failure;
extern crate tokio;

use std::env::var;

use failure::Fallible;
use tokio::runtime::Runtime;

use binance::Binance;

#[test]
fn get_account() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    ::env_logger::init();

    let mut rt = Runtime::new()?;

    let binance = Binance::with_credential(&var("BINANCE_KEY")?, &var("BINANCE_SECRET")?);
    let fut = binance.get_account()?;

    let _ = rt.block_on(fut)?;
    Ok(())
}
