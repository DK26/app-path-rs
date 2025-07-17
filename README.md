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

// Environment override for deployment
let logs = app_path!("logs/app.log", env = "LOG_PATH");
// → Uses LOG_PATH if set, otherwise /path/to/exe/logs/app.log

// Acts like std::path::Path + creates directories
if !config.exists() {
    config.create_parents()?; // Creates parent directories
    std::fs::write(&config, "default config")?;
}
```

## Why Choose AppPath?

| Approach           | Problem                                                 | AppPath Solution                                 |
| ------------------ | ------------------------------------------------------- | ------------------------------------------------ |
| Hardcoded paths    | Breaks when moved                                       | ✅ Works anywhere                                 |
| `current_dir()`    | Depends on where user runs program                      | ✅ Always relative to executable                  |
| System directories | Scatters files across system                            | ✅ Self-contained, portable                       |
| `current_exe()`    | Manual path joining, no caching, verbose error handling | ✅ Clean API, automatic caching, ergonomic macros |

## Features

- 🚀 **Zero dependencies** - Only standard library
- ✨ **Ergonomic macro** - Clean syntax with `app_path!`
- 🌍 **Cross-platform** - Windows, Linux, macOS  
- ⚡ **High performance** - Static caching, minimal allocations
- 🔧 **Flexible deployment** - Environment overrides
- 🛡️ **Thread-safe** - Concurrent access safe
- 📦 **Portable** - Entire app moves as one unit

## API Overview

### The `app_path!` Macro (Recommended)

```rust
use app_path::app_path;

// Simple paths
let config = app_path!("config.toml");
let database = app_path!("data/users.db");

// Environment overrides
let logs = app_path!("logs/app.log", env = "LOG_PATH");
let cache = app_path!("cache", env = "CACHE_DIR");

// Custom override logic
let data_dir = app_path!("data", override = {
    std::env::var("DATA_DIR")
        .or_else(|_| std::env::var("XDG_DATA_HOME").map(|p| format!("{p}/myapp")))
        .ok()
});

// Function-based override (great for XDG support)
let config_dir = app_path!("config", fn = || {
    std::env::var("XDG_CONFIG_HOME")
        .or_else(|_| std::env::var("HOME").map(|h| format!("{h}/.config/myapp")))
        .ok()
});

// Variable capturing
let version = "1.0";
let versioned_cache = app_path!(format!("cache-{version}"));

// Directory creation
logs.create_parents()?;              // Creates logs/ for the file
app_path!("temp").create_dir()?;     // Creates temp/ directory itself
```

### Fallible `try_app_path!` Macro

```rust
use app_path::try_app_path;

let config = try_app_path!("config.toml")?;
let database = try_app_path!("data/users.db", env = "DATABASE_PATH")?;

// Error handling
match try_app_path!("logs/app.log") {
    Ok(log_path) => log_path.create_parents()?,
    Err(e) => eprintln!("Failed: {e}"),
}
```

### Constructor API

```rust
use app_path::AppPath;

let config = AppPath::new("config.toml");
let database = AppPath::with_override("data/users.db", std::env::var("DB_PATH").ok());
let config = AppPath::try_new("config.toml")?; // Fallible
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
    
    output.create_parents()?; // Creates output/ directory
    
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

## Error Handling

AppPath uses **fail-fast by default** for better developer experience:

- **`app_path!` and `AppPath::new()`** - Panic on critical system errors (executable location undetermined)
- **`try_app_path!` and `AppPath::try_new()`** - Return `Result` for explicit error handling

This design makes sense because if the system can't determine your executable location, there's usually no point continuing - it indicates severe system corruption or unsupported platforms.

**For most applications**: Use the panicking variants (`app_path!`) - they fail fast on unrecoverable errors.

**For libraries**: Use the fallible variants (`try_app_path!`) to let callers handle errors gracefully.

```rust
use app_path::{AppPath, AppPathError};

// Libraries should handle errors explicitly
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

## Ecosystem Integration

`app-path` integrates seamlessly with popular Rust path crates, letting you combine the best tools for your specific needs:

### 🔗 **Popular Path Crate Compatibility**

| Crate                                                   | Use Case                           | Integration Pattern                |
| ------------------------------------------------------- | ---------------------------------- | ---------------------------------- |
| **[`camino`](https://crates.io/crates/camino)**         | UTF-8 path guarantees for web apps | `Utf8PathBuf::from_path_buf(app_path.into())?` |
| **[`typed-path`](https://crates.io/crates/typed-path)** | Cross-platform type-safe paths     | `WindowsPath::new(app_path.to_bytes())` |

### 📝 **Real-World Integration Examples**

#### 🌐 **JSON-Safe Web Config** (with `camino`)
```rust
use app_path::app_path;
use camino::Utf8PathBuf;

let static_dir = app_path!("web/static", env = "STATIC_DIR");
let utf8_static = Utf8PathBuf::from_path_buf(static_dir.into())
    .map_err(|_| "Invalid UTF-8 path")?;
let config = serde_json::json!({ "static_files": utf8_static });
```

#### 🔨 **Cross-Platform Build System** (with `typed-path`)
```rust
use app_path::app_path;
use typed_path::{WindowsPath, UnixPath};

let dist_dir = app_path!("dist");
let path_bytes = dist_dir.to_bytes();
let win_path = WindowsPath::new(path_bytes);  // Uses \ on Windows
let unix_path = UnixPath::new(path_bytes);    // Uses / on Unix
```

#### ⚙️ **Configuration Files** (with `serde`)
```rust
use app_path::AppPath;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Config {
    log_file: String,    // Standard approach - readable and portable
    data_dir: String,    // Works across all platforms  
}

// Convert when using - clean separation of concerns
let config: Config = serde_json::from_str(&config_json)?;
let log_path = AppPath::new(&config.log_file);
let data_path = AppPath::new(&config.data_dir);
```

## Installation

```toml
[dependencies]
app-path = "0.2"
```

## Documentation

For comprehensive API documentation, examples, and guides, see [docs.rs/app-path](https://docs.rs/app-path).
