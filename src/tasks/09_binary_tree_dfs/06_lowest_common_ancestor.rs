//! Lowest Common Ancestor of a Binary Tree
//! Given a binary tree, find the lowest common ancestor (LCA) of two given nodes in the tree.
//! According to the definition of LCA on Wikipedia:
//! “The lowest common ancestor is defined between two nodes p and q as the lowest node in T that has both p and q as descendants.”
//! https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree

use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

use super::TreeNode;
struct Solution;

#[derive(Debug)]
struct State {
    should_break: bool,
    current_path: VecDeque<Rc<RefCell<TreeNode>>>,
    p_path: Option<VecDeque<Rc<RefCell<TreeNode>>>>,
    q_path: Option<VecDeque<Rc<RefCell<TreeNode>>>>,
}

impl Solution {
    fn traverse<CB: FnMut(&Rc<RefCell<TreeNode>>)>(
        node: &Rc<RefCell<TreeNode>>,
        state: &RefCell<State>,
        cb: &mut CB,
    ) {
        if state.borrow().should_break {
            return;
        }

        state.borrow_mut().current_path.push_back(node.clone());

        cb(node);

        let borrowed = node.borrow();

        if (!state.borrow().should_break && borrowed.left.is_some()) {
            Self::traverse(borrowed.left.as_ref().unwrap(), state, cb);
        }
        if (!state.borrow().should_break && borrowed.right.is_some()) {
            Self::traverse(borrowed.right.as_ref().unwrap(), state, cb);
        }

        state.borrow_mut().current_path.pop_back();
    }

    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let (Some(mut root), Some(mut p), Some(mut q)) = (root, p, q) else {
            return None;
        };

        let mut state = RefCell::new(State {
            should_break: false,
            current_path: VecDeque::new(),
            p_path: None,
            q_path: None,
        });

        Self::traverse(&root, &state, &mut |node| {
            if Rc::ptr_eq(node, &p) {
                let mut borrow_mut = state.borrow_mut();
                borrow_mut.p_path = Some(borrow_mut.current_path.clone());
            }
            if (Rc::ptr_eq(node, &q)) {
                let mut borrow_mut = state.borrow_mut();
                borrow_mut.q_path = Some(borrow_mut.current_path.clone());
            }
        });

        let mut state = state.borrow_mut();

        let (Some(p_path), Some(q_path)) = (state.p_path.take(), state.q_path.take()) else {
            return None;
        };

        let p_path_raw_refs: HashSet<*const RefCell<TreeNode>> =
            p_path.iter().map(Rc::as_ptr).collect();

        q_path
            .into_iter()
            .rev()
            .find(|rcref| p_path_raw_refs.contains(&Rc::as_ptr(rcref)))
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::super::{CopyToRc, TreeNode};
    use super::Solution;
    #[test]
    fn lowest_common_ancestor_1() {
        let mut q: Option<Rc<RefCell<TreeNode>>> = None;
        let mut p: Option<Rc<RefCell<TreeNode>>> = None;
        let mut lca: Option<Rc<RefCell<TreeNode>>> = None;

        let tree = TreeNode {
            val: 3,
            left: TreeNode {
                val: 5,
                left: TreeNode::new(6).into_rc(),
                right: TreeNode {
                    val: 2,
                    left: TreeNode::new(7).into_rc(),
                    right: TreeNode::new(4).into_rc(),
                }
                .into_rc(),
            }
            .into_rc()
            .copy_to_rc(&mut p),
            right: TreeNode {
                val: 1,
                left: TreeNode::new(0).into_rc(),
                right: TreeNode::new(8).into_rc(),
            }
            .into_rc()
            .copy_to_rc(&mut q),
        }
        .into_rc()
        .copy_to_rc(&mut lca);

        // we could call it with refs to actual nodes, but whats the difference
        let result = Solution::lowest_common_ancestor(tree, p, q);

        assert!(Rc::ptr_eq(&result.unwrap(), &lca.unwrap()));
    }

    #[test]
    fn lowest_common_ancestor_2() {
        let mut q: Option<Rc<RefCell<TreeNode>>> = None;
        let mut p: Option<Rc<RefCell<TreeNode>>> = None;
        let mut lca: Option<Rc<RefCell<TreeNode>>> = None;

        let tree = TreeNode {
            val: 3,
            left: TreeNode {
                val: 5,
                left: TreeNode::new(6).into_rc(),
                right: TreeNode {
                    val: 2,
                    left: TreeNode::new(7).into_rc(),
                    right: TreeNode::new(4).into_rc().copy_to_rc(&mut q),
                }
                .into_rc(),
            }
            .into_rc()
            .copy_to_rc(&mut p)
            .copy_to_rc(&mut lca),
            right: TreeNode {
                val: 1,
                left: TreeNode::new(0).into_rc(),
                right: TreeNode::new(8).into_rc(),
            }
            .into_rc(),
        }
        .into_rc();

        // we could call it with refs to actual nodes, but whats the difference
        let result = Solution::lowest_common_ancestor(tree, p, q);

        assert!(Rc::ptr_eq(&result.unwrap(), &lca.unwrap()));
    }

    #[test]
    fn lowest_common_ancestor_3() {
        let mut q: Option<Rc<RefCell<TreeNode>>> = None;
        let mut p: Option<Rc<RefCell<TreeNode>>> = None;
        let mut lca: Option<Rc<RefCell<TreeNode>>> = None;

        let tree = TreeNode {
            val: 1,
            left: TreeNode::new(2).into_rc().copy_to_rc(&mut q),
            right: None,
        }
        .into_rc()
        .copy_to_rc(&mut p)
        .copy_to_rc(&mut lca);

        // we could call it with refs to actual nodes, but whats the difference
        let result = Solution::lowest_common_ancestor(tree, p, q);

        assert!(Rc::ptr_eq(&result.unwrap(), &lca.unwrap()));
    }
}
