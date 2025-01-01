//! Maximum Twin Sum of a Linked List
//! In a linked list of size n, where n is even, the ith node (0-indexed) of the linked list is known as the twin of the (n-1-i)th node, if 0 <= i <= (n / 2) - 1.
//! Given the head of a linked list with even length, return the maximum twin sum of the linked list.

use super::ListNode;

struct Solution;

impl Solution {
    pub fn pair_sum(mut head: Option<Box<ListNode>>) -> i32 {
        // somewhat better solution is based on reversing half of node, and easily iterate through both halves
        let mut twin_sum_vec: Vec<i32> = Vec::new();

        while let Some(node) = head {
            twin_sum_vec.push(node.val);
            head = node.next;
        }

        let mut max_sum: i32 = i32::MIN;

        for i in 0..(twin_sum_vec.len() / 2) {
            max_sum = max_sum.max(twin_sum_vec[i] + twin_sum_vec[twin_sum_vec.len() - i - 1]);
        }

        max_sum
    }
}

#[cfg(test)]
mod tests {
    use crate::tasks::linked_list::leetcode_list::ListNode;

    use super::Solution;

    #[test]
    fn pair_sum_1() {
        let list_node = ListNode::from_iter([4, 2, 2, 3]);

        let result = Solution::pair_sum(Some(Box::new(list_node)));

        assert_eq!(result, 7);
    }

    #[test]
    fn pair_sum_2() {
        let list_node = ListNode::from_iter([4, 2]);

        let result = Solution::pair_sum(Some(Box::new(list_node)));

        assert_eq!(result, 6);
    }
}
