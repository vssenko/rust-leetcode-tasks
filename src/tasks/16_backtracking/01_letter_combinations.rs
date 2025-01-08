//! Letter Combinations of a Phone Number
//! Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent.
//! Return the answer in any order.
//! https://leetcode.com/problems/letter-combinations-of-a-phone-number

struct Solution;

impl Solution {
    fn letters_for_digit(digit: char) -> Vec<char> {
        match digit {
            '2' => vec!['a', 'b', 'c'],
            '3' => vec!['e', 'd', 'f'],
            '4' => vec!['g', 'h', 'i'],
            '5' => vec!['j', 'k', 'l'],
            '6' => vec!['m', 'n', 'o'],
            '7' => vec!['p', 'q', 'r', 's'],
            '8' => vec!['t', 'u', 'v'],
            '9' => vec!['w', 'x', 'y', 'z'],
            _ => unreachable!(),
        }
    }
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let mut result: Vec<String> = Vec::with_capacity(digits.len().pow(2));

        let letter_arrays: Vec<Vec<char>> = digits.chars().map(Self::letters_for_digit).collect();

        for first_c in letter_arrays[0].iter() {
            result.push(String::from(*first_c));
        }

        println!("{:?}", letter_arrays);

        for char_set in letter_arrays.into_iter().skip(1) {
            let mut new_set: Vec<String> = Vec::with_capacity(result.len());
            for c in char_set {
                for r in &result {
                    let mut new_str = r.clone();
                    new_str.push(c);
                    new_set.push(new_str);
                }
            }

            result = new_set;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn letter_combinations_1() {
        let digits = "23".to_string();

        let mut result = Solution::letter_combinations(digits);

        result.sort();

        assert_eq!(
            result,
            ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
                .into_iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
        )
    }
}
