//! Determine if Two Strings Are Close
//! Two strings are considered close if you can attain one from the other using the following operations:
//! Operation 1: Swap any two existing characters.
//! Operation 2: Transform every occurrence of one existing character into another existing character, and do the same with the other character.
//! You can use the operations on either string as many times as necessary.
//! Given two strings, word1 and word2, return true if word1 and word2 are close, and false otherwise.v
//! https://leetcode.com/problems/determine-if-two-strings-are-close

struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    fn count<T>(it: T) -> HashMap<T::Item, usize>
    where
        T: IntoIterator,
        T::Item: std::cmp::Eq + std::hash::Hash,
    {
        let mut occurences = HashMap::new();
        for ch in it {
            *occurences.entry(ch).or_default() += 1;
        }
        occurences
    }

    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        let (map1, map2) = (Self::count(word1.chars()), Self::count(word2.chars()));

        if map1.len() != map2.len() {
            return false;
        }

        for m_1_k in map1.keys() {
            if !map2.contains_key(m_1_k) {
                return false;
            }
        }

        let occur_of_occur_1 = Self::count(map1.values());
        let occur_of_occur_2 = Self::count(map2.values());

        occur_of_occur_1 == occur_of_occur_2
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn close_strings_1() {
        let result = Solution::close_strings("abc".into(), "bca".into());

        assert!(result);
    }

    #[test]
    fn close_strings_2() {
        let result = Solution::close_strings("a".into(), "aa".into());

        assert!(!result);
    }

    #[test]
    fn close_strings_3() {
        let result = Solution::close_strings("cabbba".into(), "abbccc".into());

        assert!(result);
    }

    #[test]
    fn close_strings_4() {
        let result = Solution::close_strings("Vitaly".into(), "Vitaly".into());

        assert!(result);
    }

    #[test]
    fn close_strings_5() {
        let result = Solution::close_strings("abbzzca".into(), "babzzcz".into());

        assert!(!result);
    }

    #[test]
    fn close_strings_6() {
        let result = Solution::close_strings("uau".into(), "ssx".into());

        assert!(!result);
    }
}
