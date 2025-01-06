//! Smallest Number in Infinite Set
//! You have a set which contains all positive integers [1, 2, 3, 4, 5, ...].
//! Implement the SmallestInfiniteSet class:
//! SmallestInfiniteSet() Initializes the SmallestInfiniteSet object to contain all positive integers.
//! int popSmallest() Removes and returns the smallest integer contained in the infinite set.
//! void addBack(int num) Adds a positive integer num back into the infinite set, if it is not already in the infinite set.
//! 1 <= num <= 1000
//! https://leetcode.com/problems/smallest-number-in-infinite-set

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

struct SmallestInfiniteSet {
    heap: BinaryHeap<Reverse<i32>>,
    set: HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {
    fn new() -> Self {
        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(1000);
        let mut set: HashSet<i32> = HashSet::with_capacity(1000);

        for i in 1..=1000 {
            heap.push(Reverse(i));
            set.insert(i);
        }

        SmallestInfiniteSet { heap, set }
    }

    fn pop_smallest(&mut self) -> i32 {
        let result = self.heap.pop().unwrap().0;
        self.set.remove(&result);

        result
    }

    fn add_back(&mut self, num: i32) {
        if (self.set.contains(&num)) {
            return;
        }

        self.set.insert(num);
        self.heap.push(Reverse(num));
    }
}

#[cfg(test)]
mod tests {
    use super::SmallestInfiniteSet;

    #[test]
    fn test_smallest_infinite_set_1() {
        let mut set = SmallestInfiniteSet::new();
        assert_eq!(set.pop_smallest(), 1);
        assert_eq!(set.pop_smallest(), 2);

        set.add_back(2);

        assert_eq!(set.pop_smallest(), 2);
    }
}
