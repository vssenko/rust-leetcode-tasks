//! Leaf similar tree
//! Consider all the leaves of a binary tree, from left to right order, the values of those leaves form a leaf value sequence.
//! https://leetcode.com/problems/leaf-similar-trees

use std::cell::RefCell;
use std::rc::Rc;

use super::leetcode_tree::TreeNode;
struct Solution;

impl Solution {
    fn traverse_left<T: FnMut(&TreeNode)>(node: &Option<Rc<RefCell<TreeNode>>>, cb: &mut T) {
        let Some(node) = node else {
            return;
        };

        let borrowed = node.borrow();

        Self::traverse_left(&borrowed.left, cb);
        cb(&borrowed);
        Self::traverse_left(&borrowed.right, cb);
    }

    fn get_leaves_seq(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::<i32>::new();

        Self::traverse_left(root, &mut |node| {
            if (node.left.is_none() && node.right.is_none()) {
                result.push(node.val);
            }
        });

        result
    }

    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let seq_1 = Self::get_leaves_seq(&root1);
        let seq_2 = Self::get_leaves_seq(&root2);

        dbg!(&seq_1, &seq_2);

        seq_1 == seq_2
    }
}

#[cfg(test)]
mod tests {
    use crate::tasks::binary_tree::leetcode_tree::TreeNode;

    use super::Solution;

    #[test]
    fn leaf_similar_1() {
        let tree_1 = TreeNode {
            val: 3,
            left: TreeNode {
                val: 5,
                left: TreeNode::new(6).into_leaf(),
                right: TreeNode {
                    val: 2,
                    left: TreeNode::new(7).into_leaf(),
                    right: TreeNode::new(4).into_leaf(),
                }
                .into_leaf(),
            }
            .into_leaf(),
            right: TreeNode {
                val: 1,
                left: TreeNode::new(9).into_leaf(),
                right: TreeNode::new(8).into_leaf(),
            }
            .into_leaf(),
        }
        .into_leaf();

        let tree_2 = TreeNode {
            val: 3,
            left: TreeNode {
                val: 5,
                left: TreeNode::new(6).into_leaf(),
                right: TreeNode::new(7).into_leaf(),
            }
            .into_leaf(),
            right: TreeNode {
                val: 1,
                left: TreeNode::new(4).into_leaf(),
                right: TreeNode {
                    val: 2,
                    left: TreeNode::new(9).into_leaf(),
                    right: TreeNode::new(8).into_leaf(),
                }
                .into_leaf(),
            }
            .into_leaf(),
        }
        .into_leaf();

        let result = Solution::leaf_similar(tree_1, tree_2);

        assert!(result);
    }
}
