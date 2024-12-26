/// Greates common divisor of strings
/// https://leetcode.com/problems/greatest-common-divisor-of-strings

struct Solution {}

impl Solution {
    fn check_pattern(str: &str, pattern: &str) -> bool {
        if str.len() % pattern.len() > 0 {
            return false;
        }

        for i in 0..(str.len() / pattern.len()) {
            let start = i * pattern.len();
            let end = start + pattern.len();
            if str[start..end] != *pattern {
                return false;
            }
        }

        true
    }

    fn get_max_pattern(str1: &str, str2: &str) -> String {
        let mut pattern = String::new();

        for (char_1, char_2) in std::iter::zip(str1.chars(), str2.chars()) {
            if char_1 != char_2 {
                break;
            }
            pattern.push(char_1);
        }

        pattern
    }

    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if (str1.len() < 1 || str2.len() < 1) {
            return "".into();
        }

        let mut pattern = Self::get_max_pattern(str1.as_str(), str2.as_str());

        if pattern.len() == 0 {
            return "".into();
        }

        while pattern.len() > 0 {
            let first_ok = Self::check_pattern(str1.as_str(), pattern.as_str());
            let second_ok = Self::check_pattern(str2.as_str(), pattern.as_str());

            if (first_ok && second_ok) {
                return pattern;
            }

            pattern.pop();
        }

        return "".into();
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn gcsd_1() {
        let result = Solution::gcd_of_strings("ABCABC".into(), "ABC".into());
        assert_eq!(result, "ABC");
    }

    #[test]
    fn gcsd_2() {
        let result = Solution::gcd_of_strings("AAAAAA".into(), "AAA".into());
        assert_eq!(result, "AAA");
    }

    #[test]
    fn gcsd_3() {
        let result = Solution::gcd_of_strings("LEET".into(), "CODE".into());
        assert_eq!(result, "");
    }

    #[test]
    fn gcsd_4() {
        let result = Solution::gcd_of_strings("ABABAB".into(), "ABAB".into());
        assert_eq!(result, "AB");
    }
}
