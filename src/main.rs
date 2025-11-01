mod application;
mod domain;
mod infrastructure;
mod ui;

use anyhow::{Context, Result};

use application::ChallengeRunner;
use domain::Challenge;
use infrastructure::{FileChangeWatcher, HelixEditor, LocalFileSystem};
use ui::{ChallengeScreen, ResultsScreen};

fn main() -> Result<()> {
    // Check if Helix is installed
    if !HelixEditor::is_installed() {
        eprintln!("Error: Helix editor (hx) is not installed or not in PATH.");
        eprintln!("Please install Helix from https://helix-editor.com/");
        std::process::exit(1);
    }

    // Create the hardcoded challenge
    let challenge = Challenge::new(
        "Delete the word REMOVE",
        "Remove the word 'REMOVE' from the text.",
        "Hello REMOVE world",
        "Hello world",
        "Use 'w' to move by words, select with 'v', delete with 'd'",
    );

    // Show challenge screen
    let challenge_screen = ChallengeScreen::new();
    let should_continue = challenge_screen
        .show(&challenge)
        .context("Failed to display challenge screen")?;

    if !should_continue {
        println!("Challenge cancelled.");
        return Ok(());
    }

    // Dependency injection: create concrete implementations
    let editor = HelixEditor::new();
    let watcher = FileChangeWatcher::new();
    let filesystem = LocalFileSystem::new();

    // Create the challenge runner with injected dependencies
    let mut runner = ChallengeRunner::new(editor, watcher, filesystem);

    // Run the challenge
    let solution = runner.run(&challenge).context("Failed to run challenge")?;

    // Show results screen
    let results_screen = ResultsScreen::new();
    results_screen
        .show(&solution)
        .context("Failed to display results screen")?;

    Ok(())
}
