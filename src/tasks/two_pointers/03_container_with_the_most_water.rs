//! Container with the most water
//! https://leetcode.com/problems/container-with-most-water

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        if height.len() < 2 {
            return 0;
        }

        let mut h_iter = height.iter().enumerate().peekable();

        let mut left_line = h_iter.next();
        let mut right_line = h_iter.next_back();

        let mut max_area: i32 = 0;

        unsafe {
            while let (Some((l_i, l_h)), Some((r_i, r_h))) = (left_line, right_line) {
                let area = l_h.min(r_h) * (r_i - l_i) as i32;

                if max_area < area {
                    max_area = area;
                }

                if l_h < r_h {
                    left_line = h_iter.next();
                } else {
                    right_line = h_iter.next_back();
                }
            }
        }

        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn max_area_1() {
        let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];

        let result = Solution::max_area(input);

        assert_eq!(result, 49);
    }

    #[test]
    fn max_area_2() {
        let input = vec![1, 3];

        let result = Solution::max_area(input);

        assert_eq!(result, 1);
    }

    #[test]
    fn max_area_3() {
        let input = vec![1];

        let result = Solution::max_area(input);

        assert_eq!(result, 0);
    }

    #[test]
    fn max_area_4() {
        let input = vec![1, 2, 1];

        let result = Solution::max_area(input);

        assert_eq!(result, 2);
    }
}
