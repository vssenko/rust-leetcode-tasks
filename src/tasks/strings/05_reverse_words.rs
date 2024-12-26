//! Reverse words
//! https://leetcode.com/problems/reverse-words-in-a-string

struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_ascii_whitespace()
            .map(|s| s.trim().to_string())
            .rev()
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn reverse_words_1() {
        let s = "   a good   example  ".to_string();

        let result = Solution::reverse_words(s);

        assert_eq!(result, "example good a");
    }
}
