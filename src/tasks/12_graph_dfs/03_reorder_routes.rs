//! Reorder Routes to Make All Paths Lead to the City Zero
//! There are n cities numbered from 0 to n - 1 and n - 1 roads such that there is only one way to travel between two different cities (this network form a tree).
//! Last year, The ministry of transport decided to orient the roads in one direction because they are too narrow.
//! Roads are represented by connections where connections[i] = [ai, bi] represents a road from city ai to city bi.
//! This year, there will be a big event in the capital (city 0), and many people want to travel to this city.
//! Your task consists of reorienting some roads such that each city can visit the city 0. Return the minimum number of edges changed.
//! It's guaranteed that each city can reach city 0 after reorder.
//! https://leetcode.com/problems/reorder-routes-to-make-all-paths-lead-to-the-city-zero

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    rc::Rc,
};

struct Solution;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Forward,
    Backward,
}

#[derive(Debug)]
struct Connection {
    node: Rc<RefCell<GraphNode>>,
    direction: Direction,
}

#[derive(Debug)]
struct GraphNode {
    val: i32,
    edges: Vec<Connection>,
}

impl GraphNode {
    fn new(val: i32) -> Self {
        GraphNode {
            val,
            edges: Vec::new(),
        }
    }
}

struct State {
    visited: HashMap<i32, bool>,
}

impl Solution {
    fn bidirectional_dfs<Cb>(
        node: Rc<RefCell<GraphNode>>,
        by_conn: Option<&Connection>,
        state: &mut State,
        cb: &mut Cb,
    ) where
        Cb: FnMut(Rc<RefCell<GraphNode>>, Option<&Connection>),
    {
        if (state.visited.contains_key(&node.borrow().val)) {
            return;
        }

        cb(node.clone(), by_conn);

        // .entry().insert_entry will not compile on LeetCode
        state.visited.insert(node.borrow().val, true);

        // here we go BIDIRECTIONALLY
        for next in node.borrow().edges.iter() {
            Solution::bidirectional_dfs(next.node.clone(), Some(next), state, cb);
        }
    }

    #[allow(clippy::map_entry)]
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut roads_map = HashMap::<i32, Rc<RefCell<GraphNode>>>::new();

        // I am so dumb that I really need so explicit graph representation
        // Even if it will lead to lowest leetcode score
        for road in connections {
            // it is needed to write like that instead of .entry to avoid multiple mut borrow
            if !roads_map.contains_key(&road[0]) {
                roads_map.insert(road[0], Rc::new(RefCell::new(GraphNode::new(road[0]))));
            }
            if !roads_map.contains_key(&road[1]) {
                roads_map.insert(road[1], Rc::new(RefCell::new(GraphNode::new(road[1]))));
            }

            let graph_node = roads_map.get(&road[0]).unwrap();
            let next_node = roads_map.get(&road[1]).unwrap();

            next_node.borrow_mut().edges.push(Connection {
                node: graph_node.clone(),
                direction: Direction::Backward,
            });
            graph_node.borrow_mut().edges.push(Connection {
                node: next_node.clone(),
                direction: Direction::Forward,
            });
        }

        let mut dfs_state = State {
            visited: HashMap::new(),
        };

        let capital_index = 0;

        let mut swaps = 0;

        // let's find all bidirectional paths starting from capital
        Self::bidirectional_dfs(
            roads_map[&capital_index].clone(),
            None,
            &mut dfs_state,
            &mut |node, connection| {
                let Some(connection) = connection else {
                    return;
                };

                // we should be able to reach to 0 ehm backward, traversing from 0...
                if connection.direction == Direction::Forward {
                    swaps += 1;
                }
            },
        );

        swaps
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn min_reorder_1() {
        let n = 6;
        let connections = vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]];

        let result = Solution::min_reorder(n, connections);

        assert_eq!(result, 3);
    }
}
