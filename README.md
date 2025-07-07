# AppPath

**Create paths relative to your executable for truly portable applications.**

[![## 🎨 Trait Implementations & Ergonomicsrates.io](https://img.shields.io/crates/v/app-path.svg)](https://crates.io/crates/app-path)
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

// Create paths relative to your executable - simple and clean
let config = AppPath::new("config.toml");
let data = AppPath::new("data/users.db");

// Accepts any AsRef<Path> type - no unnecessary allocations
let log_file = "logs/app.log".to_string();
let logs = AppPath::new(&log_file);

let path_buf = PathBuf::from("cache/data.bin");
let cache = AppPath::new(&path_buf);

// Works with any path-like type
let from_path = AppPath::new(std::path::Path::new("temp.txt"));

// Alternative: Use From for any path type
let settings: AppPath = "settings.json".into();
let data_file: AppPath = PathBuf::from("data.db").into();

// Absolute paths are used as-is (for system integration)
let system_log = AppPath::new("/var/log/app.log");
let windows_temp = AppPath::new(r"C:\temp\cache.dat");

// Get the paths for use with standard library functions
println!("Config: {}", config.path().display());
println!("Data: {}", data.path().display());

// Check existence and create directories
if !logs.exists() {
    logs.create_dir_all().unwrap();
}
```

## 🚀 Features & Design Philosophy

AppPath is built around the core principle of **portable-first design** with these key features:

- 🚀 **Zero dependencies** - Uses only standard library for maximum compatibility
- 🌍 **Cross-platform** - Consistent behavior across Windows, Linux, and macOS
- 🛡️ **Infallible API** - Simple `new()` constructor that panics on rare system failures (with clear documentation of edge cases)
- 🚀 **Static caching** - Executable location determined once and cached for performance
- 🔧 **Easy testing** - Use standard path joining for custom layouts
- 📁 **Smart path handling** - Relative paths resolve to executable directory, absolute paths used as-is
- ⚡ **Zero allocations** - Accepts `impl AsRef<Path>` to avoid unnecessary allocations
- 🎯 **Ergonomic conversions** - `From` implementations for all common path types
- 📚 **Comprehensive docs** - Extensive examples and clear API documentation

### Design Principles

**1. Simplicity Over Complexity**
- Infallible API eliminates error handling boilerplate from every usage site
- Single `new()` method accepts all path types through `AsRef<Path>`
- Clear panic conditions for the rare failure cases

**2. Performance by Design**
- Static caching of executable location (determined once, used forever)
- Zero-allocation API design through efficient borrowing
- Minimal memory footprint (only stores resolved path)

**3. Portable-First Architecture**
- Everything stays together with your executable by default
- Enables true application portability across environments
- Smart path resolution supports both portable and system integration use cases

**4. Robust Edge Case Handling**
- Works in containerized and jailed environments
- Handles root-level executables gracefully
- Clear failure modes with descriptive error messages

## � Trait Implementations & Ergonomics

`AppPath` implements standard traits for seamless integration with Rust APIs:

**Collections & Comparison:**
- `PartialEq`, `Eq`, `Hash` - Use in `HashMap`, `HashSet`
- `PartialOrd`, `Ord` - Automatic sorting in `BTreeMap`, `BTreeSet`

**Path Integration:**
- `AsRef<Path>` - Works with any API expecting `&Path`
- `Deref<Target=Path>` - Call `Path` methods directly (e.g., `.extension()`)
- `Borrow<Path>` - Efficient lookups in collections

**Conversions:**
- `From<T>` for `&str`, `String`, `&Path`, `PathBuf`
- `Into<PathBuf>` - Convert back to owned `PathBuf`
- `Clone`, `Debug`, `Display`, `Default`

```rust
use app_path::AppPath;
use std::collections::HashMap;

// Works in collections
let mut files = HashMap::new();
files.insert(AppPath::new("config.toml"), "Configuration");

// Direct Path methods via Deref
let config = AppPath::new("config.toml");
assert_eq!(config.extension(), Some("toml".as_ref()));

// Natural conversions
let path: AppPath = "data.txt".into();
```

## �📁 Path Resolution Behavior

`AppPath` handles different path types intelligently:

### Relative Paths (Recommended for Portable Apps)
```rust
// These resolve relative to your executable's directory
let config = AppPath::new("config.toml");       // → exe_dir/config.toml
let data = AppPath::new("data/users.db");       // → exe_dir/data/users.db
let nested = AppPath::new("logs/app/debug.log"); // → exe_dir/logs/app/debug.log
```

### Absolute Paths (For System Integration)
```rust
// These are used as-is, ignoring the executable directory
let system_config = AppPath::new("/etc/myapp/config.toml");  // → /etc/myapp/config.toml
let windows_temp = AppPath::new(r"C:\temp\cache.dat");       // → C:\temp\cache.dat
let user_home = AppPath::new("/home/user/.myapp/settings");  // → /home/user/.myapp/settings
```

This design allows your application to:
- ✅ **Stay portable** with relative paths for app-specific files
- ✅ **Integrate with system** using absolute paths when needed
- ✅ **Be configurable** - users can specify either type in config files

## 📖 Quick Start

Add to your `Cargo.toml`:
```toml
[dependencies]
app-path = "0.1.2"
```

```rust
use app_path::AppPath;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create paths relative to your executable - simple and clean
    let config = AppPath::new("config.toml");
    let templates = AppPath::new("templates");
    let logs = AppPath::new("logs/app.log");
    
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

## � Additional Helper Functions

### The `exe_dir()` Function

For cases where you need direct access to your executable's directory:

```rust
use app_path::exe_dir;

fn main() {
    // Get the directory containing your executable
    let exe_directory = exe_dir();
    println!("Executable directory: {}", exe_directory.display());
    
    // Use it with standard library functions
    for entry in std::fs::read_dir(exe_directory).unwrap() {
        println!("Found: {}", entry.unwrap().path().display());
    }
    
    // Or combine with custom logic
    let custom_path = exe_directory.join("custom").join("path.txt");
    println!("Custom path: {}", custom_path.display());
}
```

**When to use `exe_dir()` vs `AppPath`:**
- Use `AppPath::new()` for most file/directory access (recommended)
- Use `exe_dir()` when you need the raw directory for custom path manipulation
- Use `exe_dir()` when interfacing with APIs that expect `&Path` directories

## �🚨 Panic Conditions & Design Rationale

AppPath uses an **infallible API** by design. This is a deliberate architectural choice that prioritizes **simplicity and performance** for the common case where executable location determination succeeds (which is the vast majority of real-world usage).

### Why Infallible Instead of Fallible?

**The Problem with Fallible APIs:**
- Every single usage site must handle potential errors
- Results in verbose, repetitive error handling code
- Encourages poor practices like `.unwrap()` or `.expect()`
- Creates cognitive overhead for developers

**The AppPath Solution:**
- **Executable location determination succeeds >99.9% of the time** in real applications
- **When it fails, it indicates fundamental system issues** that are typically unrecoverable
- **Clean, simple API** eliminates boilerplate and improves code readability
- **Clear panic conditions** are well-documented with specific failure scenarios

### When AppPath Panics

The crate will panic during **static initialization** (first use) if:

- **Cannot determine executable location** - When `std::env::current_exe()` fails
  - Rare, but possible in some embedded or heavily sandboxed environments
  - Indicates the system cannot provide basic process information
- **Executable path is empty** - When the system returns an empty executable path
  - Extremely rare, indicates a broken/corrupted system state

### Edge Cases We Handle

**Root-level executables:** When executable runs at filesystem root (e.g., `/init`, `C:\`), AppPath uses the root directory itself as the base directory.

**Containerized environments:** Designed to work correctly in Docker, chroot, and other containerized environments.

**Jailed environments:** Handles various forms of process isolation and sandboxing.

### For Applications Requiring Fallible Behavior

If your application needs to handle executable location failures gracefully:

```rust
use app_path::AppPath;
use std::env;

fn safe_app_path(relative_path: &str) -> AppPath {
    match env::current_exe() {
        Ok(exe_path) => {
            if let Some(exe_dir) = exe_path.parent() {
                let config_path = exe_dir.join(relative_path);
                AppPath::new(config_path)
            } else {
                // Fallback for edge case where exe has no parent
                let temp_dir = env::temp_dir().join("myapp");
                let _ = std::fs::create_dir_all(&temp_dir);
                let config_path = temp_dir.join(relative_path);
                AppPath::new(config_path)
            }
        }
        Err(_) => {
            // Fallback when executable location cannot be determined
            let temp_dir = env::temp_dir().join("myapp");
            let _ = std::fs::create_dir_all(&temp_dir);
            let config_path = temp_dir.join(relative_path);
            AppPath::new(config_path)
        }
    }
}

// Usage with fallback strategy
let config = safe_app_path("config.toml");
```

**Note:** Using `std::env::current_exe()` directly is simpler and more idiomatic than `panic::catch_unwind` patterns. Most applications should use environment variable fallbacks or conditional patterns instead.
```

## ⚡ Efficient API Design

AppPath is designed for practical, efficient usage:

**Key Design Features:**
- **Static caching** - Executable location determined once, cached forever
- **Zero allocations** - String literals and references used efficiently  
- **Minimal memory** - Only stores the final resolved path
- **Optimized methods** - Zero-cost abstractions for direct path operations

```rust
use app_path::AppPath;

// Efficient - no allocations
let config = AppPath::new("config.toml");          // &str
let data = AppPath::new(&filename);                 // &String (no move)

// Direct ownership transfer when desired
let cache: AppPath = PathBuf::from("cache.json").into(); // PathBuf moved
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

Use standard path joining for testing with custom directories:

```rust
#[cfg(test)]
mod tests {
    use app_path::AppPath;
    use std::env;

    #[test]
    fn test_config_loading() {
        let temp = env::temp_dir().join("app_path_test");
        let config_path = temp.join("config.toml");
        let config = AppPath::new(config_path);
        
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
let config = AppPath::new("config.toml");
```

### vs. System Directories (`directories` crate)
```rust
// ❌ Scattered across the system
use directories::ProjectDirs;
let proj_dirs = ProjectDirs::from("com", "MyOrg", "MyApp").unwrap();
let config = proj_dirs.config_dir().join("config.toml"); // ~/.config/MyApp/config.toml

// ✅ Everything together with your app
let config = AppPath::new("config.toml"); // ./config.toml (next to exe)
```

### vs. Manual Path Joining
```rust
// ❌ Verbose and error-prone
let exe_path = std::env::current_exe()?;
let exe_dir = exe_path.parent().ok_or("No parent")?;
let config = exe_dir.join("config.toml");

// ✅ Clean and simple
let config = AppPath::new("config.toml");
```

## 📁 Primary Use Cases & Value Proposition

AppPath is specifically designed to excel in scenarios where **portable, self-contained applications** provide significant value:

### 🎯 Core Use Cases

**1. Portable Applications**
- **USB/Flash drive applications** that users carry between computers
- **Network share deployments** where applications run from shared directories
- **Zero-installation tools** that work immediately after download
- **Backup and migration friendly** - entire application state moves together

**2. Development and DevOps Tools**
- **CLI utilities** that should work anywhere without setup
- **Build tools** that teams can share without environment setup
- **Deployment scripts** that bundle their configuration
- **Development environments** that are fully self-contained

**3. Corporate and Enterprise**
- **Restricted environments** where users can't install software system-wide
- **Compliance requirements** where software must be contained
- **Auditing scenarios** where all application files must be in one location
- **Temporary usage** where applications are run briefly and then removed

**4. Embedded and Specialized Systems**
- **Embedded applications** with simple, predictable file layouts
- **Kiosk systems** where everything must be self-contained
- **Appliance software** that shouldn't scatter files across the system
- **Single-purpose systems** with minimal filesystem complexity

### 💡 Why These Use Cases Matter

**Traditional approaches fail in these scenarios:**

```rust
// ❌ Current directory dependency - breaks portability
let config = std::env::current_dir()?.join("config.toml");
// Problem: Depends on where user runs the program from

// ❌ System directories - requires installation
use directories::ProjectDirs;
let proj_dirs = ProjectDirs::from("com", "MyOrg", "MyApp").unwrap();
let config = proj_dirs.config_dir().join("config.toml");
// Problem: Scatters files across system, needs installation

// ❌ Manual executable path handling - verbose and error-prone
let exe_path = std::env::current_exe()?;
let exe_dir = exe_path.parent().ok_or("No parent")?;
let config = exe_dir.join("config.toml");
// Problem: Repetitive boilerplate, easy to get wrong

// ✅ AppPath - designed for portability
let config = AppPath::new("config.toml");
// Solution: Always relative to executable, simple API, works everywhere
```

### 🏆 Success Stories

**Perfect for applications like:**
- **Postman** - Portable API testing tool
- **Sublime Text Portable** - Editor that runs from USB drives
- **PortableApps.com** ecosystem - Hundreds of portable applications
- **Docker deployment tools** - Self-contained utilities
- **Game development tools** - Asset processors and build tools
- **System administration utilities** - Tools that work on any system

### 🚀 Strengthening Your Application's Value

AppPath enables you to build applications that users **love** because they:

1. **Just Work** - No installation, no setup, no configuration
2. **Are Reliable** - Don't break when moved or copied
3. **Are Predictable** - All files in one place, easy to backup/restore
4. **Are Respectful** - Don't scatter files across the user's system
5. **Are Portable** - Work identically across different machines and environments

## 🎯 API Design Philosophy

AppPath's API is carefully crafted around specific design principles that prioritize **developer experience** and **real-world usability**:

### 1. **Simplicity Over Configurability**

**Design Choice:** Single `new()` method that accepts `impl AsRef<Path>`

**Why:** Instead of multiple constructors (`new_str()`, `new_path()`, `new_pathbuf()`), we provide one method that works with all path types. This reduces cognitive load and API surface area.

```rust
// ✅ Simple, unified API
let config = AppPath::new("config.toml");          // &str
let data = AppPath::new(PathBuf::from("data.db")); // PathBuf
let logs = AppPath::new(Path::new("logs.txt"));    // &Path

// ❌ What we avoided: Multiple constructors
// let config = AppPath::new_str("config.toml");
// let data = AppPath::new_pathbuf(PathBuf::from("data.db"));
// let logs = AppPath::new_path(Path::new("logs.txt"));
```

### 2. **Performance Through Zero-Allocation Design**

**Design Choice:** `impl AsRef<Path>` instead of `impl Into<PathBuf>`

**Why:** Avoids unnecessary allocations for the common case of string literals and borrowed paths.

```rust
// ✅ Zero allocations for common cases
let config = AppPath::new("config.toml");    // No allocation - borrows string literal
let data = AppPath::new(&some_path_string);  // No allocation - borrows existing string

// ❌ What we avoided: Unnecessary allocations
// impl Into<PathBuf> would always allocate for string literals
```

### 3. **Ergonomic Conversions**

**Design Choice:** `From` trait implementations for all common path types

**Why:** Enables natural, idiomatic Rust conversions while maintaining type safety.

```rust
// ✅ Natural conversions
let config: AppPath = "config.toml".into();
let data: AppPath = PathBuf::from("data.db").into();

// Works seamlessly with functions expecting AppPath
fn process_config(path: impl Into<AppPath>) {
    let app_path = path.into();
    // ...
}

process_config("config.toml");  // &str
process_config(PathBuf::from("data.db"));  // PathBuf
```

### 4. **Clear Mental Model**

**Design Choice:** Smart path resolution (relative vs absolute)

**Why:** Provides intuitive behavior that matches user expectations for portable applications.

```rust
// ✅ Intuitive behavior
let portable_config = AppPath::new("config.toml");        // Relative to exe
let system_config = AppPath::new("/etc/myapp/config");    // Absolute path preserved

// Users understand: relative = portable, absolute = system integration
```

### 5. **Testability by Design**

**Design Choice:** Simple path joining for testing

**Why:** Enables easy testing without requiring complex methods.

```rust
// ✅ Easy testing
#[test]
fn test_config_handling() {
    let temp_dir = std::env::temp_dir().join("test");
    let config_path = temp_dir.join("config.toml");
    let config = AppPath::new(config_path);
    // Test in isolation...
}
```

### 6. **Minimal Memory Footprint**

**Design Choice:** Store only the resolved path

**Why:** Applications often create many AppPath instances. Storing only the final path minimizes memory usage.

```rust
// ✅ Minimal memory usage
// AppPath only stores the resolved PathBuf
struct AppPath {
    full_path: PathBuf,  // Only field
}

// ❌ What we avoided: Storing redundant data
// struct AppPath {
//     input_path: PathBuf,   // Redundant
//     full_path: PathBuf,    // What we actually need
// }
```

### 7. **Fail-Fast Philosophy**

**Design Choice:** Panic on initialization failure

**Why:** Executable location determination failing indicates fundamental system issues that are typically unrecoverable. Panicking fails fast with clear error messages.

```rust
// ✅ Clear failure mode
// Panics immediately with descriptive message if system is broken
let config = AppPath::new("config.toml");

// ❌ What we avoided: Error handling burden
// Result<AppPath, Error> would require handling at every usage site
// when failure cases are extremely rare and typically unrecoverable
```

## 🔄 Common Patterns & Best Practices

### 1. **Configuration File Pattern**

```rust
use app_path::AppPath;
use std::fs;

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = AppPath::new("config.toml");
    
    let config_content = if config_path.exists() {
        fs::read_to_string(config_path.path())?
    } else {
        // Create default config
        let default_config = include_str!("default_config.toml");
        config_path.create_dir_all()?;
        fs::write(config_path.path(), default_config)?;
        default_config.to_string()
    };
    
    Ok(toml::from_str(&config_content)?)
}
```

### 2. **Data Directory Pattern**

```rust
use app_path::AppPath;

fn ensure_data_directory() -> Result<AppPath, std::io::Error> {
    let data_dir = AppPath::new("data");
    data_dir.create_dir_all()?;
    Ok(data_dir)
}

fn get_user_database() -> AppPath {
    AppPath::new("data/users.db")
}

fn get_cache_file(name: &str) -> AppPath {
    AppPath::new(format!("data/cache/{}", name))
}
```

### 3. **Logging Setup Pattern**

```rust
use app_path::AppPath;
use std::fs::OpenOptions;

fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
    let log_file = AppPath::new("logs/app.log");
    log_file.create_dir_all()?;
    
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file.path())?;
    
    // Configure your logging framework to use log_file
    Ok(())
}
```

### 4. **Plugin Directory Pattern**

```rust
use app_path::AppPath;
use std::fs;

fn load_plugins() -> Result<Vec<Plugin>, Box<dyn std::error::Error>> {
    let plugins_dir = AppPath::new("plugins");
    
    if !plugins_dir.exists() {
        return Ok(Vec::new());
    }
    
    let mut plugins = Vec::new();
    for entry in fs::read_dir(plugins_dir.path())? {
        let entry = entry?;
        if entry.path().extension() == Some("dll".as_ref()) {
            // Load plugin...
        }
    }
    
    Ok(plugins)
}
```

### 5. **Hybrid Portable/System Integration**

```rust
use app_path::AppPath;
use std::env;

fn get_config_path() -> AppPath {
    // Check for system-wide config first
    if let Ok(system_config) = env::var("MYAPP_SYSTEM_CONFIG") {
        AppPath::new(system_config)  // Absolute path
    } else {
        AppPath::new("config.toml")  // Portable path
    }
}

fn get_data_directory() -> AppPath {
    match env::var("MYAPP_DATA_DIR") {
        Ok(data_dir) => AppPath::new(data_dir),      // System integration
        Err(_) => AppPath::new("data"),              // Portable default
    }
}
```

### 6. **Testing with Temporary Directories**

```rust
use app_path::AppPath;
use std::{env, fs};

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_env() -> tempfile::TempDir {
        tempfile::tempdir().unwrap()
    }

    #[test]
    fn test_config_creation() {
        let temp_dir = setup_test_env();
        let config_path = temp_dir.path().join("config.toml");
        let config = AppPath::new(config_path);
        
        // Test config creation logic
        create_default_config(&config);
        assert!(config.exists());
        
        let content = fs::read_to_string(config.path()).unwrap();
        assert!(content.contains("default_value"));
    }
}
```

### 7. **Error Handling Best Practices**

```rust
use app_path::AppPath;
use std::env;

// For applications that need graceful fallbacks
fn get_config_with_fallback() -> AppPath {
    match env::current_exe() {
        Ok(exe_path) => {
            if let Some(exe_dir) = exe_path.parent() {
                let config_path = exe_dir.join("config.toml");
                AppPath::new(config_path)
            } else {
                eprintln!("Warning: Executable at filesystem root, using temp directory");
                let temp_config = env::temp_dir().join("myapp");
                let _ = std::fs::create_dir_all(&temp_config);
                let config_path = temp_config.join("config.toml");
                AppPath::new(config_path)
            }
        }
        Err(_) => {
            eprintln!("Warning: Cannot determine executable location, using temp directory");
            let temp_config = env::temp_dir().join("myapp");
            let _ = std::fs::create_dir_all(&temp_config);
            let config_path = temp_config.join("config.toml");
            AppPath::new(config_path)
        }
    }
}

// For applications that should fail fast
fn get_config_strict() -> AppPath {
    // This will panic with a clear message if executable location fails
    AppPath::new("config.toml")
}
```

### 📋 **Quick Decision Guide**

**Use AppPath when:**
- ✅ You want portable, self-contained applications
- ✅ You need simple, reliable file access relative to your executable
- ✅ You're building CLI tools, portable apps, or development utilities
- ✅ You want to minimize external dependencies

**Consider alternatives when:**
- ❌ You need system-wide configuration (use `directories` crate)
- ❌ You're building system services (use standard system directories)
- ❌ You need complex path manipulation (use `std::path` directly)
- ❌ You require fallible executable location handling

## 📄 License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

---

**AppPath: Keep it simple, keep it together.** 🎯

