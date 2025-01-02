use std::{cell::RefCell, rc::Rc};

/// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    /// Push new element and return Rc<RefCell<TreeNode>>
    #[allow(clippy::needless_return)]
    pub fn push(&mut self, val: i32) -> Rc<RefCell<TreeNode>> {
        if val <= self.val {
            if let Some(leaf) = self.left.as_mut() {
                return leaf.borrow_mut().push(val);
            } else {
                self.left = TreeNode::new(val).into_rc();
                return self.left.as_mut().unwrap().clone();
            }
        } else if let Some(leaf) = self.right.as_mut() {
            return leaf.borrow_mut().push(val);
        } else {
            self.right = TreeNode::new(val).into_rc();
            return self.right.as_mut().unwrap().clone();
        }
    }

    pub fn into_rc(self) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(self)))
    }
}

pub trait CopyToRc {
    fn copy_to_rc(self, rcref: &mut Option<Rc<RefCell<TreeNode>>>) -> Self;
}

impl CopyToRc for Rc<RefCell<TreeNode>> {
    fn copy_to_rc(self, rcref: &mut Option<Rc<RefCell<TreeNode>>>) -> Self {
        *rcref = Some(self.clone());
        self
    }
}

impl CopyToRc for Option<Rc<RefCell<TreeNode>>> {
    fn copy_to_rc(self, rcref: &mut Option<Rc<RefCell<TreeNode>>>) -> Self {
        let Some(some_self) = self else {
            return self;
        };

        Some(some_self.copy_to_rc(rcref))
    }
}
