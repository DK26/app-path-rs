//! AppPath implementation split into logical modules for better maintainability.

use std::path::PathBuf;

/// Creates paths relative to the executable location for portable applications.
///
/// **AppPath** enables building truly portable applications where configuration, data,
/// and executable stay together as a deployable unit. Perfect for USB drives, network
/// shares, or any directory without installation.
///
/// ## Key Features
///
/// - **Portable**: Relative paths resolve to executable directory
/// - **System integration**: Absolute paths work as-is  
/// - **Zero-cost**: Implements `Deref<Target=Path>` and all path traits
/// - **Thread-safe**: Static caching with proper synchronization
/// - **Memory efficient**: Only stores the final resolved path
///
/// ## API Overview
///
/// - [`Self::new()`] - **Primary API**: Simple, infallible construction
/// - [`Self::try_new()`] - **Libraries**: Fallible version for error handling  
/// - [`Self::with_override()`] - **Deployment**: Environment-configurable paths
/// - [`Self::path()`] - **Access**: Get the resolved `&Path` (deprecated - use `&app_path` or `as_ref()`)
/// - **All `Path` methods**: Available directly via `Deref<Target=Path>` (e.g., `exists()`, `is_file()`, `file_name()`, `extension()`)
///
/// # Panics
///
/// Methods panic if executable location cannot be determined (extremely rare).
/// After first successful call, methods never panic (uses cached result).
///
/// # Examples
///
/// ```rust
/// use app_path::AppPath;
///
/// // Basic usage - most common pattern
/// let config = AppPath::new("config.toml");
/// let data = AppPath::new("data/users.db");
///
/// // Works like standard paths - all Path methods available
/// if config.exists() {
///     let content = std::fs::read_to_string(&config); // &config works directly
/// }
/// data.create_parents(); // Creates data/ directory for the file
///
/// // Mixed portable and system paths
/// let portable = AppPath::new("app.conf");           // → exe_dir/app.conf
/// let system = AppPath::new("/var/log/app.log");     // → /var/log/app.log
///
/// // Override for deployment flexibility
/// let config = AppPath::with_override(
///     "config.toml",
///     std::env::var("CONFIG_PATH").ok()
/// );
/// ```
#[derive(Clone, Debug)]
pub struct AppPath {
    full_path: PathBuf,
}

mod constructors;
mod deprecated;
mod directory;
mod path_ops;
mod traits;
