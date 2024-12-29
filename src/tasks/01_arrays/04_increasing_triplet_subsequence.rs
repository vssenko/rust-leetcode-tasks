//! Increasing triplet subsequences
//! https://leetcode.com/problems/increasing-triplet-subsequence
//! Given an integer array nums, return true if there exists a triple of indices (i, j, k)
//! such that i < j < k and nums[i] < nums[j] < nums[k]. If no such indices exists, return false.
//! implemented pretty badly, considered elegant (but unclear for me) best solution.

struct Solution {}

impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return false;
        }

        let mut max_ordered: Vec<i32> = nums[2..].to_vec();

        max_ordered.sort_by(|a, b| b.cmp(a));

        let mut left_min = nums[0];

        let mut max_index: usize = 0;
        let mut right_max = i32::MIN;

        for j_ind in 1..(nums.len() - 1) {
            let value = nums[j_ind];

            // going left to right, means ADDING value to mins of left side
            // and removing value from max check right side
            left_min = left_min.min(value);

            if (right_max <= value) {
                right_max = max_ordered[max_index];
                max_index += 1;
            }

            if left_min < value && right_max > value {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn increasing_triplet_1() {
        let nums: Vec<i32> = vec![1, 2, 3, 4, 5];

        let result = Solution::increasing_triplet(nums);

        assert!(result)
    }

    #[test]
    fn increasing_triplet_2() {
        let nums: Vec<i32> = vec![5, 4, 3, 2, 1];

        let result = Solution::increasing_triplet(nums);

        assert!(!result);
    }

    #[test]
    fn increasing_triplet_3() {
        let nums: Vec<i32> = vec![2, 1, 5, 0, 4, 6];

        let result = Solution::increasing_triplet(nums);

        assert!(result);
    }

    #[test]
    fn increasing_triplet_4() {
        let nums: Vec<i32> = vec![2, 1, 3, 2, 3, 4];

        let result = Solution::increasing_triplet(nums);

        assert!(result);
    }

    #[test]
    fn increasing_triplet_5() {
        let nums: Vec<i32> = vec![2, 4, -2, -3];

        let result = Solution::increasing_triplet(nums);

        assert!(!result);
    }

    #[test]
    fn increasing_triplet_6() {
        let nums: Vec<i32> = vec![20, 100, 10, 12, 5, 13];

        let result = Solution::increasing_triplet(nums);

        assert!(result);
    }

    #[test]
    fn increasing_triplet_7() {
        let nums: Vec<i32> = vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ];

        let result = Solution::increasing_triplet(nums);

        assert!(!result);
    }
}
