//! History map
//! Write map that holds only last N accessed/inserted elements

use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    fmt::{Debug, Pointer},
    rc::Rc,
};

type NodeRef<T> = Rc<RefCell<GutsDequeNode<T>>>;

struct GutsDequeNode<T> {
    value: T,
    next: Option<NodeRef<T>>,
    prev: Option<NodeRef<T>>,
}

impl<T> GutsDequeNode<T> {
    pub fn new(value: T, prev: Option<NodeRef<T>>, next: Option<NodeRef<T>>) -> Self {
        GutsDequeNode { value, next, prev }
    }

    pub fn move_to_rc(self) -> NodeRef<T> {
        Rc::new(RefCell::new(self))
    }
}

struct GutsOutsideDeque<T> {
    head: Option<NodeRef<T>>,
    tail: Option<NodeRef<T>>,
}

impl<T> Debug for GutsDequeNode<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GutsDequeNode")
            .field("value", &self.value)
            .field(
                "next",
                &if self.next.is_some() {
                    format!("Some({:?})", self.next.as_ref().unwrap().borrow().value)
                } else {
                    "None".to_string()
                },
            )
            .field(
                "prev",
                &if self.prev.is_some() {
                    format!("Some({:?})", self.prev.as_ref().unwrap().borrow().value)
                } else {
                    "None".to_string()
                },
            )
            .finish()
    }
}

impl<T> Debug for GutsOutsideDeque<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut nodes_dbg: Vec<String> = Vec::new();

        self.for_each(&mut |node| {
            // finish push
            nodes_dbg.push(format!("{:?}", node.borrow()))
        });

        write!(
            f,
            "GutsOutsideDeque {{ \nhead: {:?}, \ntail: {:?}, \nlist: [{}] }}",
            self.head,
            self.tail,
            nodes_dbg.join(",")
        )
    }
}

impl<T> GutsOutsideDeque<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    // much better to do through Iterator.
    pub fn for_each<F: FnMut(&NodeRef<T>)>(&self, cb: &mut F) {
        if let Some(head) = self.head.as_ref() {
            // complete garbage, but i don't care
            let mut walker: Option<NodeRef<T>> = Some(head.clone());
            while let Some(wk) = walker {
                cb(&wk);
                walker = wk.borrow().next.clone();
            }
        }
    }

    pub fn push_back(&mut self, item: T) {
        let item_rc = GutsDequeNode::new(item, self.tail.clone(), None).move_to_rc();
        match &self.tail {
            Some(tail) => {
                tail.borrow_mut().next = Some(item_rc.clone());
                self.tail = Some(item_rc);
            }
            None => self.tail = Some(item_rc),
        }
        if self.head.is_none() {
            self.head = self.tail.as_mut().map(|r| r.clone());
        }
    }

    pub fn push_front(&mut self, item: T) {
        let item_rc = GutsDequeNode::new(item, None, self.head.clone()).move_to_rc();
        match self.head.as_mut() {
            Some(head) => {
                head.borrow_mut().prev = Some(item_rc.clone());
                self.head = Some(item_rc);
            }
            None => self.head = Some(item_rc),
        }
        if self.tail.is_none() {
            self.tail = self.head.as_mut().map(|r| r.clone());
        }
    }

    // yes, this is ugly.
    pub fn front(&self) -> Option<&NodeRef<T>> {
        self.head.as_ref()
    }

    // yes, this is ugly too.
    pub fn back(&self) -> Option<&NodeRef<T>> {
        self.tail.as_ref()
    }

    pub fn pop_front(&mut self) -> Option<NodeRef<T>> {
        let head_item = match self.head.take() {
            Some(head) => {
                let next = head.borrow().next.clone();
                self.head = next;
                if self.head.is_some() {
                    self.head.as_ref().unwrap().borrow_mut().prev = None;
                }
                Some(head)
            }
            None => None,
        };

        if self.head.is_none() {
            self.tail = None;
        }

        head_item
    }

    pub fn pop_back(&mut self) -> Option<NodeRef<T>> {
        let tail_item = match self.tail.take() {
            Some(tail) => {
                let prev = tail.borrow().prev.clone();
                self.tail = prev;
                if self.tail.is_some() {
                    self.tail.as_ref().unwrap().borrow_mut().next = None;
                }
                Some(tail)
            }
            None => None,
        };

        if self.tail.is_none() {
            self.head = None;
        }

        tail_item
    }

    // all of that was just for this...
    pub fn move_to_back(&mut self, node: &NodeRef<T>) {
        let is_head = self
            .head
            .as_ref()
            .is_some_and(|h| h.as_ptr() == node.as_ptr());
        if is_head && node.borrow().next.is_some() {
            self.head = node.borrow().next.clone();
        }

        let original_prev = node.borrow().prev.clone();
        let original_next = node.borrow().next.clone();

        if original_prev.is_some() {
            original_prev.as_ref().unwrap().borrow_mut().next = original_next.clone();
        }

        if original_next.is_some() {
            if is_head {
                original_next.unwrap().borrow_mut().prev = None;
            } else {
                original_next.unwrap().borrow_mut().prev = original_prev.clone();
            }
        }

        let mut bor_muted = node.borrow_mut();
        bor_muted.prev = self.tail.clone();
        bor_muted.next = None;

        if (self.tail.is_some()) {
            self.tail.as_ref().unwrap().borrow_mut().next = Some(node.clone());
        }
        self.tail = Some(node.clone());
    }
}

struct HistoryMapItem<K, T> {
    item: T,
    history_node: NodeRef<K>,
}

struct HistoryMap<K, T> {
    max_size: usize,
    map: HashMap<K, HistoryMapItem<K, T>>,
    history: GutsOutsideDeque<K>,
}

impl<K, T> HistoryMap<K, T>
where
    K: Eq + std::hash::Hash + Clone + std::fmt::Debug,
    T: std::fmt::Debug,
{
    pub fn new(max_size: usize) -> Self {
        Self {
            history: GutsOutsideDeque::new(),
            map: HashMap::new(),
            max_size,
        }
    }

    fn _get_oldest_from_history(&mut self) -> Option<K> {
        self.history
            .pop_front()
            .map(|node| node.borrow().value.clone())
    }

    fn _update_existing_history_item(&mut self, current_op_key: &K) {
        if self
            .history
            .back()
            .is_some_and(|bk| bk.borrow().value.eq(current_op_key))
        {
            return;
        }

        if self
            .history
            .front()
            .is_some_and(|bk| bk.borrow().value.eq(current_op_key))
        {
            let front_node = self.history.pop_front().unwrap();

            self.history.push_back(front_node.borrow().value.clone());
            return;
        }

        let node = &self.map[current_op_key].history_node;
        self.history.move_to_back(node);
    }

    fn _ensure_max_size_treshold(&mut self) {
        while self.map.len() > self.max_size {
            if let Some(key_to_remove) = self._get_oldest_from_history() {
                self.map.remove(&key_to_remove);
            } else {
                break;
            }
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&T> {
        if self.map.contains_key(key) {
            self._update_existing_history_item(key);
            self._ensure_max_size_treshold();
        }
        self.map.get(key).map(|n| &n.item)
    }

    pub fn get_untriggered(&mut self, key: &K) -> Option<&T> {
        if self.map.contains_key(key) {
            self._update_existing_history_item(key);
        }
        self.map.get(key).map(|n| &n.item)
    }

    pub fn get_untraced(&mut self, key: &K) -> Option<&T> {
        self.map.get(key).map(|n| &n.item)
    }

    #[allow(clippy::map_entry)]
    fn _put(&mut self, key: K, value: T, ensure: bool) {
        if self.map.contains_key(&key) {
            self._update_existing_history_item(&key);
            self.map.entry(key).and_modify(|item| item.item = value);
        } else {
            self.history.push_back(key.clone());
            let new_node = self.map.insert(
                key,
                HistoryMapItem {
                    item: value,
                    history_node: self.history.tail.as_ref().unwrap().clone(),
                },
            );
        }

        if (ensure) {
            self._ensure_max_size_treshold();
        }
    }

    fn put(&mut self, key: K, value: T) {
        self._put(key, value, true);
    }

    pub fn put_untriggered(&mut self, key: K, value: T) {
        self._put(key, value, false);
    }
}

#[cfg(test)]
mod tests {
    use super::HistoryMap;

    #[test]
    fn history_map_trigger_on_insert() {
        let n = 3;
        let mut hm: HistoryMap<String, usize> = HistoryMap::new(n);

        hm.put("key_1".into(), 1);
        hm.put("key_2".into(), 2);
        hm.put("key_3".into(), 3);

        assert!(hm.get_untraced(&"key_1".into()).is_some());
        assert!(hm.get_untraced(&"key_2".into()).is_some());
        assert!(hm.get_untraced(&"key_3".into()).is_some());

        hm.put("key_4".into(), 4);

        assert!(hm.get(&"key_1".into()).is_none());
        assert!(hm.get(&"key_2".into()).is_some());
        assert!(hm.get(&"key_3".into()).is_some());
        assert!(hm.get(&"key_4".into()).is_some());
    }

    #[test]
    fn history_map_should_not_rewrite_history_on_same_key() {
        let n = 3;
        let mut hm: HistoryMap<String, usize> = HistoryMap::new(n);

        hm.put("key_1".into(), 1);
        hm.put("key_2".into(), 2);

        hm.put("key_1".into(), 3);
        hm.put("key_1".into(), 4);
        hm.put("key_1".into(), 5);
        hm.put("key_1".into(), 6);

        assert!(hm.get(&"key_1".into()).is_some());
        assert!(hm.get(&"key_2".into()).is_some());
    }

    #[test]
    fn history_map_should_count_middle_history_update() {
        let n = 3;
        let mut hm: HistoryMap<String, usize> = HistoryMap::new(n);

        hm.put("key_1".into(), 1);
        hm.put("key_2".into(), 2);
        hm.put("key_3".into(), 3);

        assert!(hm.get_untraced(&"key_1".into()).is_some());
        assert!(hm.get_untraced(&"key_2".into()).is_some());
        assert!(hm.get_untraced(&"key_3".into()).is_some());

        hm.put("key_2".into(), 22);

        assert!(hm.get_untraced(&"key_1".into()).is_some());
        assert!(hm.get_untraced(&"key_2".into()).is_some());
        assert!(hm.get_untraced(&"key_3".into()).is_some());

        hm.put("key_1".into(), 11);

        assert!(hm.get_untraced(&"key_1".into()).is_some());
        assert!(hm.get_untraced(&"key_2".into()).is_some());
        assert!(hm.get_untraced(&"key_3".into()).is_some());

        hm.put("key_4".into(), 4);

        assert!(hm.get_untraced(&"key_1".into()).is_some());
        assert!(hm.get_untraced(&"key_2".into()).is_some());
        assert!(hm.get_untraced(&"key_3".into()).is_none());
        assert!(hm.get_untraced(&"key_4".into()).is_some());

        hm.put("key_5".into(), 5);

        assert!(hm.get_untraced(&"key_1".into()).is_some());
        assert!(hm.get_untraced(&"key_2".into()).is_none());
        assert!(hm.get_untraced(&"key_3".into()).is_none());
        assert!(hm.get_untraced(&"key_4".into()).is_some());
        assert!(hm.get_untraced(&"key_5".into()).is_some());
    }
}
