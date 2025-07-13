use std::env::current_exe;
use std::path::PathBuf;

/// Error type for AppPath operations.
///
/// This enum represents the possible failures that can occur when determining
/// the executable location. These errors are rare in practice and typically
/// indicate fundamental system issues.
///
/// # When These Errors Occur
///
/// - **`ExecutableNotFound`**: When [`std::env::current_exe()`] fails
///   - Very rare, but can happen in some embedded or heavily sandboxed environments
///   - May occur if the executable has been deleted while running
///   - Can happen in some containerized environments with unusual configurations
///
/// - **`InvalidExecutablePath`**: When the executable path is empty
///   - Extremely rare, indicates a corrupted or broken system
///   - May occur with custom or non-standard program loaders
///
/// These errors represent system-level failures that are typically unrecoverable
/// for portable applications. Most applications should use the infallible API
/// (`new()`, `exe_dir()`) and handle these rare cases through environment
/// variables or fallback strategies.
///
/// # Examples
///
/// ```rust
/// use app_path::{AppPath, AppPathError};
///
/// // Handle errors explicitly
/// match AppPath::try_new("config.toml") {
///     Ok(config) => {
///         println!("Config path: {}", config.path().display());
///     }
///     Err(AppPathError::ExecutableNotFound(msg)) => {
///         eprintln!("Cannot find executable: {msg}");
///         // Fallback to alternative configuration
///     }
///     Err(AppPathError::InvalidExecutablePath(msg)) => {
///         eprintln!("Invalid executable path: {msg}");
///         // Fallback to alternative configuration
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppPathError {
    /// Failed to determine the current executable path.
    ///
    /// This error occurs when [`std::env::current_exe()`] fails, which is rare
    /// but can happen in some embedded or heavily sandboxed environments.
    ExecutableNotFound(String),

    /// Executable path is empty or invalid.
    ///
    /// This error occurs when the system returns an empty executable path,
    /// which is extremely rare and indicates a corrupted or broken system.
    InvalidExecutablePath(String),
}

impl std::fmt::Display for AppPathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppPathError::ExecutableNotFound(msg) => {
                write!(f, "Failed to determine executable location: {msg}")
            }
            AppPathError::InvalidExecutablePath(msg) => {
                write!(f, "Invalid executable path: {msg}")
            }
        }
    }
}

impl std::error::Error for AppPathError {}

/// Try to determine the executable directory (fallible version).
///
/// This is the internal fallible initialization function that both the fallible
/// and infallible APIs use. It handles all the edge cases properly without
/// exposing them as errors to API users.
pub(crate) fn try_exe_dir_init() -> Result<PathBuf, AppPathError> {
    let exe = current_exe().map_err(|e| {
        AppPathError::ExecutableNotFound(format!("std::env::current_exe() failed: {e}"))
    })?;

    if exe.as_os_str().is_empty() {
        return Err(AppPathError::InvalidExecutablePath(
            "Executable path is empty - unsupported environment".to_string(),
        ));
    }

    // Handle edge case: executable at filesystem root (jailed environments, etc.)
    // This is NOT an error - it's a valid case that should be handled internally
    let dir = match exe.parent() {
        Some(parent) => parent.to_path_buf(),
        None => {
            // If exe has no parent (e.g., running as "/init" or "C:\myapp.exe"),
            // use the root directory itself
            exe.ancestors().last().unwrap_or(&exe).to_path_buf()
        }
    };

    Ok(dir)
}
