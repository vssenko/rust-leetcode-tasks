//! Unique Number of Occurrences
//! Given an array of integers arr, return true if the number of occurrences of each value in the array is unique or false otherwise.
//! https://leetcode.com/problems/unique-number-of-occurrences

struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut counted_occurences: HashMap<i32, usize> = HashMap::with_capacity(arr.len() / 4);

        for i in arr {
            *counted_occurences.entry(i).or_default() += 1;
        }

        let mut occur_set: HashSet<usize> = HashSet::with_capacity(counted_occurences.len());

        for val in counted_occurences.values() {
            if !occur_set.insert(*val) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn unique_occurrences_1() {
        let arr = vec![1, 2, 2, 1, 1, 3];

        let result = Solution::unique_occurrences(arr);

        assert!(result);
    }
}
