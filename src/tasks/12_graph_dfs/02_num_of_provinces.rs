//! Number of Provinces
//! There are n cities. Some of them are connected, while some are not.
//! If city a is connected directly with city b, and city b is connected directly with city c, then city a is connected indirectly with city c.
//! A province is a group of directly or indirectly connected cities and no other cities outside of the group.
//! You are given an n x n matrix isConnected
//! where isConnected[i][j] = 1 if the ith city and the jth city are directly connected, and isConnected[i][j] = 0 otherwise.
//! Return the total number of provinces.
//! https://leetcode.com/problems/number-of-provinces

struct Solution;

type GraphEntry<'a> = (usize, &'a Vec<i32>);

impl Solution {
    fn dfs<'a, Cb>(
        node: GraphEntry<'a>,
        prev: Option<GraphEntry<'a>>,
        graph: &'a [Vec<i32>],
        visited: &mut [bool],
        cb: &mut Cb,
    ) where
        Cb: FnMut(GraphEntry<'a>, Option<GraphEntry<'a>>),
    {
        if (visited[node.0]) {
            return;
        }

        cb(node, prev);
        visited[node.0] = true;

        for (next_idx, is_connected) in node.1.iter().enumerate() {
            if next_idx != node.0 && *is_connected > 0 && !visited[next_idx] {
                Self::dfs((next_idx, &graph[next_idx]), Some(node), graph, visited, cb);
            }
        }
    }

    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        if is_connected.is_empty() {
            return 0;
        }

        let mut provinces_count = 0;

        let mut visited = vec![false; is_connected.len()];

        for (idx, entry) in is_connected.iter().enumerate() {
            Solution::dfs(
                (idx, entry),
                None,
                &is_connected,
                &mut visited,
                &mut |entry, prev_entry| {
                    if prev_entry.is_none() {
                        provinces_count += 1;
                    }
                },
            );
        }

        provinces_count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn find_circle_num_1() {
        let is_connected = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];

        let result = Solution::find_circle_num(is_connected);

        assert_eq!(result, 2);
    }
}
