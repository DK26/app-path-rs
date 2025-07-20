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
/// ### Constructors
///
/// - [`Self::new()`] - **Application base directory**: Returns the directory containing the executable
/// - [`Self::with()`] - **Primary API**: Create paths relative to application base directory
/// - [`Self::try_new()`] - **Libraries**: Fallible version for getting application base directory
/// - [`Self::try_with()`] - **Libraries**: Fallible version for creating relative paths
/// - [`Self::with_override()`] - **Deployment**: Environment-configurable paths
/// - [`Self::try_with_override()`] - **Deployment (Fallible)**: Fallible environment-configurable paths
/// - [`Self::with_override_fn()`] - **Advanced**: Function-based override logic
/// - [`Self::try_with_override_fn()`] - **Advanced (Fallible)**: Fallible function-based override logic
///
/// ### Directory Creation
///
/// - [`Self::create_parents()`] - **Files**: Creates parent directories for files
/// - [`Self::create_dir()`] - **Directories**: Creates directories (and parents)
///
/// ### Path Operations & Traits
///
/// - **All `Path` methods**: Available directly via `Deref<Target=Path>` (e.g., `exists()`, `is_file()`, `file_name()`, `extension()`)
/// - [`Self::into_path_buf()`] - **Conversion**: Extract owned `PathBuf` from wrapper
/// - [`Self::into_inner()`] - **Conversion**: Alias for `into_path_buf()` following Rust patterns
/// - [`Self::to_bytes()`] - **Ecosystem**: Raw bytes for specialized libraries
/// - [`Self::into_bytes()`] - **Ecosystem**: Owned bytes for specialized libraries
///
/// # Panics
///
/// Constructor methods panic if the executable location cannot be determined (an
/// extremely rare condition). After the first successful call, these methods
/// never panic because the result is cached.
///
/// # Examples
///
/// ```rust
/// use app_path::AppPath;
///
/// // Get the executable directory itself
/// let exe_dir = AppPath::new();
/// let exe_dir = AppPath::default(); // Same thing
///
/// // Create paths relative to executable
/// let config = AppPath::with("config.toml");
/// let data = AppPath::with("data/users.db");
///
/// // Chainable with join (since AppPath implements all Path methods)
/// let log_file = AppPath::new().join("logs").join("app.log");
/// let nested = AppPath::with("data").join("cache").join("temp.txt");
///
/// // Works like standard paths - all Path methods available
/// if config.exists() {
///     let content = std::fs::read_to_string(&config); // &config works directly
/// }
/// data.create_parents(); // Creates data/ directory for the file
///
/// // Mixed portable and system paths
/// let portable = AppPath::with("app.conf");           // → exe_dir/app.conf
/// let system = AppPath::with("/var/log/app.log");     // → /var/log/app.log
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
mod directory;
mod path_ops;
mod traits;
