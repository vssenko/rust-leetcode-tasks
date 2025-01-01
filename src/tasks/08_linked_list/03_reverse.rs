//! Reverse linked list
//! https://leetcode.com/problems/reverse-linked-list

use super::ListNode;

struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head?;

        let mut new_head: Option<Box<ListNode>> = None;

        let mut prev: Option<Box<ListNode>> = None;
        let mut current: Option<Box<ListNode>> = Some(head);

        while current.is_some() {
            let mutable_current = current.as_mut().unwrap();
            let mut current_next = mutable_current.next.take();

            mutable_current.next = prev.take();

            prev = current;
            current = current_next;

            if current.is_none() {
                new_head = prev;
                break;
            }
        }

        new_head
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};

    #[test]
    fn reverse_list_1() {
        let head = ListNode::from_iter([2, 1]);

        let result = Solution::reverse_list(Some(Box::new(head)));

        let result: Vec<i32> = result.unwrap().into_iter().collect();
        assert_eq!(result, [1, 2]);
    }

    #[test]
    fn reverse_list_2() {
        let head = ListNode::from_iter([1, 3, 5, 7, 9]);

        let result = Solution::reverse_list(Some(Box::new(head)));

        let result: Vec<i32> = result.unwrap().into_iter().collect();
        assert_eq!(result, [9, 7, 5, 3, 1]);
    }
}
