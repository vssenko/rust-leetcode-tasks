//! History map
//! Write map that holds only last N accessed/inserted elements

use std::collections::{HashMap, VecDeque};

struct HistoryMap<K, T> {
    max_size: usize,
    map: HashMap<K, T>,
    history: VecDeque<K>,
}

impl<K, T> HistoryMap<K, T>
where
    K: Eq + std::hash::Hash + Clone + std::fmt::Debug,
{
    pub fn new(max_size: usize) -> Self {
        Self {
            history: VecDeque::new(),
            map: HashMap::new(),
            max_size,
        }
    }

    fn _update_history(&mut self, current_op_key: &K) {
        // completely not optimized solution
        // which is implemented only to cover tests correctly
        // to update afterwards to O(1)

        let new_history: VecDeque<K> = self
            .history
            .iter()
            .filter(|&k| !k.eq(current_op_key))
            .cloned()
            .collect();

        self.history = new_history;
        self.history.push_back(current_op_key.clone());
    }

    fn _ensure_max_history_treshold(&mut self) {
        while self.history.len() > self.max_size {
            if let Some(key_to_remove) = self.history.pop_front() {
                self.map.remove(&key_to_remove);
            } else {
                break;
            }
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&T> {
        if self.map.contains_key(key) {
            self._update_history(key);
            self._ensure_max_history_treshold();
        }
        self.map.get(key)
    }

    pub fn get_untriggered(&mut self, key: &K) -> Option<&T> {
        if self.map.contains_key(key) {
            self._update_history(key);
        }
        self.map.get(key)
    }

    pub fn get_untraced(&mut self, key: &K) -> Option<&T> {
        self.map.get(key)
    }

    pub fn put(&mut self, key: K, value: T) {
        self._update_history(&key);
        self._ensure_max_history_treshold();
        self.map.insert(key, value);
    }

    pub fn put_untriggered(&mut self, key: K, value: T) {
        self._update_history(&key);
        self.map.insert(key, value);
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
        hm.put("key_2".into(), 22);
        hm.put("key_1".into(), 11);
        hm.put("key_4".into(), 4);
        hm.put("key_5".into(), 5);

        assert!(hm.get_untraced(&"key_1".into()).is_some());
        assert!(hm.get_untraced(&"key_2".into()).is_none());
        assert!(hm.get_untraced(&"key_3".into()).is_none());
        assert!(hm.get_untraced(&"key_4".into()).is_some());
        assert!(hm.get_untraced(&"key_5".into()).is_some());
    }
}
