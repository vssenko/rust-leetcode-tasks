//! Product of array except self (without division operator!)
//! https://leetcode.com/problems/product-of-array-except-self
//! Not the precisest implementation, but at least it's readable.

struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut left_going_prods: Vec<i32> = Vec::with_capacity(nums.len());
        left_going_prods.resize(nums.len(), 1);

        let mut right_going_prods: Vec<i32> = Vec::with_capacity(nums.len());
        right_going_prods.resize(nums.len(), 1);

        // This 2 array traverses could be done in single cycle (and result in much better score),
        // but that would decrease readability and simplicity a lot.

        let mut multiplier = 1;
        for (i, value) in nums.iter().enumerate() {
            left_going_prods[i] = multiplier;
            multiplier *= value;
        }

        multiplier = 1;
        for (i, value) in nums.iter().rev().enumerate() {
            right_going_prods[nums.len() - i - 1] = multiplier;
            multiplier *= value;
        }

        std::iter::zip(left_going_prods, right_going_prods)
            .map(|(v1, v2)| v1 * v2)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn product_except_self_1() {
        let nums: Vec<i32> = vec![1, 2, 3, 4];

        let result = Solution::product_except_self(nums);

        assert_eq!(result, [24, 12, 8, 6]);
    }

    #[test]
    fn product_except_self_2() {
        let nums: Vec<i32> = vec![-1, 1, 0, -3, 3];

        let result = Solution::product_except_self(nums);

        assert_eq!(result, [0, 0, 9, 0, 0]);
    }
}
