# AppPath

**Create paths relative to your executable for truly portable applications.**

[![Crates.io](https://img.shields.io/crates/v/app-path.svg)](https://crates.io/crates/app-path)
[![Documentation](https://docs.rs/app-path/badge.svg)](https://docs.rs/app-path)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![CI](https://github.com/DK26/app-path-rs/workflows/CI/badge.svg)](https://github.com/DK26/app-path-rs/actions)

## 🎯 The Problem

When building applications that need to access files (configs, templates, data), you typically have two choices:

1. **System directories** (`~/.config/`, `%APPDATA%`, etc.) - Great for installed apps, but...
   - Requires installation
   - Spreads files across the system
   - Hard to backup/move
   - Needs admin rights on some systems

2. **Hardcoded paths** - Simple but brittle and non-portable

## ✨ The Solution

**AppPath creates paths relative to your executable location**, enabling truly portable applications where everything stays together.

```rust
use app_path::AppPath;
use std::path::PathBuf;

// Create paths relative to your executable - accepts any path-like type
let config = AppPath::try_new("config.toml")?;
let data = AppPath::try_new("data/users.db")?;

// Efficient ownership transfer for owned types
let log_file = "logs/app.log".to_string();
let logs = AppPath::try_new(log_file)?; // String is moved

let path_buf = PathBuf::from("cache/data.bin");
let cache = AppPath::try_new(path_buf)?; // PathBuf is moved

// Works with any path-like type
let from_path = AppPath::try_new(std::path::Path::new("temp.txt"))?;

// Alternative: Use TryFrom for string types
let settings = AppPath::try_from("settings.json")?;

// Absolute paths are used as-is (for system integration)
let system_log = AppPath::try_new("/var/log/app.log")?;
let windows_temp = AppPath::try_new(r"C:\temp\cache.dat")?;

// Get the paths for use with standard library functions
println!("Config: {}", config.path().display());
println!("Data: {}", data.path().display());

// Check existence and create directories
if !logs.exists() {
    logs.create_dir_all()?;
}
```

## 🚀 Features

- 🚀 **Zero dependencies** - Uses only standard library
- 🌍 **Cross-platform** - Windows, Linux, macOS support
- 🛡️ **Safe API** - Uses `try_new()` following Rust conventions where `new()` implies infallible construction
- 🔧 **Easy testing** - Override base directory with `with_base()` method
- 📁 **Smart path handling** - Relative paths resolve to executable directory, absolute paths used as-is
- ⚡ **Efficient ownership** - Accepts any path-like type with optimal ownership transfer
- 🎯 **Ergonomic conversions** - `TryFrom` implementations for string types
- 📚 **Comprehensive docs** - Extensive examples and clear API documentation

## � Path Resolution Behavior

`AppPath` handles different path types intelligently:

### Relative Paths (Recommended for Portable Apps)
```rust
// These resolve relative to your executable's directory
let config = AppPath::try_new("config.toml")?;       // → exe_dir/config.toml
let data = AppPath::try_new("data/users.db")?;       // → exe_dir/data/users.db
let nested = AppPath::try_new("logs/app/debug.log")?; // → exe_dir/logs/app/debug.log
```

### Absolute Paths (For System Integration)
```rust
// These are used as-is, ignoring the executable directory
let system_config = AppPath::try_new("/etc/myapp/config.toml")?;  // → /etc/myapp/config.toml
let windows_temp = AppPath::try_new(r"C:\temp\cache.dat")?;       // → C:\temp\cache.dat
let user_home = AppPath::try_new("/home/user/.myapp/settings")?;  // → /home/user/.myapp/settings
```

This design allows your application to:
- ✅ **Stay portable** with relative paths for app-specific files
- ✅ **Integrate with system** using absolute paths when needed
- ✅ **Be configurable** - users can specify either type in config files

## 📖 Quick Start

Add to your `Cargo.toml`:
```toml
[dependencies]
app-path = "0.1"
```

```rust
use app_path::AppPath;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create paths relative to your executable
    let config = AppPath::try_new("config.toml")?;
    let templates = AppPath::try_new("templates")?;
    let logs = AppPath::try_new("logs/app.log")?;
    
    // Use them like normal paths
    if config.exists() {
        let content = fs::read_to_string(config.path())?;
        println!("Config: {}", content);
    }
    
    // Create directories automatically
    logs.create_dir_all()?;
    fs::write(logs.path(), "Application started\n")?;
    
    println!("Config: {}", config);      // Displays full path
    println!("Templates: {}", templates);
    
    Ok(())
}
```

## ⚠️ Error Handling

All AppPath constructors can fail if the executable location cannot be determined. While this is rare in normal applications, it should be handled gracefully:

```rust
use app_path::AppPath;

// Recommended: Handle errors explicitly
match AppPath::try_new("config.toml") {
    Ok(config) => {
        println!("Config: {}", config.path().display());
        // Use config.path() for file operations
    }
    Err(e) => {
        eprintln!("Cannot determine executable location: {}", e);
        // Fallback strategies:
        // 1. Use current directory: std::env::current_dir()
        // 2. Use temp directory: std::env::temp_dir()
        // 3. Exit gracefully: std::process::exit(1)
    }
}

// Alternative: Use ? operator with proper error propagation
fn setup_config() -> Result<AppPath, std::io::Error> {
    let config = AppPath::try_new("config.toml")?;
    Ok(config)
}
```

### When Errors Can Occur

- **Cannot determine executable location** - Rare, but possible in some embedded environments
- **Executable has no parent directory** - Extremely rare, when exe is at filesystem root

These are typically unrecoverable system-level issues. In normal desktop/server applications, `AppPath::try_new()` should not fail.
```

## 🔄 Ownership and Performance

AppPath accepts any path-like type with optimal ownership handling:

```rust
use app_path::AppPath;
use std::path::{Path, PathBuf};

// String literals (no allocation)
let config = AppPath::try_new("config.toml")?;

// Owned String (moves ownership, no clone)
let filename = "data.db".to_string();
let data = AppPath::try_new(filename)?; // filename is moved

// PathBuf (moves ownership, no clone)
let path_buf = PathBuf::from("logs/app.log");
let logs = AppPath::try_new(path_buf)?; // path_buf is moved

// Path reference (efficient conversion)
let path_ref = Path::new("cache.json");
let cache = AppPath::try_new(path_ref)?;

// TryFrom for ergonomic string conversions
let settings = AppPath::try_from("settings.toml")?;
let from_string = AppPath::try_from("db.sqlite".to_string())?;
```

## 🏗️ Application Structure

Your portable application structure becomes:
```
myapp.exe          # Your executable
├── config.toml    # AppPath::try_new("config.toml")
├── templates/     # AppPath::try_new("templates")
│   ├── email.html
│   └── report.html
├── data/          # AppPath::try_new("data")
│   └── cache.db
└── logs/          # AppPath::try_new("logs")
    └── app.log
```

## 🧪 Testing Support

Override the base directory for testing:

```rust
#[cfg(test)]
mod tests {
    use app_path::AppPath;
    use std::env;

    #[test]
    fn test_config_loading() {
        let temp = env::temp_dir().join("app_path_test");
        let config = AppPath::try_new("config.toml")
            .unwrap()
            .with_base(&temp);
        
        // Test with isolated temporary directory
        assert!(!config.exists());
    }
}
```

## 🎯 Why Choose AppPath?

### vs. Standard Library (`std::env::current_dir()`)
```rust
// ❌ Brittle - depends on where user runs the program
let config = std::env::current_dir()?.join("config.toml");

// ✅ Reliable - always relative to your executable
let config = AppPath::try_new("config.toml")?;
```

### vs. System Directories (`directories` crate)
```rust
// ❌ Scattered across the system
use directories::ProjectDirs;
let proj_dirs = ProjectDirs::from("com", "MyOrg", "MyApp").unwrap();
let config = proj_dirs.config_dir().join("config.toml"); // ~/.config/MyApp/config.toml

// ✅ Everything together with your app
let config = AppPath::try_new("config.toml")?; // ./config.toml (next to exe)
```

### vs. Manual Path Joining
```rust
// ❌ Verbose and error-prone
let exe_path = std::env::current_exe()?;
let exe_dir = exe_path.parent().ok_or("No parent")?;
let config = exe_dir.join("config.toml");

// ✅ Clean and simple
let config = AppPath::try_new("config.toml")?;
```

## 📁 Perfect For

- **Portable applications** that travel on USB drives
- **Development tools** that should work anywhere
- **Corporate environments** where you can't install software
- **Containerized applications** with predictable layouts
- **Embedded systems** with simple file structures
- **Quick prototypes** that need simple file access

## 🔄 Common Usage Patterns

### Replace hardcoded paths:
```rust
// Instead of brittle hardcoded paths
let config = PathBuf::from("config.toml");  // Depends on working directory

// Use AppPath for reliable, portable paths
let config = AppPath::try_new("config.toml")?;  // Always relative to executable
```

### Replace manual path construction:
```rust
// Instead of verbose manual construction
let exe = std::env::current_exe()?;
let exe_dir = exe.parent().unwrap();
let config = exe_dir.join("config.toml");

// Use AppPath for clean, simple code
let config = AppPath::try_new("config.toml")?;
```

## 📄 License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

---

**AppPath: Keep it simple, keep it together.** 🎯

