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
    pub fn recursive_sub_seq_search(chars_1: &[char], chars_2: &[char]) -> usize {
        let mut max_len: usize = 0;

        let shared_sub_seq_data = SharedSubSeqData {
            max_seq_len: chars_2.len(),
        };

        #[allow(clippy::needless_range_loop)]
        fn _recursive_sub_seq_search<T>(
            chars_1: &[char],
            chars_2: &[char],
            mut progress: CommonSubSeqProgress,
            shared_sub_seq_data: &SharedSubSeqData,
            on_end: &mut T,
        ) where
            T: FnMut(CommonSubSeqProgress),
        {
            if progress.s_1_idx >= chars_1.len()
                || progress.s_2_idx >= chars_2.len()
                || shared_sub_seq_data.max_seq_len == progress.seq_len
            {
                on_end(progress);
                return;
            }

            let mut split: Vec<CommonSubSeqProgress> = Vec::with_capacity(2);

            for i in progress.s_1_idx..chars_1.len() {
                let c = chars_1[i];
                if let Some(found_char_position) = chars_2
                    .iter()
                    .enumerate()
                    .skip(progress.s_2_idx)
                    .find(|(i, c2)| **c2 == c)
                    .map(|(i, _)| i)
                {
                    // decided to use char
                    split.push(CommonSubSeqProgress {
                        s_1_idx: i + 1,
                        s_2_idx: found_char_position + 1,
                        seq_len: progress.seq_len + 1,
                    });
                    // decided not to use char
                    split.push(CommonSubSeqProgress {
                        s_1_idx: i + 1,
                        s_2_idx: progress.s_2_idx,
                        seq_len: progress.seq_len,
                    });
                    break;
                }
            }

            // we have finished without finding new state
            if split.is_empty() {
                on_end(progress);
                return;
            }

            for new_progress in split.into_iter() {
                _recursive_sub_seq_search(
                    chars_1,
                    chars_2,
                    new_progress,
                    shared_sub_seq_data,
                    on_end,
                );
            }
        }

        _recursive_sub_seq_search(
            chars_1,
            chars_2,
            CommonSubSeqProgress {
                s_1_idx: 0,
                s_2_idx: 0,
                seq_len: 0,
            },
            &shared_sub_seq_data,
            &mut |progress| {
                max_len = max_len.max(progress.seq_len);
            },
        );

        max_len
    }

    pub fn longest_common_subsequence(s_1: String, s_2: String) -> i32 {
        let longest_subseq_grid = vec![vec![0; s_2.len()]; s_1.len()];

        let (bigger_s, lesser_s) = if s_1.len() >= s_2.len() {
            (&s_1, &s_2)
        } else {
            (&s_2, &s_1)
        };

        if bigger_s.contains(lesser_s) {
            return lesser_s.len() as i32;
        }

        let bigger_s_chars: Vec<char> = bigger_s.chars().collect();
        let lesser_s_chars: Vec<char> = lesser_s.chars().collect();

        Self::recursive_sub_seq_search(&bigger_s_chars, &lesser_s_chars) as i32
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

        assert_eq!(result, 0);
    }
}
