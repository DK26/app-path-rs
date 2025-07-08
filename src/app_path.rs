use std::path::{Path, PathBuf};

use crate::error::AppPathError;
use crate::functions::try_exe_dir;

/// Creates paths relative to the executable location for portable applications.
///
/// **AppPath** enables building truly portable applications where configuration, data,
/// and executable stay together as a deployable unit. Perfect for USB drives, network
/// shares, or any directory without installation.
///
/// ## Key Features
///
/// - **Portable**: Relative paths resolve to executable directory
/// - **System integration**: Absolute paths work as-is  
/// - **Zero-cost**: Implements `Deref<Target=Path>` and all path traits
/// - **Thread-safe**: Static caching with proper synchronization
/// - **Memory efficient**: Only stores the final resolved path
///
/// ## API Overview
///
/// - [`new()`] - **Primary API**: Simple, infallible construction
/// - [`try_new()`] - **Libraries**: Fallible version for error handling  
/// - [`new_with_override()`] - **Deployment**: Environment-configurable paths
/// - [`path()`] - **Access**: Get the resolved `&Path`
///
/// # Panics
///
/// Methods panic if executable location cannot be determined (extremely rare).
/// After first successful call, methods never panic (uses cached result).
///
/// # Examples
///
/// ```rust
/// use app_path::AppPath;
///
/// // Basic usage - most common pattern
/// let config = AppPath::new("config.toml");
/// let data = AppPath::new("data/users.db");
///
/// // Works like standard paths
/// if config.exists() {
///     let content = std::fs::read_to_string(&config);
/// }
/// data.create_dir_all(); // Creates data/ directory
///
/// // Mixed portable and system paths
/// let portable = AppPath::new("app.conf");           // → exe_dir/app.conf
/// let system = AppPath::new("/var/log/app.log");     // → /var/log/app.log
///
/// // Override for deployment flexibility
/// let config = AppPath::new_with_override(
///     "config.toml",
///     std::env::var("CONFIG_PATH").ok()
/// );
/// ```
#[derive(Clone, Debug)]
pub struct AppPath {
    full_path: PathBuf,
}

impl AppPath {
    /// Creates file paths relative to the executable location.
    ///
    /// **Recommended for most applications.** This is the simple, infallible API that handles
    /// the common case cleanly without error handling boilerplate.
    ///
    /// ## Path Resolution
    ///
    /// - **Relative paths**: `"config.toml"` → `exe_dir/config.toml` (portable)
    /// - **Absolute paths**: `"/etc/config"` → `/etc/config` (system integration)
    ///
    /// ## Performance
    ///
    /// - **Static caching**: Executable location determined once, reused forever
    /// - **Zero allocations**: Efficient path resolution
    /// - **Thread-safe**: Safe to call from multiple threads
    ///
    /// # Panics
    ///
    /// Panics if executable location cannot be determined (extremely rare):
    /// - `std::env::current_exe()` fails
    /// - Executable path is empty (system corruption)
    ///
    /// After first successful call, this method never panics (uses cached result).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    ///
    /// // Most common usage
    /// let config = AppPath::new("config.toml");
    /// let data = AppPath::new("data/users.db");
    /// let logs = AppPath::new("logs/app.log");
    ///
    /// // Mixed portable and system paths
    /// let app_config = AppPath::new("config.toml");           // → exe_dir/config.toml
    /// let system_log = AppPath::new("/var/log/myapp.log");    // → /var/log/myapp.log
    ///
    /// // Use like normal paths
    /// if config.exists() {
    ///     let content = std::fs::read_to_string(&config);
    /// }
    /// data.create_dir_all(); // Creates data/ directory
    /// ```
    ///
    /// # Panics
    ///
    /// Panics on first use if the executable location cannot be determined.
    /// This is extremely rare and indicates fundamental system issues.
    /// See [`AppPathError`] for details on the possible failure conditions.
    ///
    /// After the first successful call, this method will never panic as it uses the cached result.
    ///
    /// # Performance
    ///
    /// This method is highly optimized:
    /// - **Static caching**: Executable location determined once, reused forever
    /// - **Zero allocations**: Uses `AsRef<Path>` to avoid unnecessary conversions
    /// - **Minimal memory**: Only stores the final resolved path
    /// - **Thread-safe**: Safe to call from multiple threads concurrently
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    ///
    /// // Most common usage
    /// let config = AppPath::new("config.toml");
    /// let data = AppPath::new("data/users.db");
    /// let logs = AppPath::new("logs/app.log");
    ///
    /// // Mixed portable and system paths
    /// let app_config = AppPath::new("config.toml");           // → exe_dir/config.toml
    /// let system_log = AppPath::new("/var/log/myapp.log");    // → /var/log/myapp.log
    ///
    /// // Use like normal paths
    /// if config.exists() {
    ///     let content = std::fs::read_to_string(&config);
    /// }
    /// data.create_dir_all(); // Creates data/ directory
    /// ```
    pub fn new(path: impl AsRef<Path>) -> Self {
        match Self::try_new(path) {
            Ok(app_path) => app_path,
            Err(e) => panic!("Failed to create AppPath: {e}"),
        }
    }

    /// Creates file paths relative to the executable location (fallible).
    ///
    /// **Use this only for libraries or specialized applications requiring explicit error handling.**
    /// Most applications should use [`new()`] instead for simpler, cleaner code.
    ///
    /// ## When to Use
    ///
    /// **Use `try_new()` for:**
    /// - Reusable libraries that shouldn't panic
    /// - System tools with fallback strategies
    /// - Applications running in unusual environments
    ///
    /// **Use [`new()`] for:**
    /// - Desktop, web, server, CLI applications
    /// - When you want simple, clean code (recommended)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::{AppPath, AppPathError};
    ///
    /// // Library with graceful error handling
    /// fn load_config() -> Result<String, AppPathError> {
    ///     let config_path = AppPath::try_new("config.toml")?;
    ///     // Load configuration...
    ///     Ok("config loaded".to_string())
    /// }
    ///
    /// // Better: Use override API for environment variables
    /// fn load_config_with_override() -> Result<String, AppPathError> {
    ///     let config_path = AppPath::try_new_with_override(
    ///         "config.toml",
    ///         std::env::var("APP_CONFIG").ok()
    ///     )?;
    ///     // Load configuration...
    ///     Ok("config loaded".to_string())
    /// }
    ///
    /// // Multiple environment variable fallback (better approach)
    /// fn get_config_with_fallback() -> Result<AppPath, AppPathError> {
    ///     AppPath::try_new_with_override_fn("config.toml", || {
    ///         std::env::var("APP_CONFIG").ok()
    ///             .or_else(|| std::env::var("CONFIG_FILE").ok())
    ///             .or_else(|| std::env::var("XDG_CONFIG_HOME").ok().map(|dir| format!("{}/myapp/config.toml", dir)))
    ///     })
    /// }
    /// ```
    ///
    /// **Reality check:** Executable location determination failing is extremely rare:
    /// - It requires fundamental system issues or unusual deployment scenarios
    /// - When it happens, it usually indicates unrecoverable system problems
    /// - Most applications can't meaningfully continue without knowing their location
    /// - The error handling overhead isn't worth it for typical applications
    ///
    /// **Better approaches for most applications:**
    /// ```rust
    /// use app_path::AppPath;
    /// use std::env;
    ///
    /// // Use our override API for environment variables (recommended)
    /// fn get_config_path() -> AppPath {
    ///     AppPath::new_with_override("config.toml", env::var("MYAPP_CONFIG_DIR").ok())
    /// }
    ///
    /// // Or fallible version for libraries
    /// fn try_get_config_path() -> Result<AppPath, app_path::AppPathError> {
    ///     AppPath::try_new_with_override("config.toml", env::var("MYAPP_CONFIG_DIR").ok())
    /// }
    /// ```
    ///
    /// ## Global Caching Behavior
    ///
    /// Once the executable directory is successfully determined by either this method or [`AppPath::new()`],
    /// the result is cached globally and all subsequent calls to both methods will use the cached value.
    /// This means that after the first successful call, `try_new()` will never return an error.
    ///
    /// # Arguments
    ///
    /// * `path` - A path that will be resolved according to AppPath's resolution strategy.
    ///   Accepts any type implementing [`AsRef<Path>`].
    ///
    /// # Returns
    ///
    /// * `Ok(AppPath)` - Successfully created AppPath with resolved path
    /// * `Err(AppPathError)` - Failed to determine executable location (extremely rare)
    ///
    /// # Examples
    ///
    /// ## Library Error Handling
    ///
    /// ```rust
    /// use app_path::{AppPath, AppPathError};
    ///
    /// // Library function that returns Result instead of panicking
    /// pub fn create_config_manager() -> Result<ConfigManager, AppPathError> {
    ///     let config_path = AppPath::try_new("config.toml")?;
    ///     Ok(ConfigManager::new(config_path))
    /// }
    ///
    /// pub struct ConfigManager {
    ///     config_path: AppPath,
    /// }
    ///
    /// impl ConfigManager {
    ///     fn new(config_path: AppPath) -> Self {
    ///         Self { config_path }
    ///     }
    /// }
    /// ```
    ///
    /// ## Error Propagation Pattern
    ///
    /// ```rust
    /// use app_path::{AppPath, AppPathError};
    ///
    /// fn initialize_app() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = AppPath::try_new("config.toml")?;
    ///     let data = AppPath::try_new("data/app.db")?;
    ///     
    ///     // Initialize application with these paths
    ///     println!("Config: {}", config.path().display());
    ///     println!("Data: {}", data.path().display());
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn try_new(path: impl AsRef<Path>) -> Result<Self, AppPathError> {
        let exe_dir = try_exe_dir()?;
        let input_path = path.as_ref();

        // Apply the same resolution strategy as new()
        let full_path = if input_path.is_absolute() {
            input_path.to_path_buf()
        } else {
            exe_dir.join(input_path)
        };

        Ok(Self { full_path })
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

    /// Creates a path with override support (infallible).
    ///
    /// This method provides a one-line solution for creating paths that can be overridden
    /// by external configuration. If an override is provided, it takes precedence over
    /// the default path. Otherwise, the default path is used with normal AppPath resolution.
    ///
    /// **This is the primary method for implementing configurable paths in applications.**
    /// It combines the simplicity of [`AppPath::new()`] with the flexibility of external
    /// configuration overrides.
    ///
    /// ## Common Use Cases
    ///
    /// - **Environment variable overrides**: Allow users to customize file locations
    /// - **Command-line argument overrides**: CLI tools with configurable paths
    ///
    /// ## How It Works
    ///
    /// **If override is provided**: Use the override path directly (can be relative or absolute)
    /// **If override is `None`**: Use the default path with normal AppPath resolution
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    /// use std::env;
    ///
    /// // Environment variable override
    /// let config = AppPath::new_with_override(
    ///     "config.toml",
    ///     env::var("APP_CONFIG").ok()
    /// );
    ///
    /// // CLI argument override
    /// fn get_config(cli_override: Option<&str>) -> AppPath {
    ///     AppPath::new_with_override("config.toml", cli_override)
    /// }
    ///
    /// // Configuration file override
    /// struct Config {
    ///     data_dir: Option<String>,
    /// }
    ///
    /// let config = load_config();
    /// let data_dir = AppPath::new_with_override("data", config.data_dir.as_deref());
    /// # fn load_config() -> Config { Config { data_dir: None } }
    /// ```
    /// ```
    pub fn new_with_override(
        default: impl AsRef<Path>,
        override_option: Option<impl AsRef<Path>>,
    ) -> Self {
        match override_option {
            Some(override_path) => Self::new(override_path),
            None => Self::new(default),
        }
    }

    /// Creates a path with dynamic override support.
    ///
    /// **Use this for complex override logic or lazy evaluation.** The closure is called once
    /// to determine if an override should be applied.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    /// use std::env;
    ///
    /// // Multiple fallback sources
    /// let config = AppPath::new_with_override_fn("config.toml", || {
    ///     env::var("APP_CONFIG").ok()
    ///         .or_else(|| env::var("CONFIG_FILE").ok())
    ///         .or_else(|| {
    ///             // Only check expensive operations if needed
    ///             if env::var("USE_SYSTEM_CONFIG").is_ok() {
    ///                 Some("/etc/myapp/config.toml".to_string())
    ///             } else {
    ///                 None
    ///             }
    ///         })
    /// });
    ///
    /// // Development mode override
    /// let data_dir = AppPath::new_with_override_fn("data", || {
    ///     if env::var("DEVELOPMENT").is_ok() {
    ///         Some("dev_data".to_string())
    ///     } else {
    ///         None
    ///     }
    /// });
    /// ```
    pub fn new_with_override_fn<F, P>(default: impl AsRef<Path>, override_fn: F) -> Self
    where
        F: FnOnce() -> Option<P>,
        P: AsRef<Path>,
    {
        match override_fn() {
            Some(override_path) => Self::new(override_path),
            None => Self::new(default),
        }
    }

    /// Creates a path with override support (fallible).
    ///
    /// **Fallible version of [`new_with_override()`].** Most applications should use the
    /// infallible version instead for cleaner code.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::{AppPath, AppPathError};
    /// use std::env;
    ///
    /// fn get_config() -> Result<AppPath, AppPathError> {
    ///     AppPath::try_new_with_override("config.toml", env::var("CONFIG").ok())
    /// }
    /// ```
    /// cleaner, more idiomatic code.
    ///
    /// ## When to Use This Method
    ///
    /// - **Reusable libraries** that should handle errors gracefully
    /// - **System-level tools** that need to handle broken environments
    /// - **Applications with custom fallback strategies** for rare edge cases
    ///
    /// See [`AppPath::try_new()`] for detailed guidance on when to use fallible APIs.
    ///
    /// # Arguments
    ///
    /// * `default` - The default path to use if no override is provided
    /// * `override_option` - Optional override path that takes precedence if provided
    ///
    /// # Returns
    ///
    /// * `Ok(AppPath)` - Successfully created AppPath with resolved path
    /// * `Err(AppPathError)` - Failed to determine executable location
    ///
    /// # Examples
    ///
    /// ## Library with Error Handling
    ///
    /// ```rust
    /// use app_path::{AppPath, AppPathError};
    /// use std::env;
    ///
    /// fn create_config_path() -> Result<AppPath, AppPathError> {
    ///     let config_override = env::var("MYAPP_CONFIG").ok();
    ///     AppPath::try_new_with_override("config.toml", config_override.as_deref())
    /// }
    /// ```
    ///
    /// ## Error Propagation
    ///
    /// ```rust
    /// use app_path::{AppPath, AppPathError};
    ///
    /// fn setup_paths(config_override: Option<&str>) -> Result<(AppPath, AppPath), AppPathError> {
    ///     let config = AppPath::try_new_with_override("config.toml", config_override)?;
    ///     let data = AppPath::try_new_with_override("data", None::<&str>)?;
    ///     Ok((config, data))
    /// }
    /// ```
    pub fn try_new_with_override(
        default: impl AsRef<Path>,
        override_option: Option<impl AsRef<Path>>,
    ) -> Result<Self, AppPathError> {
        match override_option {
            Some(override_path) => Self::try_new(override_path),
            None => Self::try_new(default),
        }
    }

    /// Creates a path with dynamic override support (fallible).
    ///
    /// This is the fallible version of [`AppPath::new_with_override_fn()`]. Use this method
    /// when you need explicit error handling combined with dynamic override logic.
    ///
    /// **Most applications should use [`AppPath::new_with_override_fn()`] instead** for
    /// cleaner, more idiomatic code.
    ///
    /// ## When to Use This Method
    ///
    /// - **Reusable libraries** with complex override logic that should handle errors gracefully
    /// - **System-level tools** with dynamic configuration that need to handle broken environments
    /// - **Applications with custom fallback strategies** for rare edge cases
    ///
    /// See [`AppPath::try_new()`] for detailed guidance on when to use fallible APIs.
    ///
    /// # Arguments
    ///
    /// * `default` - The default path to use if the override function returns `None`
    /// * `override_fn` - A function that returns an optional override path
    ///
    /// # Returns
    ///
    /// * `Ok(AppPath)` - Successfully created AppPath with resolved path
    /// * `Err(AppPathError)` - Failed to determine executable location
    ///
    /// # Examples
    ///
    /// ## Library with Complex Override Logic
    ///
    /// ```rust
    /// use app_path::{AppPath, AppPathError};
    /// use std::env;
    ///
    /// fn create_data_path() -> Result<AppPath, AppPathError> {
    ///     AppPath::try_new_with_override_fn("data", || {
    ///         // Complex override logic with multiple sources
    ///         env::var("DATA_DIR").ok()
    ///             .or_else(|| env::var("MYAPP_DATA_DIR").ok())
    ///             .or_else(|| {
    ///                 if env::var("DEVELOPMENT").is_ok() {
    ///                     Some("dev_data".to_string())
    ///                 } else {
    ///                     None
    ///                 }
    ///             })
    ///     })
    /// }
    /// ```
    ///
    /// ## Error Propagation with Dynamic Logic
    ///
    /// ```rust
    /// use app_path::{AppPath, AppPathError};
    ///
    /// fn setup_logging() -> Result<AppPath, AppPathError> {
    ///     AppPath::try_new_with_override_fn("logs/app.log", || {
    ///         // Dynamic override based on multiple conditions
    ///         if std::env::var("SYSLOG").is_ok() {
    ///             Some("/var/log/myapp.log".to_string())
    ///         } else if std::env::var("LOG_TO_TEMP").is_ok() {
    ///             Some(std::env::temp_dir().join("myapp.log").to_string_lossy().into_owned())
    ///         } else {
    ///             None
    ///         }
    ///     })
    /// }
    /// ```
    pub fn try_new_with_override_fn<F, P>(
        default: impl AsRef<Path>,
        override_fn: F,
    ) -> Result<Self, AppPathError>
    where
        F: FnOnce() -> Option<P>,
        P: AsRef<Path>,
    {
        match override_fn() {
            Some(override_path) => Self::try_new(override_path),
            None => Self::try_new(default),
        }
    }
}
