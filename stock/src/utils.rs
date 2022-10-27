use crate::{Price, StockTrend};

pub(crate) fn get_trend(prices: &Vec<Price>) -> StockTrend {
    let size = prices.len();

    if size <= 1000 {
        StockTrend::NotEnoughData
    } else {
        let start_index = size - 1000 - 1;
        let low_mid_index = start_index + 250;
        let high_mid_index = low_mid_index + 250;
        let end_index = high_mid_index + 500;

        if prices[start_index] <= prices[low_mid_index]
            && prices[high