//! Odd even list
//! Given the head of a singly linked list, group all the nodes with odd indices together
//! followed by the nodes with even indices, and return the reordered list.
//! https://leetcode.com/problems/odd-even-linked-list/description

use std::ptr;

use super::ListNode;

struct Solution;

#[derive(Debug)]
struct NodeManager {
    head: Option<Box<ListNode>>,
    tail: *mut ListNode,
}
impl NodeManager {
    fn new(head: Option<Box<ListNode>>) -> Self {
        let mut manager = NodeManager {
            head,
            tail: ptr::null_mut(),
        };

        if let Some(ref mut node) = manager.head {
            let mut current = node.as_mut();
            while let Some(ref mut next_node) = current.next {
                current = next_node.as_mut();
            }
            manager.tail = current as *mut ListNode;
        }
        manager
    }

    fn push(&mut self, val: i32) {
        let mut new_node = Box::new(ListNode { val, next: None });
        let new_node_ptr = &mut *new_node as *mut ListNode;
        unsafe {
            if self.tail.is_null() {
                self.head = Some(new_node);
                self.tail = new_node_ptr;
            } else {
                (*self.tail).next = Some(new_node);
                self.tail = new_node_ptr;
            }
        }
    }
}

// Implementing IntoIterator for NodeManager
struct NodeManagerIntoIter {
    current: Option<Box<ListNode>>,
}

impl Iterator for NodeManagerIntoIter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|mut node| {
            self.current = node.next.take();
            node.val
        })
    }
}
impl IntoIterator for NodeManager {
    type Item = i32;
    type IntoIter = NodeManagerIntoIter;
    fn into_iter(self) -> Self::IntoIter {
        NodeManagerIntoIter { current: self.head }
    }
}

impl Solution {
    #[allow(clippy::collapsible_else_if)]
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let head = head?;

        let input_list_manager = NodeManager::new(Some(head));
        let mut new_list_manager = NodeManager::new(None);
        let mut odd_list_manager = NodeManager::new(None);
        let mut even_list_manager = NodeManager::new(None);

        for (idx, input) in input_list_manager.into_iter().enumerate() {
            if (idx + 1) % 2 == 1 {
                odd_list_manager.push(input);
            } else {
                even_list_manager.push(input);
            }
        }

        for odd in odd_list_manager.into_iter() {
            new_list_manager.push(odd);
        }

        for even in even_list_manager.into_iter() {
            new_list_manager.push(even);
        }

        new_list_manager.head
    }
}

#[cfg(test)]
mod tests {
    use crate::tasks::linked_list::leetcode_list::ListNode;

    use super::Solution;

    #[test]
    fn odd_even_list_1() {
        let list_node = ListNode::from_iter([1, 2, 3, 4, 5]);

        let result = Solution::odd_even_list(Some(Box::new(list_node)));

        let result: Vec<i32> = result.unwrap().into_iter().collect();
        assert_eq!(result, [1, 3, 5, 2, 4]);
    }

    #[test]
    fn odd_even_list_2() {
        let list_node = ListNode::from_iter([2, 1, 3, 5, 6, 4, 7]);

        let result = Solution::odd_even_list(Some(Box::new(list_node)));

        let result: Vec<i32> = result.unwrap().into_iter().collect();
        assert_eq!(result, [2, 3, 6, 7, 1, 5, 4]);
    }

    #[test]
    fn odd_even_list_4() {
        let list_node = ListNode::from_iter([2]);

        let result = Solution::odd_even_list(Some(Box::new(list_node)));

        let result: Vec<i32> = result.unwrap().into_iter().collect();
        assert_eq!(result, [2]);
    }
}
