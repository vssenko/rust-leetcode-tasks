//! Delete the Middle Node of a Linked List
//! You are given the head of a linked list. Delete the middle node, and return the head of the modified linked list.
//! https://leetcode.com/problems/delete-the-middle-node-of-a-linked-list

// on LeetCode submit comment from here ---

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

// --- till there.

impl Solution {
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head?;

        let mut len = 1;

        let mut walker: &ListNode = &head;

        while walker.next.is_some() {
            len += 1;
            walker = walker.next.as_ref().unwrap();
        }

        if len == 1 {
            return None;
        }

        let before_middle_idx = len / 2 - 1;

        let mut idx = 0;
        let mut walker: &mut ListNode = &mut head;
        while walker.next.is_some() {
            if (idx == before_middle_idx) {
                break;
            }
            walker = walker.next.as_mut().unwrap();
            idx += 1;
        }

        let box_next = walker.next.as_mut().unwrap().next.take();
        walker.next = box_next;

        Some(head)
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};

    impl ListNode {
        fn append(&mut self, val: i32) -> &Self {
            let mut walker: &mut ListNode = self;

            while walker.next.is_some() {
                walker = walker.next.as_mut().unwrap();
            }

            walker.next = Some(Box::new(Self::new(val)));

            self
        }
    }

    impl FromIterator<i32> for ListNode {
        fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
            let mut iterator = iter.into_iter();
            let mut new_list = Self::new(iterator.next().unwrap());
            iterator.for_each(|v| {
                new_list.append(v);
            });

            new_list
        }
    }

    pub struct ListIterator {
        current: Option<Box<ListNode>>,
    }

    impl Iterator for ListIterator {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            if let Some(node) = self.current.take() {
                self.current = node.next;
                Some(node.val)
            } else {
                None
            }
        }
    }

    impl IntoIterator for ListNode {
        type Item = i32;

        type IntoIter = ListIterator;

        fn into_iter(self) -> Self::IntoIter {
            ListIterator {
                // ugly
                current: Some(Box::new(self)),
            }
        }
    }

    #[test]
    fn delete_middle_1() {
        let head = ListNode::from_iter([1, 3, 4, 7, 1, 2, 6]);

        let result = Solution::delete_middle(Some(Box::new(head)));

        assert!(result.is_some());

        let result = result.unwrap();

        let result: Vec<i32> = result.into_iter().collect();

        assert_eq!(result, [1, 3, 4, 1, 2, 6]);
    }

    #[test]
    fn delete_middle_2() {
        let head = ListNode::from_iter([1, 2, 3, 4]);

        let result = Solution::delete_middle(Some(Box::new(head)));

        assert!(result.is_some());

        let result = result.unwrap();

        let result: Vec<i32> = result.into_iter().collect();

        assert_eq!(result, [1, 2, 4]);
    }

    #[test]
    fn delete_middle_3() {
        let head = ListNode::from_iter([2, 1]);

        let result = Solution::delete_middle(Some(Box::new(head)));

        assert!(result.is_some());

        let result = result.unwrap();

        let result: Vec<i32> = result.into_iter().collect();

        assert_eq!(result, [2]);
    }
}
