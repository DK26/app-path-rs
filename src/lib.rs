//! # app-path
//!
//! Create portable applications that keep files together with the executable.
//!
//! ## Quick Start
//!
//! ```rust
//! use app_path::{AppPath, app_path};
//!
//! // Files relative to your executable
//! let config = AppPath::new("config.toml");
//! let database = AppPath::new("data/users.db");
//! let logs = AppPath::new("logs/app.log");
//!
//! // Or use the convenient macro
//! let config_alt = app_path!("config.toml");
//! let database_alt = app_path!("data/users.db");
//! let logs_alt = app_path!("logs/app.log");
//!
//! // Environment variable overrides for deployment flexibility
//! let config_deploy = AppPath::with_override("config.toml", std::env::var("CONFIG_PATH").ok());
//! let database_deploy = app_path!("data/users.db", env = "DATABASE_PATH");
//! let logs_deploy = app_path!("logs/app.log", override = std::env::var("LOG_DIR").ok());
//!
//! // Works like standard paths
//! if config.exists() {
//!     let content = std::fs::read_to_string(&config)?;
//! }
//!
//! // Create directories with clear intent
//! logs.ensure_parent_dirs()?; // Creates logs/ directory for the file
//! database.ensure_parent_dirs()?; // Creates data/ directory for the file
//!
//! // Create directories themselves
//! let cache_dir = AppPath::new("cache");
//! let temp_dir = AppPath::new("temp");
//! cache_dir.ensure_dir_exists()?; // Creates cache/ directory
//! temp_dir.ensure_dir_exists()?; // Creates temp/ directory
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Key Features
//!
//! - **Portable**: Relative paths resolve to executable directory  
//! - **System integration**: Absolute paths work as-is
//! - **Zero dependencies**: Only standard library
//! - **High performance**: Static caching, minimal allocations
//! - **Thread-safe**: Concurrent access safe
//!
//! ## API Design
//!
//! - [`AppPath::new()`] - **Recommended**: Simple constructor (panics on failure)
//! - [`AppPath::try_new()`] - **Libraries**: Fallible version for error handling
//! - [`AppPath::with_override()`] - **Deployment**: Environment-configurable paths
//! - [`app_path!`] - **Macro**: Convenient syntax with optional environment overrides
//! - [`AppPath::ensure_parent_dirs()`] - **Files**: Creates parent directories for files
//! - [`AppPath::ensure_dir_exists()`] - **Directories**: Creates directories (and parents)
//! - [`exe_dir()`] - **Advanced**: Direct access to executable directory (panics on failure)
//! - [`try_exe_dir()`] - **Libraries**: Fallible executable directory access
//!
//! ## Function Variants
//!
//! This crate provides both panicking and fallible variants for most operations:
//!
//! | Panicking (Recommended) | Fallible (Libraries) | Use Case |
//! |------------------------|---------------------|----------|
//! | [`AppPath::new()`] | [`AppPath::try_new()`] | Most applications vs. libraries |
//! | [`exe_dir()`] | [`try_exe_dir()`] | Direct directory access |
//!
//! ### Panic Conditions
//!
//! [`AppPath::new()`] and [`exe_dir()`] panic only if executable location cannot be determined:
//! - `std::env::current_exe()` fails (extremely rare system failure)
//! - Executable path is empty (indicates system corruption)
//!
//! These represent unrecoverable system failures that occur at application startup.
//! After the first successful call, the executable directory is cached and subsequent
//! calls never panic.
//!
//! **For libraries or applications requiring graceful error handling**, use the fallible
//! variants [`AppPath::try_new()`] and [`try_exe_dir()`] instead.

mod app_path;
mod error;
mod functions;
mod traits;

#[cfg(test)]
mod tests;

// Re-export the public API
pub use app_path::AppPath;
pub use error::AppPathError;
pub use functions::{exe_dir, try_exe_dir};

/// Convenience macro for creating `AppPath` instances with optional environment variable overrides.
///
/// This macro provides a more ergonomic way to create `AppPath` instances, especially when
/// dealing with environment variable overrides.
///
/// # Syntax
///
/// - `app_path!(path)` - Simple path creation (equivalent to `AppPath::new(path)`)
/// - `app_path!(path, env = "VAR_NAME")` - With environment variable override
/// - `app_path!(path, override = expression)` - With any optional override expression
///
/// # Examples
///
/// ```rust
/// use app_path::{app_path, AppPath};
///
/// // Simple usage
/// let config = app_path!("config.toml");
/// assert_eq!(config.file_name().unwrap(), "config.toml");
///
/// // Environment variable override
/// let data_dir = app_path!("data", env = "DATA_DIR");
///
/// // Custom override expression
/// let log_file = app_path!("app.log", override = std::env::args().nth(1));
/// ```
#[macro_export]
macro_rules! app_path {
    ($path:expr) => {
        $crate::AppPath::new($path)
    };
    ($path:expr, env = $env_var:expr) => {
        $crate::AppPath::with_override($path, ::std::env::var($env_var).ok())
    };
    ($path:expr, override = $override_expr:expr) => {
        $crate::AppPath::with_override($path, $override_expr)
    };
}
