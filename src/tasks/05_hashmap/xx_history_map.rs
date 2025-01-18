//! History map
//! Write map that holds only last N accessed elements
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

struct HistoryMap<K, T> {
    max_history: usize,
    map: HashMap<K, T>,
    history: VecDeque<K>,
}

impl<K, T> HistoryMap<K, T>
where
    K: Eq + Hash + Clone,
{
    pub fn new(max_history: usize) -> Self {
        Self {
            history: VecDeque::new(),
            map: HashMap::new(),
            max_history,
        }
    }

    fn _check_history(&mut self, current_op_key: &K) {
        let new_k: K = if self.history.front().is_some_and(|fk| fk == current_op_key) {
            self.history.pop_front().unwrap()
        } else {
            current_op_key.clone()
        };

        self.history.push_back(new_k);

        if self.history.len() > self.max_history {
            if let Some(key_to_remove) = self.history.pop_front() {
                self.map.remove(&key_to_remove);
            }
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&T> {
        // can be adjusted to check or not with future options
        //self._check_history(key);
        self.map.get(key)
    }

    pub fn insert(&mut self, key: K, value: T) {
        // can be adjusted to check or not with future options
        self._check_history(&key);
        self.map.insert(key, value);
    }
}

#[cfg(test)]
mod tests {
    use super::HistoryMap;

    #[test]
    fn history_map_1() {
        let n = 3;
        let mut hm: HistoryMap<String, usize> = HistoryMap::new(n);

        hm.insert("key_1".into(), 1);
        hm.insert("key_2".into(), 2);
        hm.insert("key_3".into(), 3);

        assert!(hm.get(&"key_1".into()).is_some());
        assert!(hm.get(&"key_2".into()).is_some());
        assert!(hm.get(&"key_3".into()).is_some());

        hm.insert("key_4".into(), 4);

        assert!(hm.get(&"key_1".into()).is_none());
        assert!(hm.get(&"key_2".into()).is_some());
        assert!(hm.get(&"key_3".into()).is_some());
        assert!(hm.get(&"key_4".into()).is_some());
    }
}
