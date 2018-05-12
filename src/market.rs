use util::*;
use model::*;
use client::*;
use errors::*;
use std::collections::BTreeMap;
use serde_json::{Value, from_str};

#[derive(Clone)]
pub struct Market {
    pub client: Client,
    pub recv_window: u64,
}

// Market Data endpoints
impl Market {
    // Order book (Default 100; max 100)
    pub fn get_depth<S>(&self, symbol: S) -> Result<(OrderBook)>
        where S: Into<String>
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(&parameters);

        let data = self.client.get("/api/v1/depth", &request)?;

        let order_book: OrderBook = from_str(data.as_str())?;

        Ok(order_book)
    }

    // Latest price for ALL symbols.
    pub fn get_all_prices(&self) -> Result<(Prices)> {
        let data = self.client.get("/api/v1/ticker/allPrices", "")?;

        let prices: Prices = from_str(data.as_str())?;

        Ok(prices)
    }

    // Latest price for ONE symbol.
    pub fn get_price<S>(&self, symbol: S) -> Result<(f64)>
        where S: Into<String>
    {
        match self.get_all_prices() {
            Ok(answer) => match answer {
                Prices::AllPrices(prices) => {
                    let cmp_symbol = symbol.into();
                    for par in prices {
                        if par.symbol == cmp_symbol {
                            return Ok(par.price);
                        }
                    }
                    bail!("Symbol not found");
                }
            },
            Err(e) => Err(e),
        }
    }

    // Symbols order book ticker
    // -> Best price/qty on the order book for ALL symbols.
    pub fn get_all_book_tickers(&self) -> Result<(BookTickers)> {
        let data = self.client.get("/api/v1/ticker/allBookTickers", "")?;

        let book_tickers: BookTickers = from_str(data.as_str())?;

        Ok(book_tickers)
    }

    // -> Best price/qty on the order book for ONE symbol
    pub fn get_book_ticker<S>(&self, symbol: S) -> Result<(Tickers)> 
        where S: Into<String>
    {
        match self.get_all_book_tickers() {
            Ok(answer) => match answer {
                BookTickers::AllBookTickers(book_tickers) => {
                    let cmp_symbol = symbol.into();
                    for obj in book_tickers {
                        if obj.symbol == cmp_symbol {
                            let ticker: Tickers = obj;
                            return Ok(ticker);
                        }
                    }
                    bail!("Symbol not found");
                }
            },
            Err(e) => Err(e),
        }
    }

    // 24hr ticker price change statistics
    pub fn get_24h_price_stats<S>(&self, symbol: S) -> Result<(PriceStats)>
        where S: Into<String>
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(&parameters);

        let data = self.client.get("/api/v1/ticker/24hr", &request)?;

        let stats: PriceStats = from_str(data.as_str())?;

        Ok(stats)
    }

    // Returns up to 'limit' klines for given symbol and interval ("1m", "5m", ...)
    pub fn get_klines<S1,S2>(&self, symbol: S1, interval: S2, limit: i32) -> Result<(KlineSummaries)> 
        where S1: Into<String>, S2: Into<String>
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("interval".into(), interval.into());
        parameters.insert("limit".into(), format!("{}", limit));
        let request = build_request(&parameters);

        let data = self.client.get("/api/v1/klines", &request)?;
        let parsed_data: Vec<Vec<Value>> = from_str(data.as_str())?;

        let klines = KlineSummaries::AllKlineSummaries(
            parsed_data.iter().map(|row|KlineSummary{
                open_time: to_i64(&row[0]),
                open: to_f64(&row[1]),
                high: to_f64(&row[2]),
                low: to_f64(&row[3]),
                close: to_f64(&row[4]),
                volume: to_f64(&row[5]),
                close_time: to_i64(&row[6]),
                quote_asset_volume: to_f64(&row[7]),
                number_of_trades: to_i64(&row[8]),
                taker_buy_base_asset_volume: to_f64(&row[9]),
                taker_buy_quote_asset_volume: to_f64(&row[10]),
            }).collect());
        Ok(klines)
    }
}
