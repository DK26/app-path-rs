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
    let template = AppPath::new("templates").join(format!("{}.hbs", name));
    let output = AppPath::new("output").join("generated.html");
    
    // Ensure output directory exists
    output.create_dir_all()?;
    
    let template_content = std::fs::read_to_string(&template)?;
    let result = render_template(&template_content)?;
    std::fs::write(&output, result)?;
    
    println!("Generated: {}", output);
    Ok(())
}
```

### Logging Setup
```rust
use app_path::AppPath;

fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
    let log_file = AppPath::new("logs/app.log");
    log_file.create_dir_all()?; // Creates logs/ directory
    
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

// Production: uses default paths
// Development: override via environment variables
// Testing: override via CLI arguments

let config = AppPath::with_override(
    "config.toml",
    env::var("APP_CONFIG").ok()
);

let data_dir = AppPath::with_override_fn("data", || {
    env::var("DATA_DIR").ok()
        .or_else(|| env::var("TMPDIR").map(|tmp| format!("{}/myapp", tmp)))
});
```

## Path Resolution

- **Relative paths** → executable directory: `"config.toml"` becomes `./config.toml`
- **Absolute paths** → used as-is: `"/etc/app.conf"` stays `/etc/app.conf`

This enables both portable deployment and system integration.

## Error Handling

AppPath panics only on system-level failures (determining executable location). This is extremely rare and indicates unrecoverable system issues.

**For libraries requiring fallible behavior:**
```rust
use app_path::AppPath;

// Recommended: Use override API for environment variables
let config = AppPath::with_override(
    "config.toml",
    env::var("APP_CONFIG").ok()
);

// Alternative: Direct try_new() for unusual cases
let config = AppPath::try_new("config.toml")?;
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
