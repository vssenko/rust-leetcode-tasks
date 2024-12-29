//! Asteroid Collision
//! We are given an array asteroids of integers representing asteroids in a row.
//! The indices of the asteriod in the array represent their relative position in space.
//! For each asteroid, the absolute value represents its size, and the sign represents its direction (positive meaning right, negative meaning left).
//! Each asteroid moves at the same speed.
//! Find out the state of the asteroids after all collisions.
//! If two asteroids meet, the smaller one will explode. If both are the same size, both will explode.
//! Two asteroids moving in the same direction will never meet.
//! https://leetcode.com/problems/asteroid-collision

struct Solution;

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        if asteroids.is_empty() {
            return vec![];
        }

        let mut result = Vec::<i32>::with_capacity(asteroids.len() / 2);

        for asteroid in asteroids {
            loop {
                match result.last() {
                    Some(v) => {
                        if (v.is_positive() == asteroid.is_positive())
                            || (asteroid.is_positive() && v.is_negative())
                        {
                            result.push(asteroid);
                            break;
                        } else {
                            match asteroid.abs().cmp(&v.abs()) {
                                std::cmp::Ordering::Less => {
                                    break;
                                }
                                std::cmp::Ordering::Equal => {
                                    result.pop();
                                    break;
                                }
                                std::cmp::Ordering::Greater => {
                                    result.pop();
                                    // continue with the next element in result
                                }
                            }
                        }
                    }
                    None => {
                        result.push(asteroid);
                        break;
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn asteroid_collision_1() {
        let asteroids = vec![5, 10, -5];

        let result = Solution::asteroid_collision(asteroids);

        assert_eq!(result, [5, 10]);
    }

    #[test]
    fn asteroid_collision_2() {
        let asteroids = vec![8, -8];

        let result = Solution::asteroid_collision(asteroids);

        assert_eq!(result, []);
    }

    #[test]
    fn asteroid_collision_3() {
        let asteroids = vec![10, 2, -5];

        let result = Solution::asteroid_collision(asteroids);

        assert_eq!(result, [10]);
    }

    #[test]
    fn asteroid_collision_4() {
        let asteroids = vec![-2, -1, 1, 2];

        let result = Solution::asteroid_collision(asteroids);

        assert_eq!(result, [-2, -1, 1, 2]);
    }
}
