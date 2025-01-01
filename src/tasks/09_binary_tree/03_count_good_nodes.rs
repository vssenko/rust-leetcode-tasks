//! Count Good Nodes in Binary Tree
//! Given a binary tree root, a node X in the tree is named good if in the path from root to X there are no nodes with a value greater than X.
//! Return the number of good nodes in the binary tree.
//! https://leetcode.com/problems/count-good-nodes-in-binary-tree

use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

use super::leetcode_tree::TreeNode;

struct Solution;

impl Solution {
    fn traverse_center_with_check<T: FnMut(&TreeNode, i32)>(
        node: &Option<Rc<RefCell<TreeNode>>>,
        max_val_in_path: i32,
        cb: &mut T,
    ) {
        let Some(existing_node) = node.as_ref() else {
            return;
        };

        let borrowed = existing_node.borrow_mut();

        cb(&borrowed, max_val_in_path);
        let new_max_in_path = borrowed.val.max(max_val_in_path);
        Self::traverse_center_with_check(&borrowed.left, new_max_in_path, cb);
        Self::traverse_center_with_check(&borrowed.right, new_max_in_path, cb);
    }

    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut good_nodes_counter = 0;

        Self::traverse_center_with_check(&root, i32::MIN, &mut |node, max_in_path| {
            if node.val >= max_in_path {
                good_nodes_counter += 1;
            }
        });

        good_nodes_counter
    }
}

#[cfg(test)]
mod tests {
    use crate::tasks::binary_tree::leetcode_tree::TreeNode;

    use super::Solution;

    #[test]
    fn good_nodes_1() {
        let tree = TreeNode {
            val: 3,
            left: TreeNode {
                val: 1,
                left: TreeNode::new(3).into_leaf(),
                right: None,
            }
            .into_leaf(),
            right: TreeNode {
                val: 4,
                left: TreeNode::new(1).into_leaf(),
                right: TreeNode::new(5).into_leaf(),
            }
            .into_leaf(),
        }
        .into_leaf();

        let result = Solution::good_nodes(tree);
        assert_eq!(result, 4);
    }
}
