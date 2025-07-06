//! # app-path
//!
//! Create file paths relative to your executable for truly portable applications.
//!
//! ## Quick Start
//!
//! ```rust
//! use app_path::AppPath;
//! use std::convert::TryFrom;
//! use std::path::PathBuf;
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
//! let from_path = AppPath::try_new(std::path::Path::new("temp.txt"))?;
//!
//! // Alternative: Use TryFrom for string types
//! let settings = AppPath::try_from("settings.json")?;
//!
//! // Absolute paths are used as-is (for system integration)
//! let system_log = AppPath::try_new("/var/log/app.log")?;
//! let windows_temp = AppPath::try_new(r"C:\temp\cache.dat")?;
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

/// Ergonomic fallible conversion from string types.
///
/// These implementations allow you to create `AppPath` instances directly
/// from strings, with proper error handling for the fallible operation.
/// For other path types like `PathBuf`, use [`AppPath::try_new`] directly.
///
/// # Examples
///
/// ```rust
/// use app_path::AppPath;
/// use std::convert::TryFrom;
/// use std::path::PathBuf;
///
/// // From &str
/// let config = AppPath::try_from("config.toml")?;
///
/// // From String (moves ownership)
/// let data_file = "data/users.db".to_string();
/// let data = AppPath::try_from(data_file)?;
///
/// // For PathBuf, use try_new directly (moves ownership)
/// let path_buf = PathBuf::from("logs/app.log");
/// let logs = AppPath::try_new(path_buf)?;
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
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
}
