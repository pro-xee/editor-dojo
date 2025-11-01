use std::cell::RefCell;
use std::path::Path;
use std::process::{Child, Command};

use anyhow::{Context, Result};

use crate::application::EditorSpawner;

/// Helix editor spawner implementation
pub struct HelixEditor {
    process: RefCell<Option<Child>>,
}

impl HelixEditor {
    pub fn new() -> Self {
        Self {
            process: RefCell::new(None),
        }
    }

    /// Checks if Helix is installed on the system
    pub fn is_installed() -> bool {
        Command::new("hx").arg("--version").output().is_ok()
    }
}

impl Default for HelixEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl EditorSpawner for HelixEditor {
    fn spawn(&mut self, file_path: &Path) -> Result<()> {
        let child = Command::new("hx")
            .arg(file_path)
            .spawn()
            .context("Failed to spawn Helix editor. Is 'hx' installed?")?;

        *self.process.borrow_mut() = Some(child);
        Ok(())
    }

    fn terminate(&mut self) -> Result<()> {
        if let Some(mut process) = self.process.borrow_mut().take() {
            // Try to kill gracefully
            process
                .kill()
                .context("Failed to terminate editor process")?;
            process
                .wait()
                .context("Failed to wait for editor process")?;
        }
        Ok(())
    }

    fn is_running(&self) -> bool {
        let mut process_guard = self.process.borrow_mut();
        if let Some(ref mut process) = *process_guard {
            // Try to check if process has exited without blocking
            match process.try_wait() {
                Ok(Some(_)) => {
                    // Process has exited
                    false
                }
                Ok(None) => {
                    // Process is still running
                    true
                }
                Err(_) => {
                    // Error checking, assume not running
                    false
                }
            }
        } else {
            false
        }
    }
}

impl Drop for HelixEditor {
    fn drop(&mut self) {
        let _ = self.terminate();
    }
}
