//! Number of Recent Calls
//! You have a RecentCounter class which counts the number of recent requests within a certain time frame.
//! https://leetcode.com/problems/number-of-recent-calls

struct RecentCounter {
    requests: Vec<i32>,
}

impl RecentCounter {
    pub fn new() -> Self {
        RecentCounter { requests: vec![] }
    }

    pub fn ping(&mut self, t: i32) -> i32 {
        self.requests.push(t);

        self.requests.retain(|&i| i >= t - 3000 && i <= t);

        self.requests.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::RecentCounter;

    #[test]
    fn recent_counter_1() {
        let mut recent_counter = RecentCounter::new();

        assert_eq!(recent_counter.ping(1), 1);
        assert_eq!(recent_counter.ping(100), 2);
        assert_eq!(recent_counter.ping(3001), 3);
        assert_eq!(recent_counter.ping(3002), 3);
    }
}
