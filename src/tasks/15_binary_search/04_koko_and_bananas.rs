//! Koko Eating Bananas
//! Koko loves to eat bananas. There are n piles of bananas, the ith pile has piles[i] bananas. The guards have gone and will come back in h hours.
//! Koko can decide her bananas-per-hour eating speed of k. Each hour, she chooses some pile of bananas and eats k bananas from that pile. If the pile has less than k bananas, she eats all of them instead and will not eat any more bananas during this hour.
//! Koko likes to eat slowly but still wants to finish eating all the bananas before the guards return.
//! Return the minimum integer k such that she can eat all the bananas within h hours.
//! https://leetcode.com/problems/koko-eating-bananas

struct Solution;

impl Solution {
    fn divide_ceil(a: i32, b: i32) -> i32 {
        // not sure about this...
        (a + b - 1) / b
    }
    pub fn min_eating_speed(mut piles: Vec<i32>, h: i32) -> i32 {
        if piles.len() as i32 > h {
            return -1;
        }

        piles.sort();

        let calc_hours_for_speed = |speed: i32| -> i32 {
            let mut spent_hours = 0;

            for bananas in piles.iter() {
                spent_hours += Self::divide_ceil(*bananas, speed);
            }

            spent_hours
        };

        let max_pile = *piles.last().unwrap();

        // let's imagine possible speed range from 1 to max_pile
        // and try to apply binary sorting here
        let mut left = 1;
        let mut right = max_pile;

        while left < right {
            let medium_val = (right + left) / 2;
            let hours_spent_to_eat = calc_hours_for_speed(medium_val);

            match hours_spent_to_eat.cmp(&h) {
                std::cmp::Ordering::Greater => {
                    left = medium_val + 1;
                }
                std::cmp::Ordering::Less => {
                    right = medium_val;
                }
                std::cmp::Ordering::Equal => {
                    right = medium_val;
                }
            }
        }

        left
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn min_eating_speed_1() {
        let piles = vec![3, 6, 7, 11];
        let h = 8;

        let result = Solution::min_eating_speed(piles, h);

        assert_eq!(result, 4);
    }
}
