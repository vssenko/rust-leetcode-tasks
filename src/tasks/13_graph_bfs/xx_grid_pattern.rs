//! Grid pattern
// Given a signature:

// ```rust
//    fn search(vec: &Vec<u8>, size: usize, pattern: &Vec<u8>) -> Option<usize>;
// ```
// Where vec is a collection of values filling imaginary, discrete, 2 dimensional field, where one side is of this field is of size `size`. For example
// `vec: "A C T C T A G A C A T U G A G A U C U G U A", size: 2` would describe 2 dimensional field with DNA pairs, looking like this:
// ```
//   0 1 2 3 4 5 6 7 8 9 10 X
// 0 A C T C T A G A C A T
// 1 U G A G A U C U G U A
// Y
// ```
// X/Y
// 1,0+ 2,1 + 2,0
// 3,0+ 3,1 + 2,0
// invalid, visited the same twice AND 
// 0,1, + 1,1 + 0,1

// We would like to perform a search on that field,  and find all chains that matches the pattern. 
// Rules are as follows:
// 1. You can chain any adjacent byte, both diagonally or orthogonally i.e.
// ```
//  A <-C-> T
//     /|\
//  U   G  A
// ```
// 2. by visiting a byte, you are adding it to chain
// 3. you can not use (visit) the same byte twice in the chain

// `pattern: CAT` on above field would result in 4 matches.

use std::collections::HashSet;


#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct GridCell {
    row: usize,
    col: usize
}

impl GridCell {
    pub fn of<'a, T>(&self, grid: &'a [Vec<T>]) -> Option<&'a T> {
        grid.get(self.row).and_then(|row| row.get(self.col))
    }

    pub fn left_in<T>(&self, grid: &[Vec<T>]) -> Option<Self> {
        if self.col > 0 {
            return Some(Self { row: self.row, col: self.col - 1 });
        }
        None
    }

    pub fn right_in<T>(&self, grid: &[Vec<T>]) -> Option<Self> {
        if self.col < grid[0].len() - 1 {
            return Some(Self { row: self.row, col: self.col + 1 });
        }
        None
    }

    pub fn top_in<T>(&self, grid: &[Vec<T>]) -> Option<Self> {
        if self.row > 0 {
            return Some(Self { row: self.row - 1, col: self.col });
        }
        None
    }

    pub fn bottom_in<T>(&self, grid: &[Vec<T>]) -> Option<Self> {
        if self.row < grid.len() - 1 {
            return Some(Self { row: self.row + 1, col: self.col });
        }
        None
    }

    pub fn left_top_in<T>(&self, grid: &[Vec<T>]) -> Option<Self> {
        self.left_in(grid).and_then(|lc| lc.top_in(grid))
    }

    pub fn right_top_in<T>(&self, grid: &[Vec<T>]) -> Option<Self> {
        self.right_in(grid).and_then(|lc| lc.top_in(grid))
    }

    pub fn left_bottom_in<T>(&self, grid: &[Vec<T>]) -> Option<Self> {
        self.left_in(grid).and_then(|lc| lc.bottom_in(grid))
    }

    pub fn right_bottom_in<T>(&self, grid: &[Vec<T>]) -> Option<Self> {
        self.right_in(grid).and_then(|lc| lc.bottom_in(grid))
    }
}

// Find path patterns in grid and call cb for each occurence. I like callback way of data processing...
fn find_pattern<T: Eq + std::fmt::Debug, F: FnMut(Vec<GridCell>)>(grid: &[Vec<T>], gc: GridCell, pattern: &[T], cb: &mut F) {
    fn find_pattern_rec<T: Eq + std::fmt::Debug, F: FnMut(Vec<GridCell>)>(grid: &[Vec<T>], gc: Option<GridCell>, pattern: &[T], pattern_idx: usize, cb: &mut F, acc: &mut Vec<GridCell>, acc_set: &mut HashSet<GridCell>) {
        let Some(gc) = gc else {
            return;
        };

        if !gc.of(grid).is_some_and(|v| pattern.get(pattern_idx).is_some_and(|pv| pv.eq(v))) {
            return;
        }
        if acc_set.contains(&gc) {
            return;
        }

        acc.push(gc.clone());
        acc_set.insert(gc.clone());

        if pattern_idx >= pattern.len() - 1 {
            cb(acc.clone());
        } else {
            find_pattern_rec(grid, gc.left_in(grid), pattern, pattern_idx + 1, cb, acc, acc_set);
            find_pattern_rec(grid, gc.right_in(grid), pattern, pattern_idx + 1, cb, acc, acc_set);
            find_pattern_rec(grid, gc.top_in(grid), pattern, pattern_idx + 1, cb, acc, acc_set);
            find_pattern_rec(grid, gc.bottom_in(grid), pattern, pattern_idx + 1, cb, acc, acc_set);
            find_pattern_rec(grid, gc.left_top_in(grid), pattern, pattern_idx + 1, cb, acc, acc_set);
            find_pattern_rec(grid, gc.right_top_in(grid), pattern, pattern_idx + 1, cb, acc, acc_set);
            find_pattern_rec(grid, gc.left_bottom_in(grid), pattern, pattern_idx + 1, cb, acc, acc_set);
            find_pattern_rec(grid, gc.right_bottom_in(grid), pattern, pattern_idx + 1, cb, acc, acc_set);
        }
        acc.pop();
        acc_set.remove(&gc);
    }

    let mut acc_set = HashSet::with_capacity(pattern.len());
    let mut acc_vec = Vec::with_capacity(pattern.len());
    find_pattern_rec(grid, Some(gc), pattern, 0, cb, &mut acc_vec, &mut acc_set);
}

fn search<T>(vec: &[T], size: usize, pattern: &[T]) -> usize
    where T: Eq + Clone + std::fmt::Debug
{
    if vec.len() % size != 0 {
        return 0;
    }

    let grid: Vec<Vec<T>> = vec.chunks(vec.len() / size).map(|chunk| chunk.to_vec()).collect();

    let mut occurencies = 0;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            find_pattern(&grid, GridCell { row, col }, pattern, &mut |vec| {
                // here you have _vec param, path itself which can be processed out of inner recursion logic.
                occurencies +=1;
            });
        }
    }

    occurencies
}

#[cfg(test)]
mod tests {
    #[test]
    fn search_dna() {
        let vec: Vec<char> = "A C T C T A G A C A T U G A G A U C U G U A".split(" ").map(|s| s.chars().next().unwrap()).collect();
        let pattern: Vec<char> = "C A T".split(" ").map(|s| s.chars().next().unwrap()).collect();
        let result = super::search(&vec, 2, &pattern);


        // FOUND: [GridCell { row: 0, col: 1 }, GridCell { row: 1, col: 2 }, GridCell { row: 0, col: 2 }]
        // FOUND: [GridCell { row: 0, col: 3 }, GridCell { row: 1, col: 2 }, GridCell { row: 0, col: 2 }]
        // FOUND: [GridCell { row: 0, col: 3 }, GridCell { row: 1, col: 4 }, GridCell { row: 0, col: 4 }]
        // FOUND: [GridCell { row: 0, col: 8 }, GridCell { row: 0, col: 9 }, GridCell { row: 0, col: 10 }]
        // FOUND: [GridCell { row: 1, col: 6 }, GridCell { row: 0, col: 5 }, GridCell { row: 0, col: 4 }]
        assert_eq!(result, 5);
    }
}