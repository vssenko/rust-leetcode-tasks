//! Longest Common Subsequence
//! Given two strings text1 and text2, return the length of their longest common subsequence.
//! If there is no common subsequence, return 0.
//! A subsequence of a string is a new string generated from the original string
//! with some characters (can be none) deleted without changing the relative order of the remaining characters.
//! For example, "ace" is a subsequence of "abcde".
//! A common subsequence of two strings is a subsequence that is common to both strings.

use std::{cell::RefCell, collections::HashMap, rc::Rc};

struct Solution;

// I am always too dumb to do things implicit.
#[derive(Debug)]
struct CommonSubSeqProgress {
    s_1_idx: usize,
    s_2_idx: usize,
    seq_len: usize,
}

struct SharedSubSeqData {
    max_seq_len: usize,
}

impl Solution {
    pub fn longest_common_subsequence(s_1: String, s_2: String) -> i32 {
        let s_1 = s_1.chars().collect::<Vec<char>>();
        let s_2 = s_2.chars().collect::<Vec<char>>();

        let mut grid: Vec<Vec<usize>> = vec![vec![0; s_2.len()]; s_1.len()];

        // grid[i][j] is longest seq between s_1[0..=i] and s_2[0..=j]
        for (i_1, c_1) in s_1.into_iter().enumerate() {
            for (i_2, c_2) in s_2.iter().enumerate() {
                grid[i_1][i_2] = if c_1 == *c_2 {
                    let prev = if i_1 == 0 || i_2 == 0 {
                        0
                    } else {
                        grid[i_1 - 1][i_2 - 1]
                    };
                    prev + 1
                } else {
                    let prev_1 = if i_1 == 0 { 0 } else { grid[i_1 - 1][i_2] };
                    let prev_2 = if i_2 == 0 { 0 } else { grid[i_1][i_2 - 1] };
                    prev_1.max(prev_2)
                }
            }
        }

        *grid.last().unwrap().last().unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn longest_common_subsequence_1() {
        let s_1 = "abcde".to_string();
        let s_2 = "ace".to_string();

        let result = Solution::longest_common_subsequence(s_1, s_2);

        assert_eq!(result, 3);
    }

    #[test]
    fn longest_common_subsequence_2() {
        let s_1 = "abc".to_string();
        let s_2 = "abc".to_string();

        let result = Solution::longest_common_subsequence(s_1, s_2);

        assert_eq!(result, 3);
    }

    #[test]
    fn longest_common_subsequence_3() {
        let s_1 = "abc".to_string();
        let s_2 = "def".to_string();

        let result = Solution::longest_common_subsequence(s_1, s_2);

        assert_eq!(result, 0);
    }

    #[test]
    fn longest_common_subsequence_4() {
        let s_1 = "hergrwzsjgjmnwfwjyxyhafstetgbydobynmxabavodsfwbqbevozkjkpwvw".to_string();
        let s_2 = "pgrwlabutilctsrgbgxorwjezspgxwredqjklabwterwzyzstwpobwjujwjkb".to_string();

        let result = Solution::longest_common_subsequence(s_1, s_2);

        assert_eq!(result, 19);
    }
}
