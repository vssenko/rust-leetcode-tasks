/// Can place flowers
/// https://leetcode.com/problems/can-place-flowers
struct Solution {}

impl Solution {
    #[allow(unused)]
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
        if n == 0 {
            return true;
        }

        let mut placed_flowers = 0;
        let mut i = 0;

        while (i < flowerbed.len()) {
            let curr = *flowerbed.get(i).unwrap();
            let prev = *flowerbed
                .get(i.checked_sub(1).unwrap_or(usize::MAX))
                .unwrap_or(&0);

            let next = *flowerbed.get(i + 1).unwrap_or(&0);

            if curr == 0 && prev == 0 && next == 0 {
                placed_flowers += 1;

                if placed_flowers == n {
                    return true;
                }

                i += 2;
            } else {
                i += 1;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn can_place_flowers_1() {
        let flowerbed: Vec<i32> = vec![1, 0, 0, 0, 1];
        let n = 1;

        let result = Solution::can_place_flowers(flowerbed, n);

        assert_eq!(result, true);
    }

    #[test]
    fn can_place_flowers_2() {
        let flowerbed: Vec<i32> = vec![1, 0, 0, 0, 1];
        let n = 2;

        let result = Solution::can_place_flowers(flowerbed, n);

        assert_eq!(result, false);
    }
}
