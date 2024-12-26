/// Greates common divisor of strings
/// https://leetcode.com/problems/greatest-common-divisor-of-strings

struct Solution {}

impl Solution {
    #[allow(unused)]
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        unimplemented!();
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
}
