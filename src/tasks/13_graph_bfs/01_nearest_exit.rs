//! Nearest Exit from Entrance in Maze
//! You are given an m x n matrix maze (0-indexed) with empty cells (represented as '.') and walls (represented as '+').
//! You are also given the entrance of the maze,
//! where entrance = [entrancerow, entrancecol] denotes the row and column of the cell you are initially standing at.
//! In one step, you can move one cell up, down, left, or right.
//! You cannot step into a cell with a wall, and you cannot step outside the maze.
//! Your goal is to find the nearest exit from the entrance.
//! An exit is defined as an empty cell that is at the border of the maze.
//! The entrance does not count as an exit.
//! Return the number of steps in the shortest path from the entrance to the nearest exit, or -1 if no such path exists.
//! https://leetcode.com/problems/nearest-exit-from-entrance-in-maze

use std::{
    collections::{vec_deque, HashSet, VecDeque},
    hash::Hash,
};

struct Solution;

impl Solution {
    const ROAD: char = '.';
    const WALL: char = '+';

    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let height = maze.len();
        let width = maze[0].len();

        type Position = (usize, usize);

        //(x,y)
        let mut visited_positions: HashSet<Position> = HashSet::new();
        let mut positions: Vec<Position> = vec![(entrance[1] as usize, entrance[0] as usize)];

        let mut current_deep: usize = 0;
        let mut min_escape_route: usize = usize::MAX;
        while !positions.is_empty() {
            let mut next_layer: Vec<Position> = Vec::new();

            for position in positions.iter() {
                if visited_positions.contains(position) {
                    continue;
                }

                visited_positions.insert(*position);
                let current_maze_type = maze[position.1][position.0];
                if (current_maze_type == Self::WALL) {
                    continue;
                }

                let is_x_edge = position.0 == 0 || position.0 == width - 1;
                let is_y_edge = position.1 == 0 || position.1 == height - 1;
                if ((is_x_edge || is_y_edge) && current_deep > 0) {
                    min_escape_route = min_escape_route.min(current_deep);
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

            current_deep += 1;
            positions = next_layer;
        }

        if min_escape_route == usize::MAX {
            -1_i32
        } else {
            min_escape_route as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn nearest_exit_1() {
        let maze = vec![
            vec!['+', '+', '.', '+'],
            vec!['.', '.', '.', '+'],
            vec!['+', '+', '+', '.'],
        ];
        let entrance = vec![1, 2];

        let result = Solution::nearest_exit(maze, entrance);

        assert_eq!(result, 1);
    }

    #[test]
    fn nearest_exit_2() {
        let maze = vec![
            vec!['+', '+', '+'],
            vec!['.', '.', '.'],
            vec!['+', '+', '+'],
        ];
        let entrance = vec![1, 0];

        let result = Solution::nearest_exit(maze, entrance);

        assert_eq!(result, 2);
    }
}
