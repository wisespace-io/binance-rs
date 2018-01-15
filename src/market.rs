use util::*;
use model::*;
use client::*;
use errors::*;
use std::collections::BTreeMap;
use serde_json::from_str;

#[derive(Clone)]
pub struct Market {
    pub client: Client,
    pub recv_window: u64,
}

// Market Data endpoints
impl Market {
    // Order book (Default 100; max 100)
    pub fn get_depth(&self, symbol: String) -> Result<(OrderBook)> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol);
        let request = build_request(&parameters);

        let data = self.client.get("/api/v1/depth", &request)?;

        let order_book: OrderBook = from_str(data.as_str()).unwrap();

        Ok(order_book)
    }

    // Latest price for ALL symbols.
    pub fn get_all_prices(&self) -> Result<(Prices)> {
        let data = self.client.get("/api/v1/ticker/allPrices", "")?;

        let prices: Prices = from_str(data.as_str()).unwrap();

        Ok(prices)
    }

    // Latest price for ONE symbol.
    pub fn get_price(&self, symbol: &str) -> Result<(f64)> {
        match self.get_all_prices() {
            Ok(answer) => match answer {
                Prices::AllPrices(prices) => {
                    for par in prices {
                        if par.symbol == symbol {
                            let price: f64 = par.price.parse()?;
                            return Ok(price);
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

        let book_tickers: BookTickers = from_str(data.as_str()).unwrap();

        Ok(book_tickers)
    }

    // -> Best price/qty on the order book for ONE symbol
    pub fn get_book_ticker(&self, symbol: &str) -> Result<(Tickers)> {
        match self.get_all_book_tickers() {
            Ok(answer) => match answer {
                BookTickers::AllBookTickers(book_tickers) => {
                    for obj in book_tickers {
                        if obj.symbol == symbol {
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
    pub fn get_24h_price_stats(&self, symbol: String) -> Result<(PriceStats)> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol);
        let request = build_request(&parameters);

        let data = self.client.get("/api/v1/ticker/24hr", &request)?;

        let stats: PriceStats = from_str(data.as_str()).unwrap();

        Ok(stats)
    }
}
