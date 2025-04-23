//! Minimum Flips to Make a OR b Equal to c
//! Given 3 positives numbers a, b and c.
//! Return the minimum flips required in some bits of a and b to make ( a OR b == c ).
//! (bitwise OR operation).
//! Flip operation consists of change any single bit 1 to 0 or change the bit 0 to 1 in their binary representation.
//! https://leetcode.com/problems/minimum-flips-to-make-a-or-b-equal-to-c

// converting to array of bits is lame, but explicit

struct Solution;

impl Solution {
    fn i32_to_bits(n: i32) -> [u8; 32] {
        let mut bits = [0; 32];
        for i in 0..32 {
            bits[31 - i] = ((n >> i) & 1) as u8;
        }
        bits
    }

    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let a_bits = Solution::i32_to_bits(a);
        let b_bits = Solution::i32_to_bits(b);
        let c_bits = Solution::i32_to_bits(c);

        let mut operations_required = 0;

        for (position, c_bit) in c_bits.iter().enumerate() {
            operations_required = operations_required
                + match (a_bits[position], b_bits[position], c_bit) {
                    (1, 1, 1) => 0,
                    (1, 0, 1) => 0,
                    (0, 1, 1) => 0,
                    (0, 0, 1) => 1,
                    (1, 1, 0) => 2,
                    (1, 0, 0) => 1,
                    (0, 1, 0) => 1,
                    (0, 0, 0) => 0,
                    _ => panic!("impossible"),
                };
        }

        operations_required
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn min_flips_1() {
        let a = 2;
        let b = 6;
        let c = 5;

        assert_eq!(Solution::min_flips(a, b, c), 3);
    }
}
