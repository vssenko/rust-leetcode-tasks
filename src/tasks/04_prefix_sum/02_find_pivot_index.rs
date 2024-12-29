//! Find Pivot Index
//! Given an array of integers nums, calculate the pivot index of this array.
//! The pivot index is the index where the sum of all the numbers strictly to the left of the index is equal to the sum of all the numbers strictly to the index's right.
//! If the index is on the left edge of the array, then the left sum is 0 because there are no elements to the left. This also applies to the right edge of the array.
//! Return the leftmost pivot index. If no such index exists, return -1.
//! https://leetcode.com/problems/find-pivot-index

struct Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut left_sum: i32 = 0;
        let mut right_sum: i32 = nums.iter().sum();

        for (i, v) in nums.iter().enumerate() {
            right_sum -= *v;

            if left_sum == right_sum {
                return i as i32;
            }

            left_sum += *v;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn pivot_index_1() {
        let nums = vec![1, 7, 3, 6, 5, 6];

        let result = Solution::pivot_index(nums);

        assert_eq!(result, 3);
    }

    #[test]
    fn pivot_index_2() {
        let nums = vec![1, 2, 3];

        let result = Solution::pivot_index(nums);

        assert_eq!(result, -1);
    }

    #[test]
    fn pivot_index_3() {
        let nums = vec![2, 1, -1];

        let result = Solution::pivot_index(nums);

        assert_eq!(result, 0);
    }

    #[test]
    fn pivot_index_4() {
        let nums = vec![1, -1, 2];

        let result = Solution::pivot_index(nums);

        assert_eq!(result, 2);
    }

    fn pivot_index_5() {
        let nums = vec![-1, -1, 0, 1, 1, 0];

        let result = Solution::pivot_index(nums);

        assert_eq!(result, 5);
    }
}
