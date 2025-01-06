//! Kth Largest Element in an Array
//! Solution should not include sorting
//! https://leetcode.com/problems/kth-largest-element-in-an-array

use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution;

impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        // use binary heap
        let mut heap = BinaryHeap::new();
        for num in nums {
            heap.push(Reverse(num));
            if (heap.len() as i32 > k) {
                heap.pop();
            }
        }

        let value = heap.pop().unwrap();

        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn find_kth_largest_1() {
        let nums = vec![3, 2, 1, 5, 6, 4];
        let k = 2;

        let result = Solution::find_kth_largest(nums, k);

        assert_eq!(result, 5)
    }
}
