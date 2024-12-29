//! Max number of k-sum pairs
//! You are given an integer array nums and an integer k.
//! In one operation, you can pick two numbers from the array whose sum equals k and remove them from the array.
//! Return the maximum number of operations you can perform on the array.
//! https://leetcode.com/problems/max-number-of-k-sum-pairs

use std::{backtrace, collections::HashMap};

struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut left_to_sum_map: HashMap<i32, usize> = HashMap::new();

        let lesser_nums: Vec<i32> = nums.into_iter().filter(|i| *i <= k).collect();

        for n in lesser_nums.iter() {
            *left_to_sum_map.entry(*n).or_default() += 1;
        }

        let mut result = 0;

        let half = k / 2;

        for term_1 in lesser_nums.iter().map(|n| k - n) {
            left_to_sum_map.entry(term_1).and_modify(|c| {
                if (*c > 0) {
                    result += 1;
                    *c -= 1;
                }
            });
        }

        result / 2
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn max_operations_1() {
        let nums = vec![2, 1, 3, 4];
        let k = 5;

        let result = Solution::max_operations(nums, k);

        assert_eq!(result, 2);
    }

    #[test]
    fn max_operations_2() {
        let nums = vec![3, 1, 3, 4, 3];
        let k = 6;

        let result = Solution::max_operations(nums, k);

        assert_eq!(result, 1);
    }

    #[test]
    fn max_operations_3() {
        let nums = vec![3];
        let k = 3;

        let result = Solution::max_operations(nums, k);

        assert_eq!(result, 0);
    }
}
