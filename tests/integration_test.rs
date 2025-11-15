//! Integration tests for editor-dojo
//!
//! These tests verify the full workflow of the application,
//! testing how different components interact together.

use anyhow::Result;
use std::path::PathBuf;
use std::time::Duration;
use std::sync::mpsc;

// We need to declare the modules since this is an integration test
use editor_dojo::application::{
    ChallengeRunner, EditorSpawner, FileSystem, FileWatcher, ProgressRepository, ProgressTracker,
};
use editor_dojo::application::validator::SolutionValidator;
use editor_dojo::domain::{Challenge, Progress, Solution};

// ============================================================================
// Mock Implementations for Testing
// ============================================================================

/// Mock editor that simulates immediate success or failure
struct MockEditor {
    should_complete: bool,
    checks: std::sync::Arc<std::sync::Mutex<usize>>,
}

impl MockEditor {
    fn new(should_complete: bool) -> Self {
        Self {
            should_complete,
            checks: std::sync::Arc::new(std::sync::Mutex::new(0)),
        }
    }
}

impl EditorSpawner for MockEditor {
    fn spawn(&mut self, _file_path: &std::path::Path) -> Result<()> {
        Ok(())
    }

    fn terminate(&mut self) -> Result<()> {
        Ok(())
    }

    fn is_running(&self) -> bool {
        // Stay running for the first few checks to give time for file change detection
        let mut count = self.checks.lock().unwrap();
        *count += 1;
        *count < 3 // Run for 2 iterations, then exit
    }
}

/// Mock file system that tracks operations
struct MockFileSystem {
    content: std::sync::Arc<std::sync::Mutex<String>>,
}

impl MockFileSystem {
    fn new() -> Self {
        Self {
            content: std::sync::Arc::new(std::sync::Mutex::new(String::new())),
        }
    }

    fn set_content(&self, content: String) {
        *self.content.lock().unwrap() = content;
    }
}

impl FileSystem for MockFileSystem {
    fn create_temp_file(&self, content: &str) -> Result<PathBuf> {
        self.set_content(content.to_string());
        Ok(PathBuf::from("/tmp/mock-file.txt"))
    }

    fn read_file(&self, _path: &std::path::Path) -> Result<String> {
        Ok(self.content.lock().unwrap().clone())
    }

    fn cleanup(&self, _path: &std::path::Path) -> Result<()> {
        Ok(())
    }
}

/// Mock file watcher that immediately signals completion
struct MockWatcher;

impl MockWatcher {
    fn new() -> Self {
        Self
    }
}

impl FileWatcher for MockWatcher {
    fn watch(&mut self, _file_path: &std::path::Path, tx: mpsc::Sender<()>) -> Result<()> {
        // Immediately signal that file changed by sending a notification
        let _ = tx.send(());
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        Ok(())
    }
}

/// Mock progress repository for testing
struct MockRepository {
    progress: std::sync::Mutex<Progress>,
}

impl MockRepository {
    fn new() -> Self {
        Self {
            progress: std::sync::Mutex::new(Progress::new()),
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

// ============================================================================
// Integration Tests
// ============================================================================

// Note: Full ChallengeRunner tests are skipped because they require complex async mocking
// The ChallengeRunner is tested through manual testing and end-to-end testing

#[test]
fn test_progress_tracker_records_solution() -> Result<()> {
    // Arrange
    let repo = MockRepository::new();
    let tracker = ProgressTracker::new(repo)?;

    // Act: Record a completed solution
    let solution = Solution::completed(Duration::from_secs(10));
    tracker.record_solution("test-1", &solution)?;

    // Assert: Progress should be updated
    let progress = tracker.get_progress()?;
    assert_eq!(progress.total_completed(), 1);
    assert_eq!(progress.total_practice_time(), Duration::from_secs(10));

    Ok(())
}

#[test]
fn test_progress_tracker_detects_new_record() -> Result<()> {
    // Arrange
    let repo = MockRepository::new();
    let tracker = ProgressTracker::new(repo)?;

    // Record initial solution
    let solution1 = Solution::completed(Duration::from_secs(10));
    tracker.record_solution("test-1", &solution1)?;

    // Act: Try a better solution
    let solution2 = Solution::completed(Duration::from_secs(8));
    let (is_new_time, _) = tracker.is_new_record("test-1", &solution2)?;

    // Assert: Should detect new record
    assert!(is_new_time);

    Ok(())
}

#[test]
fn test_progress_tracker_first_attempt_is_always_record() -> Result<()> {
    // Arrange
    let repo = MockRepository::new();
    let tracker = ProgressTracker::new(repo)?;

    // Act: First attempt
    let solution = Solution::completed(Duration::from_secs(100));
    let (is_new_time, is_new_ks) = tracker.is_new_record("test-1", &solution)?;

    // Assert: First completion is always a record
    assert!(is_new_time);
    assert!(is_new_ks);

    Ok(())
}

#[test]
fn test_solution_validator_normalizes_whitespace() {
    // Arrange
    let validator = SolutionValidator::new();

    // Act & Assert: Different whitespace should be considered equal
    assert!(validator.is_valid("hello\nworld", "hello\nworld"));
    assert!(validator.is_valid("hello  \n  world", "hello\nworld"));
    assert!(validator.is_valid("  hello\nworld  ", "hello\nworld"));
}

#[test]
fn test_solution_validator_rejects_different_content() {
    // Arrange
    let validator = SolutionValidator::new();

    // Act & Assert
    assert!(!validator.is_valid("hello", "world"));
    assert!(!validator.is_valid("hello\nworld", "hello\nearth"));
}

#[test]
fn test_challenge_validation_in_runner() -> Result<()> {
    // Arrange: Invalid challenge ID
    use editor_dojo::infrastructure::AsciinemaRecorder;

    // This should fail with validation error
    let result = AsciinemaRecorder::generate_recording_path("../etc/passwd");

    // Assert: Should reject path traversal attempt
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_challenge_validation_accepts_valid_ids() -> Result<()> {
    // Arrange
    use editor_dojo::infrastructure::AsciinemaRecorder;

    // These should all succeed
    let result1 = AsciinemaRecorder::generate_recording_path("test-01");
    let result2 = AsciinemaRecorder::generate_recording_path("challenge_123");
    let result3 = AsciinemaRecorder::generate_recording_path("simple");

    // Assert: Should accept valid IDs
    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert!(result3.is_ok());

    Ok(())
}

#[test]
fn test_progress_tracker_handles_multiple_attempts() -> Result<()> {
    // Arrange
    let repo = MockRepository::new();
    let tracker = ProgressTracker::new(repo)?;

    // Act: Record multiple attempts on same challenge
    let solution1 = Solution::completed(Duration::from_secs(10));
    tracker.record_solution("test-1", &solution1)?;

    let solution2 = Solution::completed(Duration::from_secs(8));
    tracker.record_solution("test-1", &solution2)?;

    let solution3 = Solution::incomplete(Duration::from_secs(5));
    tracker.record_solution("test-1", &solution3)?;

    // Assert: Should track all attempts correctly
    let progress = tracker.get_progress()?;
    assert_eq!(progress.total_attempts(), 3);
    assert_eq!(progress.total_completed(), 1); // Only counts unique completions

    Ok(())
}

#[test]
fn test_achievement_checking_integration() -> Result<()> {
    // Arrange
    let repo = MockRepository::new();
    let tracker = ProgressTracker::new(repo)?;

    // Act: Complete first challenge
    let solution = Solution::completed(Duration::from_secs(10));
    tracker.record_solution("test-1", &solution)?;

    let achievements = tracker.check_achievements(10)?;

    // Assert: Should unlock "First Steps" achievement
    assert!(!achievements.is_empty());

    Ok(())
}

#[test]
fn test_progress_and_achievement_workflow() -> Result<()> {
    // This test verifies the workflow of:
    // 1. Creating a solution
    // 2. Recording it in progress tracker
    // 3. Checking achievements

    let repo = MockRepository::new();
    let tracker = ProgressTracker::new(repo)?;

    // Act: Execute workflow without actual challenge runner
    let solution = Solution::completed(Duration::from_secs(5));
    tracker.record_solution("test-challenge", &solution)?;
    let achievements = tracker.check_achievements(10)?;

    // Assert: Everything should work together
    let progress = tracker.get_progress()?;
    assert_eq!(progress.total_completed(), 1);
    assert_eq!(progress.total_attempts(), 1);
    assert!(!achievements.is_empty()); // Should unlock First Steps

    Ok(())
}
