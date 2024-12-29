//! Maximum Number of Vowels in a Substring of Given Length
//! Given a string s and an integer k, return the maximum number of vowel letters in any substring of s with length k.
//! https://leetcode.com/problems/maximum-number-of-vowels-in-a-substring-of-given-length

struct Solution;

impl Solution {
    #[allow(clippy::explicit_counter_loop)]
    pub fn max_vowels(s: String, k: i32) -> i32 {
        fn is_vowel(c: &char) -> bool {
            matches!(*c, 'a' | 'e' | 'i' | 'o' | 'u')
        }

        let k = k as usize;

        let chars = s.chars().collect::<Vec<char>>();

        let mut current_vow_count = 0;

        chars[0..k].iter().for_each(|c| {
            if (is_vowel(c)) {
                current_vow_count += 1;
            }
        });

        let mut max_vow_count = current_vow_count;

        let mut win_start_ind = 0;
        for i in k..chars.len() {
            match (is_vowel(&chars[win_start_ind]), is_vowel(&chars[i])) {
                (true, true) => {}
                (true, false) => {
                    current_vow_count -= 1;
                }
                (false, true) => current_vow_count += 1,
                (false, false) => {}
            }

            max_vow_count = max_vow_count.max(current_vow_count);

            if (max_vow_count == k) {
                return max_vow_count as i32;
            }
            win_start_ind += 1;
        }

        max_vow_count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn max_vowels_1() {
        let s = "abciiidef".to_string();

        let k = 3;

        let result = Solution::max_vowels(s, k);

        assert_eq!(result, 3);
    }
}
