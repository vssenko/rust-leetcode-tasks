//! Successful Pairs of Spells and Potions
//! You are given two positive integer arrays spells and potions, of length n and m respectively,
//! where spells[i] represents the strength of the ith spell and potions[j] represents the strength of the jth potion.
//! You are also given an integer success. A spell and potion pair is considered successful if the product of their strengths is at least success.
//! Return an integer array pairs of length n where pairs[i] is the number of potions that will form a successful pair with the ith spell.
//! https://leetcode.com/problems/successful-pairs-of-spells-and-potions

struct Solution;

impl Solution {
    /// count how many array elements are GREATER OR EQUAL than N. Arr should be sorted DESC
    fn count_greater_in_sorted_array<T: PartialOrd + std::fmt::Debug + std::fmt::Display>(
        arr: &[T],
        n: T,
    ) -> usize {
        if arr.is_empty() {
            return 0;
        }

        let mut left = 0;
        let mut right = arr.len() - 1;

        while right - left > 2 {
            let mid = (left + right) / 2;
            match arr[mid].partial_cmp(&n).unwrap() {
                std::cmp::Ordering::Less => right = mid,
                std::cmp::Ordering::Equal => {
                    left = mid;
                    break;
                }
                std::cmp::Ordering::Greater => left = mid,
            }
        }

        while left < arr.len() && arr[left] >= n {
            left += 1;
        }

        left
    }

    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        potions.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());

        spells
            .into_iter()
            .map(|spell_str| {
                let scaled_potions: Vec<i64> = potions
                    .iter()
                    .map(|p| *p as i64 * spell_str as i64)
                    .collect();
                Self::count_greater_in_sorted_array(&scaled_potions, success) as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn successful_pairs_1() {
        let spells = vec![5, 1, 3];
        let potions = vec![1, 2, 3, 4, 5];
        let success = 7;

        let result = Solution::successful_pairs(spells, potions, success);

        assert_eq!(result, [4, 0, 3]);
    }

    #[test]
    pub fn successful_pairs_2() {
        let spells = vec![3, 1, 2];
        let potions = vec![8, 5, 8];
        let success = 16;

        let result = Solution::successful_pairs(spells, potions, success);

        assert_eq!(result, [2, 0, 2]);
    }

    #[test]
    pub fn successful_pairs_3() {
        let spells = vec![100, 1, 2];
        let potions = vec![8, 5, 8];
        let success = 16;

        let result = Solution::successful_pairs(spells, potions, success);

        assert_eq!(result, [3, 0, 2]);
    }

    #[test]
    pub fn successful_pairs_4() {
        let spells = vec![
            40, 11, 24, 28, 40, 22, 26, 38, 28, 10, 31, 16, 10, 37, 13, 21, 9, 22, 21, 18, 34, 2,
            40, 40, 6, 16, 9, 14, 14, 15, 37, 15, 32, 4, 27, 20, 24, 12, 26, 39, 32, 39, 20, 19,
            22, 33, 2, 22, 9, 18, 12, 5,
        ];
        let potions = vec![
            31, 40, 29, 19, 27, 16, 25, 8, 33, 25, 36, 21, 7, 27, 40, 24, 18, 26, 32, 25, 22, 21,
            38, 22, 37, 34, 15, 36, 21, 22, 37, 14, 31, 20, 36, 27, 28, 32, 21, 26, 33, 37, 27, 39,
            19, 36, 20, 23, 25, 39, 40,
        ];
        let success = 600;

        let result = Solution::successful_pairs(spells, potions, success);

        assert_eq!(
            result,
            [
                48, 0, 32, 37, 48, 22, 33, 47, 37, 0, 43, 6, 0, 46, 0, 21, 0, 22, 21, 14, 46, 0,
                48, 48, 0, 6, 0, 0, 0, 3, 46, 3, 45, 0, 34, 20, 32, 0, 33, 47, 45, 47, 20, 18, 22,
                45, 0, 22, 0, 14, 0, 0
            ]
        );
    }
}
