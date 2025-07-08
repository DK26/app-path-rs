//! # app-path
//!
//! Create portable applications that keep files together with the executable.
//!
//! ## Quick Start
//!
//! ```rust
//! use app_path::AppPath;
//!
//! // Files relative to your executable
//! let config = AppPath::new("config.toml");
//! let database = AppPath::new("data/users.db");
//! let logs = AppPath::new("logs/app.log");
//!
//! // Works like standard paths
//! if config.exists() {
//!     let content = std::fs::read_to_string(&config)?;
//! }
//!
//! logs.create_dir_all()?; // Creates logs/ directory
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
//! - [`AppPath::new()`] - **Recommended**: Simple, infallible constructor
//! - [`AppPath::try_new()`] - **Libraries**: Fallible version for error handling
//! - [`AppPath::with_override()`] - **Deployment**: Environment-configurable paths
//! - [`exe_dir()`] - **Advanced**: Direct access to executable directory
//!
//! ## Panic Conditions
//!
//! [`AppPath::new()`] and [`exe_dir()`] panic only if executable location cannot be determined:
//! - `std::env::current_exe()` fails (extremely rare)
//! - Executable path is empty (system corruption)
//!
//! These represent unrecoverable system failures. For fallible behavior, use [`AppPath::try_new()`].

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
        $crate::AppPath::with_override($path, std::env::var($env_var).ok())
    };
    ($path:expr, override = $override_expr:expr) => {
        $crate::AppPath::with_override($path, $override_expr)
    };
}
