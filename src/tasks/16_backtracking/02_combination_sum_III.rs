//! Combination Sum III
//! Find all valid combinations of k numbers that sum up to n such that the following conditions are true:
//! Only numbers 1 through 9 are used.
//! Each number is used at most once.
//! Return a list of all possible valid combinations. The list must not contain the same combination twice, and the combinations may be returned in any order.
//! https://leetcode.com/problems/combination-sum-iii

// Not optimized, but i like this style.

use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    fn generate_combinations<F>(k: usize, max_sum: i32, callback: &mut F)
    where
        F: FnMut(&Vec<i32>),
    {
        fn _generate_combinations<F>(
            current: HashSet<i32>,
            k: usize,
            current_sum: i32,
            max_sum: i32,
            callback: &mut F,
        ) where
            F: FnMut(&Vec<i32>),
        {
            if current.len() == k {
                if current_sum == max_sum {
                    callback(&current.into_iter().collect::<Vec<i32>>());
                }
                return;
            }

            if current_sum > max_sum {
                return;
            }

            for digit in 1..=9 {
                let mut new_current = current.clone();
                if !new_current.contains(&digit) {
                    new_current.insert(digit);
                    _generate_combinations(new_current, k, current_sum + digit, max_sum, callback);
                }
            }
        }

        _generate_combinations(HashSet::new(), k, 0, max_sum, callback);
    }

    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut result: HashSet<Vec<i32>> = HashSet::new();

        Self::generate_combinations(k as usize, n, &mut |combination| {
            if combination.iter().sum::<i32>() == n {
                let mut cloned = combination.clone();
                cloned.sort();

                result.insert(cloned);
            }
        });

        result.into_iter().collect::<Vec<Vec<i32>>>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn combination_sum3_1() {
        let k = 3;
        let n = 9;

        let mut result = Solution::combination_sum3(k, n);

        result.sort();

        assert_eq!(result, vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]]);
    }
}
