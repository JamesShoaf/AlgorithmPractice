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

/* 
You are given an integer array prices where prices[i] is the price of a given stock on the ith day.

Design an algorithm to find the maximum profit. You may complete at most k transactions.
*/

pub fn max_profit_iv(k: i32, prices: Vec<i32>) -> i32 {
    let mut transactions = vec![0; 2 * cmp::min(k as usize, prices.len() / 2)];
    if transactions.len() == 0 { return 0; }
    for i in (0..transactions.len()).step_by(2) {
        transactions[i] = i32::MAX;
    }
    for p in prices {
        transactions[0] = cmp::min(p, transactions[0]);
        for i in 1..transactions.len() {
            transactions[i] = if i % 2 == 1 {
                cmp::max(transactions[i], p - transactions[i - 1])
            } else {
                cmp::min(transactions[i], p - transactions[i - 1])
            };
        }
    }
    transactions[transactions.len() - 1]
}

/* 
/*  */
#1 solution for comparison
pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
    
    let n = prices.len();
    
    if n == 0 || k == 0 {
        return 0;
    }
    
    
    let mut profit = 0;
    // If 2 * k > n, this problem simplifies to finding
    // the maximum profit when you can do as many transactions as
    // possible. 
    // In such cases, we're only interested in non-decreasing 
    // sequences of prices. Suppose p_1, p_2, ... p_n is a 
    // such sequence. Then, trivially the maximum profit you could 
    // get from it, would be p_n - p_1. However, also note that it 
    // is also equal to sum(p_i - p_{i-1}) over all valid i's.
    // So while iterating over the prices, whenever we see p_{i-1} < p_i
    // add it to the profit. 
    if 2 * k as usize > n {
        for i in 1..n {
            if prices[i] > prices[i-1] {
                profit += prices[i] - prices[i-1]
            }
        }
        return profit;
    }
    
    // Now otherwise, we use dynamic programming.
    // Idea:
    // At each day, there are two states: having a stock or not. 
    // For each state, there are two actions to take respectively. 
    // 1. Suppose we currently have a stock
    //   1) Keep holding the stock
    //   2) Sell stock
    //      We switch to state not having a stock
    // 2. We don't have a stock
    //   1) Buy one stock
    //      We switch to state having a stock, and used one transaction. 
    //   2) Do nothing
    
    // dp(i, j, l): the maximum profit we can have at the end of i-th
    // day with j number of transactions, and l denotes whether we 
    // hold (1) or not (0) a stock. 
    // 
    
    let mut dp = vec![vec![vec![std::i32::MIN; 2]; k as usize + 1]; n];
    
    // Initialize
    // dp(0, 0, 0) = 0
    // dp(0, 1, 1) = -prices[0]
    // dp(i, j, l) = - infinity

    dp[0][0][0] = 0;
    dp[0][1][1] = - prices[0];
    
    // dp(i, j, 0) = max(dp(i-1, j, 0), dp(i-1, j, 1) + prices[i])
    // dp(i, j, 1) = max(dp(i-1, j, 1), dp(i-1, j-1, 0) - prices[i])
    
    for i in 1..n {
        for j in 0..k as usize+1 {
            match dp[i-1][j][1].checked_add(prices[i]) {
                Some(res) => {
                    dp[i][j][0] = std::cmp::max(dp[i-1][j][0], res);
                }
                None => {
                    dp[i][j][0] = dp[i-1][j][0];
                }
            }
            if j > 0 {
                match dp[i-1][j-1][0].checked_sub(prices[i]) {
                    Some(res) => {
                        dp[i][j][1] = std::cmp::max(dp[i-1][j][1], res);                
                    }
                    None => {
                        dp[i][j][1] = dp[i-1][j][1];
                    }
                }
            }
        }
    }
    
    // The answer for original problem would be
    // max(dp(n-1, j, 0)) with 0 <= j <= k. 
    profit = (0..=k as usize).map(|j| dp[n-1][j][0]).max().unwrap();
    
    // The number of subproblems is n*(k+1)*2, and we can order the subproblems
    // from i = 0 to n-1 (inclusive) and j = 0 to k (inclusive)
    
    profit
}
*/

/* 
Heap-based implementation
/*  */
pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
    
    let n = prices.len();
    
    if n == 0 || k == 0 {
        return 0;
    }
    
    
    let mut profit = 0;
    // If 2 * k > n, this problem simplifies to finding
    // the maximum profit when you can do as many transactions as
    // possible. 
    // In such cases, we're only interested in non-decreasing 
    // sequences of prices. Suppose p_1, p_2, ... p_n is a 
    // such sequence. Then, trivially the maximum profit you could 
    // get from it, would be p_n - p_1. However, also note that it 
    // is also equal to sum(p_i - p_{i-1}) over all valid i's.
    // So while iterating over the prices, whenever we see p_{i-1} < p_i
    // add it to the profit. 
    if 2 * k as usize > n {
        for i in 1..n {
            if prices[i] > prices[i-1] {
                profit += prices[i] - prices[i-1]
            }
        }
        return profit;
    }
    
    // Now otherwise, we use dynamic programming.
    // Idea:
    // At each day, there are two states: having a stock or not. 
    // For each state, there are two actions to take respectively. 
    // 1. Suppose we currently have a stock
    //   1) Keep holding the stock
    //   2) Sell stock
    //      We switch to state not having a stock
    // 2. We don't have a stock
    //   1) Buy one stock
    //      We switch to state having a stock, and used one transaction. 
    //   2) Do nothing
    
    // dp(i, j, l): the maximum profit we can have at the end of i-th
    // day with j number of transactions, and l denotes whether we 
    // hold (1) or not (0) a stock. 
    // 
    
    let mut dp = vec![vec![vec![std::i32::MIN; 2]; k as usize + 1]; n];
    
    // Initialize
    // dp(0, 0, 0) = 0
    // dp(0, 1, 1) = -prices[0]
    // dp(i, j, l) = - infinity

    dp[0][0][0] = 0;
    dp[0][1][1] = - prices[0];
    
    // dp(i, j, 0) = max(dp(i-1, j, 0), dp(i-1, j, 1) + prices[i])
    // dp(i, j, 1) = max(dp(i-1, j, 1), dp(i-1, j-1, 0) - prices[i])
    
    for i in 1..n {
        for j in 0..k as usize+1 {
            match dp[i-1][j][1].checked_add(prices[i]) {
                Some(res) => {
                    dp[i][j][0] = std::cmp::max(dp[i-1][j][0], res);
                }
                None => {
                    dp[i][j][0] = dp[i-1][j][0];
                }
            }
            if j > 0 {
                match dp[i-1][j-1][0].checked_sub(prices[i]) {
                    Some(res) => {
                        dp[i][j][1] = std::cmp::max(dp[i-1][j][1], res);                
                    }
                    None => {
                        dp[i][j][1] = dp[i-1][j][1];
                    }
                }
            }
        }
    }
    
    // The answer for original problem would be
    // max(dp(n-1, j, 0)) with 0 <= j <= k. 
    profit = (0..=k as usize).map(|j| dp[n-1][j][0]).max().unwrap();
    
    // The number of subproblems is n*(k+1)*2, and we can order the subproblems
    // from i = 0 to n-1 (inclusive) and j = 0 to k (inclusive)
    
    profit
}
*/

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

#[test]
fn test_max_profit_iv() {
    let test_tuples = vec![
        (3, Vec::new(), 0),
        (3, vec![1], 0),
        (2, vec![2, 4, 1], 2),
        (2, vec![3, 2, 6, 5, 0, 3], 7),
        (0, vec![0, 1, 0, 2, 0, 3], 0),
        (1, vec![0, 1, 0, 2, 0, 3], 3),
        (2, vec![0, 1, 0, 2, 0, 3], 5),
        (3, vec![0, 1, 0, 2, 0, 3], 6),
    ];
    for (k, prices, expected) in test_tuples {
        assert_eq!(max_profit_iv(k, prices), expected);
    }
}