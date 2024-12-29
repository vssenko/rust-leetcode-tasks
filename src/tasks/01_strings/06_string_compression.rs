//! String compression
//! https://leetcode.com/problems/string-compression

struct Solution {}

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        if chars.is_empty() {
            return 0;
        }

        let mut result: Vec<char> = Vec::with_capacity(chars.len());
        let mut i = 0;
        let mut saved_char: char = chars[0];
        let mut char_counter = 0;

        for c in chars.iter() {
            if saved_char == *c {
                char_counter += 1;
            } else {
                result.push(saved_char);
                if (char_counter > 1) {
                    result.extend(char_counter.to_string().chars());
                }
                saved_char = *c;
                char_counter = 1;
            }
        }

        // process last saved
        result.push(saved_char);
        if (char_counter > 1) {
            result.extend(char_counter.to_string().chars());
        }

        chars.clear();
        chars.extend_from_slice(&result);

        chars.len().try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn string_compression_1() {
        let mut input: Vec<char> = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];

        let result = Solution::compress(&mut input);

        assert_eq!(result, 6);
        assert_eq!(input[0..6], ['a', '2', 'b', '2', 'c', '3']);
    }

    #[test]
    fn string_compression_2() {
        let mut input: Vec<char> = vec!['a'];

        let result = Solution::compress(&mut input);

        assert_eq!(result, 1);
        assert_eq!(input, ['a']);
    }

    #[test]
    fn string_compression_3() {
        let mut input: Vec<char> = vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
        ];

        let result = Solution::compress(&mut input);

        assert_eq!(result, 4);
        assert_eq!(input, ['a', 'b', '1', '2']);
    }
}
