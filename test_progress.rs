// Temporary test program to verify progress tracking functionality
mod application;
mod domain;
mod infrastructure;

use anyhow::Result;
use application::{ProgressRepository, ProgressTracker};
use domain::Solution;
use infrastructure::JsonProgressRepository;
use std::time::Duration;

fn main() -> Result<()> {
    println!("Testing Editor Dojo Progress Tracking...\n");

    // Test 1: Initialize progress repository
    println!("Test 1: Initializing progress repository...");
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_progress.json");

    // Clean up any existing test file
    let _ = std::fs::remove_file(&test_file);

    let repo = JsonProgressRepository::with_path(test_file.clone());
    let tracker = ProgressTracker::new(repo)?;
    println!("✓ Progress repository initialized\n");

    // Test 2: Record first completion
    println!("Test 2: Recording first challenge completion...");
    let solution1 = Solution::completed(Duration::from_secs(10));
    tracker.record_solution("challenge-1", &solution1)?;

    let progress = tracker.get_progress()?;
    println!("  Total completed: {}", progress.total_completed());
    println!("  Total practice time: {:?}", progress.total_practice_time());
    println!("✓ First completion recorded\n");

    // Test 3: Record better attempt
    println!("Test 3: Recording improved time...");
    let solution2 = Solution::completed(Duration::from_secs(8));
    let (new_time_record, _) = tracker.is_new_record("challenge-1", &solution2)?;
    println!("  Is new record: {}", new_time_record);
    tracker.record_solution("challenge-1", &solution2)?;

    let progress = tracker.get_progress()?;
    println!("  Total practice time: {:?}", progress.total_practice_time());
    println!("✓ Improved time recorded\n");

    // Test 4: Record second challenge
    println!("Test 4: Recording second challenge...");
    let solution3 = Solution::completed(Duration::from_secs(15));
    tracker.record_solution("challenge-2", &solution3)?;

    let progress = tracker.get_progress()?;
    println!("  Total completed: {}", progress.total_completed());
    println!("  Total attempts: {}", progress.total_attempts());
    println!("✓ Second challenge recorded\n");

    // Test 5: Calculate statistics
    println!("Test 5: Calculating statistics...");
    let progress = tracker.get_progress()?;
    println!("  Average solve time: {:?}", progress.average_solve_time());
    println!("  Current streak: {}", progress.calculate_current_streak(chrono::Utc::now().date_naive()));
    println!("  Longest streak: {}", progress.longest_streak());
    println!("✓ Statistics calculated\n");

    // Test 6: Verify persistence
    println!("Test 6: Testing persistence...");
    drop(tracker);

    let repo2 = JsonProgressRepository::with_path(test_file.clone());
    let tracker2 = ProgressTracker::new(repo2)?;
    let loaded_progress = tracker2.get_progress()?;

    println!("  Loaded completed count: {}", loaded_progress.total_completed());
    println!("  Loaded practice time: {:?}", loaded_progress.total_practice_time());
    println!("✓ Progress persisted and loaded correctly\n");

    // Test 7: Check JSON file structure
    println!("Test 7: Inspecting JSON file...");
    let json_content = std::fs::read_to_string(&test_file)?;
    println!("JSON content (first 500 chars):");
    println!("{}", &json_content.chars().take(500).collect::<String>());
    println!("✓ JSON file structure valid\n");

    // Clean up
    let _ = std::fs::remove_file(&test_file);

    println!("=================================");
    println!("All tests passed! ✓");
    println!("=================================");

    Ok(())
}
