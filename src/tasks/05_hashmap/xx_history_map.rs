//! History map
//! Write map that holds only last N accessed/inserted elements

use std::collections::{HashMap, VecDeque};

struct HistoryMap<K, T> {
    max_history: usize,
    map: HashMap<K, T>,
    history: VecDeque<K>,
}

impl<K, T> HistoryMap<K, T>
where
    K: Eq + std::hash::Hash + Clone,
{
    pub fn new(max_history: usize) -> Self {
        Self {
            history: VecDeque::new(),
            map: HashMap::new(),
            max_history,
        }
    }

    fn _update_history(&mut self, current_op_key: &K) {
        let new_k: K = if self.history.front().is_some_and(|fk| fk == current_op_key) {
            self.history.pop_front().unwrap()
        } else {
            current_op_key.clone()
        };

        self.history.push_back(new_k);
    }

    fn _ensure_max_history_treshold(&mut self) {
        while self.history.len() > self.max_history {
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

    pub fn insert(&mut self, key: K, value: T) {
        self._update_history(&key);
        self._ensure_max_history_treshold();
        self.map.insert(key, value);
    }

    pub fn insert_untriggered(&mut self, key: K, value: T) {
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

        hm.insert("key_1".into(), 1);
        hm.insert("key_2".into(), 2);
        hm.insert("key_3".into(), 3);

        assert!(hm.get_untraced(&"key_1".into()).is_some());
        assert!(hm.get_untraced(&"key_2".into()).is_some());
        assert!(hm.get_untraced(&"key_3".into()).is_some());

        hm.insert("key_4".into(), 4);

        assert!(hm.get_untraced(&"key_1".into()).is_none());
        assert!(hm.get_untraced(&"key_2".into()).is_some());
        assert!(hm.get_untraced(&"key_3".into()).is_some());
        assert!(hm.get_untraced(&"key_4".into()).is_some());
    }

    #[test]
    fn history_map_trigger_on_get() {
        let n = 3;
        let mut hm: HistoryMap<String, usize> = HistoryMap::new(n);

        hm.insert_untriggered("key_1".into(), 1);
        hm.insert_untriggered("key_2".into(), 2);
        hm.insert_untriggered("key_3".into(), 3);
        hm.insert_untriggered("key_4".into(), 4);

        assert!(hm.get_untraced(&"key_1".into()).is_some());
        assert!(hm.get_untraced(&"key_2".into()).is_some());
        assert!(hm.get_untraced(&"key_3".into()).is_some());
        assert!(hm.get_untraced(&"key_4".into()).is_some());

        // by this operation accessing "key_1" is the newest operation
        // while "key_2" become the oldest one and out of max treshold.
        assert!(hm.get(&"key_1".into()).is_some());
        assert!(hm.get(&"key_2".into()).is_none());
        assert!(hm.get(&"key_3".into()).is_some());
        assert!(hm.get(&"key_4".into()).is_some());
        assert!(hm.get(&"key_1".into()).is_some());
    }
}
