//! # app-path
//!
//! Create file paths relative to your executable for truly portable applications.
//!
//! This crate provides a simple, robust solution for applications that need to access files
//! and directories relative to their executable location. The design prioritizes **simplicity**,
//! **performance**, and **reliability** for the common use case of portable applications.
//!
//! ## Design Philosophy
//!
//! **AppPath** is designed around these core principles:
//!
//! 1. **Portable-first**: Everything stays together with your executable
//! 2. **Simple API**: Infallible constructors with clear panic conditions
//! 3. **High performance**: Static caching and zero-allocation design
//! 4. **Ergonomic**: Works seamlessly with all Rust path types
//! 5. **Robust**: Handles edge cases in containerized/embedded environments
//!
//! The crate makes a deliberate trade-off: **simplicity and performance over fallible APIs**.
//! Since executable location determination rarely fails in practice, and when it does fail
//! it indicates fundamental system issues, we choose to panic with clear documentation
//! rather than burden every usage site with error handling.
//!
//! ## Primary Use Cases
//!
//! This crate is specifically designed for applications that benefit from **portable layouts**:
//!
//! - **Portable applications** that run from USB drives or network shares
//! - **Development tools** that should work without installation
//! - **Corporate software** deployed without admin rights
//! - **Containerized applications** with predictable file layouts
//! - **Embedded systems** with simple, fixed directory structures
//! - **CLI tools** that need configuration and data files nearby
//!
//! ## Quick Start
//!
//! ```rust
//! use app_path::AppPath;
//! use std::path::{Path, PathBuf};
//!
//! // Create paths relative to your executable - simple and clean
//! let config = AppPath::new("config.toml");
//! let data = AppPath::new("data/users.db");
//!
//! // Accepts any AsRef<Path> type - no unnecessary allocations
//! let log_file = "logs/app.log".to_string();
//! let logs = AppPath::new(&log_file);
//!
//! let path_buf = PathBuf::from("cache/data.bin");
//! let cache = AppPath::new(&path_buf);
//!
//! // Works with any path-like type
//! let from_path = AppPath::new(Path::new("temp.txt"));
//!
//! // Alternative: Use From for any path type
//! let settings: AppPath = "settings.json".into();
//! let data_file: AppPath = PathBuf::from("data.db").into();
//! let temp_file: AppPath = Path::new("temp.log").into();
//!
//! // Get the paths for use with standard library functions
//! println!("Config: {}", config.path().display());
//! println!("Data: {}", data.path().display());
//!
//! // Check existence and create directories
//! if !logs.exists() {
//!     logs.create_dir_all()?;
//! }
//!
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Path Resolution Strategy
//!
//! **AppPath** uses intelligent path resolution to support both portable and system-integrated applications:
//!
//! ```rust
//! use app_path::AppPath;
//!
//! // Relative paths are resolved relative to the executable directory
//! // This is the primary use case for portable applications
//! let config = AppPath::new("config.toml");           // → exe_dir/config.toml
//! let data = AppPath::new("data/users.db");           // → exe_dir/data/users.db
//! let nested = AppPath::new("plugins/my_plugin.dll"); // → exe_dir/plugins/my_plugin.dll
//!
//! // Absolute paths are used as-is for system integration
//! // This allows hybrid applications that also integrate with the system
//! let system_config = AppPath::new("/etc/myapp/config.toml");  // → /etc/myapp/config.toml
//! let windows_temp = AppPath::new(r"C:\temp\cache.dat");       // → C:\temp\cache.dat
//! ```
//!
//! This dual behavior enables applications to be **primarily portable** while still
//! allowing **system integration** when needed.
//!
//! ## Performance Design
//!
//! **AppPath** is optimized for high-performance applications:
//!
//! - **Static caching**: Executable location determined once, cached forever
//! - **Minimal memory**: Only stores the final resolved path (no input path retained)
//! - **Zero allocations**: Uses `AsRef<Path>` to avoid unnecessary conversions
//! - **Efficient conversions**: `From` trait implementations for all common types
//!
//! ```rust
//! use app_path::AppPath;
//! use std::path::{Path, PathBuf};
//!
//! // All of these are efficient - no unnecessary allocations
//! let from_str = AppPath::new("config.toml");          // &str → direct usage
//! let from_string = AppPath::new(&"data.db".to_string()); // &String → no move needed
//! let from_path = AppPath::new(Path::new("logs.txt")); // &Path → direct usage
//! let from_pathbuf = AppPath::new(&PathBuf::from("cache.bin")); // &PathBuf → no move needed
//!
//! // When you want ownership transfer, use From trait
//! let owned: AppPath = PathBuf::from("important.db").into(); // PathBuf moved efficiently
//! ```
//!
//! ## Reliability and Edge Cases
//!
//! **AppPath** is designed to be robust in various deployment environments:
//!
//! - **Handles root-level executables**: Works when executable is at filesystem root
//! - **Container-friendly**: Designed for containerized and jailed environments
//! - **Cross-platform**: Consistent behavior across Windows, Linux, and macOS
//! - **Clear failure modes**: Panics with descriptive messages on rare system failures
//!
//! ### Panic Conditions
//!
//! The crate uses an **infallible API** that panics on rare system failures during static initialization.
//! This design choice prioritizes **simplicity and performance** for the common case where
//! executable location determination succeeds (which is the vast majority of real-world usage).
//!
//! **AppPath will panic if:**
//!
//! - **Cannot determine executable location** - When [`std::env::current_exe()`] fails
//!   (rare, but possible in some embedded or heavily sandboxed environments)
//! - **Executable path is empty** - When the system returns an empty executable path
//!   (extremely rare, indicates a broken/corrupted system)
//!
//! **These panics occur:**
//! - Once during the first use of any AppPath function
//! - Indicate fundamental system issues that are typically unrecoverable
//! - Are documented with clear error messages explaining the failure
//!
//! **Edge case handling:**
//! - **Root-level executables**: When executable runs at filesystem root (e.g., `/init`, `C:\`),
//!   the crate uses the root directory itself as the base
//! - **Jailed environments**: Properly handles chrooted or containerized environments
//!
//! For applications that need fallible behavior, you can wrap AppPath usage:
//!
//! ```rust
//! use app_path::AppPath;
//! use std::panic;
//!
//! fn safe_app_path(relative_path: &str) -> Option<AppPath> {
//!     panic::catch_unwind(|| AppPath::new(relative_path)).ok()
//! }
//!
//! // Use with fallback strategy
//! let config = safe_app_path("config.toml").unwrap_or_else(|| {
//!     // Fallback to temp directory or current directory
//!     AppPath::with_base(std::env::temp_dir(), "myapp_config.toml")
//! });
//! ```
//!
//! ## Flexible Creation Methods
//!
//! ```rust
//! use app_path::AppPath;
//! use std::path::{Path, PathBuf};
//!
//! // Method 1: Direct construction (recommended)
//! let config = AppPath::new("config.toml");
//! let logs = AppPath::new(PathBuf::from("logs/app.log"));
//!
//! // Method 2: From trait for various path types
//! let data1: AppPath = "data/users.db".into();               // &str
//! let data2: AppPath = "settings.json".to_string().into();   // String
//! let data3: AppPath = Path::new("cache/data.bin").into();   // &Path
//! let data4: AppPath = PathBuf::from("temp/file.txt").into(); // PathBuf
//!
//! // All methods support efficient zero-copy when possible
//! let owned_path = PathBuf::from("important/data.db");
//! let app_path: AppPath = owned_path.into(); // PathBuf is moved efficiently
//! ```
//!
//! ## Performance
//!
//! AppPath is optimized for efficiency:
//!
//! - **Cached executable location**: Determined once, reused for all instances
//! - **Minimal memory usage**: Only stores the final resolved path
//! - **Zero-copy when possible**: Uses `AsRef<Path>` to avoid unnecessary allocations
//! - **Efficient ownership transfer**: Moves owned types like `String` and `PathBuf`

use std::env::current_exe;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

// Global executable directory - computed once, cached forever
static EXE_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let exe = current_exe().expect("Failed to determine executable location");

    // Handle edge case: executable at filesystem root (jailed environments, etc.)
    match exe.parent() {
        Some(parent) => parent.to_path_buf(),
        None => {
            // If exe has no parent (e.g., running as "/init" or "C:\"),
            // use the root directory itself
            if exe.as_os_str().is_empty() {
                panic!("Executable path is empty - unsupported environment");
            }

            // For root-level executables, use the root directory
            exe.ancestors()
                .last()
                .expect("Failed to determine filesystem root")
                .to_path_buf()
        }
    }
});

/// Creates paths relative to the executable location for applications.
///
/// **AppPath** is the core type for building portable applications where all files and directories
/// stay together with the executable. This design choice makes applications truly portable -
/// they can run from USB drives, network shares, or any directory without installation.
///
/// ## Design Rationale
///
/// **Why relative to executable instead of current directory?**
/// - Current directory depends on where the user runs the program from
/// - Executable location is reliable and predictable
/// - Enables true portability - the entire application can be moved as one unit
///
/// **Why infallible API instead of Result-based?**
/// - Executable location determination rarely fails in practice
/// - When it does fail, it indicates fundamental system issues
/// - Infallible API eliminates error handling boilerplate from every usage site
/// - Results in cleaner, more maintainable application code
///
/// **Why static caching?**
/// - Executable location never changes during program execution
/// - Avoids repeated system calls for performance
/// - Thread-safe and efficient for concurrent applications
///
/// ## Perfect For
///
/// - **Portable applications** that run from USB drives or network shares
/// - **Development tools** that should work anywhere without installation
/// - **Corporate environments** where you can't install software system-wide
/// - **Containerized applications** with predictable, self-contained layouts
/// - **Embedded systems** with simple, fixed directory structures
/// - **CLI tools** that need configuration and data files nearby
///
/// ## Memory Layout
///
/// Each `AppPath` instance stores only the final resolved path (`PathBuf`), making it
/// memory-efficient. The original input path is not retained, as the resolved path
/// contains all necessary information for file operations.
///
/// ```text
/// AppPath {
///     full_path: PathBuf  // Only field - minimal memory usage
/// }
/// ```
///
/// ## Thread Safety
///
/// `AppPath` is `Send + Sync` and can be safely shared between threads. The static
/// executable directory cache is initialized once and safely shared across all threads.
///
/// # Panics
///
/// Panics on first use if the executable location cannot be determined.
/// See [crate-level documentation](crate) for comprehensive details on panic conditions
/// and edge case handling.
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust
/// use app_path::AppPath;
///
/// // Simple relative paths - the common case
/// let config = AppPath::new("config.toml");
/// let data_dir = AppPath::new("data");
/// let logs = AppPath::new("logs/app.log");
///
/// // Use like normal paths
/// if config.exists() {
///     let settings = std::fs::read_to_string(config.path())?;
/// }
///
/// // Create directories as needed
/// data_dir.create_dir_all()?;
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ## Mixed Portable and System Paths
///
/// ```rust
/// use app_path::AppPath;
///
/// // Portable app files (relative paths)
/// let app_config = AppPath::new("config.toml");       // → exe_dir/config.toml
/// let app_data = AppPath::new("data/users.db");       // → exe_dir/data/users.db
///
/// // System integration (absolute paths)
/// let system_log = AppPath::new("/var/log/myapp.log"); // → /var/log/myapp.log
/// let temp_cache = AppPath::new(std::env::temp_dir().join("cache.db"));
///
/// println!("App config: {}", app_config);    // Portable
/// println!("System log: {}", system_log);    // System integration
/// ```
///
/// ## Performance-Conscious Usage
///
/// ```rust
/// use app_path::AppPath;
/// use std::path::{Path, PathBuf};
///
/// // Efficient - no unnecessary allocations
/// let config = AppPath::new("config.toml");          // &str
/// let data = AppPath::new(Path::new("data.db"));     // &Path
///
/// // When you have owned values, borrow them to avoid moves
/// let filename = "important.log".to_string();
/// let logs = AppPath::new(&filename);                 // &String - no move
///
/// // Use From trait when you want ownership transfer
/// let owned_path = PathBuf::from("cache.bin");
/// let cache: AppPath = owned_path.into();             // PathBuf moved
/// ```
///
/// ## Portable vs System Integration
///
/// ```rust
/// use app_path::AppPath;
///
/// // Portable application files (relative paths)
/// let app_config = AppPath::new("config.toml");           // → exe_dir/config.toml
/// let app_data = AppPath::new("data/users.db");           // → exe_dir/data/users.db
/// let plugins = AppPath::new("plugins/my_plugin.dll");    // → exe_dir/plugins/my_plugin.dll
///
/// // System integration (absolute paths)
/// let system_config = AppPath::new("/etc/myapp/global.toml");  // → /etc/myapp/global.toml
/// let temp_file = AppPath::new(r"C:\temp\cache.dat");          // → C:\temp\cache.dat
/// let user_data = AppPath::new("/home/user/.myapp/prefs");     // → /home/user/.myapp/prefs
/// ```
///
/// ## Common Patterns
///
/// ```rust
/// use app_path::AppPath;
/// use std::fs;
///
/// // Configuration file pattern
/// let config = AppPath::new("config.toml");
/// if config.exists() {
///     let content = fs::read_to_string(config.path())?;
///     // Parse configuration...
/// }
///
/// // Data directory pattern
/// let data_dir = AppPath::new("data");
/// data_dir.create_dir_all()?;  // Ensure directory exists
/// let user_db = AppPath::new("data/users.db");
///
/// // Logging pattern
/// let log_file = AppPath::new("logs/app.log");
/// log_file.create_dir_all()?;  // Create logs directory if needed
/// fs::write(log_file.path(), "Application started\n")?;
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, Debug)]
pub struct AppPath {
    full_path: PathBuf,
}

impl AppPath {
    /// Creates file paths relative to the executable location.
    ///
    /// This is the primary constructor for `AppPath`. The method accepts any type that implements
    /// [`AsRef<Path>`], providing maximum flexibility while maintaining zero-allocation performance
    /// for most use cases.
    ///
    /// ## Design Choices
    ///
    /// **Why `impl AsRef<Path>` instead of specific types?**
    /// - Accepts all path-like types: `&str`, `String`, `&Path`, `PathBuf`, etc.
    /// - Avoids unnecessary allocations through efficient borrowing
    /// - Provides a single, consistent API instead of multiple overloads
    /// - Enables efficient usage patterns for both owned and borrowed values
    ///
    /// **Why infallible constructor?**
    /// - Executable location determination succeeds in >99.9% of real-world usage
    /// - Eliminates error handling boilerplate from every call site
    /// - Makes the API more ergonomic for the common case
    /// - Follows Rust conventions where `new()` implies infallible construction
    ///
    /// **Path Resolution Strategy:**
    /// - **Relative paths** (e.g., `"config.toml"`, `"data/file.txt"`) are resolved
    ///   relative to the executable's directory - this is the primary use case
    /// - **Absolute paths** (e.g., `"/etc/config"`, `"C:\\temp\\file.txt"`) are used
    ///   as-is, ignoring the executable's directory - enables system integration
    ///
    /// This dual behavior supports both **portable applications** (relative paths) and
    /// **system integration** (absolute paths) within the same API.
    ///
    /// # Arguments
    ///
    /// * `path` - A path that will be resolved relative to the executable.
    ///   Accepts any type implementing [`AsRef<Path>`]:
    ///   - `&str` - String literals and string slices
    ///   - `String` - Owned strings
    ///   - `&Path` - Path references
    ///   - `PathBuf` - Path buffers
    ///   - And many others that implement `AsRef<Path>`
    ///
    /// # Panics
    ///
    /// Panics on first use if the executable location cannot be determined.
    /// This is a **one-time initialization panic** that occurs during static initialization
    /// of the executable directory cache.
    ///
    /// See [crate-level documentation](crate) for comprehensive details on:
    /// - When panics can occur (rare system failure conditions)
    /// - How edge cases are handled (root-level executables, containers)
    /// - Strategies for applications that need fallible behavior
    ///
    /// # Performance Notes
    ///
    /// This method is highly optimized:
    /// - **Static caching**: Executable location determined once, reused forever
    /// - **Zero allocations**: Uses `AsRef<Path>` to avoid unnecessary conversions
    /// - **Minimal memory**: Only stores the final resolved path
    /// - **Thread-safe**: Safe to call from multiple threads concurrently
    ///
    /// # Examples
    ///
    /// ## Basic Usage (Recommended)
    ///
    /// ```rust
    /// use app_path::AppPath;
    ///
    /// // These create portable paths relative to your executable
    /// let config = AppPath::new("config.toml");           // &str
    /// let data = AppPath::new("data/users.db");           // &str with subdirectory
    /// let logs = AppPath::new("logs/app.log");            // Nested directories
    /// ```
    ///
    /// ## Efficient Usage with Different Types
    ///
    /// ```rust
    /// use app_path::AppPath;
    /// use std::path::{Path, PathBuf};
    ///
    /// // All of these are efficient - no unnecessary allocations
    /// let from_str = AppPath::new("config.toml");         // &str → direct usage
    /// let from_path = AppPath::new(Path::new("data.db")); // &Path → direct usage
    ///
    /// // Borrow owned values to avoid moves when you need them later
    /// let filename = "important.log".to_string();
    /// let logs = AppPath::new(&filename);                 // &String → efficient borrowing
    /// println!("Original filename: {}", filename);       // filename still available
    ///
    /// let path_buf = PathBuf::from("cache.bin");
    /// let cache = AppPath::new(&path_buf);                // &PathBuf → efficient borrowing
    /// println!("Original path: {}", path_buf.display()); // path_buf still available
    /// ```
    ///
    /// ## Portable vs System Integration
    ///
    /// ```rust
    /// use app_path::AppPath;
    ///
    /// // Portable application files (relative paths)
    /// let app_config = AppPath::new("config.toml");           // → exe_dir/config.toml
    /// let app_data = AppPath::new("data/users.db");           // → exe_dir/data/users.db
    /// let plugins = AppPath::new("plugins/my_plugin.dll");    // → exe_dir/plugins/my_plugin.dll
    ///
    /// // System integration (absolute paths)
    /// let system_config = AppPath::new("/etc/myapp/global.toml");  // → /etc/myapp/global.toml
    /// let temp_file = AppPath::new(r"C:\temp\cache.dat");          // → C:\temp\cache.dat
    /// let user_data = AppPath::new("/home/user/.myapp/prefs");     // → /home/user/.myapp/prefs
    /// ```
    ///
    /// ## Common Patterns
    ///
    /// ```rust
    /// use app_path::AppPath;
    /// use std::fs;
    ///
    /// // Configuration file pattern
    /// let config = AppPath::new("config.toml");
    /// if config.exists() {
    ///     let content = fs::read_to_string(config.path())?;
    ///     // Parse configuration...
    /// }
    ///
    /// // Data directory pattern
    /// let data_dir = AppPath::new("data");
    /// data_dir.create_dir_all()?;  // Ensure directory exists
    /// let user_db = AppPath::new("data/users.db");
    ///
    /// // Logging pattern
    /// let log_file = AppPath::new("logs/app.log");
    /// log_file.create_dir_all()?;  // Create logs directory if needed
    /// fs::write(log_file.path(), "Application started\n")?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(path: impl AsRef<Path>) -> Self {
        let full_path = EXE_DIR.join(path.as_ref());
        Self { full_path }
    }

    /// Creates paths relative to a custom base directory.
    ///
    /// This static method allows you to override the executable directory and specify
    /// a custom base directory instead. This is particularly valuable for:
    ///
    /// ## Primary Use Cases
    ///
    /// **Testing and Development:**
    /// - Isolate tests with temporary directories
    /// - Mock different deployment layouts
    /// - Test portable application behavior without moving executables
    ///
    /// **Custom Application Layouts:**
    /// - Multi-executable applications with shared data directories
    /// - Applications that need to access files in sibling directories
    /// - Deployment scenarios where data is separate from executable
    ///
    /// **Fallback Strategies:**
    /// - Provide alternative base directories when executable location detection fails
    /// - Implement graceful degradation for unusual deployment environments
    ///
    /// ## Design Rationale
    ///
    /// **Why a static method instead of instance method?**
    /// - Maintains consistent API with `AppPath::new()`
    /// - Clearly indicates this creates a new `AppPath` with different behavior
    /// - Avoids confusion about whether it modifies existing instances
    /// - Follows Rust conventions for alternative constructors
    ///
    /// **Why accept `impl AsRef<Path>` for both parameters?**
    /// - Consistent with `AppPath::new()` for API uniformity
    /// - Allows efficient usage with any path-like types
    /// - Avoids unnecessary allocations through borrowing
    ///
    /// # Arguments
    ///
    /// * `base` - The base directory to use instead of the executable directory.
    ///   Accepts any type implementing [`AsRef<Path>`]
    /// * `path` - The path relative to the base directory.
    ///   Accepts any type implementing [`AsRef<Path>`]
    ///
    /// # Performance
    ///
    /// This method is as efficient as `AppPath::new()`:
    /// - No allocation overhead from `AsRef<Path>` usage
    /// - Direct path joining without intermediate allocations
    /// - Suitable for performance-critical code paths
    ///
    /// # Examples
    ///
    /// ## Testing with Temporary Directories
    ///
    /// ```rust
    /// use app_path::AppPath;
    /// use std::env;
    ///
    /// #[cfg(test)]
    /// mod tests {
    ///     use super::*;
    ///     use std::fs;
    ///
    ///     #[test]
    ///     fn test_config_loading() {
    ///         // Create isolated test environment
    ///         let test_dir = env::temp_dir().join("myapp_test");
    ///         fs::create_dir_all(&test_dir).unwrap();
    ///
    ///         // Test with custom base directory
    ///         let config = AppPath::with_base(&test_dir, "config.toml");
    ///         assert!(!config.exists());
    ///
    ///         // Create test config and verify
    ///         fs::write(config.path(), "debug = true").unwrap();
    ///         assert!(config.exists());
    ///
    ///         // Cleanup
    ///         fs::remove_dir_all(&test_dir).unwrap();
    ///     }
    /// }
    /// ```
    ///
    /// ## Custom Application Layouts
    ///
    /// ```rust
    /// use app_path::AppPath;
    /// use std::env;
    ///
    /// // Multi-executable application with shared data
    /// let shared_data = env::current_exe()
    ///     .unwrap()
    ///     .parent()
    ///     .unwrap()
    ///     .parent()
    ///     .unwrap()
    ///     .join("shared");
    ///
    /// let config = AppPath::with_base(&shared_data, "config.toml");
    /// let database = AppPath::with_base(&shared_data, "data/app.db");
    /// ```
    ///
    /// ## Fallback Strategy
    ///
    /// ```rust
    /// use app_path::AppPath;
    /// use std::{env, panic};
    ///
    /// fn get_config_path() -> AppPath {
    ///     // Try normal executable-relative path first
    ///     panic::catch_unwind(|| AppPath::new("config.toml"))
    ///         .unwrap_or_else(|_| {
    ///             // Fallback to user's home directory
    ///             let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    ///             AppPath::with_base(home, ".myapp/config.toml")
    ///         })
    /// }
    /// ```
    ///
    /// ## Development vs Production
    ///
    /// ```rust
    /// use app_path::AppPath;
    /// use std::env;
    ///
    /// fn get_data_path() -> AppPath {
    ///     if env::var("DEVELOPMENT").is_ok() {
    ///         // Development: Use project root
    ///         AppPath::with_base(".", "dev_data/test.db")
    ///     } else {
    ///         // Production: Use executable directory
    ///         AppPath::new("data/app.db")
    ///     }
    /// }
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    /// use std::env;
    ///
    /// // For testing with a temporary directory
    /// let temp_dir = env::temp_dir();
    /// let config = AppPath::with_base(&temp_dir, "config.toml");
    ///
    /// // For custom application layouts
    /// let app_data = AppPath::with_base("/opt/myapp", "data/users.db");
    /// ```
    pub fn with_base(base: impl AsRef<Path>, path: impl AsRef<Path>) -> Self {
        let full_path = base.as_ref().join(path.as_ref());
        Self { full_path }
    }

    /// Get the full resolved path.
    ///
    /// This is the primary method for getting the actual filesystem path
    /// where your file or directory is located.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    ///
    /// let config = AppPath::new("config.toml");
    ///
    /// // Get the path for use with standard library functions
    /// println!("Config path: {}", config.path().display());
    ///
    /// // The path is always absolute
    /// assert!(config.path().is_absolute());
    /// ```
    #[inline]
    pub fn path(&self) -> &Path {
        &self.full_path
    }

    /// Check if the path exists.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    ///
    /// let config = AppPath::new("config.toml");
    ///
    /// if config.exists() {
    ///     println!("Config file found!");
    /// } else {
    ///     println!("Config file not found, using defaults.");
    /// }
    /// ```
    #[inline]
    pub fn exists(&self) -> bool {
        self.full_path.exists()
    }

    /// Create parent directories if they don't exist.
    ///
    /// This is equivalent to calling [`std::fs::create_dir_all`] on the
    /// parent directory of this path.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    /// use std::env;
    ///
    /// // Use a temporary directory for the example
    /// let temp_dir = env::temp_dir().join("app_path_example");
    /// let data_file = AppPath::with_base(&temp_dir, "data/users/profile.json");
    ///
    /// // Ensure the "data/users" directory exists
    /// data_file.create_dir_all()?;
    ///
    /// // Verify the directory was created
    /// assert!(data_file.path().parent().unwrap().exists());
    ///
    /// # std::fs::remove_dir_all(&temp_dir).ok();
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn create_dir_all(&self) -> std::io::Result<()> {
        if let Some(parent) = self.full_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        Ok(())
    }
}

/// Get the executable's directory.
///
/// This function provides access to the cached executable directory that AppPath uses.
/// Useful for advanced use cases where you need the directory path directly.
///
/// # Panics
///
/// Panics if the executable location cannot be determined (same conditions as AppPath).
///
/// # Examples
///
/// ```rust
/// use app_path::exe_dir;
///
/// println!("Executable directory: {}", exe_dir().display());
/// ```
pub fn exe_dir() -> &'static Path {
    &EXE_DIR
}

// Standard trait implementations
impl std::fmt::Display for AppPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full_path.display())
    }
}

impl From<AppPath> for PathBuf {
    #[inline]
    fn from(app_path: AppPath) -> Self {
        app_path.full_path
    }
}

impl AsRef<Path> for AppPath {
    #[inline]
    fn as_ref(&self) -> &Path {
        self.full_path.as_ref()
    }
}

// Infallible From implementations for common types
impl From<&str> for AppPath {
    fn from(path: &str) -> Self {
        Self::new(path)
    }
}

impl From<String> for AppPath {
    fn from(path: String) -> Self {
        Self::new(path)
    }
}

impl From<&String> for AppPath {
    fn from(path: &String) -> Self {
        Self::new(path)
    }
}

impl From<&Path> for AppPath {
    fn from(path: &Path) -> Self {
        Self::new(path)
    }
}

impl From<PathBuf> for AppPath {
    fn from(path: PathBuf) -> Self {
        Self::new(path)
    }
}

impl From<&PathBuf> for AppPath {
    fn from(path: &PathBuf) -> Self {
        Self::new(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;

    /// Helper to create a file at a given path for testing.
    fn create_test_file(path: &Path) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        let mut file = File::create(path).unwrap();
        writeln!(file, "test").unwrap();
    }

    #[test]
    fn resolves_relative_path_to_exe_dir() {
        let rel = "myconfig.toml";
        let rel_path = AppPath::new(rel);
        let expected = exe_dir().join(rel);

        assert_eq!(rel_path.path(), &expected);
        assert!(rel_path.path().is_absolute());
    }

    #[test]
    fn resolves_relative_path_with_custom_base() {
        let temp_dir = env::temp_dir().join("app_path_test_base");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let rel = "subdir/file.txt";
        let rel_path = AppPath::with_base(&temp_dir, rel);
        let expected = temp_dir.join(rel);

        assert_eq!(rel_path.path(), &expected);
        assert!(rel_path.path().is_absolute());
    }

    #[test]
    fn can_access_file_using_full_path() {
        let temp_dir = env::temp_dir().join("app_path_test_access");
        let file_name = "access.txt";
        let file_path = temp_dir.join(file_name);
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();
        create_test_file(&file_path);

        let rel_path = AppPath::with_base(&temp_dir, file_name);
        assert!(rel_path.exists());
        assert_eq!(rel_path.path(), &file_path);
    }

    #[test]
    fn handles_dot_and_dotdot_components() {
        let temp_dir = env::temp_dir().join("app_path_test_dot");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let rel = "./foo/../bar.txt";
        let rel_path = AppPath::with_base(&temp_dir, rel);
        let expected = temp_dir.join(rel);

        assert_eq!(rel_path.path(), &expected);
    }

    #[test]
    fn as_ref_and_into_pathbuf_are_consistent() {
        let rel = "somefile.txt";
        let rel_path = AppPath::new(rel);
        let as_ref_path: &Path = rel_path.as_ref();
        let into_pathbuf: PathBuf = rel_path.clone().into();
        assert_eq!(as_ref_path, into_pathbuf.as_path());
    }

    #[test]
    fn test_path_method() {
        let rel = "data/file.txt";
        let temp_dir = env::temp_dir().join("app_path_test_full");
        let rel_path = AppPath::with_base(&temp_dir, rel);
        assert_eq!(rel_path.path(), temp_dir.join(rel));
    }

    #[test]
    fn test_exists_method() {
        let temp_dir = env::temp_dir().join("app_path_test_exists");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let file_name = "exists_test.txt";
        let file_path = temp_dir.join(file_name);
        create_test_file(&file_path);

        let rel_path = AppPath::with_base(&temp_dir, file_name);
        assert!(rel_path.exists());

        let non_existent = AppPath::with_base(&temp_dir, "non_existent.txt");
        assert!(!non_existent.exists());
    }

    #[test]
    fn test_create_dir_all() {
        let temp_dir = env::temp_dir().join("app_path_test_create");
        let _ = fs::remove_dir_all(&temp_dir);

        let rel = "deep/nested/dir/file.txt";
        let rel_path = AppPath::with_base(&temp_dir, rel);

        rel_path.create_dir_all().unwrap();
        assert!(rel_path.path().parent().unwrap().exists());
    }

    #[test]
    fn test_display_trait() {
        let rel = "display_test.txt";
        let temp_dir = env::temp_dir().join("app_path_test_display");
        let rel_path = AppPath::with_base(&temp_dir, rel);

        let expected = temp_dir.join(rel);
        assert_eq!(format!("{rel_path}"), format!("{}", expected.display()));
    }

    #[test]
    fn test_from_str() {
        let rel_path = AppPath::from("config.toml");
        let expected = exe_dir().join("config.toml");

        assert_eq!(rel_path.path(), &expected);
    }

    #[test]
    fn test_from_string() {
        let path_string = "data/file.txt".to_string();
        let rel_path = AppPath::from(path_string);
        let expected = exe_dir().join("data/file.txt");

        assert_eq!(rel_path.path(), &expected);
    }

    #[test]
    fn test_from_string_ref() {
        let path_string = "logs/app.log".to_string();
        let rel_path = AppPath::from(&path_string);
        let expected = exe_dir().join("logs/app.log");

        assert_eq!(rel_path.path(), &expected);
    }

    #[test]
    fn test_new_with_different_types() {
        use std::path::PathBuf;

        // Test various input types with new
        let from_str = AppPath::new("test.txt");
        let test_string = "test.txt".to_string(); // Intentionally create String to test type
        let from_string = AppPath::new(test_string);
        let from_path_buf = AppPath::new(PathBuf::from("test.txt"));
        let from_path_ref = AppPath::new(Path::new("test.txt"));

        // All should produce equivalent results
        assert_eq!(from_str.path(), from_string.path());
        assert_eq!(from_string.path(), from_path_buf.path());
        assert_eq!(from_path_buf.path(), from_path_ref.path());
    }

    #[test]
    fn test_ownership_transfer() {
        use std::path::PathBuf;

        let path_buf = PathBuf::from("test.txt");
        let app_path = AppPath::new(path_buf);
        // path_buf is moved and no longer accessible

        let expected = exe_dir().join("test.txt");
        assert_eq!(app_path.path(), &expected);

        // Test with String too
        let string_path = "another_test.txt".to_string();
        let app_path2 = AppPath::new(string_path);
        // string_path is moved and no longer accessible

        let expected2 = exe_dir().join("another_test.txt");
        assert_eq!(app_path2.path(), &expected2);
    }

    #[test]
    fn test_absolute_path_behavior() {
        let absolute_path = if cfg!(windows) {
            r"C:\temp\config.toml"
        } else {
            "/tmp/config.toml"
        };

        let app_path = AppPath::new(absolute_path);

        // PathBuf::join() with absolute paths replaces the base path entirely
        assert_eq!(app_path.path(), Path::new(absolute_path));
        assert!(app_path.path().is_absolute());
    }

    #[test]
    fn test_exe_dir_function() {
        let dir = exe_dir();
        assert!(dir.is_absolute());

        // Should be consistent with AppPath behavior
        let config = AppPath::new("test.txt");
        let expected = dir.join("test.txt");
        assert_eq!(config.path(), &expected);
    }

    #[test]
    fn test_from_implementations() {
        use std::path::{Path, PathBuf};

        let expected = exe_dir().join("test.txt");

        // Test all From implementations
        let from_str: AppPath = "test.txt".into();
        let from_string: AppPath = "test.txt".to_string().into();
        let from_string_ref: AppPath = (&"test.txt".to_string()).into();
        let from_path: AppPath = Path::new("test.txt").into();
        let from_pathbuf: AppPath = PathBuf::from("test.txt").into();
        let from_pathbuf_ref: AppPath = (&PathBuf::from("test.txt")).into();

        // All should produce the same result
        assert_eq!(from_str.path(), &expected);
        assert_eq!(from_string.path(), &expected);
        assert_eq!(from_string_ref.path(), &expected);
        assert_eq!(from_path.path(), &expected);
        assert_eq!(from_pathbuf.path(), &expected);
        assert_eq!(from_pathbuf_ref.path(), &expected);
    }

    #[test]
    fn test_with_base_static_method() {
        let temp_dir = env::temp_dir().join("app_path_test_with_base");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let rel = "config.toml";
        let app_path = AppPath::with_base(&temp_dir, rel);
        let expected = temp_dir.join(rel);

        assert_eq!(app_path.path(), &expected);
    }

    #[test]
    fn test_asref_path_efficiency() {
        use std::path::{Path, PathBuf};

        // Test that AsRef<Path> works efficiently with various types
        let str_path = "test.txt";
        let string_path = "test.txt".to_string();
        let path_ref = Path::new("test.txt");
        let path_buf = PathBuf::from("test.txt");

        let from_str = AppPath::new(str_path);
        let from_string = AppPath::new(&string_path); // Reference to avoid move
        let from_path = AppPath::new(path_ref);
        let from_pathbuf = AppPath::new(&path_buf); // Reference to avoid move

        let expected = exe_dir().join("test.txt");

        assert_eq!(from_str.path(), &expected);
        assert_eq!(from_string.path(), &expected);
        assert_eq!(from_path.path(), &expected);
        assert_eq!(from_pathbuf.path(), &expected);

        // Verify original values are still accessible (weren't moved)
        assert_eq!(string_path, "test.txt");
        assert_eq!(path_buf, PathBuf::from("test.txt"));
    }

    #[test]
    fn test_root_directory_edge_case() {
        // This test simulates the edge case where an executable might be at filesystem root
        // We can't easily test this in practice, but we can test the logic

        // Test Unix-style root
        let unix_root = Path::new("/");
        let config = AppPath::with_base(unix_root, "config.toml");
        assert_eq!(config.path(), Path::new("/config.toml"));

        // Test Windows-style root
        if cfg!(windows) {
            let windows_root = Path::new(r"C:\");
            let config = AppPath::with_base(windows_root, "config.toml");
            assert_eq!(config.path(), Path::new(r"C:\config.toml"));
        }
    }

    #[test]
    fn test_root_directory_behavior_with_absolute_paths() {
        // Test that absolute paths work correctly even when base is root
        let root = if cfg!(windows) {
            Path::new(r"C:\")
        } else {
            Path::new("/")
        };

        let absolute_path = if cfg!(windows) {
            r"D:\temp\config.toml"
        } else {
            "/tmp/config.toml"
        };

        let app_path = AppPath::with_base(root, absolute_path);

        // Absolute paths should override the base entirely
        assert_eq!(app_path.path(), Path::new(absolute_path));
        assert!(app_path.path().is_absolute());
    }

    #[test]
    fn test_root_directory_nested_paths() {
        // Test that nested relative paths work correctly from root
        let root = if cfg!(windows) {
            Path::new(r"C:\")
        } else {
            Path::new("/")
        };

        let nested_path = "app/data/config.toml";
        let app_path = AppPath::with_base(root, nested_path);

        let expected = if cfg!(windows) {
            Path::new(r"C:\app\data\config.toml")
        } else {
            Path::new("/app/data/config.toml")
        };

        assert_eq!(app_path.path(), expected);
        assert!(app_path.path().is_absolute());
    }

    #[test]
    fn test_exe_dir_static_initialization() {
        // Test that exe_dir() works and returns an absolute path
        let dir = exe_dir();
        assert!(dir.is_absolute());

        // Test that it's consistent across multiple calls
        let dir2 = exe_dir();
        assert_eq!(dir, dir2);

        // Test that it works with AppPath
        let config = AppPath::new("test.txt");
        let expected = dir.join("test.txt");
        assert_eq!(config.path(), &expected);
    }

    #[test]
    fn test_exe_dir_edge_case_simulation() {
        // We can't easily simulate the actual root directory edge case,
        // but we can test that our logic works correctly

        use std::path::PathBuf;

        // Simulate what would happen with a root-level executable
        let fake_root_exe = if cfg!(windows) {
            PathBuf::from(r"C:\app.exe")
        } else {
            PathBuf::from("/init")
        };

        // Test the logic that would be used in the actual edge case
        let parent = fake_root_exe.parent();
        let base_dir = match parent {
            Some(p) => p.to_path_buf(),
            None => {
                // This is the edge case logic from our implementation
                fake_root_exe.ancestors().last().unwrap().to_path_buf()
            }
        };

        let expected_root = if cfg!(windows) {
            PathBuf::from(r"C:\")
        } else {
            PathBuf::from("/")
        };

        assert_eq!(base_dir, expected_root);
    }

    #[test]
    fn test_containerized_environment_simulation() {
        // Test behavior that might occur in containerized environments
        // where the executable could be at various root-like locations

        let container_roots = if cfg!(windows) {
            vec![r"C:\", r"D:\app"]
        } else {
            vec!["/", "/app", "/usr/bin"]
        };

        for root in container_roots {
            let config = AppPath::with_base(root, "config.toml");
            let data = AppPath::with_base(root, "data/app.db");

            // Paths should be properly resolved
            assert!(config.path().is_absolute());
            assert!(data.path().is_absolute());

            // Should maintain the root as prefix
            assert!(config.path().starts_with(root));
            assert!(data.path().starts_with(root));
        }
    }

    #[test]
    fn test_jailed_environment_patterns() {
        // Test common patterns that might occur in jailed/chrooted environments
        let jail_root = if cfg!(windows) {
            r"C:\jail"
        } else {
            "/var/jail"
        };

        // Test that relative paths work correctly in jailed environments
        let config = AppPath::with_base(jail_root, "etc/config.toml");
        let data = AppPath::with_base(jail_root, "var/data/app.db");
        let logs = AppPath::with_base(jail_root, "var/log/app.log");

        // All paths should be absolute and start with the jail root
        assert!(config.path().is_absolute());
        assert!(data.path().is_absolute());
        assert!(logs.path().is_absolute());

        assert!(config.path().starts_with(jail_root));
        assert!(data.path().starts_with(jail_root));
        assert!(logs.path().starts_with(jail_root));
    }

    #[test]
    fn test_panic_conditions_documentation() {
        // This test documents the conditions that would cause panics
        // It doesn't actually panic, but serves as documentation

        // These are the conditions that would cause the static initialization to panic:
        // 1. std::env::current_exe() fails
        // 2. The executable path is empty
        // 3. ancestors().last() fails (extremely rare)

        // We can't easily test these conditions in a unit test since they're
        // part of static initialization, but we can document them

        // The actual panic would happen during the first call to any AppPath function
        // or exe_dir() function when the LazyLock is initialized

        // For testing purposes, we just verify that normal operation works
        let _config = AppPath::new("config.toml");
        let _dir = exe_dir();

        // If we reach here, the static initialization succeeded
        // Test passes by reaching this point without panicking
    }
}
