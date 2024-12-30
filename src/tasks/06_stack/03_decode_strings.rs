//! Decode String
//! Given an encoded string, return its decoded string.
//! The encoding rule is: k[encoded_string], where the encoded_string inside the square brackets is being repeated exactly k times.
//! You may assume that the input string is always valid, containing only digits, square brackets, and lowercase English letters.
//! https://leetcode.com/problems/decode-string

// Solution represents recursive finite state machine.

struct Solution;

const OPEN_BRACKET: char = '[';
const CLOSING_BRACKET: char = ']';

#[derive(Debug)]
enum DecoderState {
    Base,
    ReadingDigit,
    InBrackets,
}

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut result: String = String::with_capacity(s.len());

        let mut state: DecoderState = DecoderState::Base;

        let mut digits_cache: String = String::with_capacity(10);
        let mut in_bracket_cache: String = String::with_capacity(10);

        // for nested brackets
        let mut brackets_count = 0;

        for char in s.chars() {
            // separated internal counter
            match char {
                OPEN_BRACKET => brackets_count += 1,
                CLOSING_BRACKET => brackets_count -= 1,
                _ => {}
            }

            match state {
                DecoderState::Base => match char.is_ascii_digit() {
                    true => {
                        digits_cache.push(char);
                        state = DecoderState::ReadingDigit;
                    }
                    false => result.push(char),
                },
                DecoderState::ReadingDigit => match (char.is_ascii_digit(), char) {
                    (true, _) => {
                        digits_cache.push(char);
                    }
                    (false, OPEN_BRACKET) => {
                        state = DecoderState::InBrackets;
                    }
                    (false, _) => {
                        result.push_str(digits_cache.as_str());
                        digits_cache.clear();
                        result.push(char);
                    }
                },
                DecoderState::InBrackets => match (char, brackets_count) {
                    (CLOSING_BRACKET, 0) => {
                        let parsed_digit: usize = digits_cache.parse().unwrap();

                        let decoded_in_bracket_str = Self::decode_string(in_bracket_cache.clone());

                        for _ in 0..parsed_digit {
                            result.push_str(decoded_in_bracket_str.as_str());
                        }
                        in_bracket_cache.clear();
                        digits_cache.clear();
                        state = DecoderState::Base;
                    }
                    (CLOSING_BRACKET, _) => {
                        in_bracket_cache.push(char);
                    }
                    (_, _) => {
                        in_bracket_cache.push(char);
                    }
                },
            }
        }

        if !digits_cache.is_empty() {
            result.push_str(&digits_cache);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn decode_string_1() {
        let s = "3[a]2[bc]".to_string();

        let result = Solution::decode_string(s);

        assert_eq!(result, "aaabcbc");
    }

    #[test]
    fn decode_string_2() {
        let s = "3[a2[c]]".to_string();

        let result = Solution::decode_string(s);

        assert_eq!(result, "accaccacc");
    }

    #[test]
    fn decode_string_3() {
        let s = "2[a2b]22".to_string();

        let result = Solution::decode_string(s);

        assert_eq!(result, "a2ba2b22");
    }
}
