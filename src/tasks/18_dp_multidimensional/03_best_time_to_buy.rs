//! Best Time to Buy and Sell Stock with Transaction Fee
//! You are given an array prices where prices[i] is the price of a given stock on the ith day,
//! and an integer fee representing a transaction fee.
//! Find the maximum profit you can achieve.
//! You may complete as many transactions as you like, but you need to pay the transaction fee for each transaction.

use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        //TODO: Finish one day.

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
