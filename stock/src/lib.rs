
use rand::{self, prelude::ThreadRng, Rng};
use std::collections::HashMap;
use utils::{get_trend, moving_average};
mod utils;
use serde::{Deserialize, Serialize};

const STOCKS: [&'static str; 6] = ["GOOG", "APPL", "TSLA", "AMZN", "MSFT", "FB"];
pub(crate) type Price = f64;

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub enum StockTrend {
    Uptrend,
    Downtrend,
    Sideways,
    NotEnoughData,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub struct StockSummary {
    pub trend: StockTrend,
    pub lowest_price: Option<Price>,
    pub highest_price: Option<Price>,
    pub moving_average: Price,
}

/// Holds our stock data
#[derive(Debug)]
pub struct StockData {
    lowest: HashMap<&'static str, Option<Price>>,
    highest: HashMap<&'static str, Option<Price>>,
    data: HashMap<&'static str, Vec<Price>>,
    summaries: HashMap<&'static str, Option<StockSummary>>,
}

impl StockData {
    pub fn initialize() -> Self {
        let mut data = HashMap::new();
        let mut highest = HashMap::new();
        let mut lowest = HashMap::new();
        let mut summaries = HashMap::new();

        for stock in STOCKS {
            data.insert(stock, vec![]);
            lowest.insert(stock, None);
            highest.insert(stock, None);
            summaries.insert(stock, None);
        }

        StockData {
            lowest,
            highest,
            data,
            summaries,
        }
    }

    /// randomly generates new price for each stock and adds it to the hash maps
    pub fn generate_next_tick(&mut self, thread_rng: &mut ThreadRng) {
        let next_prices: [Price; STOCKS.len()] = thread_rng.gen();
        let mut iter = next_prices.iter().map(|v| v * 100f64);

        for stock in STOCKS {
            let next_price = iter.next().unwrap();
            self.insert_next(stock, next_price);
            self.insert_lowest(stock, next_price);
            self.insert_highest(stock, next_price);

            let stock_summary = self.get_summary(stock);
            self.summaries.insert(stock, stock_summary);
        }
    }

    /// get all sumarries
    pub fn get_summaries(&self) -> &HashMap<&'static str, Option<StockSummary>> {
        &self.summaries
    }

    /// get last recorded price for a stock
    pub fn get_last_price(&self, stock: &str) -> Option<Price> {
        if let Some(prices) = self.get_prices(stock) {