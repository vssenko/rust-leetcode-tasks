//! Longest Palindromic Substring
//! https://leetcode.com/problems/longest-palindromic-substring/

pub struct Solution {}

impl Solution {
    unsafe fn _expand(arr: &[char], mut left: usize, mut right: usize) -> usize {
        if *arr.get_unchecked(left) != *arr.get_unchecked(right) {
            return 0;
        }

        let last = arr.len() - 1;

        while left > 0 && right < last {
            if *arr.get_unchecked(left - 1) == *arr.get_unchecked(right + 1) {
                left -= 1;
                right += 1;
            } else {
                break;
            }
        }

        right - left + 1
    }

    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return "".into();
        }
        let s_chars = s.chars().collect::<Vec<_>>();

        let mut start = 0;
        let mut len = 1;
        let mut half_len = 0;

        for i in 0..(s_chars.len() - 1) {
            if (s_chars.len() - i < half_len) {
                break;
            }
            let single_centered_len = unsafe { Self::_expand(&s_chars, i, i) };
            let pair_centered_len = unsafe { Self::_expand(&s_chars, i, i + 1) };

            if single_centered_len > len {
                start = i - single_centered_len / 2;
                len = single_centered_len;
                half_len = len / 2;
            }
            if pair_centered_len > len {
                start = i + 1 - pair_centered_len / 2;
                len = pair_centered_len;
                half_len = len / 2;
            }
        }

        s_chars[start..(start + len)].iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palindrome_test_1() {
        let s = "babad".to_string();
        let result = Solution::longest_palindrome(s);
        if result == "aba" {
            assert_eq!(result, "aba");
        } else {
            assert_eq!(result, "bab")
        }
    }

    #[test]
    fn palindrome_test_2() {
        let s = "cbbd".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, "bb");
    }

    #[test]
    fn palindrome_test_3() {
        let s = "a".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, "a");
    }

    #[test]
    fn palindrome_test_4() {
        let s = "ccc".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, "ccc");
    }

    #[test]
    fn palindrome_test_5() {
        let s = "abcde".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, "a");
    }

    #[test]
    fn palindrome_test_6() {
        let s = "bsnetpqmwhqjunkooftuosgkmkxpmvuehtlpwpktltwlvpdaycnhewdbdrhluyjldecezujgxixehsmjjuyerpllrvzqskizkulqzowzfvqcdsllvgupndbaiuzihcxklvxbodpnrymwobhlvllybdlfabfvnizjpriapuzszdhohfgezayokrivbgbgingspoxsridokhwekawchjdcpylvefubulvxneuizglrjktfcdirwnpqztdpsicslzaeiaibrepifxpxfkczwoumkkuaqkbjhxvasxflmrctponwwenvmtdaqaavubyrzbqjbjxpejmucwunanxwpomqyondyjuzxmzpevxqmbkrwcpdiiph".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, "fxpxf");
    }
}
