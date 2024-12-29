//! Removing Stars From a String
//! You are given a string s, which contains stars *.
//! In one operation, you can:
//! Choose a star in s.
//! Remove the closest non-star character to its left, as well as remove the star itself.
//! Return the string after all stars have been removed.
//! https://leetcode.com/problems/removing-stars-from-a-string

struct Solution;

impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut result: String = String::with_capacity(s.len());

        for c in s.chars() {
            if c == '*' {
                result.pop();
            } else {
                result.push(c);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn remove_stars_1() {
        let s = "leet**cod*e".to_string();

        let result = Solution::remove_stars(s);

        assert_eq!(result, "lecoe");
    }

    #[test]
    fn remove_stars_2() {
        let s = "erase*****".to_string();

        let result = Solution::remove_stars(s);

        assert_eq!(result, "");
    }
}
