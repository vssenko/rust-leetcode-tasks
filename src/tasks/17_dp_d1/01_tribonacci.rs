//! The Tribonacci sequence Tn is defined as follows:
//! T0 = 0, T1 = 1, T2 = 1, and Tn+3 = Tn + Tn+1 + Tn+2 for n >= 0.
//! Given n, return the value of Tn.

use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        // 0 1 1 2 4 7 13 24...
        let mut tribo_deque: VecDeque<i32> = VecDeque::from([0, 1, 1]);

        if n <= 2 {
            return tribo_deque[n as usize];
        }

        for i in 3..=n {
            let sum = tribo_deque.iter().sum::<i32>();
            tribo_deque.push_back(sum);
            tribo_deque.pop_front();
        }

        tribo_deque.pop_back().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn tribonacci_1() {
        assert_eq!(Solution::tribonacci(0), 0);
    }

    #[test]
    fn tribonacci_2() {
        assert_eq!(Solution::tribonacci(1), 1);
    }

    #[test]
    fn tribonacci_3() {
        assert_eq!(Solution::tribonacci(2), 1);
    }

    #[test]
    fn tribonacci_4() {
        assert_eq!(Solution::tribonacci(3), 2);
    }

    #[test]
    fn tribonacci_5() {
        assert_eq!(Solution::tribonacci(4), 4);
    }

    #[test]
    fn tribonacci_6() {
        assert_eq!(Solution::tribonacci(25), 1389537);
    }
}
