//! Domino and Tromino Tiling
//! You have two types of tiles: a 2 x 1 domino shape and a tromino shape. You may rotate these shapes.
//! Given an integer n, return the number of ways to tile an 2 x n board.
//! Since the answer may be very large, return it modulo 109 + 7.
//! In a tiling, every square must be covered by a tile.
//! Two tilings are different if and only if
//! there are two 4-directionally adjacent cells on the board such that
//! exactly one of the tilings has both squares occupied by a tile.
//! https://leetcode.com/problems/domino-and-tromino-tiling

//! So hard task can be trivilased by simple pattern.

use std::{collections::VecDeque, num::ParseFloatError};

struct Solution;

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        let modulo = (10_i128.pow(9) + 7);
        let n_as_ind = (n - 1) as usize;
        let mut patterns: VecDeque<i128> = VecDeque::from([1, 2, 5]);

        if n_as_ind <= 2 {
            return patterns[n_as_ind] as i32;
        }

        for i in 4..=n {
            let prev_prev_prev_val = patterns.pop_front().unwrap();
            let new_value = 2 * patterns.back().unwrap() + prev_prev_prev_val;

            patterns.push_back(new_value % modulo);
        }

        *patterns.back().unwrap() as i32
        //(*patterns.back().unwrap() % (10_i128.pow(9) + 7)) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn num_tilings_1() {
        assert_eq!(Solution::num_tilings(3), 5);
    }

    #[test]
    fn num_tilings_2() {
        assert_eq!(Solution::num_tilings(1), 1);
    }

    #[test]
    fn num_tilings_3() {
        assert_eq!(Solution::num_tilings(5), 24);
    }

    #[test]
    fn num_tilings_4() {
        assert_eq!(Solution::num_tilings(30), 312342182);
    }

    #[test]
    fn num_tilings_5() {
        assert_eq!(Solution::num_tilings(699), 939053561);
    }
}
