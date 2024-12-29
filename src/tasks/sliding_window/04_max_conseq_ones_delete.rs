//! Longest Subarray of 1's After Deleting One Element
//! Given a binary array nums, you should delete one element from it.
//! Return the size of the longest non-empty subarray containing only 1's in the resulting array. Return 0 if there is no such subarray.
//! https://leetcode.com/problems/longest-subarray-of-1s-after-deleting-one-element

struct Solution;

impl Solution {
    #[allow(clippy::explicit_counter_loop)]
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let k = 1;
        let mut zeroes_pos: Vec<i32> = Vec::with_capacity(nums.len() / 2);
        zeroes_pos.push(-1);

        nums.iter()
            .enumerate()
            .filter(|(i, v)| **v == 0)
            .for_each(|(i, v)| zeroes_pos.push(i as i32));

        zeroes_pos.push(nums.len() as i32);

        if zeroes_pos.len() - 2 < k as usize {
            return nums.len() as i32 - 1;
        }

        let mut win_first_idx = 0;
        let mut win_last_idx = k;

        let mut max_len = 0;
        for win_last_idx in (k as usize)..(zeroes_pos.len() - 1) {
            // this solution is the same as 03_max_conseq_ones,
            // with the only difference that we need to decrease len by additional 1 (k)
            // bc we remove 0 instead of flipping it to 1.
            let len = zeroes_pos[win_last_idx + 1] - zeroes_pos[win_first_idx] - 1 - k;
            max_len = max_len.max(len);
            win_first_idx += 1;
        }

        max_len
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn longest_subarray_1() {
        let nums = vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0];

        let result = Solution::longest_subarray(nums);

        assert_eq!(result, 4);
    }

    #[test]
    fn longest_subarray_2() {
        let nums = vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1];

        let result = Solution::longest_subarray(nums);

        assert_eq!(result, 5);
    }

    #[test]
    fn longest_subarray_3() {
        let nums = vec![1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1, 1];

        let result = Solution::longest_subarray(nums);

        assert_eq!(result, 8);
    }

    #[test]
    fn longest_subarray_5() {
        let nums = vec![1, 1, 0, 1];

        let result = Solution::longest_subarray(nums);

        // after removing 0 there will be 3 subseq 1
        assert_eq!(result, 3);
    }

    #[test]
    fn longest_subarray_6() {
        let nums = vec![1, 1, 1, 1];

        let result = Solution::longest_subarray(nums);

        // even wihtout zeroes we still need to remove one element
        assert_eq!(result, 3);
    }
}
