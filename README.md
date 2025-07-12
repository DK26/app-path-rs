# app-path

**Create portable applications that keep files together with the executable.**

[![Crates.io](https://img.shields.io/crates/v/app-path.svg)](https://crates.io/crates/app-path)
[![Documentation](https://docs.rs/app-path/badge.svg)](https://docs.rs/app-path)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![CI](https://github.com/DK26/app-path-rs/workflows/CI/badge.svg)](https://github.com/DK26/app-path-rs/actions)

Simple, zero-dependency library for creating portable applications where configuration, data, and executable stay together as a deployable unit.

```rust
use app_path::AppPath;

// All files relative to your executable
let config = AppPath::new("config.toml");
let database = AppPath::new("data/users.db");

// Works like standard paths
if config.exists() {
    let content = std::fs::read_to_string(&config)?;
}

// Create directories with clear intent
database.ensure_parent_dirs()?; // Creates data/ directory for the file
```

## Quick Start

```rust
use app_path::{AppPath, app_path};

// Files relative to your executable
let config = AppPath::new("config.toml");
let database = AppPath::new("data/users.db");
let logs = AppPath::new("logs/app.log");

// Or use the convenient macro
let config_alt = app_path!("config.toml");
let database_alt = app_path!("data/users.db");

// Environment variable overrides for deployment flexibility
let config_deploy = AppPath::with_override("config.toml", std::env::var("CONFIG_PATH").ok());
let database_deploy = app_path!("data/users.db", env = "DATABASE_PATH");
let logs_deploy = app_path!("logs/app.log", override = std::env::var("LOG_DIR").ok());

// Works like standard paths
if config.exists() {
    let content = std::fs::read_to_string(&config)?;
}

// Create directories with clear intent
logs.ensure_parent_dirs()?; // Creates logs/ directory for the file
database.ensure_parent_dirs()?; // Creates data/ directory for the file

// Create directories themselves
let cache_dir = AppPath::new("cache");
let temp_dir = AppPath::new("temp");
cache_dir.ensure_dir_exists()?; // Creates cache/ directory
temp_dir.ensure_dir_exists()?; // Creates temp/ directory
```

## Why Choose AppPath?

| Approach           | Problem                            | AppPath Solution                |
| ------------------ | ---------------------------------- | ------------------------------- |
| `current_dir()`    | Depends on where user runs program | ✅ Always relative to executable |
| System directories | Scatters files across system       | ✅ Self-contained, portable      |
| Hardcoded paths    | Breaks when moved                  | ✅ Works anywhere                |

## Real-World Examples

### Configuration Management
```rust
use app_path::AppPath;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    database_url: String,
    log_level: String,
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = AppPath::new("config.toml");
    
    if !config_path.exists() {
        // Create default config file
        std::fs::write(&config_path, include_str!("default_config.toml"))?;
    }
    
    let content = std::fs::read_to_string(&config_path)?;
    Ok(toml::from_str(&content)?)
}
```

### CLI Tool with Templates
```rust
use app_path::AppPath;

fn generate_from_template(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let template = AppPath::new("templates").join(format!("{name}.hbs"));
    let output = AppPath::new("output").join("generated.html");
    
    // Ensure output directory exists
    output.ensure_parent_dirs()?; // Creates output/ directory (parent of file)
    
    let template_content = std::fs::read_to_string(&template)?;
    let result = render_template(&template_content)?;
    std::fs::write(&output, result)?;
    
    println!("Generated: {output}");
    Ok(())
}
```

### Logging Setup
```rust
use app_path::AppPath;

fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
    let log_file = AppPath::new("logs/app.log");
    log_file.ensure_parent_dirs()?; // Creates logs/ directory
    
    // Use with any logging framework
    let file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file)?;
    
    // Setup your logger with this file...
    Ok(())
}
```

## Override API for Flexible Deployment

Deploy the same binary with different configurations:

```rust
use app_path::AppPath;
use std::env;

// Use environment variable if available, fallback to default
let config = AppPath::with_override(
    "config.toml",
    env::var("APP_CONFIG").ok()
);

// Support multiple environment variables with priority
let data_dir = AppPath::with_override_fn("data", || {
    env::var("DATA_DIR")
        .or_else(|_| env::var("XDG_DATA_HOME"))
        .ok()
});

// Example usage in different environments:
// Production:   APP_CONFIG not set → uses "./config.toml"
// Development:  APP_CONFIG="/dev/config.toml" → uses absolute path
// CI/Testing:   DATA_DIR="/tmp/test-data" → uses custom location
```

## Path Resolution

- **Relative paths** → executable directory: `"config.toml"` becomes `./config.toml`
- **Absolute paths** → used as-is: `"/etc/app.conf"` stays `/etc/app.conf`

This enables both portable deployment and system integration.

## Directory Creation

AppPath provides clear, intuitive methods for directory creation:

```rust
use app_path::AppPath;

// For files: create parent directories
let config_file = AppPath::new("config/app.toml");
config_file.ensure_parent_dirs()?; // Creates config/ directory
std::fs::write(&config_file, "key = value")?;

// For directories: create the directory itself
let cache_dir = AppPath::new("cache");
cache_dir.ensure_dir_exists()?; // Creates cache/ directory
let data_file = cache_dir.join("data.json");
```

**Method Guide:**
- `ensure_parent_dirs()` - Creates parent directories for file paths
- `ensure_dir_exists()` - Creates the path as a directory

*Note: `create_dir_all()` is deprecated. Use the methods above for clearer intent.*

## Ergonomic Macro

The `app_path!` macro provides clean syntax for common patterns:

```rust
use app_path::app_path;

// Simple paths
let config = app_path!("config.toml");
let data = app_path!("data/users.db");

// Environment variable override
let config = app_path!("config.toml", env = "APP_CONFIG");

// Custom override
let logs = app_path!("logs", override = custom_log_dir);
```

## Error Handling

AppPath panics only on system-level failures (determining executable location). This is extremely rare and indicates unrecoverable system issues.

**For libraries requiring fallible behavior:**
```rust
use app_path::AppPath;

let config = AppPath::try_new("config.toml")?;

let config = AppPath::try_with_override(
    "config.toml",
    env::var("APP_CONFIG").ok()
)?;
```

### Fallible API (For Libraries)
```rust
use app_path::{AppPath, AppPathError};

// Handle potential errors explicitly
let config = AppPath::try_new("config.toml")?;

match AppPath::try_new("config.toml") {
    Ok(path) => println!("Config: {}", path.display()),
    Err(AppPathError::ExecutableNotFound(msg)) => {
        eprintln!("Cannot find executable: {}", msg);
    }
    Err(AppPathError::InvalidExecutablePath(msg)) => {
        eprintln!("Invalid executable path: {}", msg);
    }
}
```

## Features

- 🚀 **Zero dependencies** - Only standard library
- 🌍 **Cross-platform** - Windows, Linux, macOS  
- ⚡ **High performance** - Static caching, minimal allocations
- 🔧 **Ergonomic** - Works with all path types
- 🛡️ **Thread-safe** - Concurrent access safe
- 📦 **Portable** - Entire app moves as one unit

## Installation

```toml
[dependencies]
app-path = "0.2"
```

For more examples and API documentation, see [docs.rs/app-path](https://docs.rs/app-path).
