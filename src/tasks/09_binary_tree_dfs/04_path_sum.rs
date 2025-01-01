//! Count Good Nodes in Binary Tree
//! Given a binary tree root, a node X in the tree is named good if in the path from root to X there are no nodes with a value greater than X.
//! Return the number of good nodes in the binary tree.
//! https://leetcode.com/problems/count-good-nodes-in-binary-tree

use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

use super::leetcode_tree::TreeNode;

struct Solution;

impl Solution {
    fn traverse_pre_post_cb<
        State: std::fmt::Debug,
        T: FnMut(&TreeNode, &mut State),
        T2: FnMut(&TreeNode, &mut State),
    >(
        node: &Option<Rc<RefCell<TreeNode>>>,
        state: &mut State,
        pre_cb: &mut T,
        post_cb: &mut T2,
    ) {
        let Some(existing_node) = node.as_ref() else {
            return;
        };

        let borrowed = existing_node.borrow_mut();

        pre_cb(&borrowed, state);

        Self::traverse_pre_post_cb(&borrowed.left, state, pre_cb, post_cb);
        Self::traverse_pre_post_cb(&borrowed.right, state, pre_cb, post_cb);

        post_cb(&borrowed, state);
    }

    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut same_sum_path_counter = 0;

        Self::traverse_pre_post_cb(
            &root,
            &mut VecDeque::<i32>::new(),
            &mut |node, prev_path_queue| {
                let mut sum: i64 = node.val as i64;
                for prev_val in std::iter::once(&0).chain(prev_path_queue.iter().rev()) {
                    sum += *prev_val as i64;
                    if (sum == target_sum as i64) {
                        same_sum_path_counter += 1;
                    }
                }

                prev_path_queue.push_back(node.val);
            },
            &mut |node, prev_path| {
                prev_path.pop_back();
            },
        );

        same_sum_path_counter
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use super::TreeNode;

    #[test]
    fn path_sum_1() {
        let tree = TreeNode {
            val: 10,
            left: TreeNode {
                val: 5,
                left: TreeNode {
                    val: 3,
                    left: TreeNode::new(3).into_rc(),
                    right: TreeNode::new(-2).into_rc(),
                }
                .into_rc(),
                right: TreeNode {
                    val: 2,
                    left: None,
                    right: TreeNode::new(1).into_rc(),
                }
                .into_rc(),
            }
            .into_rc(),
            right: TreeNode {
                val: -3,
                left: None,
                right: TreeNode::new(11).into_rc(),
            }
            .into_rc(),
        }
        .into_rc();

        let result = Solution::path_sum(tree, 8);
        assert_eq!(result, 3);
    }

    #[test]
    fn path_sum_2() {
        let tree = TreeNode {
            val: 0,
            left: TreeNode::new(1).into_rc(),
            right: TreeNode::new(1).into_rc(),
        }
        .into_rc();

        let result = Solution::path_sum(tree, 1);
        assert_eq!(result, 4);
    }

    #[test]
    fn path_sum_3() {
        let tree = TreeNode {
            val: 1000000000,
            left: TreeNode {
                val: 1000000000,
                left: TreeNode {
                    val: 294967296,
                    left: TreeNode::new(1000000000).into_rc(),
                    right: None,
                }
                .into_rc(),
                right: None,
            }
            .into_rc(),
            right: None,
        }
        .into_rc();

        let result = Solution::path_sum(tree, 0);
        assert_eq!(result, 0);
    }
}
