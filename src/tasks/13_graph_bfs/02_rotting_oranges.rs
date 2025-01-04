use std::collections::HashSet;

struct Solution;

impl Solution {
    const FRESH_ORANGE: i32 = 1;
    const ROTTEN_ORANGE: i32 = 2;

    #[allow(clippy::needless_range_loop)]
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        type Position = (usize, usize);

        let width = grid[0].len();
        let height = grid.len();

        let mut positions: Vec<Position> = Vec::new();
        for y in 0..height {
            for x in 0..width {
                if grid[y][x] == Self::ROTTEN_ORANGE {
                    positions.push((x, y));
                }
            }
        }

        let mut visited_positions: HashSet<Position> = HashSet::new();
        let mut iteration = 0;
        while !positions.is_empty() {
            let mut next_layer: Vec<Position> = Vec::new();

            for position in positions.iter() {
                if visited_positions.contains(position) {
                    continue;
                }

                visited_positions.insert(*position);
                if (grid[position.1][position.0] == Self::FRESH_ORANGE) {
                    grid[position.1][position.0] = Self::ROTTEN_ORANGE;
                }
                if grid[position.1][position.0] == 0 {
                    continue;
                }

                if (position.0 > 0) {
                    next_layer.push((position.0 - 1, position.1));
                }
                if position.0 < width - 1 {
                    next_layer.push((position.0 + 1, position.1));
                }
                if position.1 > 0 {
                    next_layer.push((position.0, position.1 - 1));
                }
                if position.1 < height - 1 {
                    next_layer.push((position.0, position.1 + 1));
                }
            }

            iteration += 1;
            positions = next_layer;
        }

        for row in grid {
            for val in row {
                if val == Self::FRESH_ORANGE {
                    return -1;
                }
            }
        }

        if iteration > 1 {
            iteration - 2
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn oranges_rotting_1() {
        let grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];

        let result = Solution::oranges_rotting(grid);

        assert_eq!(result, 4);
    }

    #[test]
    fn oranges_rotting_2() {
        let grid = vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]];

        let result = Solution::oranges_rotting(grid);

        assert_eq!(result, -1);
    }

    #[test]
    fn oranges_rotting_3() {
        let grid = vec![vec![2]];

        let result = Solution::oranges_rotting(grid);

        assert_eq!(result, 0);
    }
}
