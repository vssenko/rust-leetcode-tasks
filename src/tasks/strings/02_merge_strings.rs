/// Merge Strings Alternately
/// https://leetcode.com/problems/merge-strings-alternately
pub struct Solution {}

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        if (word1.len() < 1) {
            return word2;
        }

        if (word2.len() < 1) {
            return word1;
        }

        let result = std::iter::zip(word1.chars(), word2.chars()).fold(
            String::with_capacity(word1.len() + word2.len()),
            |mut acc, (char_1, char_2)| {
                acc.push(char_1);
                acc.push(char_2);
                acc
            },
        );

        result
            + if word1.len() > word2.len() {
                &word1[word2.len()..]
            } else {
                &word2[word1.len()..]
            }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_strings_test_1() {
        let result = Solution::merge_alternately("abc".into(), "pqr".into());
        assert_eq!(result, "apbqcr");
    }

    #[test]
    fn merge_strings_test_2() {
        let result = Solution::merge_alternately("ab".into(), "pqrs".into());
        assert_eq!(result, "apbqrs");
    }

    #[test]
    fn merge_strings_test_3() {
        let result = Solution::merge_alternately("abcd".into(), "pq".into());
        assert_eq!(result, "apbqcd");
    }

    #[test]
    fn merge_strings_test_4() {
        let result = Solution::merge_alternately("".into(), "pq".into());
        assert_eq!(result, "pq");
    }

    #[test]
    fn merge_strings_test_5() {
        let result = Solution::merge_alternately("123qwe".into(), "".into());
        assert_eq!(result, "123qwe");
    }

    #[test]
    fn merge_strings_test_6() {
        let result = Solution::merge_alternately("".into(), "".into());
        assert_eq!(result, "");
    }

    #[test]
    fn merge_strings_test_7() {
        let result = Solution::merge_alternately("qweйцукasd".into(), "ячсzxc".into());
        assert_eq!(result, "qяwчeсйzцxуcкasd");
    }
}
