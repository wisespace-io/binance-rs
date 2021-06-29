/*!
## Implemented functionality
- [x] `Order Book`
- [x] `Recent Trades List`
- [ ] `Old Trades Lookup (MARKET_DATA)`
- [x] `Compressed/Aggregate Trades List`
- [x] `Kline/Candlestick Data`
- [x] `Mark Price`
- [ ] `Get Funding Rate History (MARKET_DATA)`
- [x] `24hr Ticker Price Change Statistics`
- [x] `Symbol Price Ticker`
- [x] `Symbol Order Book Ticker`
- [x] `Get all Liquidation Orders`
- [x] `Open Interest`
- [ ] `Notional and Leverage Brackets (MARKET_DATA)`
- [ ] `Open Interest Statistics (MARKET_DATA)`
- [ ] `Top Trader Long/Short Ratio (Accounts) (MARKET_DATA)`
- [ ] `Top Trader Long/Short Ratio (Positions) (MARKET_DATA)`
- [ ] `Long/Short Ratio (MARKET_DATA)`
- [ ] `Taker Buy/Sell Volume (MARKET_DATA)`
*/

use crate::util::*;
use crate::futures::model::*;
use crate::client::*;
use crate::errors::*;
use std::collections::BTreeMap;
use serde_json::Value;
use crate::api::API;
use crate::api::Futures;

// TODO
// Make enums for Strings
// Add limit parameters to functions
// Implement all functions

#[derive(Clone)]
pub struct FuturesMarket {
    pub client: Client,
    pub recv_window: u64,
}

impl FuturesMarket {
    // Order book (Default 100; max 1000)
    pub fn get_depth<S>(&self, symbol: S) -> Result<OrderBook>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);

        self.client.get(API::Futures(Futures::Depth), Some(request))
    }

    pub fn get_trades<S>(&self, symbol: S) -> Result<Trades>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);
        self.client.get(API::Futures(Futures::Trades), Some(request))
    }

    // TODO This may be incomplete, as it hasn't been tested
    pub fn get_historical_trades<S1, S2, S3>(
        &self, symbol: S1, from_id: S2, limit: S3,
    ) -> Result<Trades>
    where
        S1: Into<String>,
        S2: Into<Option<u64>>,
        S3: Into<Option<u16>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());

        // Add three optional parameters
        if let Some(lt) = limit.into() {
            parameters.insert("limit".into(), format!("{}", lt));
        }
        if let Some(fi) = from_id.into() {
            parameters.insert("fromId".into(), format!("{}", fi));
        }

        let request = build_signed_request(parameters, self.recv_window)?;

        self.client.get_signed(API::Futures(Futures::HistoricalTrades), Some(request))
    }

    pub fn get_agg_trades<S1, S2, S3, S4, S5>(
        &self, symbol: S1, from_id: S2, start_time: S3, end_time: S4, limit: S5,
    ) -> Result<AggTrades>
    where
        S1: Into<String>,
        S2: Into<Option<u64>>,
        S3: Into<Option<u64>>,
        S4: Into<Option<u64>>,
        S5: Into<Option<u16>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());

        // Add three optional parameters
        if let Some(lt) = limit.into() {
            parameters.insert("limit".into(), format!("{}", lt));
        }
        if let Some(st) = start_time.into() {
            parameters.insert("startTime".into(), format!("{}", st));
        }
        if let Some(et) = end_time.into() {
            parameters.insert("endTime".into(), format!("{}", et));
        }
        if let Some(fi) = from_id.into() {
            parameters.insert("fromId".into(), format!("{}", fi));
        }

        let request = build_request(parameters);

        self.client.get(API::Futures(Futures::AggTrades), Some(request))
    }

    // Returns up to 'limit' klines for given symbol and interval ("1m", "5m", ...)
    // https://github.com/binance-exchange/binance-official-api-docs/blob/master/rest-api.md#klinecandlestick-data
    pub fn get_klines<S1, S2, S3, S4, S5>(
        &self, symbol: S1, interval: S2, limit: S3, start_time: S4, end_time: S5,
    ) -> Result<KlineSummaries>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<Option<u16>>,
        S4: Into<Option<u64>>,
        S5: Into<Option<u64>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("interval".into(), interval.into());

        // Add three optional parameters
        if let Some(lt) = limit.into() {
            parameters.insert("limit".into(), format!("{}", lt));
        }
        if let Some(st) = start_time.into() {
            parameters.insert("startTime".into(), format!("{}", st));
        }
        if let Some(et) = end_time.into() {
            parameters.insert("endTime".into(), format!("{}", et));
        }

        let request = build_request(parameters);

        let data: Vec<Vec<Value>> = self
            .client
            .get(API::Futures(Futures::Klines), Some(request))?;

        let klines = KlineSummaries::AllKlineSummaries(
            data.iter()
                .map(|row| KlineSummary {
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
                })
                .collect(),
        );
        Ok(klines)
    }

    // 24hr ticker price change statistics
    pub fn get_24h_price_stats<S>(&self, symbol: S) -> Result<PriceStats>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);

        self.client.get(API::Futures(Futures::Ticker24hr), Some(request))
    }

    // 24hr ticker price change statistics for all symbols
    pub fn get_all_24h_price_stats(&self) -> Result<Vec<PriceStats>>
    {
        self.client.get(API::Futures(Futures::Ticker24hr), None)
    }

    // Latest price for ONE symbol.
    pub fn get_price<S>(&self, symbol: S) -> Result<SymbolPrice>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);

        self.client.get(API::Futures(Futures::TickerPrice), Some(request))
    }
    
    // Latest price for all symbols.
    pub fn get_all_prices(&self) -> Result<crate::model::Prices>
    {
        self.client.get(API::Futures(Futures::TickerPrice), None)
    }

    // Symbols order book ticker
    // -> Best price/qty on the order book for ALL symbols.
    pub fn get_all_book_tickers(&self) -> Result<BookTickers> {
        self.client.get(API::Futures(Futures::BookTicker), None)
    }

    // -> Best price/qty on the order book for ONE symbol
    pub fn get_book_ticker<S>(&self, symbol: S) -> Result<Tickers>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);
        self.client.get(API::Futures(Futures::BookTicker), Some(request))
    }

    pub fn get_mark_prices(&self) -> Result<MarkPrices> {
        self.client.get(API::Futures(Futures::PremiumIndex), None)
    }

    pub fn get_all_liquidation_orders(&self) -> Result<LiquidationOrders> {
        self.client.get(API::Futures(Futures::AllForceOrders), None)
    }

    pub fn open_interest<S>(&self, symbol: S) -> Result<OpenInterest>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);
        self.client.get(API::Futures(Futures::OpenInterest), Some(request))
    }
}
