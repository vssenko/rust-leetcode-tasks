//! Counting Bits
//! Given an integer n, return an array ans of length n + 1 such
//! that for each i (0 <= i <= n), ans[i] is the number of 1's in the binary representation of i.

struct Solution;

impl Solution {
    fn int_to_bit_array(n: i32) -> [u8; 32] {
        let mut bits = [0u8; 32];
        for i in 0..32 {
            bits[31 - i] = ((n >> i) & 1) as u8;
        }
        bits
    }

    pub fn count_bits(n: i32) -> Vec<i32> {
        (0..=n)
            .map(|n| {
                Self::int_to_bit_array(n)
                    .iter()
                    .filter(|b| **b == 1)
                    .count() as i32
            })
            .collect::<Vec<i32>>()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn count_bits_1() {
        let n = 2;
        let result = super::Solution::count_bits(n);
        assert_eq!(result, [0, 1, 1])
    }

    #[test]
    fn count_bits_2() {
        let n = 5;
        let result = super::Solution::count_bits(n);
        assert_eq!(result, [0, 1, 1, 2, 1, 2])
    }
}
