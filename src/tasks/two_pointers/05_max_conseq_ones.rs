//! Max consequites ones
//! Given a binary array nums and an integer k, return the maximum number of consecutive 1's in the array if you can flip at most k 0's.
//! https://leetcode.com/problems/max-consecutive-ones-iii

struct Solution;

impl Solution {
    #[allow(clippy::explicit_counter_loop)]
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut zeroes_pos: Vec<i32> = Vec::with_capacity(nums.len() / 2);
        zeroes_pos.push(-1);

        nums.iter()
            .enumerate()
            .filter(|(i, v)| **v == 0)
            .for_each(|(i, v)| zeroes_pos.push(i as i32));

        zeroes_pos.push(nums.len() as i32);

        if zeroes_pos.len() - 2 < k as usize {
            return nums.len() as i32;
        }

        let mut win_first_idx = 0;
        let mut win_last_idx = k;

        let mut max_len = 0;
        for win_last_idx in (k as usize)..(zeroes_pos.len() - 1) {
            let len = zeroes_pos[win_last_idx + 1] - zeroes_pos[win_first_idx] - 1;
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
    fn longest_ones_1() {
        let nums = vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0];
        let k = 2;

        let result = Solution::longest_ones(nums, k);

        assert_eq!(result, 6);
    }

    #[test]
    fn longest_ones_2() {
        let nums = vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1];
        let k = 3;

        let result = Solution::longest_ones(nums, k);

        assert_eq!(result, 10);
    }

    #[test]
    fn longest_ones_3() {
        let nums = vec![1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1, 1];
        let k = 1;

        let result = Solution::longest_ones(nums, k);

        assert_eq!(result, 9);
    }

    #[test]
    fn longest_ones_4() {
        let nums = vec![1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1, 1];
        let k = 0;

        let result = Solution::longest_ones(nums, k);

        assert_eq!(result, 4);
    }

    #[test]
    fn longest_ones_5() {
        let nums = vec![1, 1, 0, 1];
        let k = 2;

        let result = Solution::longest_ones(nums, k);

        assert_eq!(result, 4);
    }

    #[test]
    fn longest_ones_6() {
        let nums = vec![1, 1, 1, 1];
        let k = 2;

        let result = Solution::longest_ones(nums, k);

        assert_eq!(result, 4);
    }
}
