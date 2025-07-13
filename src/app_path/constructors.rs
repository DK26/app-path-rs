use std::path::Path;

use crate::{try_exe_dir, AppPath, AppPathError};

impl AppPath {
    /// Creates file paths relative to the executable location.
    ///
    /// **This is the primary method for creating AppPath instances.** It provides clean,
    /// idiomatic code for the 99% of applications that don't need explicit error handling.
    ///
    /// AppPath automatically resolves relative paths based on your executable's location,
    /// making file access portable and predictable across different deployment scenarios.
    ///
    /// ## When to Use
    ///
    /// **Use `new()` for:**
    /// - Desktop, web, server, CLI applications (recommended)
    /// - When you want simple, clean code
    /// - When executable location issues should halt the application
    ///
    /// **Use [`Self::try_new()`] for:**
    /// - Reusable libraries that shouldn't panic  
    /// - System tools with fallback strategies
    /// - Applications running in unusual environments
    ///
    /// ## Path Resolution
    ///
    /// - **Relative paths**: Resolved relative to executable directory
    /// - **Absolute paths**: Used as-is (not recommended - defeats portability)
    /// - **Path separators**: Automatically normalized for current platform
    ///
    /// ## Global Caching Behavior
    ///
    /// The executable directory is determined once on first call and cached globally.
    /// All subsequent calls use the cached value for maximum performance.
    ///
    /// # Panics
    ///
    /// Panics only if the executable location cannot be determined, which is extremely rare
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
    /// // Configuration file next to executable
    /// let config = AppPath::new("config.toml");
    ///
    /// // Data directory relative to executable
    /// let data_dir = AppPath::new("data");
    ///
    /// // Nested paths work naturally
    /// let user_profile = AppPath::new("data/users/profile.json");
    /// let log_file = AppPath::new("logs/app.log");
    /// ```
    ///
    /// ## Real Application Examples
    ///
    /// ```rust
    /// use app_path::AppPath;
    /// use std::fs;
    ///
    /// // Load configuration with fallback to defaults
    /// let config_path = AppPath::new("config.toml");
    /// let config = if config_path.exists() {
    ///     fs::read_to_string(config_path.path())?
    /// } else {
    ///     "default_config = true".to_string() // Use defaults
    /// };
    ///
    /// // Set up application data directory
    /// let data_dir = AppPath::new("data");
    /// data_dir.create_dir()?; // Creates directory if needed
    ///
    /// // Prepare for log file creation
    /// let log_file = AppPath::new("logs/app.log");
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
    /// // These paths work regardless of where the executable is installed:
    /// // - C:\Program Files\MyApp\config.toml (Windows)
    /// // - /usr/local/bin/config.toml (Linux)
    /// // - /Applications/MyApp.app/Contents/MacOS/config.toml (macOS)
    /// // - .\myapp\config.toml (development)
    ///
    /// let settings = AppPath::new("settings.json");
    /// let cache = AppPath::new("cache");
    /// let templates = AppPath::new("templates/default.html");
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
    pub fn new(path: impl AsRef<Path>) -> Self {
        match Self::try_new(path) {
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
    ///     let config_path = AppPath::try_new("config.toml")?;
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
    #[inline]
    pub fn try_new(path: impl AsRef<Path>) -> Result<Self, AppPathError> {
        let exe_dir = try_exe_dir()?;
        let full_path = exe_dir.join(path);
        Ok(Self { full_path })
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
    pub fn with_override_fn<F, P>(default: impl AsRef<Path>, override_fn: F) -> Self
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
    #[inline]
    pub fn try_with_override(
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
    #[inline]
    pub fn try_with_override_fn<F, P>(
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
