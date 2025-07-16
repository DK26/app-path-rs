use crate::AppPath;

impl AppPath {
    /// Creates parent directories needed for this file path.
    ///
    /// This method creates all parent directories for a file path, making it ready
    /// for file creation. It does not create the file itself.
    ///
    /// **Use this when you know the path represents a file and you want to prepare
    /// the directory structure for writing the file.**
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    /// use std::env;
    ///
    /// let temp_dir = env::temp_dir().join("app_path_example");
    ///
    /// // Prepare directories for a config file
    /// let config_file = AppPath::new(temp_dir.join("config/app.toml"));
    /// config_file.ensure_parent_dirs()?; // Creates config/ directory
    ///
    /// // Now you can write the file
    /// std::fs::write(&config_file, "key = value")?;
    /// assert!(config_file.exists());
    ///
    /// // Prepare directories for a log file
    /// let log_file = AppPath::new(temp_dir.join("logs/2024/app.log"));
    /// log_file.ensure_parent_dirs()?; // Creates logs/2024/ directories
    ///
    /// # std::fs::remove_dir_all(&temp_dir).ok();
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[deprecated(
        since = "0.2.4",
        note = "Use `create_parents()` instead for clearer intent"
    )]
    #[inline]
    pub fn ensure_parent_dirs(&self) -> std::io::Result<()> {
        if let Some(parent) = self.parent() {
            std::fs::create_dir_all(&parent)
        } else {
            Ok(())
        }
    }

    /// Creates this path as a directory, including all parent directories.
    ///
    /// This method treats the path as a directory and creates it along with
    /// all necessary parent directories. The created directory will exist
    /// after this call succeeds.
    ///
    /// **Use this when you know the path represents a directory that should be created.**
    ///
    /// # Behavior
    ///
    /// - **Creates the directory itself**: Unlike `ensure_parent_dirs()`, this creates the full path as a directory
    /// - **Creates all parents**: Any missing parent directories are created automatically
    /// - **Idempotent**: Safe to call multiple times - won't fail if directory already exists
    /// - **Atomic-like**: Either all directories are created or the operation fails
    ///
    /// # Examples
    ///
    /// ## Basic Directory Creation
    ///
    /// ```rust
    /// use app_path::AppPath;
    /// use std::env;
    ///
    /// let temp_dir = env::temp_dir().join("app_path_dir_example");
    ///
    /// // Create a cache directory
    /// let cache_dir = AppPath::new(temp_dir.join("cache"));
    /// cache_dir.ensure_dir_exists()?; // Creates cache/ directory
    /// assert!(cache_dir.exists());
    /// assert!(cache_dir.is_dir());
    ///
    /// # std::fs::remove_dir_all(&temp_dir).ok();
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// ## Nested Directory Structures
    ///
    /// ```rust
    /// use app_path::AppPath;
    /// use std::env;
    ///
    /// let temp_dir = env::temp_dir().join("app_path_nested_example");
    ///
    /// // Create deeply nested directories
    /// let deep_dir = AppPath::new(temp_dir.join("data/backups/daily"));
    /// deep_dir.ensure_dir_exists()?; // Creates data/backups/daily/ directories
    /// assert!(deep_dir.exists());
    /// assert!(deep_dir.is_dir());
    ///
    /// // All parent directories are also created
    /// let backups_dir = AppPath::new(temp_dir.join("data/backups"));
    /// assert!(backups_dir.exists());
    /// assert!(backups_dir.is_dir());
    ///
    /// # std::fs::remove_dir_all(&temp_dir).ok();
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// ## Practical Application Setup
    ///
    /// ```rust
    /// use app_path::AppPath;
    /// use std::env;
    ///
    /// let temp_dir = env::temp_dir().join("app_setup_example");
    ///
    /// // Set up application directory structure
    /// let config_dir = AppPath::new(temp_dir.join("config"));
    /// let data_dir = AppPath::new(temp_dir.join("data"));
    /// let cache_dir = AppPath::new(temp_dir.join("cache"));
    /// let logs_dir = AppPath::new(temp_dir.join("logs"));
    ///
    /// // Create all directories
    /// config_dir.ensure_dir_exists()?;
    /// data_dir.ensure_dir_exists()?;
    /// cache_dir.ensure_dir_exists()?;
    /// logs_dir.ensure_dir_exists()?;
    ///
    /// // Now create subdirectories
    /// let daily_logs = logs_dir.join("daily");
    /// daily_logs.ensure_dir_exists()?;
    ///
    /// // Verify structure
    /// assert!(config_dir.is_dir());
    /// assert!(data_dir.is_dir());
    /// assert!(cache_dir.is_dir());
    /// assert!(logs_dir.is_dir());
    /// assert!(daily_logs.is_dir());
    ///
    /// # std::fs::remove_dir_all(&temp_dir).ok();
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// ## Comparison with `create_parents()`
    ///
    /// ```rust
    /// use app_path::AppPath;
    /// use std::env;
    ///
    /// let temp_dir = env::temp_dir().join("app_comparison_example");
    ///
    /// let file_path = AppPath::new(temp_dir.join("logs/app.log"));
    /// let dir_path = AppPath::new(temp_dir.join("logs"));
    ///
    /// // For files: prepare parent directories
    /// file_path.create_parents()?; // Creates logs/ directory
    /// assert!(dir_path.exists()); // logs/ directory exists
    /// assert!(!file_path.exists()); // app.log file does NOT exist
    ///
    /// // For directories: create the directory itself  
    /// dir_path.create_dir()?; // Creates logs/ directory (idempotent)
    /// assert!(dir_path.exists()); // logs/ directory exists
    /// assert!(dir_path.is_dir()); // and it's definitely a directory
    ///
    /// # std::fs::remove_dir_all(&temp_dir).ok();
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[deprecated(
        since = "0.2.4",
        note = "Use `create_dir()` instead for clearer intent"
    )]
    #[inline]
    pub fn ensure_dir_exists(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(self)
    }

    /// Creates all directories needed for this path.
    ///
    /// **DEPRECATED**: Use [`create_parents()`](Self::create_parents) for file paths
    /// or [`create_dir()`](Self::create_dir) for directory paths instead.
    /// This method name was confusing as it didn't always create directories for the path itself.
    ///
    /// This method intelligently determines whether the path represents a file
    /// or directory and creates the appropriate directories:
    /// - **For existing directories**: does nothing (already exists)
    /// - **For existing files**: creates parent directories if needed
    /// - **For non-existing paths**: treats as file path and creates parent directories
    ///
    /// # Migration Guide
    ///
    /// ```rust
    /// use app_path::AppPath;
    ///
    /// let file_path = AppPath::new("logs/app.log");
    /// let dir_path = AppPath::new("cache");
    ///
    /// // Old (deprecated):
    /// // file_path.create_dir_all()?;
    /// // dir_path.create_dir_all()?; // This was confusing!
    ///
    /// // New (clear):
    /// file_path.create_parents()?; // Creates logs/ for the file
    /// dir_path.create_dir()?;      // Creates cache/ directory
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[deprecated(
        since = "0.2.2",
        note = "Use `create_parents()` for file paths or `create_dir()` for directory paths instead"
    )]
    #[inline]
    pub fn create_dir_all(&self) -> std::io::Result<()> {
        if let Some(parent) = self.full_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        Ok(())
    }
}
