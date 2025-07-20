use std::path::{Path, PathBuf};

use crate::{try_exe_dir, AppPath, AppPathError};

impl AppPath {
    /// Returns the application's base directory as an AppPath.
    ///
    /// **This is the primary method for getting the application's base directory.** It provides clean,
    /// idiomatic code for the common case of needing the application's base location.
    ///
    /// By default, the application's base directory is the executable's directory, but this can
    /// be overridden through environment variables or other configuration mechanisms using the
    /// override methods like [`Self::with_override()`].
    ///
    /// Use this when you need the application's base directory itself, then use [`join()`](Self::join)
    /// to build paths relative to it, or use [`Self::with()`] directly for one-step creation.
    ///
    /// ## Global Caching Behavior
    ///
    /// The application's base directory is determined once on first call and cached globally.
    /// All subsequent calls use the cached value for maximum performance.
    ///
    /// # Panics
    ///
    /// Panics only if the application's base directory cannot be determined, which is extremely rare
    /// and typically indicates fundamental system issues (corrupted installation, permission problems).
    /// After the first successful call, this method never panics.
    ///
    /// # Examples
    ///
    /// ## Basic Usage
    ///
    /// ```rust
    /// use app_path::AppPath;
    ///
    /// // Get the application's base directory (defaults to executable directory)
    /// let app_base = AppPath::new();
    ///
    /// // Build paths relative to it
    /// let config = app_base.join("config.toml");
    /// let data_dir = app_base.join("data");
    ///
    /// // Or chain operations
    /// let log_file = AppPath::new().join("logs").join("app.log");
    /// ```
    ///
    /// ## Real Application Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    /// use std::fs;
    ///
    /// // Get application base directory for setup operations
    /// let app_base = AppPath::new();
    /// println!("Application base directory: {}", app_base.display());
    ///
    /// // Create application directory structure
    /// app_base.join("data").create_dir()?;
    /// app_base.join("logs").create_dir()?;
    /// app_base.join("config").create_dir()?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// ## Portable Directory Access
    ///
    /// ```rust
    /// use app_path::AppPath;
    ///
    /// // Get application base directory regardless of installation location:
    /// // - C:\Program Files\MyApp\ (Windows)
    /// // - /usr/local/bin/ (Linux)
    /// // - /Applications/MyApp.app/Contents/MacOS/ (macOS)
    /// // - .\target\debug\ (development)
    ///
    /// let exe_dir = AppPath::new();
    /// let readme = exe_dir.join("README.txt");
    /// let license = exe_dir.join("LICENSE");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn new() -> Self {
        match Self::try_new() {
            Ok(app_path) => app_path,
            Err(e) => panic!("Failed to create AppPath: {e}"),
        }
    }

    /// Creates file paths relative to the executable location (fallible).
    ///
    /// **Use this only for libraries or specialized applications requiring explicit error handling.**
    /// Most applications should use [`Self::new()`] instead for simpler, cleaner code.
    ///
    /// ## When to Use
    ///
    /// **Use `try_new()` for:**
    /// - Reusable libraries that shouldn't panic
    /// - System tools with fallback strategies
    /// - Applications running in unusual environments
    ///
    /// **Use [`Self::new()`] for:**
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
    ///     let config_path = AppPath::try_with("config.toml")?;
    ///     // Load configuration...
    ///     Ok("config loaded".to_string())
    /// }
    ///
    /// // Better: Use override API for environment variables
    /// fn load_config_with_override() -> Result<String, AppPathError> {
    ///     let config_path = AppPath::try_with_override(
    ///         "config.toml",
    ///         std::env::var("APP_CONFIG").ok()
    ///     )?;
    ///     // Load configuration...
    ///     Ok("config loaded".to_string())
    /// }
    ///
    /// // Multiple environment variable fallback (better approach)
    /// fn get_config_with_fallback() -> Result<AppPath, AppPathError> {
    ///     AppPath::try_with_override_fn("config.toml", || {
    ///         std::env::var("APP_CONFIG").ok()
    ///             .or_else(|| std::env::var("CONFIG_FILE").ok())
    ///             .or_else(|| std::env::var("XDG_CONFIG_HOME").ok().map(|dir| format!("{dir}/myapp/config.toml")))
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
    ///     AppPath::with_override("config.toml", env::var("MYAPP_CONFIG_DIR").ok())
    /// }
    ///
    /// // Or fallible version for libraries
    /// fn try_get_config_path() -> Result<AppPath, app_path::AppPathError> {
    ///     AppPath::try_with_override("config.toml", env::var("MYAPP_CONFIG_DIR").ok())
    /// }
    /// ```
    ///
    /// ## Global Caching Behavior
    ///
    /// Once the application's base directory is successfully determined by either this method or [`AppPath::new()`],
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
    ///     let config_path = AppPath::try_with("config.toml")?;
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
    ///     let config = AppPath::try_with("config.toml")?;
    ///     let data = AppPath::try_with("data/app.db")?;
    ///     
    ///     // Initialize application with these paths
    ///     println!("Config: {}", config.path().display());
    ///     println!("Data: {}", data.path().display());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`AppPathError`] if the executable location cannot be determined:
    /// - [`AppPathError::ExecutableNotFound`] - `std::env::current_exe()` fails (extremely rare)
    /// - [`AppPathError::InvalidExecutablePath`] - Executable path is empty (system corruption)
    ///
    /// These errors represent unrecoverable system failures that occur at application startup.
    /// After the first successful call, the application's base directory is cached and this method
    /// will never return an error.
    #[inline]
    pub fn try_new() -> Result<Self, AppPathError> {
        let exe_dir = try_exe_dir()?;
        Ok(Self {
            full_path: exe_dir.to_path_buf(),
        })
    }

    /// Creates file paths relative to the application's base directory (fallible).
    ///
    /// **Use this only for libraries or specialized applications requiring explicit error handling.**
    /// Most applications should use [`Self::with()`] instead for simpler, cleaner code.
    ///
    /// ## When to Use
    ///
    /// **Use `try_with()` for:**
    /// - Reusable libraries that shouldn't panic
    /// - System tools with fallback strategies
    /// - Applications running in unusual environments
    ///
    /// **Use [`Self::with()`] for:**
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
    ///     let config_path = AppPath::try_with("config.toml")?;
    ///     // Load configuration...
    ///     Ok("config loaded".to_string())
    /// }
    ///
    /// // Better: Use override API for environment variables
    /// fn load_config_with_override() -> Result<String, AppPathError> {
    ///     let config_path = AppPath::try_with_override(
    ///         "config.toml",
    ///         std::env::var("APP_CONFIG").ok()
    ///     )?;
    ///     // Load configuration...
    ///     Ok("config loaded".to_string())
    /// }
    /// ```
    ///
    /// After the first successful call, the application's base directory is cached and this method
    /// will never return an error.
    #[inline]
    pub fn try_with(path: impl AsRef<Path>) -> Result<Self, AppPathError> {
        let exe_dir = try_exe_dir()?;
        let full_path = exe_dir.join(path);
        Ok(Self { full_path })
    }

    /// Creates an AppPath from an absolute path.
    ///
    /// This is an internal helper for operations that need to create AppPath
    /// instances from already-resolved absolute paths (like join, with_extension, etc).
    ///
    /// # Arguments
    ///
    /// * `path` - An absolute path that will be stored directly
    #[inline]
    pub(crate) fn from_absolute_path(path: impl Into<PathBuf>) -> Self {
        Self {
            full_path: path.into(),
        }
    }

    /// Creates file paths relative to the application's base directory.
    ///
    /// **This is the primary method for creating paths relative to your application's base directory.**
    /// It provides clean, idiomatic code for the 99% of applications that don't need explicit error handling.
    ///
    /// AppPath automatically resolves relative paths based on your application's base directory,
    /// making file access portable and predictable across different deployment scenarios.
    ///
    /// ## When to Use
    ///
    /// **Use `with()` for:**
    /// - Desktop, web, server, CLI applications (recommended)
    /// - When you want simple, clean code
    /// - When application location issues should halt the application
    ///
    /// **Use [`Self::try_with()`] for:**
    /// - Reusable libraries that shouldn't panic  
    /// - System tools with fallback strategies
    /// - Applications running in unusual environments
    ///
    /// ## Path Resolution
    ///
    /// - **Relative paths**: Resolved relative to application's base directory
    /// - **Absolute paths**: Used as-is (not recommended - defeats portability)
    /// - **Path separators**: Automatically normalized for current platform
    ///
    /// ## Global Caching Behavior
    ///
    /// The application's base directory is determined once on first call and cached globally.
    /// All subsequent calls use the cached value for maximum performance.
    ///
    /// # Panics
    ///
    /// Panics only if the application's base directory cannot be determined, which is extremely rare
    /// and typically indicates fundamental system issues (corrupted installation, permission problems).
    /// After the first successful call, this method never panics.
    ///
    /// # Arguments
    ///
    /// * `path` - A path that will be resolved according to AppPath's resolution strategy.
    ///   Accepts any type implementing [`AsRef<Path>`] (strings, Path, PathBuf, etc.).
    ///
    /// # Examples
    ///
    /// ## Basic Usage
    ///
    /// ```rust
    /// use app_path::AppPath;
    ///
    /// // Configuration file next to application
    /// let config = AppPath::with("config.toml");
    ///
    /// // Data directory relative to application
    /// let data_dir = AppPath::with("data");
    ///
    /// // Nested paths work naturally
    /// let user_profile = AppPath::with("data/users/profile.json");
    /// let log_file = AppPath::with("logs/app.log");
    /// ```
    ///
    /// ## Real Application Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    /// use std::fs;
    ///
    /// // Load configuration with fallback to defaults
    /// let config_path = AppPath::with("config.toml");
    /// let config = if config_path.exists() {
    ///     fs::read_to_string(&config_path)?
    /// } else {
    ///     "default_config = true".to_string() // Use defaults
    /// };
    ///
    /// // Set up application data directory
    /// let data_dir = AppPath::with("data");
    /// data_dir.create_dir()?; // Creates directory if needed
    ///
    /// // Prepare for log file creation
    /// let log_file = AppPath::with("logs/app.log");
    /// log_file.create_parents()?; // Ensures logs/ directory exists
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// ## Portable File Access
    ///
    /// ```rust
    /// use app_path::AppPath;
    /// use std::fs;
    ///
    /// // These paths work regardless of where the application is installed:
    /// // - C:\Program Files\MyApp\config.toml (Windows)
    /// // - /usr/local/bin/config.toml (Linux)
    /// // - /Applications/MyApp.app/Contents/MacOS/config.toml (macOS)
    /// // - .\myapp\config.toml (development)
    ///
    /// let settings = AppPath::with("settings.json");
    /// let cache = AppPath::with("cache");
    /// let templates = AppPath::with("templates/default.html");
    ///
    /// // Use with standard library functions
    /// if settings.exists() {
    ///     let content = fs::read_to_string(&settings)?;
    /// }
    /// cache.create_dir()?; // Creates cache/ directory
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn with(path: impl AsRef<Path>) -> Self {
        match Self::try_with(path) {
            Ok(app_path) => app_path,
            Err(e) => panic!("Failed to create AppPath: {e}"),
        }
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
    /// let config = AppPath::with_override(
    ///     "config.toml",
    ///     env::var("APP_CONFIG").ok()
    /// );
    ///
    /// // CLI argument override
    /// fn get_config(cli_override: Option<&str>) -> AppPath {
    ///     AppPath::with_override("config.toml", cli_override)
    /// }
    ///
    /// // Configuration file override
    /// struct Config {
    ///     data_dir: Option<String>,
    /// }
    ///
    /// let config = load_config();
    /// let data_dir = AppPath::with_override("data", config.data_dir.as_deref());
    /// # fn load_config() -> Config { Config { data_dir: None } }
    /// ```
    #[inline]
    pub fn with_override(
        default: impl AsRef<Path>,
        override_option: Option<impl AsRef<Path>>,
    ) -> Self {
        match override_option {
            Some(override_path) => Self::with(override_path),
            None => Self::with(default),
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
    /// let config = AppPath::with_override_fn("config.toml", || {
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
    /// let data_dir = AppPath::with_override_fn("data", || {
    ///     if env::var("DEVELOPMENT").is_ok() {
    ///         Some("dev_data".to_string())
    ///     } else {
    ///         None
    ///     }
    /// });
    /// ```
    #[inline]
    pub fn with_override_fn<P: AsRef<Path>>(
        default: impl AsRef<Path>,
        override_fn: impl FnOnce() -> Option<P>,
    ) -> Self {
        match override_fn() {
            Some(override_path) => Self::with(override_path),
            None => Self::with(default),
        }
    }

    /// Creates a path with override support (fallible).
    ///
    /// **Fallible version of [`Self::with_override()`].** Most applications should use the
    /// infallible version instead for cleaner code.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_path::{AppPath, AppPathError};
    /// use std::env;
    ///
    /// fn get_config() -> Result<AppPath, AppPathError> {
    ///     AppPath::try_with_override("config.toml", env::var("CONFIG").ok())
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
    ///     AppPath::try_with_override("config.toml", config_override.as_deref())
    /// }
    /// ```
    ///
    /// ## Error Propagation
    ///
    /// ```rust
    /// use app_path::{AppPath, AppPathError};
    ///
    /// fn setup_paths(config_override: Option<&str>) -> Result<(AppPath, AppPath), AppPathError> {
    ///     let config = AppPath::try_with_override("config.toml", config_override)?;
    ///     let data = AppPath::try_with_override("data", None::<&str>)?;
    ///     Ok((config, data))
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`AppPathError`] if the executable location cannot be determined:
    /// - [`AppPathError::ExecutableNotFound`] - `std::env::current_exe()` fails (extremely rare)
    /// - [`AppPathError::InvalidExecutablePath`] - Executable path is empty (system corruption)
    ///
    /// See [`AppPath::try_new()`] for detailed error conditions. After the first successful call
    /// to any AppPath method, this method will never return an error (uses cached result).
    #[inline]
    pub fn try_with_override(
        default: impl AsRef<Path>,
        override_option: Option<impl AsRef<Path>>,
    ) -> Result<Self, AppPathError> {
        match override_option {
            Some(override_path) => Self::try_with(override_path),
            None => Self::try_with(default),
        }
    }

    /// Creates a path with dynamic override support (fallible).
    ///
    /// This is the fallible version of [`AppPath::with_override_fn()`]. Use this method
    /// when you need explicit error handling combined with dynamic override logic.
    ///
    /// **Most applications should use [`AppPath::with_override_fn()`] instead** for
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
    ///     AppPath::try_with_override_fn("data", || {
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
    ///     AppPath::try_with_override_fn("logs/app.log", || {
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
    ///
    /// # Errors
    ///
    /// Returns [`AppPathError`] if the executable location cannot be determined:
    /// - [`AppPathError::ExecutableNotFound`] - `std::env::current_exe()` fails (extremely rare)  
    /// - [`AppPathError::InvalidExecutablePath`] - Executable path is empty (system corruption)
    ///
    /// See [`AppPath::try_new()`] for detailed error conditions. After the first successful call
    /// to any AppPath method, this method will never return an error (uses cached result).
    #[inline]
    pub fn try_with_override_fn<P: AsRef<Path>>(
        default: impl AsRef<Path>,
        override_fn: impl FnOnce() -> Option<P>,
    ) -> Result<Self, AppPathError> {
        match override_fn() {
            Some(override_path) => Self::try_with(override_path),
            None => Self::try_with(default),
        }
    }
}
