use std::time::Duration;
use super::recording::Recording;

/// Represents the result of completing a challenge
///
/// This is a value object that captures the outcome of an attempt.
#[derive(Debug, Clone)]
pub struct Solution {
    completed: bool,
    elapsed_time: Duration,
    recording: Option<Recording>,
}

impl Solution {
    pub fn completed(elapsed_time: Duration) -> Self {
        Self {
            completed: true,
            elapsed_time,
            recording: None,
        }
    }

    pub fn incomplete(elapsed_time: Duration) -> Self {
        Self {
            completed: false,
            elapsed_time,
            recording: None,
        }
    }

    pub fn with_recording(mut self, recording: Recording) -> Self {
        self.recording = Some(recording);
        self
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn elapsed_seconds(&self) -> u64 {
        self.elapsed_time.as_secs()
    }

    pub fn recording(&self) -> Option<&Recording> {
        self.recording.as_ref()
    }
}
