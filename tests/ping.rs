extern crate binance_async as binance;
extern crate dotenv;
extern crate env_logger;
extern crate failure;
extern crate tokio;

use failure::Fallible;
use tokio::runtime::Runtime;

use binance::Binance;

#[test]
fn ping() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    ::env_logger::init();

    let mut rt = Runtime::new()?;
    let binance = Binance::new();

    let fut = binance.ping()?;

    let _ = rt.block_on(fut)?;
    Ok(())
}
