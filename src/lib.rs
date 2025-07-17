//! # app-path
//!
//! Create portable applications that keep files together with the executable.
//!
//! ## Quick Start
//!
//! ```rust
//! use app_path::app_path;
//!
//! // Files relative to your executable - not current directory!
//! let config = app_path!("config.toml");        // → /path/to/exe/config.toml
//! let database = app_path!("data/users.db");    // → /path/to/exe/data/users.db
//!
//! // Environment overrides for deployment
//! let logs = app_path!("logs/app.log", env = "LOG_PATH");
//! // → Uses LOG_PATH if set, otherwise /path/to/exe/logs/app.log
//!
//! // Works like standard paths - all Path methods available
//! if config.exists() {
//!     let content = std::fs::read_to_string(&config)?;
//! }
//!
//! // Directory creation
//! logs.create_parents()?;                 // Creates logs/ directory for the file
//! app_path!("cache").create_dir()?;       // Creates cache/ directory itself
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
//! - **Zero-cost**: All `Path` methods available via `Deref` (e.g., `exists()`, `is_file()`, `extension()`)
//!
//! ## API Design
//!
//! - [`AppPath::new()`] - **Recommended**: Simple constructor (panics on failure)
//! - [`AppPath::try_new()`] - **Libraries**: Fallible version for error handling
//! - [`AppPath::with_override()`] - **Deployment**: Environment-configurable paths
//! - [`AppPath::with_override_fn()`] - **Advanced**: Function-based override logic
//! - [`app_path!`] - **Macro**: Convenient syntax with optional environment overrides
//! - [`try_app_path!`] - **Macro (Fallible)**: Returns `Result` for explicit error handling
//! - [`AppPath::create_parents()`] - **Files**: Creates parent directories for files
//! - [`AppPath::create_dir()`] - **Directories**: Creates directories (and parents)
//! - [`AppPath::to_bytes()`] - **Ecosystem**: Raw bytes for specialized libraries
//! - [`AppPath::into_bytes()`] - **Ecosystem**: Owned bytes for specialized libraries
//! - [`exe_dir()`] - **Advanced**: Direct access to executable directory (panics on failure)
//! - [`try_exe_dir()`] - **Libraries**: Fallible executable directory access
//!
//! ## Function Variants
//!
//! This crate provides both panicking and fallible variants for most operations:
//!
//! | Panicking (Recommended) | Fallible (Libraries) | Use Case |
//! |------------------------|---------------------|----------|
//! | [`AppPath::new()`] | [`AppPath::try_new()`] | Constructor methods |
//! | [`app_path!`] | [`try_app_path!`] | Convenient macros |
//! | [`exe_dir()`] | [`try_exe_dir()`] | Direct directory access |
//!
//! ### Macro Syntax Variants
//!
//! Both `app_path!` and `try_app_path!` macros support four syntax forms for maximum flexibility:
//!
//! ```rust
//! # use app_path::{app_path, try_app_path};
//! // 1. Direct value
//! let config = app_path!("config.toml");
//! // → /path/to/exe/config.toml
//!
//! // 2. With environment override
//! let config = app_path!("config.toml", env = "CONFIG_PATH");
//! // → Uses CONFIG_PATH if set, otherwise /path/to/exe/config.toml
//!
//! // 3. With optional override value
//! let config = app_path!("config.toml", override = std::env::var("CONFIG_PATH").ok());
//! // → Uses CONFIG_PATH if available, otherwise /path/to/exe/config.toml
//!
//! // 4. With function-based override
//! let config = app_path!("config.toml", fn = || {
//!     std::env::var("CONFIG_PATH").ok()
//! });
//! // → Uses function result if Some, otherwise /path/to/exe/config.toml
//! ```
//!
//! ### Variable Capturing
//!
//! Both macros support variable capturing in complex expressions:
//!
//! ```rust
//! # use app_path::app_path;
//! let version = "1.0";
//! let cache = app_path!(format!("cache-{version}"));
//!
//! let user_ids = vec![123, 456];
//! let logs: Vec<_> = user_ids.iter()
//!     .map(|id| app_path!(format!("logs/user-{id}.log")))
//!     .collect();
//! ```
//!
//! ## Ecosystem Integration
//!
//! AppPath works seamlessly with ecosystem crates through `Deref<Target=Path>`:
//!
//! ### Serde Integration
//!
//! ```rust
//! use app_path::app_path;
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize)]
//! struct Config {
//!     db_path: String,
//! }
//!
//! let config = Config {
//!     db_path: app_path!("data/app.db").display().to_string(),
//! };
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ### UTF-8 Path Serialization (camino)
//!
//! ```rust
//! use app_path::app_path;
//! use camino::Utf8PathBuf;
//!
//! let static_dir = app_path!("web/static");
//! let utf8_static = Utf8PathBuf::from_path_buf(static_dir.into())
//!     .map_err(|_| "Invalid UTF-8 path")?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ### Cross-Platform Path Types (typed-path)
//!
//! ```rust
//! use app_path::app_path;
//! use typed_path::{WindowsPath, UnixPath};
//!
//! let dist_dir = app_path!("dist");
//! let win_path = WindowsPath::new(&dist_dir.to_bytes());
//! let unix_path = UnixPath::new(&dist_dir.to_bytes());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Panic Conditions
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

#[cfg(test)]
mod tests;

// Re-export the public API
pub use app_path::AppPath;
pub use error::AppPathError;
pub use functions::{exe_dir, try_exe_dir};

/// Convenience macro for creating `AppPath` instances with optional environment variable overrides.
///
/// # Syntax
///
/// - `app_path!(path)` - Simple path creation
/// - `app_path!(path, env = "VAR_NAME")` - With environment variable override
/// - `app_path!(path, override = expression)` - With optional override expression
/// - `app_path!(path, fn = function)` - With function-based override logic
///
/// # Examples
///
/// ```rust
/// use app_path::app_path;
///
/// let config = app_path!("config.toml");
/// let data_dir = app_path!("data", env = "DATA_DIR");
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
    ($path:expr, fn = $override_fn:expr) => {
        $crate::AppPath::with_override_fn($path, $override_fn)
    };
}

/// Fallible version of [`app_path!`] that returns a [`Result`] instead of panicking.
///
/// This macro provides the same convenient syntax as [`app_path!`] but returns
/// [`Result<AppPath, AppPathError>`] for explicit error handling. Perfect for
/// libraries and applications that need graceful error handling.
///
/// # Syntax
///
/// - `try_app_path!(path)` - Simple path creation (equivalent to `AppPath::try_new(path)`)
/// - `try_app_path!(path, env = "VAR_NAME")` - With environment variable override
/// - `try_app_path!(path, override = expression)` - With any optional override expression
/// - `try_app_path!(path, fn = function)` - With function-based override logic
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust
/// use app_path::try_app_path;
///
/// let config = try_app_path!("config.toml")?;
/// let database = try_app_path!("data/users.db")?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ## Environment Variable Overrides
///
/// ```rust
/// use app_path::try_app_path;
///
/// let log_file = try_app_path!("logs/app.log", env = "LOG_PATH")?;
/// log_file.create_parents()?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ## Custom Override Logic
///
/// ```rust
/// use app_path::try_app_path;
///
/// let custom_path = std::env::var("DATA_HOME").ok();
/// let data_dir = try_app_path!("data", override = custom_path)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ## Function-Based Override
///
/// ```rust
/// use app_path::try_app_path;
///
/// let cache_dir = try_app_path!("cache", fn = || std::env::var("CACHE_DIR").ok())?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ## Error Handling
///
/// ```rust
/// use app_path::{try_app_path, AppPathError};
///
/// match try_app_path!("config.toml") {
///     Ok(config) => println!("Config: {}", config.display()),
///     Err(e) => eprintln!("Error: {e}"),
/// }
/// ```
///
/// ## Library Usage
///
/// ```rust
/// use app_path::try_app_path;
///
/// pub fn load_config() -> Result<String, Box<dyn std::error::Error>> {
///     let config_path = try_app_path!("config.toml")?;
///     std::fs::read_to_string(&config_path).map_err(Into::into)
/// }
/// ```
///
/// # Comparison with [`app_path!`]
///
/// | Feature | [`app_path!`] | [`try_app_path!`] |
/// |---------|---------------|-------------------|
/// | **Return type** | [`AppPath`] | [`Result<AppPath, AppPathError>`] |
/// | **Error handling** | Panics on failure | Returns [`Err`] on failure |
/// | **Use case** | Applications | Libraries, explicit error handling |
/// | **Syntax** | Same | Same |
/// | **Performance** | Same | Same |
///
/// # When to Use
///
/// - **Use [`try_app_path!`]** for libraries, when you need graceful error handling,
///   or when integrating with other fallible operations
/// - **Use [`app_path!`]** for applications where you want to fail fast on system errors
///
/// # See Also
///
/// - [`app_path!`] - Panicking version with identical syntax
/// - [`AppPath::try_new`] - Constructor equivalent
/// - [`AppPath::try_with_override`] - Constructor with override equivalent
#[macro_export]
macro_rules! try_app_path {
    ($path:expr) => {
        $crate::AppPath::try_new($path)
    };
    ($path:expr, env = $env_var:expr) => {
        $crate::AppPath::try_with_override($path, ::std::env::var($env_var).ok())
    };
    ($path:expr, override = $override_expr:expr) => {
        $crate::AppPath::try_with_override($path, $override_expr)
    };
    ($path:expr, fn = $override_fn:expr) => {
        $crate::AppPath::try_with_override_fn($path, $override_fn)
    };
}
