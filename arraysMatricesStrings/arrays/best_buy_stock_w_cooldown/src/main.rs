struct Solution {}

impl Solution {
    // O(n) time, O(1) space implementation
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        use std::cmp::max;
        if prices.len() == 0 { return 0; }
        let mut primed: i32 = 0;
        let mut loaded: i32 = -prices[0];
        let mut fired: i32 = 0;
        for price in prices {
            let next_primed = max(fired, primed);
            let next_loaded = max(primed - price, loaded);
            let next_fired = loaded + price;
            primed = next_primed;
            loaded = next_loaded;
            fired = next_fired;
        }
        return max(primed, fired)
    }

    // initial O(n^2) time, O(n) space implementation
    // pub fn max_profit(prices: Vec<i32>) -> i32 {
    //     if prices.len() < 2 { return 0; }
    //     if prices.len() == 2 { return if prices[1] > prices[0] { prices[1] - prices[0] } else { 0 }; }
    //     let mut sales: Vec<i32> = vec![-1; prices.len()];
    //     sales[prices.len() - 1] = 0;
    //     fn helper(today: usize, prices: &Vec<i32>, sales: &Vec<i32>) -> i32 {
    //         let mut max: i32 = sales[today + 1];
    //         let buy_price = prices[today];
    //         for sale in today + 1..prices.len() {
    //             let sale_profit = prices[sale] - buy_price
    //                 + if sale < prices.len() - 3 { sales[sale + 2]} else {0};
    //             if sale_profit > max { max = sale_profit; }
    //         }
    //         max
    //     }
    //     let mut max: i32 = 0;
    //     for i in 2..=prices.len() {
    //         let day: usize = prices.len() - i;
    //         let profit: i32 = helper(day, &prices, &sales);
    //         if profit > max { max = profit; }
    //         sales[day] = profit;
    //     }
    //     max
    // }
}

fn main() {
    let test_tuples: Vec<(Vec<i32>, i32)> = vec![
        (Vec::new(), 0),
        (vec![0], 0),
        (vec![0, 1], 1),
        (vec![0, 1, 2], 2),
        (vec![1, 2, 3, 2], 2),
        (vec![1, 2, 3, 0, 2], 3),
        (vec![1, 2, 3, 0, 0, 2], 4),
        (vec![5, 4, 3, 2, 1], 0),
        (vec![0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 6, 1, 2, 3, 4, 5], 21),
    ];
    for (input, expected) in test_tuples {
        assert_eq!(Solution::max_profit(input.to_vec()), expected, "{:?} did not return {}", input, expected);
    }
}
