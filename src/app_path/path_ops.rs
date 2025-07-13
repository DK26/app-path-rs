use std::path::Path;

use crate::AppPath;

impl AppPath {
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

    /// Joins additional path segments to create a new AppPath.
    ///
    /// This creates a new `AppPath` by joining the current path with additional segments.
    /// The new path inherits the same resolution behavior as the original.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    ///
    /// let data_dir = AppPath::new("data");
    /// let users_db = data_dir.join("users.db");
    /// let backups = data_dir.join("backups").join("daily");
    ///
    /// // Chain operations for complex paths
    /// let log_file = AppPath::new("logs")
    ///     .join("2024")
    ///     .join("app.log");
    /// ```
    #[inline]
    pub fn join(&self, path: impl AsRef<Path>) -> Self {
        Self::new(self.full_path.join(path))
    }

    /// Returns the parent directory as an AppPath, if it exists.
    ///
    /// Returns `None` if this path is a root directory or has no parent.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    ///
    /// let config = AppPath::new("config/app.toml");
    /// let config_dir = config.parent().unwrap();
    ///
    /// let logs_dir = AppPath::new("logs");
    /// let _app_dir = logs_dir.parent(); // Points to exe directory
    /// ```
    #[inline]
    pub fn parent(&self) -> Option<Self> {
        self.full_path.parent().map(Self::new)
    }

    /// Creates a new AppPath with the specified file extension.
    ///
    /// If the path has an existing extension, it will be replaced.
    /// If no extension exists, the new extension will be added.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    ///
    /// let config = AppPath::new("config");
    /// let config_toml = config.with_extension("toml");
    /// let config_json = config.with_extension("json");
    ///
    /// let log_file = AppPath::new("app.log");
    /// let backup_file = log_file.with_extension("bak");
    /// ```
    #[inline]
    pub fn with_extension(&self, ext: &str) -> Self {
        Self::new(self.full_path.with_extension(ext))
    }

    /// Returns the file name of this path as an `OsStr`, if it exists.
    ///
    /// This is a convenience method that delegates to the underlying `Path`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    ///
    /// let config = AppPath::new("config/app.toml");
    /// assert_eq!(config.file_name().unwrap(), "app.toml");
    /// ```
    #[inline]
    pub fn file_name(&self) -> Option<&std::ffi::OsStr> {
        self.full_path.file_name()
    }

    /// Returns the file stem of this path as an `OsStr`, if it exists.
    ///
    /// This is a convenience method that delegates to the underlying `Path`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    ///
    /// let config = AppPath::new("config/app.toml");
    /// assert_eq!(config.file_stem().unwrap(), "app");
    /// ```
    #[inline]
    pub fn file_stem(&self) -> Option<&std::ffi::OsStr> {
        self.full_path.file_stem()
    }

    /// Returns the extension of this path as an `OsStr`, if it exists.
    ///
    /// This is a convenience method that delegates to the underlying `Path`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    ///
    /// let config = AppPath::new("config/app.toml");
    /// assert_eq!(config.extension().unwrap(), "toml");
    /// ```
    #[inline]
    pub fn extension(&self) -> Option<&std::ffi::OsStr> {
        self.full_path.extension()
    }

    /// Returns `true` if this path points to a directory.
    ///
    /// This is a convenience method that delegates to the underlying `Path`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    ///
    /// let data_dir = AppPath::new("data");
    /// if data_dir.is_dir() {
    ///     println!("Data directory exists");
    /// }
    /// ```
    #[inline]
    pub fn is_dir(&self) -> bool {
        self.full_path.is_dir()
    }

    /// Returns `true` if this path points to a regular file.
    ///
    /// This is a convenience method that delegates to the underlying `Path`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    ///
    /// let config = AppPath::new("config.toml");
    /// if config.is_file() {
    ///     println!("Config file exists");
    /// }
    /// ```
    #[inline]
    pub fn is_file(&self) -> bool {
        self.full_path.is_file()
    }
}
