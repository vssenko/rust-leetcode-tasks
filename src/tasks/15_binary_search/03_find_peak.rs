//! Find Peak Element
//! A peak element is an element that is strictly greater than its neighbors.
//! Given a 0-indexed integer array nums, find a peak element, and return its index. If the array contains multiple peaks, return the index to any of the peaks.
//! You may imagine that nums[-1] = nums[n] = -∞. In other words, an element is always considered to be strictly greater than a neighbor that is outside the array.
//! You must write an algorithm that runs in O(log n) time.
//! https://leetcode.com/problems/find-peak-element

//! ???

struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let max_ind = nums.len() - 1;
        for i in 0..=max_ind {
            let prev = if i == 0 { i32::MIN } else { nums[i - 1] };

            let next = if i == max_ind { i32::MIN } else { nums[i + 1] };

            if nums[i] > prev && nums[i] > next {
                return i as i32;
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn find_peak_element_1() {
        let nums = vec![1, 2, 3, 1];

        let result = Solution::find_peak_element(nums);

        assert_eq!(result, 2);
    }
}
