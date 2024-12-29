//! Equal Row and Column Pairs
//! Given a 0-indexed n x n integer matrix grid, return the number of pairs (ri, cj) such that row ri and column cj are equal.
//! A row and column pair is considered equal if they contain the same elements in the same order (i.e., an equal array).
//! https://leetcode.com/problems/equal-row-and-column-pairs

struct Solution;

use std::collections::HashMap;

impl Solution {
    fn nums_to_key<I, T>(nums: I, len: usize) -> String
    where
        I: Iterator<Item = T>,
        T: ToString,
    {
        let mut result = String::with_capacity(len * 2);
        for num in nums {
            result.push_str(num.to_string().as_str());
            result.push('_');
        }
        result
    }

    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let square_len = grid.len();

        let mut col_map: HashMap<String, usize> = HashMap::with_capacity(grid.len());
        let mut row_map: HashMap<String, usize> = HashMap::with_capacity(grid.len());

        for row_idx in 0..grid.len() {
            let row_key = Self::nums_to_key(grid[row_idx].iter(), square_len);
            *row_map.entry(row_key).or_default() += 1;
            let col_key = Self::nums_to_key(grid.iter().map(|r| r[row_idx]), square_len);

            *col_map.entry(col_key).or_default() += 1;
        }

        let mut result = 0;

        for row_seq in row_map {
            let col_seq = col_map.get(&row_seq.0);
            if let Some(col_counts) = col_seq {
                result += col_counts * row_seq.1;
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn equal_pairs_1() {
        let grid = vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]];

        let result = Solution::equal_pairs(grid);

        assert_eq!(result, 1);
    }

    #[test]
    fn equal_pairs_2() {
        let grid = vec![
            vec![3, 1, 2, 2],
            vec![1, 4, 4, 5],
            vec![2, 4, 2, 2],
            vec![2, 4, 2, 2],
        ];

        let result = Solution::equal_pairs(grid);

        assert_eq!(result, 3);
    }
}
