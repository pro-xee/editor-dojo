use crate::application::{AchievementChecker, ProgressRepository};
use crate::domain::{Achievement, ChallengeStats, Progress, Solution};
use anyhow::{Context, Result};
use chrono::Utc;
use std::sync::{Arc, Mutex};

/// Application service for tracking and managing user progress
pub struct ProgressTracker<R: ProgressRepository> {
    repository: Arc<R>,
    progress: Arc<Mutex<Progress>>,
}

impl<R: ProgressRepository> ProgressTracker<R> {
    /// Create new progress tracker with given repository
    pub fn new(repository: R) -> Result<Self> {
        let progress = repository.load()?;
        Ok(Self {
            repository: Arc::new(repository),
            progress: Arc::new(Mutex::new(progress)),
        })
    }

    /// Get current progress (thread-safe read)
    pub fn get_progress(&self) -> Result<Progress> {
        let progress = self.progress
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to acquire progress lock - mutex was poisoned"))?;
        Ok(progress.clone())
    }

    /// Record a challenge attempt
    pub fn record_solution(&self, challenge_id: &str, solution: &Solution) -> Result<()> {
        let mut progress = self.progress
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to acquire progress lock - mutex was poisoned"))?;

        let keystrokes = solution
            .recording()
            .map(|r| r.keystroke_count() as u32);

        progress.record_attempt(
            challenge_id.to_string(),
            solution.is_completed(),
            solution.elapsed_time(),
            keystrokes,
            Utc::now(),
        );

        self.repository.save(&progress)?;
        Ok(())
    }

    /// Get stats for a specific challenge
    pub fn get_challenge_stats(&self, challenge_id: &str) -> Result<Option<ChallengeStats>> {
        let progress = self.progress
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to acquire progress lock - mutex was poisoned"))?;
        Ok(progress.get_challenge_stats(challenge_id).cloned())
    }

    /// Check if this solution beats any personal record
    pub fn is_new_record(&self, challenge_id: &str, solution: &Solution) -> Result<(bool, bool)> {
        let progress = self.progress
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to acquire progress lock - mutex was poisoned"))?;

        if let Some(stats) = progress.get_challenge_stats(challenge_id) {
            let keystrokes = solution
                .recording()
                .map(|r| r.keystroke_count() as u32);
            Ok(stats.is_new_record(solution.elapsed_time(), keystrokes))
        } else {
            // First attempt is always a new record if completed
            Ok((solution.is_completed(), solution.is_completed()))
        }
    }

    /// Set editor preference
    pub fn set_editor_preference(&self, editor: String) -> Result<()> {
        let mut progress = self.progress
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to acquire progress lock - mutex was poisoned"))?;
        *progress = progress.clone().set_editor_preference(editor);
        self.repository.save(&progress)?;
        Ok(())
    }

    /// Persist current progress to storage
    pub fn save(&self) -> Result<()> {
        let progress = self.progress
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to acquire progress lock - mutex was poisoned"))?;
        self.repository.save(&progress)
    }

    /// Check for new achievements and update progress
    pub fn check_achievements(&self, total_challenges: usize) -> Result<Vec<Achievement>> {
        let mut progress = self.progress
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to acquire progress lock - mutex was poisoned"))?;
        let newly_unlocked = AchievementChecker::check_achievements(&mut *progress, total_challenges);

        if !newly_unlocked.is_empty() {
            self.repository.save(&progress)?;
        }

        Ok(newly_unlocked)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Solution;
    use std::time::Duration;

    // Mock repository for testing
    struct MockRepository {
        progress: Mutex<Progress>,
    }

    impl MockRepository {
        fn new() -> Self {
            Self {
                progress: Mutex::new(Progress::new()),
            }
        }
    }

    impl ProgressRepository for MockRepository {
        fn load(&self) -> Result<Progress> {
            Ok(self.progress.lock().unwrap().clone())
        }

        fn save(&self, progress: &Progress) -> Result<()> {
            *self.progress.lock().unwrap() = progress.clone();
            Ok(())
        }

        fn exists(&self) -> bool {
            true
        }
    }

    #[test]
    fn test_record_completed_solution() {
        let repo = MockRepository::new();
        let tracker = ProgressTracker::new(repo).unwrap();

        let solution = Solution::completed(Duration::from_secs(10));
        tracker.record_solution("test-1", &solution).unwrap();

        let progress = tracker.get_progress().unwrap();
        assert_eq!(progress.total_completed(), 1);
        assert_eq!(progress.total_practice_time(), Duration::from_secs(10));
    }

    #[test]
    fn test_is_new_record_first_attempt() {
        let repo = MockRepository::new();
        let tracker = ProgressTracker::new(repo).unwrap();

        let solution = Solution::completed(Duration::from_secs(10));
        let (new_time, new_ks) = tracker.is_new_record("test-1", &solution).unwrap();

        assert!(new_time); // First completion is always a new record
        assert!(new_ks);
    }

    #[test]
    fn test_is_new_record_better_time() {
        let repo = MockRepository::new();
        let tracker = ProgressTracker::new(repo).unwrap();

        let first = Solution::completed(Duration::from_secs(10));
        tracker.record_solution("test-1", &first).unwrap();

        let second = Solution::completed(Duration::from_secs(8));
        let (new_time, _) = tracker.is_new_record("test-1", &second).unwrap();

        assert!(new_time);
    }

    #[test]
    fn test_is_new_record_worse_time() {
        let repo = MockRepository::new();
        let tracker = ProgressTracker::new(repo).unwrap();

        let first = Solution::completed(Duration::from_secs(10));
        tracker.record_solution("test-1", &first).unwrap();

        let second = Solution::completed(Duration::from_secs(12));
        let (new_time, _) = tracker.is_new_record("test-1", &second).unwrap();

        assert!(!new_time);
    }
}
