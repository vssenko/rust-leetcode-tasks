//! Binary Tree Right Side View
//! Given the root of a binary tree, imagine yourself standing on the right side of it, return the values of the nodes you can see ordered from top to bottom.
//! https://leetcode.com/problems/binary-tree-right-side-view

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

    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        let Some(root) = root else { return result };

        Self::traverse_bfs(&[root], &mut |current_nodes_level| {
            if current_nodes_level.is_empty() {
                return;
            }
            let rightest_in_level = current_nodes_level.last().unwrap().borrow().val;
            result.push(rightest_in_level);
        });

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::tasks::binary_tree_dfs::TreeNode;

    use super::Solution;

    #[test]
    fn right_side_view_1() {
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

        let result = Solution::right_side_view(tree);

        assert_eq!(result, [1, 3, 4]);
    }
}
