//! Unique Paths
//! There is a robot on an m x n grid. The robot is initially located at the top-left corner (i.e., grid[0][0]).
//! The robot tries to move to the bottom-right corner (i.e., grid[m - 1][n - 1]).
//! The robot can only move either down or right at any point in time.
//! Given the two integers m and n, return the number of possible unique paths that the robot can take
//!  to reach the bottom-right corner.
//! The test cases are generated so that the answer will be less than or equal to 2 * 109.

struct Solution;

struct Grid<T: Clone + Default> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Clone + Default> Grid<T> {
    fn new(rows: usize, cols: usize) -> Self {
        let total = rows * cols;
        let mut data: Vec<T> = Vec::with_capacity(total);
        data.resize(total, T::default());

        Grid { data, rows, cols }
    }

    fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.data.get(self.cols * row + col)
    }

    fn get_unchecked(&mut self, row: usize, col: usize) -> &mut T {
        &mut self.data[self.cols * row + col]
    }
}

impl Solution {
    pub fn unique_paths(rows: i32, columns: i32) -> i32 {
        let rows = rows as usize;
        let columns = columns as usize;
        let mut grid_with_max_path_counts: Grid<i32> = Grid::new(rows, columns);

        if rows == 1 || columns == 1 {
            return 1;
        }

        // max paths looks like
        //
        //   X 1 1
        //   1 2 3
        //   1 3 6
        //
        // every square max paths are sum of max paths of left one and of top one

        *grid_with_max_path_counts.get_unchecked(0, 1) = 1;
        *grid_with_max_path_counts.get_unchecked(1, 0) = 1;
        *grid_with_max_path_counts.get_unchecked(1, 1) = 2;

        for r in 0..rows {
            *grid_with_max_path_counts.get_unchecked(r, 0) = 1;
        }

        for c in 0..columns {
            *grid_with_max_path_counts.get_unchecked(0, c) = 1;
        }

        dbg!(&grid_with_max_path_counts.data);

        for r in 1..rows {
            for c in 1..columns {
                let left_cell_max_path = if c == 0 {
                    0
                } else {
                    *grid_with_max_path_counts.get_unchecked(r, c - 1)
                };

                let top_cell_max_path = if r == 0 {
                    0
                } else {
                    *grid_with_max_path_counts.get_unchecked(r - 1, c)
                };

                *grid_with_max_path_counts.get_unchecked(r, c) =
                    left_cell_max_path + top_cell_max_path;
            }
        }

        dbg!(&grid_with_max_path_counts.data);

        *grid_with_max_path_counts.get_unchecked(rows - 1, columns - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn unique_paths_1() {
        let m = 3;
        let n = 7;

        let result = Solution::unique_paths(m, n);

        assert_eq!(result, 28);
    }
}
