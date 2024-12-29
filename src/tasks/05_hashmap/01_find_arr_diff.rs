//! Find the difference between two arrays
//! Given two 0-indexed integer arrays nums1 and nums2, return a list answer of size 2 where:
//! answer[0] is a list of all distinct integers in nums1 which are not present in nums2.
//! answer[1] is a list of all distinct integers in nums2 which are not present in nums1.
//! Note that the integers in the lists may be returned in any order.
//! https://leetcode.com/problems/find-the-difference-of-two-arrays

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        // Note that HashSet::from_iter works way faster than nums1.iter().collect()
        let hash1: HashSet<i32> = HashSet::from_iter(nums1);
        let hash2: HashSet<i32> = HashSet::from_iter(nums2);

        vec![
            hash1.difference(&hash2).copied().collect::<Vec<i32>>(),
            hash2.difference(&hash1).copied().collect::<Vec<i32>>(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn find_difference_1() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![2, 4, 6];

        let result = Solution::find_difference(nums1, nums2);

        let mut result0 = result[0].clone();
        result0.sort();

        let mut result1 = result[1].clone();
        result1.sort();

        assert_eq!(vec![result0, result1], [[1, 3], [4, 6]]);
    }
}
