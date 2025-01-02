//! Keys and Rooms
//! There are n rooms labeled from 0 to n - 1 and all the rooms are locked except for room 0.
//! Your goal is to visit all the rooms. However, you cannot enter a locked room without having its key.
//! https://leetcode.com/problems/keys-and-rooms

struct Solution;

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut visited_rooms: Vec<bool> = vec![false; rooms.len()];

        let mut unused_keys: Vec<i32> = vec![0];

        let mut current_room = 0;

        while !unused_keys.is_empty() {
            let mut next_level_keys: Vec<i32> = vec![];

            for key in unused_keys.iter() {
                if !visited_rooms[*key as usize] {
                    visited_rooms[*key as usize] = true;
                    next_level_keys.extend(rooms[*key as usize].iter().cloned());
                }
            }

            unused_keys = next_level_keys;
        }

        visited_rooms.iter().all(|b| *b)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn can_visit_all_rooms_1() {
        let rooms = vec![vec![1], vec![2], vec![3], vec![]];

        let result = Solution::can_visit_all_rooms(rooms);

        assert!(result)
    }

    #[test]
    fn can_visit_all_rooms_2() {
        let rooms = vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]];

        let result = Solution::can_visit_all_rooms(rooms);

        assert!(!result)
    }
}
