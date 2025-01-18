//! Best Time to Buy and Sell Stock with Transaction Fee
//! You are given an array prices where prices[i] is the price of a given stock on the ith day,
//! and an integer fee representing a transaction fee.
//! Find the maximum profit you can achieve.
//! You may complete as many transactions as you like, but you need to pay the transaction fee for each transaction.

use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        //1 6 4 5
        //buy 1, sell 6 is 3
        //buy 1 sell 4 is 1
        //buy 1 sell 5 is 2
        //buy 1 sell 6 is 3, + buy 4 sell five is -1 = 2
        //

        // this represent transations
        // for grid[i] are all transaction amounts when you buy stock at i and sell at j

        let mut transaction_grid: Vec<Vec<i32>> = vec![vec![0; prices.len()]; prices.len()];

        for (i, buy_price) in prices.iter().enumerate() {
            for (j, sell_price) in prices.iter().enumerate().skip(i + 1) {
                transaction_grid[i][j] = prices[j] - prices[i] - fee;
            }
        }

        dbg!(transaction_grid);

        0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[ignore]
    #[test]
    fn max_profit_1() {
        let prices = vec![1, 3, 2, 8, 4, 9];
        let fee = 2;

        let result = Solution::max_profit(prices, fee);

        assert_eq!(result, 8);
    }
}
