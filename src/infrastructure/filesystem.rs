use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use tempfile::Builder;

use crate::application::FileSystem;

/// Concrete implementation of FileSystem using the standard library and tempfile
pub struct LocalFileSystem;

impl LocalFileSystem {
    pub fn new() -> Self {
        Self
    }
}

impl Default for LocalFileSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl FileSystem for LocalFileSystem {
    fn create_temp_file(&self, content: &str) -> Result<PathBuf> {
        // Create a persistent temp file that won't be auto-deleted
        let temp_file = Builder::new()
            .prefix("editor-dojo-")
            .suffix(".txt")
            .tempfile()
            .context("Failed to create temporary file")?;

        use std::io::Write;
        let mut file = temp_file.as_file();
        file.write_all(content.as_bytes())
            .context("Failed to write content to temporary file")?;
        file.sync_all().context("Failed to sync temporary file")?;

        // Persist the temp file so it won't be deleted when dropped
        let (_, path) = temp_file
            .keep()
            .context("Failed to persist temporary file")?;

        Ok(path)
    }

    fn read_file(&self, path: &PathBuf) -> Result<String> {
        fs::read_to_string(path).with_context(|| format!("Failed to read file: {}", path.display()))
    }

    fn cleanup(&self, path: &PathBuf) -> Result<()> {
        // If the file still exists, remove it
        if path.exists() {
            fs::remove_file(path)
                .with_context(|| format!("Failed to remove temporary file: {}", path.display()))?;
        }
        Ok(())
    }
}
