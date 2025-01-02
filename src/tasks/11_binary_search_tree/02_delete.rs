//! Delete Node in a BST
//! Given a root node reference of a BST and a key, delete the node with the given key in the BST. Return the root node reference (possibly updated) of the BST.
//! https://leetcode.com/problems/delete-node-in-a-bst

use std::cell::RefCell;
use std::rc::Rc;

use super::TreeNode;

struct Solution;

impl Solution {
    fn find_min(node: Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
        let mut current = node;
        loop {
            if (current.borrow().left.is_some()) {
                let left_ref = current.borrow().left.as_ref().unwrap().clone();
                current = left_ref;
            } else {
                return current;
            }
        }
    }

    #[allow(clippy::comparison_chain)]
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        unsafe {
            let node = root?;

            let mut node_ptr = node.as_ptr();

            if key < (*node_ptr).val {
                (*node_ptr).left = Self::delete_node((*node_ptr).left.clone(), key);
                return Some(node);
            } else if key > (*node_ptr).val {
                (*node_ptr).right = Self::delete_node((*node_ptr).right.clone(), key);
                return Some(node);
            }

            // Single or no leaf case
            if (*node_ptr).left.is_none() {
                return (*node_ptr).right.clone();
            } else if (*node_ptr).right.is_none() {
                return (*node_ptr).left.clone();
            }

            let min_node = Solution::find_min((*node_ptr).right.clone().unwrap());

            (*node_ptr).val = min_node.borrow().val;
            (*node_ptr).right = Self::delete_node((*node_ptr).right.clone(), min_node.borrow().val);

            Some(node)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Solution, TreeNode};

    #[test]
    fn delete_node_1() {
        let mut tree = TreeNode::new(5);
        tree.push(3);
        tree.push(6);
        tree.push(2);
        tree.push(4);
        tree.push(7);

        let result = Solution::delete_node(tree.into_rc(), 3);
    }
}
