use std::path::Path;

use crate::AppPath;

impl AppPath {
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
    /// let data_dir = AppPath::with("data");
    /// let users_db = data_dir.join("users.db");
    /// let backups = data_dir.join("backups").join("daily");
    ///
    /// // Chain operations for complex paths
    /// let log_file = AppPath::with("logs")
    ///     .join("2024")
    ///     .join("app.log");
    /// ```
    #[inline]
    pub fn join(&self, path: impl AsRef<Path>) -> Self {
        Self {
            full_path: self.full_path.join(path),
        }
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
    /// let config = AppPath::with("config/app.toml");
    /// let config_dir = config.parent().unwrap();
    ///
    /// let logs_dir = AppPath::with("logs");
    /// let _app_dir = logs_dir.parent(); // Points to exe directory
    /// ```
    #[inline]
    pub fn parent(&self) -> Option<Self> {
        self.full_path.parent().map(Self::with)
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
    /// let config = AppPath::with("config");
    /// let config_toml = config.with_extension("toml");
    /// let config_json = config.with_extension("json");
    ///
    /// let log_file = AppPath::with("app.log");
    /// let backup_file = log_file.with_extension("bak");
    /// ```
    #[inline]
    pub fn with_extension(&self, ext: &str) -> Self {
        Self::with(self.full_path.with_extension(ext))
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
    /// let app_path = AppPath::with("config.toml");
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
    /// let app_path = AppPath::with("config.toml");
    /// let path_buf: PathBuf = app_path.into_inner();
    ///
    /// // Now you have a regular PathBuf for operations that need ownership
    /// assert!(path_buf.is_absolute());
    /// ```
    #[inline]
    pub fn into_inner(self) -> std::path::PathBuf {
        self.into_path_buf()
    }

    /// Returns the path as encoded bytes for low-level path operations.
    ///
    /// This provides access to the platform-specific byte representation of the path.
    /// The returned bytes use platform-specific encoding and are only valid within
    /// the same Rust version and target platform.
    ///
    /// **Safety Note**: These bytes should not be sent over networks, stored in files,
    /// or used across different platforms, as the encoding is implementation-specific.
    ///
    /// Use cases include:
    /// - **Platform-specific path parsing** with precise byte-level control
    /// - **Custom path validation** that works with raw path bytes
    /// - **Integration with platform APIs** that expect encoded path bytes
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    ///
    /// let config = AppPath::with("config.toml");
    /// let bytes = config.to_bytes();
    ///
    /// // Platform-specific byte operations
    /// assert!(!bytes.is_empty());
    /// ```
    #[inline]
    pub fn to_bytes(&self) -> Vec<u8> {
        // Use stable methods for getting bytes from OsStr
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStrExt;
            self.as_os_str().as_bytes().to_vec()
        }
        #[cfg(windows)]
        {
            use std::os::windows::ffi::OsStrExt;
            let wide: Vec<u16> = self.as_os_str().encode_wide().collect();
            wide.iter().flat_map(|&w| w.to_le_bytes()).collect()
        }
        #[cfg(not(any(unix, windows)))]
        {
            // Fallback for other platforms - convert through string representation
            self.to_string_lossy().as_bytes().to_vec()
        }
    }

    /// Returns the path as owned encoded bytes.
    ///
    /// This consumes the AppPath and returns owned bytes using the same platform-specific
    /// encoding as `to_bytes()`. The returned bytes are only valid within the same
    /// Rust version and target platform.
    ///
    /// **Safety Note**: These bytes should not be sent over networks, stored in files,
    /// or used across different platforms, as the encoding is implementation-specific.
    ///
    /// Use cases include:
    /// - **Moving path data** across ownership boundaries
    /// - **Temporary storage** of path bytes during processing
    /// - **Platform-specific algorithms** requiring owned byte data
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    ///
    /// let config = AppPath::with("config.toml");
    /// let owned_bytes = config.into_bytes();
    ///
    /// // Owned bytes can be moved and stored
    /// assert!(!owned_bytes.is_empty());
    /// ```
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        // Use stable methods for getting bytes from OsString
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStringExt;
            self.into_path_buf().into_os_string().into_vec()
        }
        #[cfg(windows)]
        {
            use std::os::windows::ffi::OsStrExt;
            let wide: Vec<u16> = self.as_os_str().encode_wide().collect();
            wide.iter().flat_map(|&w| w.to_le_bytes()).collect()
        }
        #[cfg(not(any(unix, windows)))]
        {
            // Fallback for other platforms - convert through string representation
            self.to_string_lossy().into_owned().into_bytes()
        }
    }
}
