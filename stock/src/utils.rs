use crate::{Price, StockTrend};

pub(crate) fn get_trend(prices: &Vec<Price>) -> StockTrend {
    let size = prices.len();

    if size <= 1000 {
        StockTrend::NotEnoughData
    }