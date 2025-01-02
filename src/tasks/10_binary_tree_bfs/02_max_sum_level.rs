//! Maximum Level Sum of a Binary Tree
//! Given the root of a binary tree, the level of its root is 1, the level of its children is 2, and so on.
//! Return the smallest level x such that the sum of all the values of nodes at level x is maximal.
//! https://leetcode.com/problems/maximum-level-sum-of-a-binary-tree

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::tasks::binary_tree_dfs::TreeNode;
struct Solution;

impl Solution {
    fn traverse_bfs<Cb: FnMut(&[Rc<RefCell<TreeNode>>])>(
        node_level: &[Rc<RefCell<TreeNode>>],
        level_cb: &mut Cb,
    ) {
        level_cb(node_level);
        let mut next_level: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        for node in node_level.iter() {
            let borrowed = node.borrow();
            if let Some(left) = &borrowed.left {
                next_level.push(left.clone());
            }
            if let Some(right) = &borrowed.right {
                next_level.push(right.clone());
            }
        }

        if !next_level.is_empty() {
            Self::traverse_bfs(&next_level, level_cb);
        }
    }

    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_level: i32 = 0;
        let mut max_level_sum: i64 = i64::MIN;

        let Some(root) = root else {
            return max_level;
        };

        let mut curr_level = 0;
        Self::traverse_bfs(&[root], &mut |current_nodes_level| {
            curr_level += 1;

            let sum: i64 = current_nodes_level
                .iter()
                .map(|v| v.borrow().val as i64)
                .sum();

            if (sum > max_level_sum) {
                max_level_sum = sum;
                max_level = curr_level;
            }
        });

        max_level
    }
}

#[cfg(test)]
mod tests {
    use crate::tasks::binary_tree_dfs::TreeNode;

    use super::Solution;

    #[test]
    fn max_level_sum_1() {
        let tree = TreeNode {
            val: 1,
            left: TreeNode {
                val: 2,
                left: None,
                right: TreeNode::new(5).into_rc(),
            }
            .into_rc(),
            right: TreeNode {
                val: 3,
                left: None,
                right: TreeNode::new(4).into_rc(),
            }
            .into_rc(),
        }
        .into_rc();

        let result = Solution::max_level_sum(tree);

        assert_eq!(result, 3);
    }
}
