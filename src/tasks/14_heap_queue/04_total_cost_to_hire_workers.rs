//! Total Cost to Hire K Workers
//! You are given a 0-indexed integer array costs where costs[i] is the cost of hiring the ith worker.
//! You are also given two integers k and candidates.
//! We want to hire exactly k workers according to the following rules:
//! You will run k sessions and hire exactly one worker in each session.
//! In each hiring session, choose the worker with the lowest cost from either the first candidates workers or the last candidates workers.
//! Break the tie by the smallest index.
//! For example, if costs = [3,2,7,7,1,2] and candidates = 2, then in the first hiring session, we will choose the 4th worker because they have the lowest cost [3,2,7,7,1,2].
//! In the second hiring session, we will choose 1st worker because they have the same lowest cost as 4th worker but they have the smallest index [3,2,7,7,2].
//! Please note that the indexing may be changed in the process.
//! If there are fewer than candidates workers remaining, choose the worker with the lowest cost among them.
//! Break the tie by the smallest index.
//! A worker can only be chosen once.
//! Return the total cost to hire exactly k workers.
//! https://leetcode.com/problems/total-cost-to-hire-k-workers

//! Worst. Description. Ever.

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

struct Solution;

impl Solution {
    pub fn total_cost(mut costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let candidates = candidates as usize;
        let k = k as usize;
        if costs.len() <= k {
            return costs.iter().map(|n| *n as i64).sum();
        }
        if costs.len() <= candidates * 2 {
            costs.sort();
            return costs[0..k].iter().map(|n| *n as i64).sum();
        }

        let mut result: Vec<i64> = Vec::with_capacity(k);

        let mut start_min_heap: BinaryHeap<Reverse<i32>> =
            BinaryHeap::from_iter(costs.iter().take(candidates).map(|n| Reverse(*n)));
        let mut end_min_heap: BinaryHeap<Reverse<i32>> =
            BinaryHeap::from_iter(costs.iter().rev().take(candidates).map(|n| Reverse(*n)));
        let mut middle_queue: VecDeque<i32> = VecDeque::from_iter(
            costs
                .iter()
                .skip(candidates)
                .take(costs.len() - 2 * candidates)
                .cloned(),
        );
        let mut is_overlapped = false;

        for i in 0..k {
            let start_candidate = start_min_heap.peek().map(|n| n.0).unwrap_or(i32::MAX);
            let end_candidate = end_min_heap.peek().map(|n| n.0).unwrap_or(i32::MAX);
            if start_candidate <= end_candidate {
                result.push(start_min_heap.pop().unwrap().0 as i64);
                if !middle_queue.is_empty() {
                    start_min_heap.push(middle_queue.pop_front().map(Reverse).unwrap());
                }
            } else {
                result.push(end_min_heap.pop().unwrap().0 as i64);
                if !middle_queue.is_empty() {
                    end_min_heap.push(middle_queue.pop_back().map(Reverse).unwrap());
                }
            }
        }

        result.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn total_cost_1() {
        let costs = vec![17, 12, 10, 2, 7, 2, 11, 20, 8];
        let k = 3;
        let candidates = 4;

        let result = Solution::total_cost(costs, k, candidates);

        assert_eq!(result, 11);
    }

    #[test]
    fn total_cost_2() {
        let costs = vec![
            31, 25, 72, 79, 74, 65, 84, 91, 18, 59, 27, 9, 81, 33, 17, 58,
        ];
        let k = 11;
        let candidates = 2;

        let result = Solution::total_cost(costs, k, candidates);

        assert_eq!(result, 423);
    }

    #[test]
    fn total_cost_3() {
        let costs = vec![
            50, 80, 34, 9, 86, 20, 67, 94, 65, 82, 40, 79, 74, 92, 84, 37, 19, 16, 85, 20, 79, 25,
            89, 55, 67, 84, 3, 79, 38, 16, 44, 2, 54, 58,
        ];
        let k = 7;
        let candidates = 12;

        let result = Solution::total_cost(costs, k, candidates);

        assert_eq!(result, 95);
    }
}
