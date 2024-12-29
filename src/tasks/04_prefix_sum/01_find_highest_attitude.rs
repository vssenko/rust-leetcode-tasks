//! Find the Highest Altitude
//! There is a biker going on a road trip.
//! The road trip consists of n + 1 points at different altitudes.
//! The biker starts his trip on point 0 with altitude equal 0.
//! You are given an integer array gain of length n where gain[i] is the net gain in altitude between points i​​​​​​ and i + 1 for all (0 <= i < n).
//! Return the highest altitude of a point.
//! https://leetcode.com/problems/find-the-highest-altitude

struct Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut highest_altitude = 0;
        let mut current_altitude = 0;
        for change in gain {
            current_altitude += change;
            highest_altitude = highest_altitude.max(current_altitude);
        }

        highest_altitude
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn largest_altitude_1() {
        let gain = vec![-5, 1, 5, 0, -7];

        let result = Solution::largest_altitude(gain);

        assert_eq!(result, 1);
    }
}
