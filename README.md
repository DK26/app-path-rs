# AppPath

**Create paths relative to your executable for truly portable applications.**

[![Crates.io](https://img.shields.io/crates/v/app-path.svg)](https://crates.io/crates/app-path)
[![Documentation](https://docs.rs/app-path/badge.svg)](https://docs.rs/app-path)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

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
let config = AppPath::new("config.toml")?;
let templates = AppPath::new("templates")?;
let data = AppPath::new("data/users.db")?;

// Check if files exist, create directories, etc.
if !config.exists() {
    config.create_dir_all()?;
    std::fs::write(config.full(), "default config")?;
}
```

## 🚀 Why Choose AppPath?

### vs. Standard Library (`std::env::current_dir()`)
```rust
// ❌ Brittle - depends on where user runs the program
let config = std::env::current_dir()?.join("config.toml");

// ✅ Reliable - always relative to your executable
let config = AppPath::new("config.toml")?;
```

### vs. System Directories (`directories` crate)
```rust
// ❌ Scattered across the system
use directories::ProjectDirs;
let proj_dirs = ProjectDirs::from("com", "MyOrg", "MyApp").unwrap();
let config = proj_dirs.config_dir().join("config.toml"); // ~/.config/MyApp/config.toml

// ✅ Everything together with your app
let config = AppPath::new("config.toml")?; // ./config.toml (next to exe)
```

### vs. Manual Path Joining
```rust
// ❌ Verbose and error-prone
let exe_path = std::env::current_exe()?;
let exe_dir = exe_path.parent().ok_or("No parent")?;
let config = exe_dir.join("config.toml");

// ✅ Clean and simple
let config = AppPath::new("config.toml")?;
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
- ✅ **Simple API** - Just `AppPath::new()` and you're done
- ✅ **Full `Path` compatibility** - Implements `AsRef<Path>`, `Display`, etc.
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
    let config = AppPath::new("config.toml")?;
    let templates = AppPath::new("templates")?;
    let logs = AppPath::new("logs/app.log")?;
    
    // Use them like normal paths
    if config.exists() {
        let content = fs::read_to_string(config.full())?;
        println!("Config: {}", content);
    }
    
    // Create directories automatically
    logs.create_dir_all()?;
    fs::write(logs.full(), "Application started\n")?;
    
    println!("Config: {}", config);      // Displays full path
    println!("Templates: {}", templates);
    
    Ok(())
}
```

## 🏗️ Application Structure

Your portable application structure becomes:
```
myapp.exe          # Your executable
├── config.toml    # AppPath::new("config.toml")
├── templates/     # AppPath::new("templates")
│   ├── email.html
│   └── report.html
├── data/          # AppPath::new("data")
│   └── cache.db
└── logs/          # AppPath::new("logs")
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
        let config = AppPath::new("config.toml")
            .unwrap()
            .with_base(&temp);
        
        // Test with isolated temporary directory
        assert!(!config.exists());
    }
}
```

## 🔄 Migration Guide

### From hardcoded paths:
```rust
// Before
let config = PathBuf::from("config.toml");

// After  
let config = AppPath::new("config.toml")?;
```

### From current_exe() manual handling:
```rust
// Before
let exe = std::env::current_exe()?;
let exe_dir = exe.parent().unwrap();
let config = exe_dir.join("config.toml");

// After
let config = AppPath::new("config.toml")?;
```

## 📄 License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

---

**AppPath: Keep it simple, keep it together.** 🎯

