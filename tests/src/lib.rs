use std::path::{Path, PathBuf};
use std::process::Command;

pub mod validation;
pub mod generation;

/// Result type for test operations
pub type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Get the template root directory
pub fn template_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf()
}

/// Run a command and return success status
pub fn run_command(cmd: &mut Command) -> TestResult<bool> {
    let status = cmd.status()?;
    Ok(status.success())
}

/// Check if a file exists
pub fn file_exists(path: &Path) -> bool {
    path.exists() && path.is_file()
}

/// Check if a directory exists
pub fn dir_exists(path: &Path) -> bool {
    path.exists() && path.is_dir()
}
