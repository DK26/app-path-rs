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

// These paths are automatically relative to your executable
let config = AppPath::try_new("config.toml")?;
let templates = AppPath::try_new("templates")?;
let data = AppPath::try_new("data/users.db")?;

// Check if files exist, create directories, etc.
if !config.exists() {
    config.create_dir_all()?;
    std::fs::write(config.path(), "default config")?;
}
```

## 🚀 Why Choose AppPath?

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

## 🛠️ Features

- ✅ **Zero dependencies** - Lightweight and fast
- ✅ **Cross-platform** - Works on Windows, Linux, macOS
- ✅ **Simple API** - Just `AppPath::try_new()` and you're done
- ✅ **Full `Path` compatibility** - Implements `AsRef<Path>`, `Display`, etc.
- ✅ **Ergonomic conversions** - `TryFrom<&str>`, `TryFrom<String>` support
- ✅ **Testing support** - Override base directory with `with_base()`
- ✅ **Directory creation** - Built-in `create_dir_all()`
- ✅ **Well tested** - Comprehensive test suite

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

### Alternative Creation Methods

For ergonomic conversions from strings, use `TryFrom`:

```rust
use app_path::AppPath;
use std::convert::TryFrom;

// Primary constructor - clear that it can fail
let config = AppPath::try_new("config.toml")?;

// From string literals using TryFrom
let data = AppPath::try_from("data.db")?;

// From String values
let filename = "cache.json".to_string();
let cache = AppPath::try_from(filename)?;

// From String references  
let path_string = "logs/app.log".to_string();
let logs = AppPath::try_from(&path_string)?;

// All methods give you the same functionality
assert_eq!(config.input(), std::path::Path::new("config.toml"));
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

