//! House Robber
//! You are a professional robber planning to rob houses along a street.
//! Each house has a certain amount of money stashed, the only constraint stopping you
//! from robbing each of them is that adjacent houses have security systems connected and
//! it will automatically contact the police if two adjacent houses were broken into on the same night.
//! Given an integer array nums representing the amount of money of each house,
//! return the maximum amount of money you can rob tonight without alerting the police.
//! https://leetcode.com/problems/house-robber

use std::{collections::VecDeque, iter::Sum};

struct Solution;

#[derive(Debug)]
struct SummarizedChoices {
    skipped_sum: i32,
    robbed_sum: i32,
}

impl Solution {
    #[allow(unused)]
    /// Not used simple and ineffective recursive solution.
    fn find_max_sum(nums: &Vec<i32>) -> i32 {
        fn _find_max_sum(nums: &Vec<i32>, i: usize, current_sum: i32) -> i32 {
            if i >= nums.len() {
                return current_sum;
            }

            let take_this_option = _find_max_sum(nums, i + 2, current_sum + nums[i]);
            let skip_this_take_second = _find_max_sum(nums, i + 1, current_sum);
            let skip_2_take_third = _find_max_sum(nums, i + 2, current_sum);

            take_this_option
                .max(skip_this_take_second)
                .max(skip_2_take_third)
        }

        _find_max_sum(nums, 0, 0)
    }

    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.len() {
            1 => return nums[0],
            2 => return nums[0].max(nums[1]),
            3 => return nums[1].max(nums[0] + nums[2]),
            _ => {}
        };

        // 5 10 3 1

        let mut last_choices: VecDeque<SummarizedChoices> = VecDeque::new();

        last_choices.push_back(SummarizedChoices {
            robbed_sum: nums[0],
            skipped_sum: 0,
        });
        last_choices.push_back(SummarizedChoices {
            robbed_sum: nums[1],
            skipped_sum: nums[0],
        });

        for rob_value in nums.into_iter().skip(2) {
            let prev_prev_choice = last_choices.pop_front().unwrap();
            let prev_choice = last_choices.back().unwrap();

            println!("Checking house with money {rob_value}");
            println!("Prev-prev choice: {:?}", prev_prev_choice);
            println!("Prev choice: {:?}", prev_choice);

            let current_choice = SummarizedChoices {
                robbed_sum: rob_value + prev_choice.skipped_sum,
                skipped_sum: prev_choice.robbed_sum.max(prev_prev_choice.robbed_sum),
            };

            println!("Current choice: {:?}", current_choice);

            last_choices.push_back(current_choice);
        }

        let last_choice = last_choices.back().unwrap();

        last_choice.robbed_sum.max(last_choice.skipped_sum)
    }
}

#[cfg(test)]
mod tests {
    use crate::tasks::dp_d1::robber::Solution;

    #[test]
    fn rob_1() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(Solution::rob(nums), 4);
    }

    #[test]
    fn rob_2() {
        let nums = vec![2, 7, 9, 3, 1];
        assert_eq!(Solution::rob(nums), 12);
    }

    #[test]
    fn rob_3() {
        let nums = vec![5, 1, 1, 4];
        assert_eq!(Solution::rob(nums), 9);
    }
}
