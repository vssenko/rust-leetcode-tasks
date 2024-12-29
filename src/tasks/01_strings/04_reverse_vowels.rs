//! Reverse vowels in string
//! https://leetcode.com/problems/reverse-vowels-of-a-string
use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        if s.len() < 2 {
            return s;
        }

        let known_vowels: HashSet<char> = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']
            .iter()
            .cloned()
            .collect();

        let mut vowels_positions: Vec<usize> = Vec::with_capacity(s.len() / 3);
        let mut vowels: Vec<char> = Vec::with_capacity(s.len() / 3);

        let mut result: Vec<char> = Vec::with_capacity(s.len());
        result.resize_with(s.len(), Default::default);

        for (i, char) in s.chars().enumerate() {
            if known_vowels.contains(&char) {
                vowels_positions.push(i);
                vowels.push(char);
            } else {
                unsafe { *result.get_unchecked_mut(i) = char }
            }
        }

        for v_pos in vowels_positions {
            unsafe { *result.get_unchecked_mut(v_pos) = vowels.pop().unwrap() }
        }

        result.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn reverse_vowels_1() {
        let result = Solution::reverse_vowels("IceCreAm".into());
        assert_eq!(result, "AceCreIm");
    }

    #[test]
    fn reverse_vowels_2() {
        let result = Solution::reverse_vowels("leetcode".into());
        assert_eq!(result, "leotcede");
    }
}
