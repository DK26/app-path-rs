use crate::{AppPath, AppPathError};

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
    /// let temp_dir = env::temp_dir().join("app_path_parent_example");
    ///
    /// // Prepare directories for a log file
    /// let log_file = AppPath::new(temp_dir.join("logs/2024/app.log"));
    /// log_file.create_parents()?; // Creates logs/2024/ directories
    ///
    /// // Parent directories exist, but file does not
    /// assert!(temp_dir.join("logs").exists());
    /// assert!(temp_dir.join("logs/2024").exists());
    /// assert!(!log_file.exists()); // File not created, only parent dirs
    ///
    /// // Now you can write the file
    /// std::fs::write(&log_file, "Log entry")?;
    /// assert!(log_file.exists());
    ///
    /// # std::fs::remove_dir_all(&temp_dir).ok();
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// ## Complex Directory Structures
    ///
    /// ```rust
    /// use app_path::AppPath;
    /// use std::env;
    ///
    /// let temp_dir = env::temp_dir().join("app_path_complex_example");
    ///
    /// // Create parents for config file
    /// let config_file = AppPath::new(temp_dir.join("config/database/settings.toml"));
    /// config_file.create_parents()?; // Creates config/database/ directories
    ///
    /// // Create parents for data file  
    /// let data_file = AppPath::new(temp_dir.join("data/users/profiles.db"));
    /// data_file.create_parents()?; // Creates data/users/ directories
    ///
    /// // All parent directories exist
    /// assert!(temp_dir.join("config").exists());
    /// assert!(temp_dir.join("config/database").exists());
    /// assert!(temp_dir.join("data").exists());
    /// assert!(temp_dir.join("data/users").exists());
    ///
    /// # std::fs::remove_dir_all(&temp_dir).ok();
    /// # Ok::<(), app_path::AppPathError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`AppPathError::IoError`] if directory creation fails:
    /// - **Insufficient permissions** - Cannot create directories due to filesystem permissions
    /// - **Disk space exhausted** - Not enough space to create directory entries
    /// - **Invalid path characters** - Path contains characters invalid for the target filesystem
    /// - **Network filesystem issues** - Problems with remote/networked filesystems
    /// - **Filesystem corruption** - Underlying filesystem errors
    ///
    /// The operation is **not atomic** - some parent directories may be created even if the
    /// operation ultimately fails.
    #[inline]
    pub fn create_parents(&self) -> Result<(), AppPathError> {
        if let Some(parent) = self.full_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        Ok(())
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
    /// - **Creates the directory itself**: Unlike `create_parents()`, this creates the full path as a directory
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
    /// cache_dir.create_dir()?; // Creates cache/ directory
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
    /// deep_dir.create_dir()?; // Creates data/backups/daily/ directories
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
    /// config_dir.create_dir()?;
    /// data_dir.create_dir()?;
    /// cache_dir.create_dir()?;
    /// logs_dir.create_dir()?;
    ///
    /// // Now create subdirectories
    /// let daily_logs = logs_dir.join("daily");
    /// daily_logs.create_dir()?;
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
    /// # Ok::<(), app_path::AppPathError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`AppPathError::IoError`] if directory creation fails:
    /// - **Insufficient permissions** - Cannot create directories due to filesystem permissions
    /// - **Disk space exhausted** - Not enough space to create directory entries  
    /// - **Invalid path characters** - Path contains characters invalid for the target filesystem
    /// - **Network filesystem issues** - Problems with remote/networked filesystems
    /// - **Path already exists as file** - A file already exists at this path (not a directory)
    /// - **Filesystem corruption** - Underlying filesystem errors
    ///
    /// The operation creates parent directories as needed, but is **not atomic** - some
    /// parent directories may be created even if the final directory creation fails.
    #[inline]
    pub fn create_dir(&self) -> Result<(), AppPathError> {
        std::fs::create_dir_all(self)?;
        Ok(())
    }
}
