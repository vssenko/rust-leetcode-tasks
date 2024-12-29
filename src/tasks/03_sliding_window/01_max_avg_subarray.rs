//! Maximum average subarray
//! You are given an integer array nums consisting of n elements, and an integer k.
//!Find a contiguous subarray whose length is equal to k that has the maximum average value and return this value.
//! Any answer with a calculation error less than 10-5 will be accepted.
//! https://leetcode.com/problems/maximum-average-subarray-i

struct Solution;

impl Solution {
    #[allow(clippy::explicit_counter_loop)]
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        if k < 0 {
            return 0.0;
        }

        let k = k as usize;

        if (nums.len() < k) {
            return 0.0;
        }

        let mut sum: i32 = nums.iter().take(k).sum();

        let mut win_start_ind = 0;

        let mut max_sum = sum;

        for win_end_ind in k..nums.len() {
            sum = sum - nums[win_start_ind] + nums[win_end_ind];
            max_sum = max_sum.max(sum);
            win_start_ind += 1;
        }

        max_sum as f64 / k as f64
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn find_max_average_1() {
        let nums = vec![1, 12, -5, -6, 50, 3];
        let k = 4;

        let result = Solution::find_max_average(nums, k);

        assert_eq!(result, 12.75);
    }

    #[test]
    fn find_max_average_2() {
        let nums = vec![1, 2, 3, 0];
        let k = 4;

        let result = Solution::find_max_average(nums, k);

        assert_eq!(result, 1.5);
    }
}
