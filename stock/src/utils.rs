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
            && prices[high_mid_index] <= prices[end_index]
        {
            StockTrend::Uptrend
        } else if prices[start_index] >= prices[low_mid_index]
            && prices[high_mid_index] >= prices[end_index]
        {
            StockTrend::Downtrend
        } else {
            StockTrend::Sideways
        }
    }
}

pub(crate) fn moving_average(prices: &Vec<Price>) -> f64 {
    if prices.is_empty() {
        0.0
    } else {
        let sum: f64 = prices.iter().sum();
        sum / prices.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moving_average() {
        let no_prices = vec![];
        assert_eq!(moving_average(&no_prices), 0.0);

        let prices = vec![1., 2., 3., 4., 5., 6.];
        assert_eq!(moving_average(&prices), 3.5);
    }

    #[test]
    fn test_get_trend() {
        let small_data_set = vec![1., 2., 3., 4., 5., 