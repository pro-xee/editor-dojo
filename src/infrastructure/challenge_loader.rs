use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::domain::Challenge;

/// Trait for loading challenges from various sources
pub trait ChallengeLoader {
    /// Loads all available challenges
    fn load_all(&self) -> Result<Vec<Challenge>>;
}

/// TOML file structure for challenge definitions
#[derive(Debug, Deserialize)]
struct TomlChallenge {
    metadata: Metadata,
    hints: Hints,
    content: Content,
}

#[derive(Debug, Deserialize)]
struct Metadata {
    id: String,
    title: String,
    description: String,
    #[serde(default)]
    difficulty: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Hints {
    #[serde(default)]
    generic: Option<String>,
    #[serde(default)]
    helix: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Content {
    starting: String,
    target: String,
}

impl TomlChallenge {
    fn into_domain(self) -> Challenge {
        // Prefer helix-specific hint, fall back to generic
        let hint = self
            .hints
            .helix
            .or(self.hints.generic)
            .unwrap_or_else(|| "No hint available".to_string());

        let mut challenge = Challenge::new(
            self.metadata.id,
            self.metadata.title,
            self.metadata.description,
            self.content.starting,
            self.content.target,
            hint,
        );

        if let Some(difficulty) = self.metadata.difficulty {
            challenge = challenge.with_difficulty(difficulty);
        }

        if !self.metadata.tags.is_empty() {
            challenge = challenge.with_tags(self.metadata.tags);
        }

        challenge
    }
}

/// Loads challenges from TOML files in a directory
pub struct TomlChallengeLoader {
    challenges_dir: PathBuf,
}

impl TomlChallengeLoader {
    pub fn new(challenges_dir: impl Into<PathBuf>) -> Self {
        Self {
            challenges_dir: challenges_dir.into(),
        }
    }

    /// Loads a single TOML file and parses it into a Challenge
    fn load_toml_file(&self, path: &Path) -> Result<Challenge> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read TOML file: {}", path.display()))?;

        let toml_challenge: TomlChallenge = toml::from_str(&content).with_context(|| {
            format!(
                "Failed to parse TOML file: {}. Check that all required fields are present.",
                path.display()
            )
        })?;

        Ok(toml_challenge.into_domain())
    }
}

impl ChallengeLoader for TomlChallengeLoader {
    fn load_all(&self) -> Result<Vec<Challenge>> {
        // Check if challenges directory exists
        if !self.challenges_dir.exists() {
            anyhow::bail!(
                "Challenges directory not found: {}\n\nPlease create the directory and add TOML challenge files.",
                self.challenges_dir.display()
            );
        }

        // Read all .toml files from the directory
        let entries = fs::read_dir(&self.challenges_dir).with_context(|| {
            format!(
                "Failed to read challenges directory: {}",
                self.challenges_dir.display()
            )
        })?;

        let mut challenges = Vec::new();
        let mut toml_files: Vec<PathBuf> = Vec::new();

        // Collect all .toml files
        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("toml") {
                toml_files.push(path);
            }
        }

        // Sort files by name for consistent ordering
        toml_files.sort();

        // Load each TOML file
        for path in toml_files {
            match self.load_toml_file(&path) {
                Ok(challenge) => challenges.push(challenge),
                Err(e) => {
                    // Fail fast on malformed TOML
                    return Err(e);
                }
            }
        }

        if challenges.is_empty() {
            anyhow::bail!(
                "No challenges found in directory: {}\n\nPlease add .toml challenge files to this directory.",
                self.challenges_dir.display()
            );
        }

        Ok(challenges)
    }
}
