//! Dota2 Senate
//! https://leetcode.com/problems/dota2-senate

use std::{
    collections::{HashSet, VecDeque},
    env::temp_dir,
};

struct Solution;

const RADIANT: char = 'R';
const DIRE: char = 'D';

const RADIANT_LABEL: &str = "Radiant";
const DIRE_LABEL: &str = "Dire";

impl Solution {
    fn all_elements_same<T: PartialEq, U: Iterator<Item = T>>(mut iter: U) -> bool {
        let Some(item) = iter.next() else {
            return false;
        };
        for next in iter {
            if next != item {
                return false;
            }
        }

        true
    }

    fn get_label(char: &char) -> String {
        match *char {
            RADIANT => RADIANT_LABEL.to_string(),
            DIRE => DIRE_LABEL.to_string(),
            _ => panic!("Unsupported"),
        }
    }

    pub fn predict_party_victory(senate: String) -> String {
        let mut queue: VecDeque<char> = senate.chars().collect();

        loop {
            let mut voted_queue = VecDeque::with_capacity(queue.len());

            while queue.front().is_some() {
                let senator = queue.pop_front().unwrap();

                let mut found_to_remove = false;
                for (i, next_senator) in queue.iter().enumerate() {
                    if *next_senator != senator {
                        queue.remove(i);
                        found_to_remove = true;
                        break;
                    }
                }

                if (!found_to_remove) {
                    for (i, next_senator) in voted_queue.iter().enumerate() {
                        if *next_senator != senator {
                            voted_queue.remove(i);
                            break;
                        }
                    }
                }

                voted_queue.push_back(senator);
            }

            if Self::all_elements_same(voted_queue.iter()) {
                return Self::get_label(&voted_queue[0]);
            }

            queue = voted_queue;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn predict_party_victory_1() {
        let result = Solution::predict_party_victory("RD".into());
        assert_eq!(result, "Radiant");
    }

    #[test]
    fn predict_party_victory_2() {
        let result = Solution::predict_party_victory("RDD".into());
        assert_eq!(result, "Dire");
    }

    #[test]
    fn predict_party_victory_3() {
        let result = Solution::predict_party_victory("DDRRR".into());
        assert_eq!(result, "Dire");
    }

    #[test]
    fn predict_party_victory_4() {
        let result = Solution::predict_party_victory("DRRD".into());
        assert_eq!(result, "Dire");
    }
}
