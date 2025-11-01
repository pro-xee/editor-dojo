mod application;
mod domain;
mod infrastructure;
mod ui;

use anyhow::{Context, Result};

use application::ChallengeRunner;
use infrastructure::{ChallengeLoader, FileChangeWatcher, HelixEditor, LocalFileSystem, TomlChallengeLoader};
use ui::{ChallengeListScreen, ChallengeScreen, ResultsScreen};

fn main() -> Result<()> {
    // Check if Helix is installed
    if !HelixEditor::is_installed() {
        eprintln!("Error: Helix editor (hx) is not installed or not in PATH.");
        eprintln!("Please install Helix from https://helix-editor.com/");
        std::process::exit(1);
    }

    // Load challenges from TOML files
    let loader = TomlChallengeLoader::new("challenges/helix");
    let challenges = loader.load_all().context("Failed to load challenges")?;

    // Show challenge list screen
    let list_screen = ChallengeListScreen::new(challenges);
    let selected_challenge = list_screen
        .show()
        .context("Failed to display challenge list screen")?;

    let challenge = match selected_challenge {
        Some(c) => c,
        None => {
            println!("No challenge selected.");
            return Ok(());
        }
    };

    // Show challenge brief screen
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
