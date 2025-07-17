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
    /// // For displaying paths - use Display trait
    /// println!("Config path: {}", config.display());
    ///
    /// // For getting &Path reference - use deref
    /// assert!(config.is_absolute());
    ///
    /// // For functions expecting &Path - use as_ref() or &config
    /// std::fs::write(&config, "content")?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    #[deprecated(
        since = "0.2.7",
        note = "Use `&app_path` or `app_path.as_ref()` instead. AppPath implements Deref<Target=Path>, so all Path methods are directly available."
    )]
    #[inline]
    pub fn path(&self) -> &Path {
        &self.full_path
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

    /// Consumes the `AppPath` and returns the internal `PathBuf`.
    ///
    /// This provides zero-cost extraction of the underlying `PathBuf` by moving
    /// it out of the wrapper. This is useful when you need owned access to the
    /// path for operations that consume `PathBuf`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    /// use std::path::PathBuf;
    ///
    /// let app_path = AppPath::new("config.toml");
    /// let path_buf: PathBuf = app_path.into_path_buf();
    ///
    /// // Now you have a regular PathBuf for operations that need ownership
    /// assert!(path_buf.is_absolute());
    /// ```
    #[inline]
    pub fn into_path_buf(self) -> std::path::PathBuf {
        self.full_path
    }

    /// Consumes the `AppPath` and returns the internal `PathBuf`.
    ///
    /// This is an alias for [`into_path_buf()`](Self::into_path_buf) following
    /// the standard Rust pattern for extracting wrapped values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    /// use std::path::PathBuf;
    ///
    /// let app_path = AppPath::new("config.toml");
    /// let path_buf: PathBuf = app_path.into_inner();
    ///
    /// // Now you have a regular PathBuf for operations that need ownership
    /// assert!(path_buf.is_absolute());
    /// ```
    #[inline]
    pub fn into_inner(self) -> std::path::PathBuf {
        self.into_path_buf()
    }
}
