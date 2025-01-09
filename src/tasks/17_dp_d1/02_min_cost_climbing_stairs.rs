//! Min Cost Climbing Stairs
//! You are given an integer array cost where cost[i] is the cost of ith step on a staircase. Once you pay the cost, you can either climb one or two steps.
//! You can either start from the step with index 0, or the step with index 1.
//! Return the minimum cost to reach the top of the floor.
//! https://leetcode.com/problems/min-cost-climbing-stairs

struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut costs_to_top: Vec<i32> = vec![0; cost.len() + 1];

        costs_to_top[cost.len() - 1] = *cost.last().unwrap();

        for i in (0..cost.len() - 1).rev() {
            costs_to_top[i] = cost[i] + costs_to_top[i + 1].min(costs_to_top[i + 2]);
        }

        costs_to_top[0].min(costs_to_top[1])
    }
}

#[cfg(test)]
mod tests {
    use crate::tasks::dp_d1::min_cost_climbing_stairs::Solution;

    #[test]
    fn min_cost_climbing_stairs_1() {
        let cost = vec![10, 15, 20];
        assert_eq!(Solution::min_cost_climbing_stairs(cost), 15);
    }

    #[test]
    fn min_cost_climbing_stairs_2() {
        let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
        assert_eq!(Solution::min_cost_climbing_stairs(cost), 6);
    }
}
