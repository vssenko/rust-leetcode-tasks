/// Kids with greatest number of candies.
/// https://leetcode.com/problems/kids-with-the-greatest-number-of-candies

struct Solution {}

impl Solution {
    #[allow(unused)]
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max_val = *candies.iter().max().unwrap();

        candies
            .iter()
            .map(|kid_candies| *kid_candies + extra_candies >= max_val)
            .collect::<Vec<bool>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kids_with_candies_1() {
        let candies: Vec<i32> = vec![2, 3, 5, 1, 3];
        let extra_candies = 3;

        let result = Solution::kids_with_candies(candies, extra_candies);

        assert_eq!(result, [true, true, true, false, true])
    }
}
