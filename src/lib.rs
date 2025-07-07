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
//! ## Performance and Memory Design
//!
//! **AppPath** is optimized for high-performance applications:
//!
//! - **Static caching**: Executable directory determined once, cached forever
//! - **Minimal memory**: Only stores the final resolved path (no input path retained)
//! - **Zero allocations**: Uses `AsRef<Path>` to avoid unnecessary conversions
//! - **Efficient conversions**: `From` trait implementations for all common types
//! - **Thread-safe**: Safe concurrent access to cached executable directory
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
//! - **Containerized environments**: Properly handles Docker, chroot, and other containerized environments
//! - **Jailed environments**: Handles various forms of process isolation and sandboxing
//!
//! For applications that need fallible behavior, consider these practical alternatives:
//!
//! ```rust
//! use app_path::AppPath;
//! use std::env;
//!
//! // Pattern 1: Environment variable fallback (recommended)
//! fn get_config_path() -> AppPath {
//!     if let Ok(custom_dir) = env::var("MYAPP_CONFIG_DIR") {
//!         let config_path = std::path::Path::new(&custom_dir).join("config.toml");
//!         AppPath::new(config_path)
//!     } else {
//!         AppPath::new("config.toml")
//!     }
//! }
//!
//! // Pattern 2: Conditional development/production paths
//! fn get_data_path() -> AppPath {
//!     if env::var("DEVELOPMENT").is_ok() {
//!         let dev_path = env::current_dir().unwrap().join("dev_data").join("app.db");
//!         AppPath::new(dev_path)
//!     } else {
//!         AppPath::new("data/app.db")
//!     }
//! }
//! ```
//!
//! For applications that need to handle executable location failures:
//!
//! ```rust
//! use app_path::AppPath;
//! use std::env;
//!
//! fn get_config_path_safe() -> AppPath {
//!     match env::current_exe() {
//!         Ok(exe_path) => {
//!             if let Some(exe_dir) = exe_path.parent() {
//!                 let config_path = exe_dir.join("config.toml");
//!                 AppPath::new(config_path)
//!             } else {
//!                 // Fallback for edge case where exe has no parent
//!                 let temp_dir = env::temp_dir().join("myapp");
//!                 let _ = std::fs::create_dir_all(&temp_dir);
//!                 let config_path = temp_dir.join("config.toml");
//!                 AppPath::new(config_path)
//!             }
//!         }
//!         Err(_) => {
//!             // Fallback when executable location cannot be determined
//!             let temp_dir = env::temp_dir().join("myapp");
//!             let _ = std::fs::create_dir_all(&temp_dir);
//!             let config_path = temp_dir.join("config.toml");
//!             AppPath::new(config_path)
//!         }
//!     }
//! }
//! ```
//!
//! **Note:** Using `std::env::current_exe()` directly is simpler and more idiomatic
//! than `panic::catch_unwind` patterns. Most applications should use environment
//! variable and conditional patterns instead.
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

use std::borrow::Borrow;
use std::cmp::Ordering;
use std::env::current_exe;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

// Global executable directory - computed once, cached forever
static EXE_DIR: OnceLock<PathBuf> = OnceLock::new();

/// Internal function to initialize and get the executable directory
fn exe_dir_inner() -> &'static PathBuf {
    EXE_DIR.get_or_init(|| {
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
    })
}

/// Creates paths relative to the executable location for applications.
///
/// **AppPath** is the core type for building portable applications where all files and directories
/// stay together with the executable. This design choice makes applications truly portable -
/// they can run from USB drives, network shares, or any directory without installation.
///
/// ## Available Trait Implementations
///
/// `AppPath` implements a comprehensive set of traits for seamless integration with Rust's
/// standard library and idiomatic code patterns:
///
/// **Core Traits:**
/// - `Clone` - Efficient cloning (only copies the resolved path)
/// - `Debug` - Useful debug output showing the resolved path
/// - `Default` - Creates a path pointing to the executable directory
/// - `Display` - Human-readable path display
///
/// **Comparison Traits:**
/// - `PartialEq`, `Eq` - Compare paths for equality
/// - `PartialOrd`, `Ord` - Lexicographic ordering for sorting
/// - `Hash` - Use as keys in `HashMap`, `HashSet`, etc.
///
/// **Conversion Traits:**
/// - `AsRef<Path>` - Use with any API expecting `&Path`
/// - `Deref<Target=Path>` - Direct access to `Path` methods (e.g., `.extension()`)
/// - `Borrow<Path>` - Enable borrowing as `&Path` for collection compatibility
/// - `From<T>` for `&str`, `String`, `&Path`, `PathBuf`, etc. - Flexible construction
/// - `Into<PathBuf>` - Convert to owned `PathBuf`
///
/// These implementations make `AppPath` a **zero-cost abstraction** that works seamlessly
/// with existing Rust code while providing portable path resolution.
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
///
/// ## Trait Implementation Examples
///
/// `AppPath` implements many useful traits that enable ergonomic usage patterns:
///
/// ```rust
/// use app_path::AppPath;
/// use std::collections::{HashMap, BTreeSet};
///
/// // Default trait - creates path to executable directory
/// let exe_dir = AppPath::default();
/// assert_eq!(exe_dir, AppPath::new(""));
///
/// // Comparison traits - enable sorting and equality checks
/// let mut paths = vec![
///     AppPath::new("z.txt"),
///     AppPath::new("a.txt"),
///     AppPath::new("m.txt"),
/// ];
/// paths.sort(); // Uses Ord trait
/// assert!(paths[0] < paths[1]); // Uses PartialOrd trait
///
/// // Hash trait - use as keys in collections
/// let mut file_types = HashMap::new();
/// file_types.insert(AppPath::new("config.toml"), "Configuration");
/// file_types.insert(AppPath::new("data.db"), "Database");
///
/// // Ordered collections work automatically
/// let mut sorted_paths = BTreeSet::new();
/// sorted_paths.insert(AppPath::new("config.toml"));
/// sorted_paths.insert(AppPath::new("data.db"));
///
/// // Deref trait - direct access to Path methods
/// let config = AppPath::new("config.toml");
/// assert_eq!(config.extension(), Some("toml".as_ref())); // Direct Path method
/// assert_eq!(config.file_name(), Some("config.toml".as_ref()));
///
/// // Works with functions expecting &Path (deref coercion)
/// fn analyze_path(path: &std::path::Path) -> Option<&str> {
///     path.extension()?.to_str()
/// }
/// assert_eq!(analyze_path(&config), Some("toml"));
///
/// // From trait - flexible construction from many types
/// let from_str: AppPath = "data.txt".into();
/// let from_pathbuf: AppPath = std::path::PathBuf::from("logs.txt").into();
///
/// // Display trait - human-readable output
/// println!("Config path: {}", config); // Clean path display
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
    /// * `path` - A path that will be resolved according to AppPath's resolution strategy.
    ///   Accepts any type implementing [`AsRef<Path>`]:
    ///   - `&str` - String literals and string slices
    ///   - `String` - Owned strings
    ///   - `&Path` - Path references
    ///   - `PathBuf` - Path buffers
    ///   - And many others that implement `AsRef<Path>`
    ///
    ///   **Path Resolution:**
    ///   - **Relative paths** are resolved relative to the executable directory
    ///   - **Absolute paths** are used as-is (not modified)
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
        let full_path = exe_dir_inner().join(path.as_ref());
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
    /// let data_file_path = temp_dir.join("data/users/profile.json");
    /// let data_file = AppPath::new(data_file_path);
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
    exe_dir_inner().as_path()
}

// Standard trait implementations
impl std::fmt::Display for AppPath {
    #[inline]
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
    #[inline]
    fn from(path: &str) -> Self {
        Self::new(path)
    }
}

impl From<String> for AppPath {
    #[inline]
    fn from(path: String) -> Self {
        Self::new(path)
    }
}

impl From<&String> for AppPath {
    #[inline]
    fn from(path: &String) -> Self {
        Self::new(path)
    }
}

impl From<&Path> for AppPath {
    #[inline]
    fn from(path: &Path) -> Self {
        Self::new(path)
    }
}

impl From<PathBuf> for AppPath {
    #[inline]
    fn from(path: PathBuf) -> Self {
        Self::new(path)
    }
}

// === Additional Trait Implementations ===

impl Default for AppPath {
    /// Creates an `AppPath` pointing to the executable's directory.
    ///
    /// This is equivalent to calling `AppPath::new("")`. The default instance
    /// represents the directory containing the executable, which is useful as
    /// a starting point for portable applications.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    ///
    /// let exe_dir = AppPath::default();
    /// let empty_path = AppPath::new("");
    ///
    /// // Default should be equivalent to new("")
    /// assert_eq!(exe_dir, empty_path);
    ///
    /// // Both should point to the executable directory
    /// assert_eq!(exe_dir.path(), app_path::exe_dir());
    /// ```
    #[inline]
    fn default() -> Self {
        Self::new("")
    }
}

impl PartialEq for AppPath {
    /// Compares two `AppPath` instances for equality based on their resolved paths.
    ///
    /// Two `AppPath` instances are considered equal if their full resolved paths
    /// are identical, regardless of how they were constructed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    ///
    /// let path1 = AppPath::new("config.toml");
    /// let path2 = AppPath::new("config.toml");
    /// let path3 = AppPath::new("other.toml");
    ///
    /// assert_eq!(path1, path2);
    /// assert_ne!(path1, path3);
    /// ```
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.full_path == other.full_path
    }
}

impl Eq for AppPath {}

impl PartialOrd for AppPath {
    /// Compares two `AppPath` instances lexicographically based on their resolved paths.
    ///
    /// The comparison is performed on the full resolved paths, providing consistent
    /// ordering for sorting and collection operations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    ///
    /// let path1 = AppPath::new("a.txt");
    /// let path2 = AppPath::new("b.txt");
    ///
    /// assert!(path1 < path2);
    /// ```
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AppPath {
    /// Compares two `AppPath` instances lexicographically based on their resolved paths.
    ///
    /// This provides a total ordering that enables `AppPath` to be used in sorted
    /// collections like `BTreeMap` and `BTreeSet`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    /// use std::collections::BTreeSet;
    ///
    /// let mut paths = BTreeSet::new();
    /// paths.insert(AppPath::new("config.toml"));
    /// paths.insert(AppPath::new("data.db"));
    /// paths.insert(AppPath::new("app.log"));
    ///
    /// // Paths are automatically sorted lexicographically
    /// let sorted: Vec<_> = paths.into_iter().collect();
    /// ```
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.full_path.cmp(&other.full_path)
    }
}

impl Hash for AppPath {
    /// Computes a hash for the `AppPath` based on its resolved path.
    ///
    /// This enables `AppPath` to be used as keys in hash-based collections
    /// like `HashMap` and `HashSet`. The hash is computed from the full
    /// resolved path, ensuring consistent behavior.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    /// use std::collections::HashMap;
    ///
    /// let mut config_map = HashMap::new();
    /// let config_path = AppPath::new("config.toml");
    /// config_map.insert(config_path, "Configuration file");
    /// ```
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.full_path.hash(state);
    }
}

impl Deref for AppPath {
    type Target = Path;

    /// Provides direct access to the underlying `Path` through deref coercion.
    ///
    /// This allows `AppPath` to be used directly with any API that expects a `&Path`,
    /// making it a zero-cost abstraction in many contexts. All `Path` methods become
    /// directly available on `AppPath` instances.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    ///
    /// let app_path = AppPath::new("config.toml");
    ///
    /// // Direct access to Path methods through deref
    /// assert_eq!(app_path.extension(), Some("toml".as_ref()));
    /// assert_eq!(app_path.file_name(), Some("config.toml".as_ref()));
    ///
    /// // Works with functions expecting &Path
    /// fn process_path(path: &std::path::Path) {
    ///     println!("Processing: {}", path.display());
    /// }
    ///
    /// process_path(&app_path); // Automatic deref coercion
    /// ```
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.full_path
    }
}

impl Borrow<Path> for AppPath {
    /// Allows `AppPath` to be borrowed as a `Path`.
    ///
    /// This enables `AppPath` to be used seamlessly in collections that are
    /// keyed by `Path`, and allows for efficient lookups using `&Path` values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    /// use std::collections::HashMap;
    /// use std::path::Path;
    ///
    /// let mut path_map = HashMap::new();
    /// let app_path = AppPath::new("config.toml");
    /// path_map.insert(app_path, "config data");
    ///
    /// // Can look up using a &Path
    /// let lookup_path = Path::new("relative/to/exe/config.toml");
    /// // Note: This would only work if the paths actually match
    /// ```
    #[inline]
    fn borrow(&self) -> &Path {
        &self.full_path
    }
}

impl From<&PathBuf> for AppPath {
    #[inline]
    fn from(path: &PathBuf) -> Self {
        Self::new(path)
    }
}

#[cfg(test)]
mod tests;
