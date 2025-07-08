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
//! - [`AppPath::new_with_override()`] - **Deployment**: Environment-configurable paths
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
