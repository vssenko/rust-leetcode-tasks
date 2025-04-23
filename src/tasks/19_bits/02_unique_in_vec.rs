//! Single number
//! Given a non-empty array of integers nums, every element appears twice except for one.
//! Find that single one.
//! You must implement a solution with a linear runtime complexity and use only constant extra space.
//! https://leetcode.com/problems/single-number

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut accumutator: i64 = 0;

        for num in nums {
            accumutator = accumutator ^ (num as i64);
        }

        accumutator as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn single_number_1() {
        let nums = vec![4, 1, 2, 1, 2];
        assert_eq!(Solution::single_number(nums), 4);
    }
}
