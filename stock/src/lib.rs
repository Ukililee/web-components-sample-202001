
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