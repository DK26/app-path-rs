# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.1] - 2025-07-21

### 🧹 **Maintenance**

- **Removed deprecated `.path()` method** - Completed deprecation cycle started in v0.2.7, method fully removed from codebase
- **Updated tests** - Migrated all test code from deprecated `.path()` to modern deref patterns (`&app_path`)
- **Improved documentation examples** - Corrected and clarified examples throughout the codebase

### 📚 **Documentation**

- **Enhanced code examples** - Better clarity and accuracy in documentation examples
- **Test suite cleanup** - Ensured all tests use current API patterns without deprecated methods

## [1.0.0] - 2025-07-20

### 🎉 **STABLE RELEASE** - Production Ready API

### 🚀 **New Features**

- **Complete Constructor API Redesign** - Separated concerns with `new()` for application base directory and `with(path)` for relative paths
- **New `AppPath::new()` constructor** - Returns the application base directory itself (no path argument)
- **New `AppPath::with(path)` method** - Primary API for creating paths relative to application base directory  
- **New `AppPath::try_new()` constructor** - Fallible version for getting application base directory
- **New `AppPath::try_with(path)` method** - Fallible version for creating relative paths
- **Low-level Path Operations** - `to_bytes()` and `into_bytes()` methods for platform-specific byte representation
- **Enhanced Path Conversion** - `into_path_buf()` and `into_inner()` methods for cleaner owned PathBuf extraction

### 📚 **Documentation & Quality**

- **Complete documentation overhaul** - Reorganized API documentation with clear categorization and practical examples  
- **Comprehensive test suite** - Independent verification eliminating circular dependencies
- **CI improvements** - Enhanced pipeline with MSRV compatibility checks

### 🔧 **Breaking Changes**

- **Constructor API completely redesigned** - `AppPath::new(path)` split into `AppPath::new()` (base directory) and `AppPath::with(path)` (relative paths)
- **Removed old `AppPath::new(path)` constructor** - Use `AppPath::with(path)` instead for creating relative paths
- **Removed old `AppPath::try_new(path)` constructor** - Use `AppPath::try_with(path)` instead for creating relative paths
- **Removed `exe_dir()` function from public API** - Use `AppPath::new()` instead to get application base directory

## [0.2.7] - 2025-07-16

### Deprecated
- `.path()` method - Use `&app_path` or `app_path.as_ref()` instead (all `Path` methods are directly available)

### Changed
- Improved performance and code organization
- Cleaner API with elimination of redundant methods

## [0.2.6] - 2025-07-16

### Fixed
- Removed false third-party crate integration examples from documentation

### Improved
- Cleaned up unused generic parameters in override methods
- Better documentation structure and clarity

## [0.2.5] - 2025-07-14

### Changed
- Directory creation methods now return `AppPathError` instead of `std::io::Error` for consistent error handling

### Enhanced
- Added comprehensive error documentation to all fallible APIs
- Added ecosystem integration guide with popular Rust path crates

## [0.2.4] - 2025-07-13

### Added
- New directory creation methods: `create_parents()` and `create_dir()` for clearer intent

### Deprecated
- Old directory creation methods: `ensure_parent_dirs()` → `create_parents()`, `ensure_dir_exists()` → `create_dir()`

### Improved
- Enhanced CI pipeline with auto-fix capabilities
- Refactored module organization for better maintainability

## [0.2.3] - 2025-01-24

### Added
- New `try_app_path!` macro for error handling scenarios (returns `Result` instead of panicking)
- Complete macro coverage with four syntax variants for both `app_path!` and `try_app_path!`

### Enhanced
- Comprehensive documentation updates with practical examples
- Cross-platform CI tooling with new `ci-local.sh` script
- Updated CONTRIBUTING.md with modern development workflow

### Fixed
- Corrected XDG environment variable test logic for better reliability

## [0.2.2] - 2025-07-10

### Added
- New directory creation methods: `ensure_parent_dirs()` and `ensure_dir_exists()` for clearer intent

### Deprecated
- `create_dir_all()` method in favor of more explicit methods

### Fixed
- Cross-platform test compatibility for Windows-style path handling

## [0.2.1] - 2025-07-08

### Added
- Complete fallible API: `try_new()` and `try_exe_dir()` for library use cases
- Advanced override API: `with_override()`, `with_override_fn()` methods for flexible deployment
- Complete trait ecosystem: `Default`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`, `Deref<Target=Path>`, `Borrow<Path>`
- Path manipulation methods: `join()`, `parent()`, `with_extension()`, file info methods
- Convenience `app_path!` macro for ergonomic path creation
- `AppPathError` with proper `std::error::Error` implementation

### Enhanced
- Simplified override method names (removed `new_` prefix)
- Complete documentation overhaul with practical examples
- Better override guidance prioritizing purpose-built methods

### Performance
- Static caching with proper thread safety
- Zero-allocation optimizations throughout the API

## [0.2.0] - 2025-07-07

### BREAKING CHANGES
- **Replaced `try_new()` with infallible `new()`** - Constructor now panics on system failure instead of returning `Result`
- **Removed `input()` method and `input_path` field** - No longer stores original input path
- **Replaced `TryFrom` with `From` trait implementations** - Conversions are now infallible 
- **Removed `AppPath::with_base()` method** - Use standard `Path::join()` for custom directories
- **Changed constructor parameter** - Now accepts `impl AsRef<Path>` instead of `impl Into<PathBuf>`

### Added
- Infallible `new()` constructor and fallible `try_new()` alternative
- Static executable directory caching using `OnceLock`
- Comprehensive trait implementations: `Default`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`, `Deref<Target=Path>`, `Borrow<Path>`
- `AppPathError` enum with descriptive error messages

### Enhanced
- Simplified API design focused on "paths relative to executable"
- Zero-allocation optimization with `#[inline]` attributes
- Better error handling examples with practical fallback patterns

### Fixed
- **MSRV Compatibility** - Replaced `std::sync::LazyLock` with `std::sync::OnceLock` for stable Rust support (≥1.70)

## [0.1.2] - 2025-07-06

### Added
- Generic `impl Into<PathBuf>` parameter for `try_new()` supporting any path-like type
- Smart path resolution: relative paths resolve to executable directory, absolute paths used as-is
- Ownership transfer optimization for `String` and `PathBuf` types

### Enhanced
- Complete documentation improvements across all files
- Enhanced examples showing different path types and ownership patterns

## [0.1.1] - 2025-07-05

### Added
- Initial stable release of `app-path` crate
- `AppPath::try_new()` - Create paths relative to executable location
- `AppPath::with_base()` - Override base directory for testing  
- `AppPath::path()` - Get the full resolved path
- `AppPath::input()` - Get the original input path before resolution
- `AppPath::exists()` - Check if the path exists
- `AppPath::create_dir_all()` - Create parent directories if needed
- `TryFrom<&str>`, `TryFrom<String>`, and `TryFrom<&String>` implementations
- `Display`, `From<AppPath>`, and `AsRef<Path>` trait implementations
- Zero dependencies - uses only standard library
- Cross-platform support (Windows, Linux, macOS)

## [0.1.0] - 2025-07-05

### Added  
- Initial release (yanked - replaced by 0.1.1 with improved API)

[Unreleased]: https://github.com/DK26/app-path-rs/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/DK26/app-path-rs/compare/v0.2.7...v1.0.0
[0.2.7]: https://github.com/DK26/app-path-rs/compare/v0.2.4...v0.2.7
[0.2.4]: https://github.com/DK26/app-path-rs/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/DK26/app-path-rs/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/DK26/app-path-rs/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/DK26/app-path-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/DK26/app-path-rs/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/DK26/app-path-rs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/DK26/app-path-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/DK26/app-path-rs/releases/tag/v0.1.0