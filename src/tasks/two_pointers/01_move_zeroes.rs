//! Move zeroes
//! Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.
//! Note that you must do this in-place without making a copy of the array.
//! https://leetcode.com/problems/move-zeroes

use std::mem::swap;

struct Solution {}

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return;
        }

        let mut j = 0;

        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(i, j);
                j += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn move_zeroes_1() {
        let mut input: Vec<i32> = vec![0, 1, 0, 3, 12];

        Solution::move_zeroes(&mut input);

        assert_eq!(input, [1, 3, 12, 0, 0]);
    }

    #[test]
    fn move_zeroes_2() {
        let mut input: Vec<i32> = vec![0];

        Solution::move_zeroes(&mut input);

        assert_eq!(input, [0]);
    }

    #[test]
    fn move_zeroes_3() {
        let mut input: Vec<i32> = vec![0, 1];

        Solution::move_zeroes(&mut input);

        assert_eq!(input, [1, 0]);
    }

    #[test]
    fn move_zeroes_4() {
        let mut input: Vec<i32> = vec![1, 0];

        Solution::move_zeroes(&mut input);

        assert_eq!(input, [1, 0]);
    }
}
