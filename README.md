# app-path

**Create portable applications that keep files together with the executable.**

[![Crates.io](https://img.shields.io/crates/v/app-path.svg)](https://crates.io/crates/app-path)
[![Documentation](https://docs.rs/app-path/badge.svg)](https://docs.rs/app-path)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![CI](https://github.com/DK26/app-path-rs/workflows/CI/badge.svg)](https://github.com/DK26/app-path-rs/actions)

Simple, zero-dependency library for creating portable applications where configuration, data, and executable stay together as a deployable unit.

```rust
use app_path::app_path;

// Files relative to your executable - not current directory!
let config = app_path!("config.toml");        // → /path/to/exe/config.toml
let database = app_path!("data/users.db");    // → /path/to/exe/data/users.db

// Acts just like std::path::Path
if config.exists() {
    let content = std::fs::read_to_string(&config)?;  // &config auto-derefs to &Path
}

// Environment override magic for deployment ✨
let logs = app_path!("logs/app.log", env = "LOG_PATH");
// → Uses LOG_PATH if set, otherwise /path/to/exe/logs/app.log
database.ensure_parent_dirs()?; // Creates data/ directory if it doesn't exist
```

## Why Choose AppPath?

| Approach           | Problem                            | AppPath Solution                |
| ------------------ | ---------------------------------- | ------------------------------- |
| `current_dir()`    | Depends on where user runs program | ✅ Always relative to executable |
| System directories | Scatters files across system       | ✅ Self-contained, portable      |
| Hardcoded paths    | Breaks when moved                  | ✅ Works anywhere                |

## API Overview

### The `app_path!` Macro (Recommended)

```rust
use app_path::app_path;

// Simple paths
let config = app_path!("config.toml");
// → /path/to/exe/config.toml
let database = app_path!("data/users.db");
// → /path/to/exe/data/users.db

// Environment variable overrides for deployment
let logs = app_path!("logs/app.log", env = "LOG_PATH");
// → Uses LOG_PATH if set, otherwise /path/to/exe/logs/app.log
let cache = app_path!("cache", env = "CACHE_DIR");
// → Uses CACHE_DIR if set, otherwise /path/to/exe/cache

// Custom override logic with block expression
let data_dir = app_path!("data", override = {
    std::env::var("DATA_DIR")
        .or_else(|_| std::env::var("XDG_DATA_HOME").map(|p| format!("{p}/myapp")))
        .ok()
});
// → Uses DATA_DIR, then XDG_DATA_HOME/myapp, finally /path/to/exe/data

// Function-based override (great for XDG support)
let config_dir = app_path!("config", fn = || {
    std::env::var("XDG_CONFIG_HOME")
        .or_else(|_| std::env::var("HOME").map(|h| format!("{h}/.config/myapp")))
        .ok()
});
// → /home/user/.config/myapp (Linux) or /path/to/exe/config (fallback)

// Simple override with optional value
let config = app_path!("config.toml", override = std::env::var("CONFIG_PATH").ok());
// → Uses CONFIG_PATH if set, otherwise /path/to/exe/config.toml

// Variable capturing in complex expressions
let version = "1.0";
let versioned_cache = app_path!(format!("cache-{version}"));
// → /path/to/exe/cache-1.0
let temp_with_env = app_path!(format!("temp-{version}"), env = "TEMP_DIR");
// → Uses TEMP_DIR if set, otherwise /path/to/exe/temp-1.0

// Directory creation with clear intent
logs.ensure_parent_dirs()?;              // Creates logs/ for the file
app_path!("temp").ensure_dir_exists()?;  // Creates temp/ directory itself
```

### Fallible `try_app_path!` Macro (Libraries)

For libraries or applications requiring explicit error handling:

```rust
use app_path::{try_app_path, AppPathError};

// Returns Result<AppPath, AppPathError> instead of panicking
let config = try_app_path!("config.toml")?;
// → Ok(/path/to/exe/config.toml) or Err(AppPathError)

let database = try_app_path!("data/users.db", env = "DATABASE_PATH")?;
// → Ok with DATABASE_PATH or default path, or Err(AppPathError)

// Variable capturing with error handling
let version = "1.0";
let versioned_cache = try_app_path!(format!("cache-{version}"))?;
// → Ok(/path/to/exe/cache-1.0) or Err(AppPathError)

let temp_with_env = try_app_path!(format!("temp-{version}"), env = "TEMP_DIR")?;
// → Ok with TEMP_DIR or default path, or Err(AppPathError)

// Same syntax, graceful error handling
match try_app_path!("logs/app.log") {
    Ok(log_path) => log_path.ensure_parent_dirs()?,
    Err(e) => eprintln!("Failed to determine log path: {e}"),
}
// → Either creates logs/ directory or prints error message
```

### Constructor API (Alternative)

```rust
use app_path::AppPath;

let config = AppPath::new("config.toml");
// → /path/to/exe/config.toml (panics on system failure)

let database = AppPath::with_override("data/users.db", std::env::var("DB_PATH").ok());
// → Uses DB_PATH if set, otherwise /path/to/exe/data/users.db

// For libraries requiring fallible behavior
let config = AppPath::try_new("config.toml")?;
// → Ok(/path/to/exe/config.toml) or Err(AppPathError)
```

## Real-World Examples

### Configuration Management
```rust
use app_path::app_path;

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = app_path!("config.toml", env = "CONFIG_PATH");
    
    if !config_path.exists() {
        std::fs::write(&config_path, include_str!("default_config.toml"))?;
    }
    
    let content = std::fs::read_to_string(&config_path)?;
    Ok(toml::from_str(&content)?)
}
```

### CLI Tool with File Management
```rust
use app_path::app_path;

fn process_templates(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let template = app_path!("templates").join(format!("{name}.hbs"));
    let output = app_path!("output", env = "OUTPUT_DIR").join("result.html");
    
    output.ensure_parent_dirs()?; // Creates output/ directory
    
    let content = std::fs::read_to_string(&template)?;
    std::fs::write(&output, render_template(&content)?)?;
    Ok(())
}
```

### Deployment Flexibility
```rust
use app_path::app_path;

// Same binary, different environments:
// Development: uses "./config/app.toml"
// Production: CONFIG_PATH="/etc/myapp/config.toml" overrides to absolute path
let config = app_path!("config/app.toml", env = "CONFIG_PATH");

// Conditional deployment paths
let logs = if cfg!(debug_assertions) {
    app_path!("debug.log")
} else {
    app_path!("logs/production.log", env = "LOG_FILE")
};
```

## Directory Creation

AppPath provides intuitive methods with clear intent:

- **`ensure_parent_dirs()`** - Creates parent directories for file paths
- **`ensure_dir_exists()`** - Creates the path as a directory

```rust
use app_path::app_path;

// Preparing to write files
let log_file = app_path!("logs/app.log");
log_file.ensure_parent_dirs()?; // Creates logs/ directory
std::fs::write(&log_file, "Starting app...")?;

// Creating directories
let cache_dir = app_path!("cache");
cache_dir.ensure_dir_exists()?; // Creates cache/ directory
```

## Path Resolution

- **Relative paths** → executable directory: `"config.toml"` → `./config.toml`
- **Absolute paths** → used as-is: `"/etc/app.conf"` → `/etc/app.conf`
- **Environment overrides** → replace default when set

## Error Handling

AppPath panics only on extremely rare system failures (executable location undetermined). For libraries requiring explicit error handling:

```rust
use app_path::{AppPath, AppPathError};

match AppPath::try_new("config.toml") {
    Ok(path) => println!("Config: {}", path.display()),
    Err(AppPathError::ExecutableNotFound(msg)) => {
        eprintln!("Cannot find executable: {msg}");
    }
    Err(AppPathError::InvalidExecutablePath(msg)) => {
        eprintln!("Invalid executable path: {msg}");
    }
}
```

## Features

- 🚀 **Zero dependencies** - Only standard library
- ✨ **Ergonomic macro** - Clean syntax with `app_path!`
- 🌍 **Cross-platform** - Windows, Linux, macOS  
- ⚡ **High performance** - Static caching, minimal allocations
- 🔧 **Flexible deployment** - Environment overrides
- 🛡️ **Thread-safe** - Concurrent access safe
- 📦 **Portable** - Entire app moves as one unit

## Installation

```toml
[dependencies]
app-path = "0.2"
```

For comprehensive API documentation, see [docs.rs/app-path](https://docs.rs/app-path).
