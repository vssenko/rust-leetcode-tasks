//! Search in a Binary Search Tree
//! You are given the root of a binary search tree (BST) and an integer val.
//! Find the node in the BST that the node's value equals val and return the subtree rooted with that node. If such a node does not exist, return null.
use std::cell::RefCell;
use std::rc::Rc;

use super::TreeNode;

struct Solution;

impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root = root.as_ref()?;

        let borrowed_root = root.borrow();
        match borrowed_root.val.cmp(&val) {
            std::cmp::Ordering::Less => Solution::search_bst(borrowed_root.right.clone(), val),
            std::cmp::Ordering::Greater => Solution::search_bst(borrowed_root.left.clone(), val),
            std::cmp::Ordering::Equal => Some(root.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::Solution;
    use super::TreeNode;

    #[test]
    fn search_bst_1() {
        let mut root = TreeNode::new(4);
        let rc_to_find = root.push(2);
        root.push(7);
        root.push(1);
        root.push(3);
        let result = Solution::search_bst(root.into_rc(), 2);

        assert!(Rc::ptr_eq(&result.unwrap(), &rc_to_find));
    }
}
