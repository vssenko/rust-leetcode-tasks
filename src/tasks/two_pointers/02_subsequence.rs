struct Solution;

impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return true;
        }

        if t.is_empty() {
            return false;
        }

        let s_chars: Vec<char> = s.chars().collect();

        let mut j = 0;

        for t_c in t.chars() {
            if s_chars[j] == t_c {
                j += 1;
                if j == s_chars.len() {
                    return true;
                }
            }
        }

        j == s_chars.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn is_subsequence_1() {
        let s = "abc".to_string();
        let t = "ahbgdc".to_string();

        let result = Solution::is_subsequence(s, t);

        assert!(result)
    }
}
