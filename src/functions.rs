use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use crate::error::{try_exe_dir_init, AppPathError};

// Global executable directory - computed once, cached forever
static EXE_DIR: OnceLock<PathBuf> = OnceLock::new();

/// Get the executable's directory (fallible).
///
/// **Use this only for libraries or specialized applications.** Most applications should
/// use [`crate::AppPath::try_new()`] for simpler, cleaner code.
///
/// # Examples
///
/// ```rust
/// use app_path::AppPath;
///
/// // Library with graceful error handling
/// match AppPath::try_new() {
///     Ok(app_base) => {
///         println!("Application base directory: {}", app_base.display());
///         let config = AppPath::with("config.toml");
///     }
///     Err(e) => {
///         eprintln!("Failed to get application base directory: {e}");
///         // Implement fallback strategy
///     }
/// }
///
/// // Use with ? operator for paths
/// fn get_config_dir() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
///     let config = AppPath::try_with("config")?;
///     Ok(config.into())
/// }
/// ```
///
/// Once the executable directory is successfully determined by this function,
/// the result is cached globally and all subsequent calls will use the cached value.
/// This means that after the first successful call, `try_exe_dir()` will never return an error.
///
/// # Returns
///
/// * `Ok(&'static Path)` - The directory containing the current executable
/// * `Err(AppPathError)` - Failed to determine executable location
///
/// # Errors
///
/// Returns [`AppPathError`] if the executable location cannot be determined:
/// - [`AppPathError::ExecutableNotFound`] - `std::env::current_exe()` fails (extremely rare)
/// - [`AppPathError::InvalidExecutablePath`] - Executable path is empty (system corruption)
///
/// These errors represent unrecoverable system failures that occur at application startup.
/// After the first successful call, the executable directory is cached and this function
/// will never return an error.
///
/// # Performance
///
/// This function is highly optimized:
/// - **First call**: Determines and caches the executable directory
/// - **Subsequent calls**: Returns the cached result immediately (no system calls)
/// - **Thread-safe**: Safe to call from multiple threads concurrently
///
/// # Examples
///
/// ## Library Error Handling
///
/// ```rust
/// use app_path::AppPath;
///
/// // Handle the error explicitly
/// match AppPath::try_new() {
///     Ok(app_base) => {
///         println!("Application base directory: {}", app_base.display());
///         // Use app_base for further operations
///     }
///     Err(e) => {
///         eprintln!("Failed to get application base directory: {e}");
///         // Implement fallback strategy
///     }
/// }
/// ```
///
/// ## Use with ? Operator
///
/// ```rust
/// use app_path::AppPath;
///
/// // Use with the ? operator in functions that return Result
/// fn get_config_dir() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
///     let config = AppPath::try_with("config")?;
///     Ok(config.into())
/// }
/// ```
pub fn try_exe_dir() -> Result<&'static Path, AppPathError> {
    // If already cached, return it immediately
    if let Some(cached_path) = EXE_DIR.get() {
        return Ok(cached_path.as_path());
    }

    // Try to initialize and cache the result
    let path = try_exe_dir_init()?;
    let cached_path = EXE_DIR.get_or_init(|| path);
    Ok(cached_path.as_path())
}
