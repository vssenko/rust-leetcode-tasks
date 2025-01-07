//! Guess number
//! We are playing the Guess Game. The game is as follows:
//! I pick a number from 1 to n. You have to guess which number I picked.
//! Every time you guess wrong, I will tell you whether the number I picked is higher or lower than your guess.
//! You call a pre-defined API int guess(int num), which returns three possible results:
//! -1: Your guess is higher than the number I picked (i.e. num > pick).
//! 1: Your guess is lower than the number I picked (i.e. num < pick).
//! 0: your guess is equal to the number I picked (i.e. num == pick).
//! Return the number that I picked.
//! https://leetcode.com/problems/guess-number-higher-or-lower

use std::sync::{Mutex, OnceLock};

// Comment this section to submit to LeetCode.
static TEST_MUTEX: Mutex<()> = Mutex::new(());
static mut PICKED_NUMBER: i32 = 0;
#[allow(static_mut_refs)]
unsafe fn guess(num: i32) -> i32 {
    match num.cmp(&PICKED_NUMBER) {
        std::cmp::Ordering::Less => Solution::NUM_IS_LOWER,
        std::cmp::Ordering::Equal => Solution::NUM_IS_EQUAL,
        std::cmp::Ordering::Greater => Solution::NUM_IS_GREATER,
    }
}

struct Solution;

impl Solution {
    const NUM_IS_GREATER: i32 = -1;
    const NUM_IS_LOWER: i32 = 1;
    const NUM_IS_EQUAL: i32 = 0;

    #[allow(non_snake_case)]
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut start_range: i64 = 0;
        let mut end_range: i64 = n as i64;
        loop {
            if end_range - start_range < 2 {
                for guess_num in start_range..=end_range {
                    if guess(guess_num as i32) == Self::NUM_IS_EQUAL {
                        return guess_num as i32;
                    }
                }

                panic!("Something went wront");
            }

            let guess_num = (end_range + start_range) / 2;
            let guess_res = guess(guess_num as i32);
            match guess_res {
                Self::NUM_IS_EQUAL => {
                    return guess_num as i32;
                }
                Self::NUM_IS_LOWER => {
                    start_range = guess_num;
                }
                Self::NUM_IS_GREATER => {
                    end_range = guess_num;
                }
                _ => unreachable!(),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn guess_number_1() {
        let _guard = super::TEST_MUTEX.lock().unwrap();
        unsafe {
            super::PICKED_NUMBER = 42;
        };

        let result = unsafe { Solution::guessNumber(128) };

        assert_eq!(result, 42);
    }

    #[test]
    fn guess_number_2() {
        let _guard = super::TEST_MUTEX.lock().unwrap();
        unsafe {
            super::PICKED_NUMBER = 43;
        }

        let result = unsafe { Solution::guessNumber(128) };

        assert_eq!(result, 43);
    }

    #[test]
    fn guess_number_3() {
        let _guard = super::TEST_MUTEX.lock().unwrap();
        unsafe { super::PICKED_NUMBER = 1 }

        let result = unsafe { Solution::guessNumber(1) };

        assert_eq!(result, 1);
    }

    #[test]
    fn guess_number_4() {
        let _guard = super::TEST_MUTEX.lock().unwrap();
        unsafe { super::PICKED_NUMBER = 1702766719 };

        let result = unsafe { Solution::guessNumber(2126753390) };

        assert_eq!(result, 1702766719);
    }
}
