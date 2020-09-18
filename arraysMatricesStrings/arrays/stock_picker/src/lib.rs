/*
Say you have an array for which the ith element is the price of a given stock on day i.

If you were only permitted to complete at most one transaction (i.e., buy one and sell one share
    of the stock), design an algorithm to find the maximum profit.
*/
use std::cmp;

pub fn max_profit(prices: Vec<i32>) -> i32 {
    prices.into_iter()
        .scan(i32::MAX, |purchase_price, current_price| {
            *purchase_price = cmp::min(*purchase_price, current_price);
            Some(current_price - *purchase_price)
        })
        .fold(0, |acc, profit| cmp::max(acc, profit))
}

#[test]
fn test_max_profit() {
    let test_tuples = vec![
        (Vec::new(), 0),
        (vec![100], 0),
        (vec![7, 1, 5, 3, 6, 4], 5),
        (vec![7, 6, 4, 3, 1], 0),
    ];
    for (prices, expected) in test_tuples {
        assert_eq!(max_profit(prices), expected);
    }
}