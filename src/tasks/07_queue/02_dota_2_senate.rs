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
            let mut new_queue = VecDeque::<char>::with_capacity(queue.len());

            let mut r_to_remove = 0;
            let mut d_to_remove = 0;

            for senator in queue.iter() {
                let (self_to_remove_counter, other_to_remove_counter) = if *senator == RADIANT {
                    (&mut r_to_remove, &mut d_to_remove)
                } else {
                    (&mut d_to_remove, &mut r_to_remove)
                };

                if *self_to_remove_counter == 0 {
                    new_queue.push_back(*senator);
                    *other_to_remove_counter += 1
                } else {
                    *self_to_remove_counter -= 1;
                }
            }

            queue.clear();
            for voted_senator in new_queue {
                let (self_to_remove_counter) = if voted_senator == RADIANT {
                    &mut r_to_remove
                } else {
                    &mut d_to_remove
                };

                if *self_to_remove_counter > 0 {
                    *self_to_remove_counter -= 1;
                } else {
                    queue.push_back(voted_senator);
                }
            }

            if Self::all_elements_same(queue.iter()) {
                return Self::get_label(&queue[0]);
            }
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
