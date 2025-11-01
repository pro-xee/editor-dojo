use std::time::Duration;

/// Represents the result of completing a challenge
///
/// This is a value object that captures the outcome of an attempt.
#[derive(Debug, Clone)]
pub struct Solution {
    completed: bool,
    elapsed_time: Duration,
}

impl Solution {
    pub fn completed(elapsed_time: Duration) -> Self {
        Self {
            completed: true,
            elapsed_time,
        }
    }

    pub fn incomplete(elapsed_time: Duration) -> Self {
        Self {
            completed: false,
            elapsed_time,
        }
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn elapsed_seconds(&self) -> u64 {
        self.elapsed_time.as_secs()
    }
}
