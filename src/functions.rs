use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use crate::error::{try_exe_dir_init, AppPathError};

// Global executable directory - computed once, cached forever
static EXE_DIR: OnceLock<PathBuf> = OnceLock::new();

/// Get the executable's directory.
///
/// **Recommended for most applications.** Returns the directory containing the current executable.
/// Use this for building custom paths or integrating with other APIs.
///
/// ## Performance
///
/// - **First call**: Determines and caches the executable directory  
/// - **Subsequent calls**: Returns cached result immediately (no system calls)
/// - **Thread-safe**: Safe to call from multiple threads
///
/// # Panics
///
/// Panics if executable location cannot be determined (extremely rare):
/// - `std::env::current_exe()` fails
/// - Executable path is empty (system corruption)
///
/// After first successful call, never panics (uses cached result).
///
/// # Examples
///
/// ```rust
/// use app_path::exe_dir;
/// use std::fs;
///
/// // Basic usage
/// let exe_directory = exe_dir();
/// println!("Executable directory: {}", exe_directory.display());
///
/// // Build custom paths
/// let config = exe_dir().join("config.toml");
/// let data_dir = exe_dir().join("data");
///
/// // Use with standard library
/// if config.exists() {
///     let content = fs::read_to_string(&config)?;
/// }
/// fs::create_dir_all(&data_dir)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn exe_dir() -> &'static Path {
    match try_exe_dir() {
        Ok(path) => path,
        Err(e) => panic!("Failed to determine executable directory: {e}"),
    }
}

/// Get the executable's directory (fallible).
///
/// **Use this only for libraries or specialized applications.** Most applications should
/// use [`exe_dir()`] for simpler, cleaner code.
///
/// # Examples
///
/// ```rust
/// use app_path::try_exe_dir;
///
/// // Library with graceful error handling
/// match try_exe_dir() {
///     Ok(exe_dir) => {
///         println!("Executable directory: {}", exe_dir.display());
///         let config = exe_dir.join("config.toml");
///     }
///     Err(e) => {
///         eprintln!("Failed to get executable directory: {e}");
///         // Implement fallback strategy
///     }
/// }
///
/// // Use with ? operator
/// fn get_config_dir() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
///     let exe_dir = try_exe_dir()?;
///     Ok(exe_dir.join("config"))
/// }
/// ```
///
/// Once the executable directory is successfully determined by either this function or [`exe_dir()`],
/// the result is cached globally and all subsequent calls to both functions will use the cached value.
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
/// use app_path::try_exe_dir;
///
/// // Handle the error explicitly
/// match try_exe_dir() {
///     Ok(exe_dir) => {
///         println!("Executable directory: {}", exe_dir.display());
///         // Use exe_dir for further operations
///     }
///     Err(e) => {
///         eprintln!("Failed to get executable directory: {e}");
///         // Implement fallback strategy
///     }
/// }
/// ```
///
/// ## Use with ? Operator
///
/// ```rust
/// use app_path::try_exe_dir;
///
/// // Use with the ? operator in functions that return Result
/// fn get_config_dir() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
///     let exe_dir = try_exe_dir()?;
///     Ok(exe_dir.join("config"))
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
