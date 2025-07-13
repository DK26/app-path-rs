//! # app-path
//!
//! Create portable applications that keep files together with the executable.
//!
//! ## Quick Start
//!
//! ```rust
//! use app_path::{AppPath, app_path, try_app_path};
//!
//! // Simple macro usage - files relative to your executable
//! let config = app_path!("config.toml");        // → /path/to/exe/config.toml
//! let database = app_path!("data/users.db");    // → /path/to/exe/data/users.db
//! let logs = app_path!("logs/app.log");         // → /path/to/exe/logs/app.log
//!
//! // Environment variable overrides for deployment
//! let config_deploy = app_path!("config.toml", env = "CONFIG_PATH");
//! // → Uses CONFIG_PATH if set, otherwise /path/to/exe/config.toml
//!
//! let db_deploy = app_path!("data/users.db", override = std::env::var("DATABASE_PATH").ok());
//! // → Uses DATABASE_PATH if set, otherwise /path/to/exe/data/users.db
//!
//! // Advanced override patterns for XDG/system integration
//! let config_xdg = app_path!("config", fn = || {
//!     std::env::var("XDG_CONFIG_HOME")
//!         .or_else(|_| std::env::var("HOME").map(|h| format!("{h}/.config/myapp")))
//!         .ok()
//! });
//! // → /home/user/.config/myapp (Linux) or /path/to/exe/config (fallback)
//!
//! // Complex override logic with block expressions
//! let data_dir = app_path!("data", override = {
//!     std::env::var("DATA_DIR")
//!         .or_else(|_| std::env::var("XDG_DATA_HOME").map(|p| format!("{p}/myapp")))
//!         .ok()
//! });
//! // → Uses DATA_DIR, then XDG_DATA_HOME/myapp, finally /path/to/exe/data
//!
//! // Variable capturing in complex expressions
//! let version = "1.0";
//! let versioned_cache = app_path!(format!("cache-{version}"));
//! // → /path/to/exe/cache-1.0
//!
//! let temp_with_env = app_path!(format!("temp-{version}"), env = "TEMP_DIR");
//! // → Uses TEMP_DIR if set, otherwise /path/to/exe/temp-1.0
//!
//! // Fallible variants for libraries (return Result instead of panicking)
//! let config_safe = try_app_path!("config.toml")?;
//! // → Ok(/path/to/exe/config.toml) or Err(AppPathError)
//!
//! let db_safe = try_app_path!("data/users.db", env = "DATABASE_PATH")?;
//! // → Ok with DATABASE_PATH or default path, or Err(AppPathError)
//!
//! let cache_safe = try_app_path!(format!("cache-{version}"))?;
//! // → Ok(/path/to/exe/cache-1.0) or Err(AppPathError)
//!
//! // Constructor API (alternative to macros)
//! let traditional = AppPath::new("config.toml");
//! // → /path/to/exe/config.toml (panics on system failure)
//!
//! let with_override = AppPath::with_override("config.toml", std::env::var("CONFIG_PATH").ok());
//! // → Uses CONFIG_PATH if set, otherwise /path/to/exe/config.toml
//!
//! let fallible = AppPath::try_new("config.toml")?; // For libraries
//! // → Ok(/path/to/exe/config.toml) or Err(AppPathError)
//!
//! // Works like standard paths - auto-derefs to &Path
//! if config.exists() {
//!     let content = std::fs::read_to_string(&config)?;
//!     // → Reads file content if config.toml exists
//! }
//!
//! // Directory creation with clear intent
//! logs.create_parents()?;                 // Creates logs/ directory for the file
//! app_path!("cache").create_dir()?;       // Creates cache/ directory itself
//! // → Both create directories if they don't exist
//! # Ok::<(), Box<dyn std::error::Error>>(())
//!
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
//! - [`AppPath::with_override_fn()`] - **Advanced**: Function-based override logic
//! - [`app_path!`] - **Macro**: Convenient syntax with optional environment overrides
//! - [`try_app_path!`] - **Macro (Fallible)**: Returns `Result` for explicit error handling
//! - [`AppPath::create_parents()`] - **Files**: Creates parent directories for files
//! - [`AppPath::create_dir()`] - **Directories**: Creates directories (and parents)
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
//! # use app_path::{app_path, try_app_path};
//! let version = "1.0";
//! let cache = app_path!(format!("cache-{version}")); // Captures `version`
//! // → /path/to/exe/cache-1.0
//!
//! // Useful in closures and async blocks
//! async fn process_data(id: u32) {
//!     let output = app_path!(format!("output-{id}.json")); // Captures `id`
//!     // → /path/to/exe/output-123.json (where id = 123)
//!     // ... async processing
//! }
//! ```
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
/// - `app_path!(path, fn = function)` - With function-based override logic
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
///
/// // Function-based override
/// let config_dir = app_path!("config", fn = || {
///     std::env::var("XDG_CONFIG_HOME")
///         .or_else(|_| std::env::var("HOME").map(|h| format!("{h}/.config")))
///         .ok()
/// });
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
/// use app_path::{try_app_path, AppPathError};
///
/// fn setup_config() -> Result<(), AppPathError> {
///     let config = try_app_path!("config.toml")?;
///     let database = try_app_path!("data/users.db")?;
///     
///     // Use paths normally
///     if config.exists() {
///         println!("Config found at: {}", config.display());
///     }
///     
///     Ok(())
/// }
/// ```
///
/// ## Environment Variable Overrides
///
/// ```rust
/// use app_path::try_app_path;
///
/// fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
///     // Uses "logs/app.log" by default, LOG_PATH env var if set
///     let log_file = try_app_path!("logs/app.log", env = "LOG_PATH")?;
///     log_file.create_parents()?;
///     
///     std::fs::write(&log_file, "Application started")?;
///     Ok(())
/// }
/// ```
///
/// ## Custom Override Logic
///
/// ```rust
/// use app_path::try_app_path;
///
/// fn get_data_dir() -> Option<String> {
///     std::env::var("XDG_DATA_HOME")
///         .or_else(|_| std::env::var("HOME").map(|h| format!("{h}/.local/share")))
///         .ok()
/// }
///
/// fn setup_data() -> Result<(), Box<dyn std::error::Error>> {
///     let data_dir = try_app_path!("data", override = get_data_dir())?;
///     data_dir.create_dir()?;
///     Ok(())
/// }
/// ```
///
/// ## Function-Based Override
///
/// ```rust
/// use app_path::try_app_path;
///
/// fn setup_cache() -> Result<(), Box<dyn std::error::Error>> {
///     let cache_dir = try_app_path!("cache", fn = || {
///         std::env::var("XDG_CACHE_HOME")
///             .or_else(|_| std::env::var("HOME").map(|h| format!("{h}/.cache")))
///             .ok()
///     })?;
///     cache_dir.create_dir()?;
///     Ok(())
/// }
/// ```
///
/// ## Error Handling Patterns
///
/// ```rust
/// use app_path::{try_app_path, AppPathError};
///
/// match try_app_path!("config.toml") {
///     Ok(config) => {
///         println!("Config path: {}", config.display());
///     }
///     Err(AppPathError::ExecutableNotFound(msg)) => {
///         eprintln!("Cannot determine executable location: {msg}");
///     }
///     Err(AppPathError::InvalidExecutablePath(msg)) => {
///         eprintln!("Invalid executable path: {msg}");
///     }
///     Err(AppPathError::IoError(msg)) => {
///         eprintln!("I/O operation failed: {msg}");
///     }
/// }
/// ```
///
/// ## Library Usage
///
/// ```rust
/// use app_path::{try_app_path, AppPathError};
///
/// /// Library function that gracefully handles path errors
/// pub fn load_user_config() -> Result<String, Box<dyn std::error::Error>> {
///     let config_path = try_app_path!("config.toml", env = "USER_CONFIG")?;
///         
///     if !config_path.exists() {
///         return Err("Config file not found".into());
///     }
///     
///     let content = std::fs::read_to_string(&config_path)?;
///     Ok(content)
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
