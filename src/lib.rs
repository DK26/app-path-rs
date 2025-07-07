//! # app-path
//!
//! Create file paths relative to your executable for truly portable applications.
//!
//! ## Quick Start
//!
//! ```rust
//! use app_path::AppPath;
//! use std::convert::TryFrom;
//! use std::path::{Path, PathBuf};
//!
//! // Create paths relative to your executable - accepts any path-like type
//! let config = AppPath::try_new("config.toml")?;
//! let data = AppPath::try_new("data/users.db")?;
//!
//! // Efficient ownership transfer for owned types
//! let log_file = "logs/app.log".to_string();
//! let logs = AppPath::try_new(log_file)?; // String is moved
//!
//! let path_buf = PathBuf::from("cache/data.bin");
//! let cache = AppPath::try_new(path_buf)?; // PathBuf is moved
//!
//! // Works with any path-like type
//! let from_path = AppPath::try_new(Path::new("temp.txt"))?;
//!
//! // Alternative: Use TryFrom for various path types
//! let settings = AppPath::try_from("settings.json")?;         // &str
//! let data_file = AppPath::try_from(PathBuf::from("data.db"))?; // PathBuf
//! let temp_file = AppPath::try_from(Path::new("temp.log"))?;   // &Path
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
//! ## Flexible Creation Methods
//!
//! ```rust
//! use app_path::AppPath;
//! use std::convert::TryFrom;
//! use std::path::{Path, PathBuf};
//!
//! // Method 1: Direct construction (recommended)
//! let config = AppPath::try_new("config.toml")?;
//! let logs = AppPath::try_new(PathBuf::from("logs/app.log"))?;
//!
//! // Method 2: TryFrom for various path types
//! let data1 = AppPath::try_from("data/users.db")?;              // &str
//! let data2 = AppPath::try_from("settings.json".to_string())?;  // String
//! let data3 = AppPath::try_from(Path::new("cache/data.bin"))?;  // &Path
//! let data4 = AppPath::try_from(PathBuf::from("temp/file.txt"))?; // PathBuf
//!
//! // All methods support efficient ownership transfer
//! let owned_path = PathBuf::from("important/data.db");
//! let app_path = AppPath::try_from(owned_path)?; // PathBuf is moved
//!
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Error Handling
//!
//! All constructors (`try_new`, `TryFrom`) can fail if the executable location
//! cannot be determined. This is rare in normal applications but should be handled:
//!
//! ```rust
//! use app_path::AppPath;
//!
//! match AppPath::try_new("config.toml") {
//!     Ok(config) => {
//!         // Use config.path() for file operations
//!         println!("Config: {}", config.path().display());
//!     }
//!     Err(e) => {
//!         eprintln!("Cannot determine executable location: {}", e);
//!         // Fallback strategy - use current directory, temp directory, or exit
//!         std::process::exit(1);
//!     }
//! }
//! ```
//!
//! ## Path Resolution Behavior
//!
//! `AppPath` intelligently handles different path types:
//!
//! - **Relative paths** (e.g., `"config.toml"`, `"data/file.txt"`) are resolved
//!   relative to the executable's directory
//! - **Absolute paths** (e.g., `"/etc/config"`, `"C:\\temp\\file.txt"`) are used
//!   as-is, ignoring the executable's directory
//!
//! This design enables both portable applications and system integration.
//!
//! ## Performance
//!
//! AppPath is optimized for efficient ownership transfer:
//!
//! - **String and PathBuf**: Moved into AppPath (no cloning)
//! - **Generic types**: Uses `impl Into<PathBuf>` for zero-copy where possible
//! - **References**: Efficient conversion without unnecessary allocations

use std::env::current_exe;
use std::path::{Path, PathBuf};

/// Creates paths relative to the executable location for applications.
///
/// All files and directories stay together with the executable, making
/// your application truly portable. Perfect for:
///
/// - Portable applications that run from USB drives
/// - Development tools that should work anywhere
/// - Corporate environments where you can't install software
///
/// # Examples
///
/// ```rust
/// use app_path::AppPath;
///
/// // Basic usage
/// let config = AppPath::try_new("settings.toml")?;
/// let data_dir = AppPath::try_new("data")?;
///
/// // Check if files exist
/// if config.exists() {
///     let settings = std::fs::read_to_string(config.path())?;
/// }
///
/// // Create directories
/// data_dir.create_dir_all()?;
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, Debug)]
pub struct AppPath {
    input_path: PathBuf,
    full_path: PathBuf,
}

impl AppPath {
    /// Creates file paths relative to the executable location.
    ///
    /// This method accepts any type that can be converted into a `PathBuf`,
    /// allowing for efficient ownership transfer when possible.
    ///
    /// **Behavior with different path types:**
    /// - **Relative paths** (e.g., `"config.toml"`, `"data/file.txt"`) are resolved
    ///   relative to the executable's directory
    /// - **Absolute paths** (e.g., `"/etc/config"`, `"C:\\temp\\file.txt"`) are used
    ///   as-is, ignoring the executable's directory
    ///
    /// # Arguments
    ///
    /// * `path` - A path that will be resolved relative to the executable.
    ///   Can be `&str`, `String`, `&Path`, `PathBuf`, etc.
    ///
    /// # Errors
    ///
    /// Returns [`std::io::Error`] in the following cases:
    ///
    /// - **Cannot determine executable location** - When [`std::env::current_exe()`] fails
    ///   (rare, but can happen in some embedded environments)
    /// - **Executable has no parent directory** - When the executable is somehow at a filesystem root
    ///   (extremely rare in normal usage)
    ///
    /// These errors are typically unrecoverable as they indicate fundamental system issues.
    /// In normal desktop/server applications, this function should not fail.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    /// use std::path::PathBuf;
    ///
    /// // Relative paths are resolved relative to the executable
    /// let config = AppPath::try_new("config.toml")?;
    /// let data = AppPath::try_new("data/users.db")?;
    ///
    /// // Absolute paths are used as-is (portable apps usually want relative paths)
    /// let system_config = AppPath::try_new("/etc/app/config.toml")?;
    /// let temp_file = AppPath::try_new(r"C:\temp\cache.dat")?;
    ///
    /// // From String (moves ownership)
    /// let filename = "logs/app.log".to_string();
    /// let logs = AppPath::try_new(filename)?; // filename is moved
    ///
    /// // From PathBuf (moves ownership)
    /// let path_buf = PathBuf::from("cache/data.bin");
    /// let cache = AppPath::try_new(path_buf)?; // path_buf is moved
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// ## Error Handling
    ///
    /// ```rust
    /// use app_path::AppPath;
    ///
    /// match AppPath::try_new("config.toml") {
    ///     Ok(config) => {
    ///         println!("Config path: {}", config.path().display());
    ///     }
    ///     Err(e) => {
    ///         eprintln!("Failed to create app path: {}", e);
    ///         // Fallback to current directory or exit
    ///         std::process::exit(1);
    ///     }
    /// }
    /// ```
    pub fn try_new(path: impl Into<PathBuf>) -> Result<Self, std::io::Error> {
        let input_path = path.into();

        let exe_dir = current_exe()?
            .parent()
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Could not determine executable parent directory",
                )
            })?
            .to_path_buf();

        let full_path = exe_dir.join(&input_path);

        Ok(Self {
            input_path,
            full_path,
        })
    }

    /// Override the base directory (useful for testing or custom layouts).
    ///
    /// This method allows you to specify a different base directory instead
    /// of using the executable's directory. Useful for testing or when you
    /// want a different layout.
    ///
    /// # Arguments
    ///
    /// * `base` - The new base directory to use
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    /// use std::env;
    ///
    /// let config = AppPath::try_new("config.toml")?
    ///     .with_base(env::temp_dir());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn with_base(mut self, base: impl AsRef<Path>) -> Self {
        self.full_path = base.as_ref().join(&self.input_path);
        self
    }

    /// Get the original input path (before resolution).
    ///
    /// Returns the path as it was originally provided to [`AppPath::try_new`],
    /// before any resolution or joining with the base directory.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    ///
    /// let app_path = AppPath::try_new("config/settings.toml")?;
    /// assert_eq!(app_path.input().to_str(), Some("config/settings.toml"));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn input(&self) -> &Path {
        &self.input_path
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
    /// let config = AppPath::try_new("config.toml")?;
    ///
    /// // Get the path for use with standard library functions
    /// println!("Config path: {}", config.path().display());
    ///
    /// // The path is always absolute
    /// assert!(config.path().is_absolute());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
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
    /// let config = AppPath::try_new("config.toml")?;
    ///
    /// if config.exists() {
    ///     println!("Config file found!");
    /// } else {
    ///     println!("Config file not found, using defaults.");
    /// }
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
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
    /// let data_file = AppPath::try_new("data/users/profile.json")?
    ///     .with_base(&temp_dir);
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

/// Ergonomic fallible conversion from common path types.
///
/// These implementations allow you to create `AppPath` instances directly
/// from various path types, with proper error handling for the fallible operation.
///
/// # Supported Types
///
/// - `&str` - String literals and string slices
/// - `String` - Owned strings (moved into `AppPath`)
/// - `&String` - String references
/// - `PathBuf` - Path buffers (moved into `AppPath`)
/// - `&PathBuf` - Path buffer references
/// - `&Path` - Path references
///
/// # Errors
///
/// Returns [`std::io::Error`] if the executable location cannot be determined.
/// See [`AppPath::try_new`] for detailed error conditions.
///
/// # Examples
///
/// ```rust
/// use app_path::AppPath;
/// use std::convert::TryFrom;
/// use std::path::{Path, PathBuf};
///
/// // From string types
/// let config1 = AppPath::try_from("config.toml")?;
/// let config2 = AppPath::try_from("config.toml".to_string())?;
/// let config3 = AppPath::try_from(&"config.toml".to_string())?;
///
/// // From path types
/// let config4 = AppPath::try_from(Path::new("config.toml"))?;
/// let config5 = AppPath::try_from(PathBuf::from("config.toml"))?;
/// let config6 = AppPath::try_from(&PathBuf::from("config.toml"))?;
///
/// // All produce equivalent results
/// assert_eq!(config1.input(), config2.input());
/// assert_eq!(config2.input(), config3.input());
/// assert_eq!(config3.input(), config4.input());
/// assert_eq!(config4.input(), config5.input());
/// assert_eq!(config5.input(), config6.input());
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ## Ownership Transfer
///
/// ```rust
/// use app_path::AppPath;
/// use std::convert::TryFrom;
/// use std::path::PathBuf;
///
/// // These move ownership (no cloning)
/// let path_buf = PathBuf::from("data/users.db");
/// let data1 = AppPath::try_from(path_buf)?; // path_buf is moved
///
/// let string_path = "logs/app.log".to_string();
/// let data2 = AppPath::try_from(string_path)?; // string_path is moved
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ## Error Handling Example
///
/// ```rust
/// use app_path::AppPath;
/// use std::convert::TryFrom;
///
/// match AppPath::try_from("data.db") {
///     Ok(path) => println!("Data path: {}", path.path().display()),
///     Err(e) => {
///         eprintln!("Failed to create path: {}", e);
///         // Handle gracefully - maybe use current directory as fallback
///     }
/// }
/// ```
impl TryFrom<&str> for AppPath {
    type Error = std::io::Error;

    fn try_from(path: &str) -> Result<Self, Self::Error> {
        AppPath::try_new(path)
    }
}

impl TryFrom<String> for AppPath {
    type Error = std::io::Error;

    fn try_from(path: String) -> Result<Self, Self::Error> {
        AppPath::try_new(path)
    }
}

impl TryFrom<&String> for AppPath {
    type Error = std::io::Error;

    fn try_from(path: &String) -> Result<Self, Self::Error> {
        AppPath::try_new(path)
    }
}

impl TryFrom<&Path> for AppPath {
    type Error = std::io::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        AppPath::try_new(path)
    }
}

impl TryFrom<PathBuf> for AppPath {
    type Error = std::io::Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        AppPath::try_new(path)
    }
}

impl TryFrom<&PathBuf> for AppPath {
    type Error = std::io::Error;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        AppPath::try_new(path)
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
        // Simulate a file relative to the executable
        let rel = "myconfig.toml";
        let rel_path = AppPath::try_new(rel).unwrap();
        let exe_dir = current_exe().unwrap().parent().unwrap().to_path_buf();
        let expected = exe_dir.join(rel);

        assert_eq!(rel_path.path(), &expected);
        assert!(rel_path.path().is_absolute());
    }

    #[test]
    fn resolves_relative_path_with_custom_base() {
        // Use a temp directory as the base
        let temp_dir = env::temp_dir().join("app_path_test_base");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let rel = "subdir/file.txt";
        let rel_path = AppPath::try_new(rel).unwrap().with_base(&temp_dir);
        let expected = temp_dir.join(rel);

        assert_eq!(rel_path.path(), &expected);
        assert!(rel_path.path().is_absolute());
    }

    #[test]
    fn can_access_file_using_full_path() {
        // Actually create a file and check that the full path points to it
        let temp_dir = env::temp_dir().join("app_path_test_access");
        let file_name = "access.txt";
        let file_path = temp_dir.join(file_name);
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();
        create_test_file(&file_path);

        let rel_path = AppPath::try_new(file_name).unwrap().with_base(&temp_dir);
        assert!(rel_path.exists());
        assert_eq!(rel_path.path(), &file_path);
    }

    #[test]
    fn handles_dot_and_dotdot_components() {
        let temp_dir = env::temp_dir().join("app_path_test_dot");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let rel = "./foo/../bar.txt";
        let rel_path = AppPath::try_new(rel).unwrap().with_base(&temp_dir);
        let expected = temp_dir.join(rel);

        assert_eq!(rel_path.path(), &expected);
    }

    #[test]
    fn as_ref_and_into_pathbuf_are_consistent() {
        let rel = "somefile.txt";
        let rel_path = AppPath::try_new(rel).unwrap();
        let as_ref_path: &Path = rel_path.as_ref();
        let into_pathbuf: PathBuf = rel_path.clone().into();
        assert_eq!(as_ref_path, into_pathbuf.as_path());
    }

    #[test]
    fn test_input_method() {
        let rel = "config/app.toml";
        let rel_path = AppPath::try_new(rel).unwrap();
        assert_eq!(rel_path.input(), Path::new(rel));
    }

    #[test]
    fn test_path_method() {
        let rel = "data/file.txt";
        let temp_dir = env::temp_dir().join("app_path_test_full");
        let rel_path = AppPath::try_new(rel).unwrap().with_base(&temp_dir);
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

        let rel_path = AppPath::try_new(file_name).unwrap().with_base(&temp_dir);
        assert!(rel_path.exists());

        let non_existent = AppPath::try_new("non_existent.txt")
            .unwrap()
            .with_base(&temp_dir);
        assert!(!non_existent.exists());
    }

    #[test]
    fn test_create_dir_all() {
        let temp_dir = env::temp_dir().join("app_path_test_create");
        let _ = fs::remove_dir_all(&temp_dir);

        let rel = "deep/nested/dir/file.txt";
        let rel_path = AppPath::try_new(rel).unwrap().with_base(&temp_dir);

        rel_path.create_dir_all().unwrap();
        assert!(rel_path.path().parent().unwrap().exists());
    }

    #[test]
    fn test_display_trait() {
        let rel = "display_test.txt";
        let temp_dir = env::temp_dir().join("app_path_test_display");
        let rel_path = AppPath::try_new(rel).unwrap().with_base(&temp_dir);

        let expected = temp_dir.join(rel);
        assert_eq!(format!("{rel_path}"), format!("{}", expected.display()));
    }

    #[test]
    fn test_try_from_str() {
        use std::convert::TryFrom;

        let rel_path = AppPath::try_from("config.toml").unwrap();
        let exe_dir = current_exe().unwrap().parent().unwrap().to_path_buf();
        let expected = exe_dir.join("config.toml");

        assert_eq!(rel_path.path(), &expected);
        assert_eq!(rel_path.input(), Path::new("config.toml"));
    }

    #[test]
    fn test_try_from_string() {
        use std::convert::TryFrom;

        let path_string = "data/file.txt".to_string();
        let rel_path = AppPath::try_from(path_string).unwrap();
        let exe_dir = current_exe().unwrap().parent().unwrap().to_path_buf();
        let expected = exe_dir.join("data/file.txt");

        assert_eq!(rel_path.path(), &expected);
        assert_eq!(rel_path.input(), Path::new("data/file.txt"));
    }

    #[test]
    fn test_try_from_string_ref() {
        use std::convert::TryFrom;

        let path_string = "logs/app.log".to_string();
        let rel_path = AppPath::try_from(&path_string).unwrap();
        let exe_dir = current_exe().unwrap().parent().unwrap().to_path_buf();
        let expected = exe_dir.join("logs/app.log");

        assert_eq!(rel_path.path(), &expected);
        assert_eq!(rel_path.input(), Path::new("logs/app.log"));
    }

    #[test]
    fn test_try_new_with_different_types() {
        use std::path::PathBuf;

        // Test various input types with try_new
        let from_str = AppPath::try_new("test.txt").unwrap();
        let from_string = AppPath::try_new("test.txt".to_string()).unwrap();
        let from_path_buf = AppPath::try_new(PathBuf::from("test.txt")).unwrap();
        let from_path_ref = AppPath::try_new(Path::new("test.txt")).unwrap();

        // All should produce equivalent results
        assert_eq!(from_str.input(), from_string.input());
        assert_eq!(from_string.input(), from_path_buf.input());
        assert_eq!(from_path_buf.input(), from_path_ref.input());
        assert_eq!(from_str.input(), Path::new("test.txt"));
    }

    #[test]
    fn test_ownership_transfer() {
        use std::path::PathBuf;

        let path_buf = PathBuf::from("test.txt");
        let app_path = AppPath::try_new(path_buf).unwrap();
        // path_buf is moved and no longer accessible

        assert_eq!(app_path.input(), Path::new("test.txt"));

        // Test with String too
        let string_path = "another_test.txt".to_string();
        let app_path2 = AppPath::try_new(string_path).unwrap();
        // string_path is moved and no longer accessible

        assert_eq!(app_path2.input(), Path::new("another_test.txt"));
    }

    #[test]
    fn test_absolute_path_behavior() {
        // Test what happens with an absolute path
        let absolute_path = if cfg!(windows) {
            r"C:\temp\config.toml"
        } else {
            "/tmp/config.toml"
        };

        let app_path = AppPath::try_new(absolute_path).unwrap();

        // PathBuf::join() has special behavior: when joining with an absolute path,
        // the absolute path replaces the base path entirely
        // So AppPath correctly handles absolute paths by using them as-is
        assert_eq!(app_path.path(), Path::new(absolute_path));
        assert_eq!(app_path.input(), Path::new(absolute_path));

        println!("Input: {absolute_path}");
        println!("Result: {}", app_path.path().display());

        // Verify it's still an absolute path
        assert!(app_path.path().is_absolute());
    }

    #[test]
    fn test_pathbuf_join_behavior_with_absolute_paths() {
        use std::path::PathBuf;

        // Let's understand how PathBuf::join works with absolute paths
        let base = PathBuf::from("/home/user");
        let absolute = PathBuf::from("/etc/config.toml");
        let relative = PathBuf::from("config.toml");

        println!("Base: {}", base.display());
        println!("Relative join: {}", base.join(&relative).display());
        println!("Absolute join: {}", base.join(&absolute).display());

        // PathBuf::join has special behavior for absolute paths:
        // If the right-hand side is absolute, it replaces the left-hand side
        assert_eq!(
            base.join(&relative),
            PathBuf::from("/home/user/config.toml")
        );
        assert_eq!(base.join(&absolute), PathBuf::from("/etc/config.toml"));

        // Same on Windows
        if cfg!(windows) {
            let win_base = PathBuf::from(r"C:\Users\User");
            let win_absolute = PathBuf::from(r"D:\temp\file.txt");
            let win_relative = PathBuf::from("file.txt");

            assert_eq!(
                win_base.join(&win_relative),
                PathBuf::from(r"C:\Users\User\file.txt")
            );
            assert_eq!(
                win_base.join(&win_absolute),
                PathBuf::from(r"D:\temp\file.txt")
            );
        }
    }

    #[test]
    fn test_try_from_path_types() {
        use std::convert::TryFrom;
        use std::path::{Path, PathBuf};

        let exe_dir = current_exe().unwrap().parent().unwrap().to_path_buf();
        let expected = exe_dir.join("test.txt");

        // Test &Path
        let from_path_ref = AppPath::try_from(Path::new("test.txt")).unwrap();
        assert_eq!(from_path_ref.path(), &expected);
        assert_eq!(from_path_ref.input(), Path::new("test.txt"));

        // Test PathBuf (moves ownership)
        let path_buf = PathBuf::from("test.txt");
        let from_path_buf = AppPath::try_from(path_buf).unwrap();
        assert_eq!(from_path_buf.path(), &expected);
        assert_eq!(from_path_buf.input(), Path::new("test.txt"));

        // Test &PathBuf
        let path_buf_ref = PathBuf::from("test.txt");
        let from_path_buf_ref = AppPath::try_from(&path_buf_ref).unwrap();
        assert_eq!(from_path_buf_ref.path(), &expected);
        assert_eq!(from_path_buf_ref.input(), Path::new("test.txt"));
    }

    #[test]
    fn test_try_from_ownership_transfer() {
        use std::convert::TryFrom;
        use std::path::PathBuf;

        // Test that PathBuf ownership is transferred
        let path_buf = PathBuf::from("ownership_test.txt");
        let app_path = AppPath::try_from(path_buf).unwrap();
        // path_buf is moved and no longer accessible
        assert_eq!(app_path.input(), Path::new("ownership_test.txt"));

        // Test that String ownership is transferred
        let string_path = "string_ownership_test.txt".to_string();
        let app_path2 = AppPath::try_from(string_path).unwrap();
        // string_path is moved and no longer accessible
        assert_eq!(app_path2.input(), Path::new("string_ownership_test.txt"));
    }

    #[test]
    fn test_try_from_all_types_equivalent() {
        use std::convert::TryFrom;
        use std::path::{Path, PathBuf};

        // All these should produce equivalent results
        let from_str = AppPath::try_from("equivalent.txt").unwrap();
        let from_string = AppPath::try_from("equivalent.txt".to_string()).unwrap();
        let from_string_ref = AppPath::try_from(&"equivalent.txt".to_string()).unwrap();
        let from_path = AppPath::try_from(Path::new("equivalent.txt")).unwrap();
        let from_pathbuf = AppPath::try_from(PathBuf::from("equivalent.txt")).unwrap();
        let from_pathbuf_ref = AppPath::try_from(&PathBuf::from("equivalent.txt")).unwrap();

        // All should have the same input path
        assert_eq!(from_str.input(), from_string.input());
        assert_eq!(from_string.input(), from_string_ref.input());
        assert_eq!(from_string_ref.input(), from_path.input());
        assert_eq!(from_path.input(), from_pathbuf.input());
        assert_eq!(from_pathbuf.input(), from_pathbuf_ref.input());

        // All should have the same full path
        assert_eq!(from_str.path(), from_string.path());
        assert_eq!(from_string.path(), from_string_ref.path());
        assert_eq!(from_string_ref.path(), from_path.path());
        assert_eq!(from_path.path(), from_pathbuf.path());
        assert_eq!(from_pathbuf.path(), from_pathbuf_ref.path());
    }

    #[test]
    fn test_try_from_with_absolute_paths() {
        use std::convert::TryFrom;
        use std::path::{Path, PathBuf};

        let absolute_path = if cfg!(windows) {
            r"C:\temp\absolute_test.txt"
        } else {
            "/tmp/absolute_test.txt"
        };

        // Test all TryFrom implementations with absolute paths
        let from_str = AppPath::try_from(absolute_path).unwrap();
        let from_string = AppPath::try_from(absolute_path.to_string()).unwrap();
        let from_path = AppPath::try_from(Path::new(absolute_path)).unwrap();
        let from_pathbuf = AppPath::try_from(PathBuf::from(absolute_path)).unwrap();

        // All should preserve the absolute path
        assert_eq!(from_str.path(), Path::new(absolute_path));
        assert_eq!(from_string.path(), Path::new(absolute_path));
        assert_eq!(from_path.path(), Path::new(absolute_path));
        assert_eq!(from_pathbuf.path(), Path::new(absolute_path));

        // All should be absolute
        assert!(from_str.path().is_absolute());
        assert!(from_string.path().is_absolute());
        assert!(from_path.path().is_absolute());
        assert!(from_pathbuf.path().is_absolute());
    }
}
