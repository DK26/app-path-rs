//! Trait implementations for `AppPath`.
//!
//! This module contains all the standard trait implementations that make `AppPath`
//! work seamlessly with Rust's standard library and idiomatic code patterns.

use crate::AppPath;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::path::{Path, PathBuf};

// === Core Display and Conversion Traits ===

/// Default implementation returns the executable's directory.
///
/// This provides a natural default for AppPath - the directory where
/// the executable is located.
///
/// # Examples
///
/// ```rust
/// use app_path::AppPath;
/// use std::path::Path;
///
/// let exe_dir = AppPath::default();
/// let explicit = AppPath::new();
/// assert_eq!(exe_dir.as_ref() as &Path, explicit.as_ref() as &Path);
/// ```
impl Default for AppPath {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

// === Core Display and Conversion Traits ===

impl std::fmt::Display for AppPath {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full_path.display())
    }
}

impl AsRef<Path> for AppPath {
    #[inline]
    fn as_ref(&self) -> &Path {
        &self.full_path
    }
}

// === Infallible From Implementations ===

impl From<&str> for AppPath {
    #[inline]
    fn from(path: &str) -> Self {
        Self::with(path)
    }
}

impl From<String> for AppPath {
    #[inline]
    fn from(path: String) -> Self {
        Self::with(path)
    }
}

impl From<&String> for AppPath {
    #[inline]
    fn from(path: &String) -> Self {
        Self::with(path)
    }
}

impl From<&Path> for AppPath {
    #[inline]
    fn from(path: &Path) -> Self {
        Self::with(path)
    }
}

impl From<PathBuf> for AppPath {
    #[inline]
    fn from(path: PathBuf) -> Self {
        Self::with(path)
    }
}

impl From<&PathBuf> for AppPath {
    #[inline]
    fn from(path: &PathBuf) -> Self {
        Self::with(path)
    }
}

// === Additional Trait Implementations ===

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
    /// let path1 = AppPath::with("config.toml");
    /// let path2 = AppPath::with("config.toml");
    /// let path3 = AppPath::with("other.toml");
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
    /// let path1 = AppPath::with("a.txt");
    /// let path2 = AppPath::with("b.txt");
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
    /// paths.insert(AppPath::with("config.toml"));
    /// paths.insert(AppPath::with("data.db"));
    /// paths.insert(AppPath::with("app.log"));
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
    /// let config_path = AppPath::with("config.toml");
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
    /// let app_path = AppPath::with("config.toml");
    ///
    /// // Direct access to Path methods through deref
    /// assert_eq!(app_path.extension(), Some("toml".as_ref()));
    /// assert_eq!(app_path.file_name(), Some("config.toml".as_ref()));
    /// assert!(app_path.is_absolute());
    ///
    /// // Works with functions expecting &Path
    /// fn process_path(path: &std::path::Path) {
    ///     println!("Processing: {}", path.display());
    /// }
    /// process_path(&app_path); // Automatic deref coercion
    ///
    /// // For explicit &Path reference when needed
    /// let path_ref: &std::path::Path = &app_path;        // Via deref
    /// let path_ref2: &std::path::Path = app_path.as_ref(); // Via AsRef
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
    /// let app_path = AppPath::with("config.toml");
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

// === Additional Conversion Traits ===

impl AsRef<std::ffi::OsStr> for AppPath {
    /// Converts `AppPath` to `&OsStr` for FFI operations.
    ///
    /// This is useful when interfacing with operating system APIs that require `OsStr`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    /// use std::ffi::OsStr;
    ///
    /// let config = AppPath::with("config.toml");
    /// let os_str: &OsStr = config.as_ref();
    /// ```
    #[inline]
    fn as_ref(&self) -> &std::ffi::OsStr {
        self.full_path.as_os_str()
    }
}

impl From<AppPath> for PathBuf {
    /// Converts `AppPath` to `PathBuf` for owned path operations.
    ///
    /// This moves the internal `PathBuf` out of the `AppPath`, providing
    /// efficient conversion without cloning.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    /// use std::path::PathBuf;
    ///
    /// let config = AppPath::with("config.toml");
    /// let path_buf: PathBuf = config.into();
    /// ```
    #[inline]
    fn from(app_path: AppPath) -> Self {
        app_path.full_path
    }
}

impl From<AppPath> for std::ffi::OsString {
    /// Converts `AppPath` to `OsString` for owned FFI operations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    /// use std::ffi::OsString;
    ///
    /// let config = AppPath::with("config.toml");
    /// let os_string: OsString = config.into();
    /// ```
    #[inline]
    fn from(app_path: AppPath) -> Self {
        app_path.full_path.into_os_string()
    }
}
