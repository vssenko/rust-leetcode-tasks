//! Maximum Depth of Binary Tree
//! Given the root of a binary tree, return its maximum depth.
//! https://leetcode.com/problems/maximum-depth-of-binary-tree

use std::{cell::RefCell, rc::Rc};

use super::TreeNode;

struct Solution;

impl Solution {
    pub fn m_depth_rec(node: &Option<Rc<RefCell<TreeNode>>>, current_depth: usize) -> usize {
        let Some(node) = node else {
            return current_depth - 1;
        };

        let left_depth = Self::m_depth_rec(&node.borrow().left, current_depth + 1);
        let right_depth = Self::m_depth_rec(&node.borrow().right, current_depth + 1);

        left_depth.max(right_depth)
    }

    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::m_depth_rec(&root, 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::TreeNode;

    use super::Solution;

    #[test]
    fn max_depth_1() {
        let tree = TreeNode {
            val: 3,
            left: TreeNode::new(9).into_rc(),
            right: TreeNode {
                val: 20,
                left: TreeNode::new(15).into_rc(),
                right: TreeNode::new(7).into_rc(),
            }
            .into_rc(),
        };

        let result = Solution::max_depth(tree.into_rc());

        assert_eq!(result, 3);
    }

    #[test]
    fn max_depth_2() {
        let tree = TreeNode {
            val: 1,
            left: None,
            right: TreeNode::new(2).into_rc(),
        };

        let result = Solution::max_depth(tree.into_rc());

        assert_eq!(result, 2);
    }
}
