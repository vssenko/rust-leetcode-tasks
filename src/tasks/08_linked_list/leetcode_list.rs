#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[cfg(test)]
mod extensions {
    use super::ListNode;

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
}
