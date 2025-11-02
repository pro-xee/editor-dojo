mod application;
mod domain;
mod infrastructure;
mod ui;

use anyhow::{Context, Result};
use std::io::{self, Write};

use application::ChallengeRunner;
use infrastructure::{
    AsciinemaRecorder, ChallengeLoader, FileChangeWatcher, HelixEditor, LocalFileSystem,
    Recorder, TomlChallengeLoader,
};
use ui::{ChallengeListScreen, ChallengeScreen, ResultsScreen};

fn main() -> Result<()> {
    // Check if Helix is installed
    if !HelixEditor::is_installed() {
        eprintln!("Error: Helix editor (hx) is not installed or not in PATH.");
        eprintln!("Please install Helix from https://helix-editor.com/");
        std::process::exit(1);
    }

    // Check if asciinema is installed (optional but recommended)
    let use_recording = if !AsciinemaRecorder::is_available() {
        eprintln!("\n┌─────────────────────────────────────────────┐");
        eprintln!("│           Setup Recommended                 │");
        eprintln!("├─────────────────────────────────────────────┤");
        eprintln!("│                                             │");
        eprintln!("│  asciinema is not installed.                │");
        eprintln!("│                                             │");
        eprintln!("│  Without it, you won't see:                 │");
        eprintln!("│   - Keystroke counts                        │");
        eprintln!("│   - Key sequence feedback                   │");
        eprintln!("│   - Session recordings                      │");
        eprintln!("│                                             │");
        eprintln!("│  Install: https://asciinema.org/docs/       │");
        eprintln!("│                                             │");
        eprintln!("└─────────────────────────────────────────────┘");
        eprint!("\nContinue without recording? [y/N] ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        if input != "y" && input != "yes" {
            println!("Please install asciinema and try again.");
            std::process::exit(0);
        }
        false
    } else {
        true
    };

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

    // Add recorder if available
    if use_recording {
        let recorder = AsciinemaRecorder::new("hx");
        runner = runner.with_recorder(Box::new(recorder));
    }

    // Run the challenge
    let solution = runner.run(&challenge).context("Failed to run challenge")?;

    // Show results screen
    let results_screen = ResultsScreen::new();
    results_screen
        .show(&solution)
        .context("Failed to display results screen")?;

    Ok(())
}
