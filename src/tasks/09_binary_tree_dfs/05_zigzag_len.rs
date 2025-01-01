//! Longest ZigZag Path in a Binary Tree
//! Return the longest ZigZag path contained in that tree.
//! https://leetcode.com/problems/longest-zigzag-path-in-a-binary-tree

use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

use super::TreeNode;

struct Solution;

#[derive(Debug)]
enum Dir {
    Left,
    Right,
}

#[derive(Debug)]
struct TraverseState {
    from: Option<Dir>,
    counter: usize,
}

impl TraverseState {
    fn for_left(counter: usize) -> Self {
        TraverseState {
            from: Some(Dir::Right),
            counter,
        }
    }

    fn for_right(counter: usize) -> Self {
        TraverseState {
            from: Some(Dir::Left),
            counter,
        }
    }
}

impl Solution {
    fn traverse_with_separate_state<
        State: std::fmt::Debug,
        T: FnMut(&TreeNode, State) -> (State, State),
    >(
        node: &Option<Rc<RefCell<TreeNode>>>,
        state: State,
        cb: &mut T,
    ) {
        let Some(existing_node) = node.as_ref() else {
            return;
        };

        let borrowed = existing_node.borrow_mut();

        let (left_state, right_state) = cb(&borrowed, state);

        Self::traverse_with_separate_state(&borrowed.left, left_state, cb);
        Self::traverse_with_separate_state(&borrowed.right, right_state, cb);
    }

    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_nodes_in_path = 0;

        Self::traverse_with_separate_state(
            &root,
            TraverseState {
                from: None,
                counter: 1,
            },
            &mut |node, state| {
                max_nodes_in_path = max_nodes_in_path.max(state.counter);

                let Some(from_state) = state.from else {
                    return (TraverseState::for_left(2), TraverseState::for_right(2));
                };

                let (left_c, right_c) = match from_state {
                    Dir::Right => (2, state.counter + 1),
                    Dir::Left => (state.counter + 1, 2),
                };

                (
                    TraverseState::for_left(left_c),
                    TraverseState::for_right(right_c),
                )
            },
        );

        max_nodes_in_path as i32 - 1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use super::TreeNode;

    #[test]
    fn longest_zig_zag_1() {
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

        let result = Solution::longest_zig_zag(tree);
        assert_eq!(result, 2);
    }
}
