# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.3] - 2025-01-24

### Added
- **Fallible macro variant** - New `try_app_path!` macro for error handling scenarios
  - Returns `Result<AppPath, AppPathError>` instead of panicking
  - Same four syntax variants as `app_path!` macro
  - Perfect for library code and applications requiring graceful error handling
- **Complete macro coverage** - All macro variants support the same four syntax forms
  - Direct value: `app_path!("com.example.app")` and `try_app_path!("com.example.app")`
  - Environment override: `app_path!("com.example.app", env_var)` and `try_app_path!("com.example.app", env_var)`
  - Optional override: `app_path!("com.example.app", option_value)` and `try_app_path!("com.example.app", option_value)`
  - Function-based override: `app_path_fn!(|| "dynamic")` and `try_app_path_fn!(|| "dynamic")`
  - Essential for closures, async blocks, and complex control flow scenarios
  - Maintains API symmetry and completeness between panicking and fallible variants

### Enhanced
- **Documentation completeness** - Comprehensive updates to all documentation
  - Added clear output examples throughout README.md showing exact generated paths
  - Enhanced Quick Start section with → comments demonstrating actual results
  - Updated all macro documentation with practical examples and expected outputs
  - Added examples for all `_fn` macro variants throughout README.md
  - Corrected documentation about override parameter availability in macros
  - Updated API design section with complete coverage of all variants
  - All 142 tests (88 unit + 54 doc tests) pass with comprehensive coverage
- **Cross-platform CI tooling** - New `ci-local.sh` script for local development
  - Supports Windows (PowerShell/Git Bash), macOS, and Linux
  - Runs format, clippy, compile, test, and documentation checks
  - Auto-detects cargo installation and provides clear error reporting
- **Developer experience** - Updated CONTRIBUTING.md with modern development workflow
  - Clear setup instructions for all platforms
  - Integration with new CI script for consistent testing
  - Improved code quality guidelines and testing requirements

### Fixed
- **Test reliability** - Corrected XDG environment variable test logic
  - Fixed false positive when XDG variables weren't properly isolated
  - Enhanced test coverage for realistic XDG directory scenarios
  - All tests now pass consistently across platforms

### Documentation
- **Macro examples** - Complete examples for all four macro syntax variants
  - Direct value: `app_path!("com.example.app")`
  - With override: `app_path!("com.example.app", "/custom/path")`
  - Field access: `app_path!(value, cache_dir)`
  - Complex expressions: `app_path!(format!("app-{}", version))`
- **Error handling patterns** - Comprehensive `try_app_path!` usage examples
- **API symmetry documentation** - Clear explanation of panicking vs fallible variants
- **Real-world usage patterns** - Practical examples for different application scenarios

## [0.2.2] - 2025-07-10

### Added
- **New directory creation methods** - `ensure_parent_dirs()` and `ensure_dir_exists()` for clearer intent and better ergonomics
  - `ensure_parent_dirs()` - Creates parent directories for file paths (use when preparing to write a file)
  - `ensure_dir_exists()` - Creates the path as a directory, including all parents (use when creating directories)

### Deprecated
- **`create_dir_all()` method** - Deprecated in favor of the new, more explicit methods
  - The old method name was confusing as it didn't always create directories for the path itself
  - Migration guide included in deprecation notice with clear examples

### Enhanced
- **API clarity** - Method names now clearly indicate their intended purpose
  - `ensure_parent_dirs()` makes it clear you're preparing directories for a file
  - `ensure_dir_exists()` makes it clear you're creating a directory
- **Documentation improvements** - Updated all examples to use the new methods
  - Fixed documentation syntax error that caused empty code block warnings
  - Updated lib.rs examples to demonstrate new directory creation methods
  - Added comprehensive migration examples in deprecation notices

### Fixed
- **Cross-platform test compatibility** - Fixed `test_windows_separator_handling` to handle platform differences correctly
  - On Windows: validates full path equality for Windows-style paths
  - On Unix: validates filename only since Windows-style paths are treated as relative
  - Resolves CI test failures on non-Windows platforms

### Testing
- **Enhanced test coverage** - Added comprehensive tests for new directory creation methods
- **Cross-platform reliability** - All 76 unit tests and 43 doc tests pass on all platforms
- **Backward compatibility** - Existing `create_dir_all()` functionality preserved during deprecation period

### Documentation
- **Method documentation** - Complete documentation for new methods with practical examples
- **Migration guidance** - Clear examples showing how to migrate from deprecated method
- **Consistent examples** - All documentation now uses the new, clearer methods

## [0.2.1] - 2025-07-08

### Added
- **Comprehensive fallible API** - Complete implementation of `try_new()` and `try_exe_dir()` for library use cases
- **Advanced override API** - `with_override()`, `with_override_fn()`, `try_with_override()`, `try_with_override_fn()` for flexible deployment
- **Complete trait ecosystem** - `Default`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`, `Deref<Target=Path>`, `Borrow<Path>` implementations
- **Enhanced trait support** - `AsRef<OsStr>`, `From<AppPath>` for `OsString` implementations for better FFI integration
- **Path manipulation methods** - `join()`, `parent()`, `with_extension()`, `file_name()`, `file_stem()`, `extension()`, `is_dir()`, `is_file()`
- **Convenience macro** - `app_path!` macro for ergonomic path creation with optional overrides
- **Modular architecture** - Split codebase into focused modules: `app_path`, `functions`, `traits`, `error` for better maintainability
- **Comprehensive error handling** - `AppPathError` with proper `std::error::Error` implementation for library integration

### Enhanced
- **API naming improvements** - Simplified override method names (removed `new_` prefix) for better ergonomics
- **Documentation overhaul** - Streamlined all documentation for clarity, practicality, and consistency
- **API consistency** - Fixed documentation to properly showcase override API instead of manual environment variable handling
- **Real-world examples** - All code examples now focus on practical usage patterns and compile correctly
- **Better override guidance** - Clear prioritization of `with_override()` over `try_new()` for environment variable handling
- **Path manipulation ergonomics** - Direct access to common `Path` methods without explicit dereferencing

### Performance
- **CI pipeline optimization** - Removed unnecessary security audit (zero dependencies) and duplicate tests, ~27% cost reduction
- **Static caching reliability** - Enhanced executable directory caching with proper thread safety
- **Zero-allocation optimizations** - Improved path handling efficiency throughout the API

### Documentation
- **README.md rewrite** - Concise, practical focus with working code examples and clear feature comparison table
- **Crate-level docs streamlined** - Clear and focused on real-world usage without redundancy
- **Method documentation overhaul** - Fixed all code examples to compile, clarified panic conditions, improved practical usage guidance
- **Override API documentation** - Comprehensive examples showing environment variables, CLI arguments, and dynamic configuration patterns
- **Consistent messaging** - All documentation now promotes purpose-built override methods as primary approach

### Testing
- **Comprehensive test coverage** - 68 unit tests + 42 documentation tests covering all features
- **Path manipulation testing** - Complete coverage of new `join()`, `parent()`, `with_extension()` methods
- **Macro testing** - Full validation of `app_path!` macro functionality with all syntax variants
- **Enhanced trait testing** - Verification of new `AsRef<OsStr>` and `From<AppPath>` for `OsString` implementations
- **Edge case coverage** - Root directory execution, containerized environments, cross-platform compatibility
- **Override API testing** - Complete coverage of all override method combinations and priority handling
- **Trait implementation testing** - Full verification of all collection integrations and standard trait behaviors
- **Performance testing** - Validation of zero-allocation patterns and static caching behavior

### Internal
- **Code organization** - Complete modularization with clear separation of concerns
- **CI optimization** - Removed redundant jobs while maintaining quality gates across all platforms
- **Error handling patterns** - Consistent error propagation and recovery strategies throughout codebase

### Quality Assurance
- **All tests passing** - 100% test suite success rate across Windows, Linux, macOS
- **Clippy compliance** - Zero warnings with strict linting enabled
- **Documentation accuracy** - All 42 documentation examples verified to compile and execute correctly
- **API coherence** - Consistent patterns and conventions across all public interfaces

## [0.2.0] - 2025-07-07

### BREAKING CHANGES
- **Replaced `try_new()` with infallible `new()`** - Constructor now panics on system failure instead of returning `Result`
- **Removed `input()` method and `input_path` field** - No longer stores the original input path for cleaner API
- **Replaced `TryFrom` with `From` trait implementations** - Conversions are now infallible 
- **Removed `AppPath::with_base()` method** - This method was confusing and broke the core mental model of "paths relative to executable"
- **Changed constructor parameter** - Now accepts `impl AsRef<Path>` instead of `impl Into<PathBuf>` for zero-allocation optimization

**Migration Guide:**
```rust
// 0.1.2 (old)
let config = AppPath::try_new("config.toml")?;
let input_path = config.input(); // No longer available
let custom = AppPath::with_base(&dir, "file.txt");

// 0.2.0 (new) 
let config = AppPath::new("config.toml"); // Infallible
// OR for error handling:
let config = AppPath::try_new("config.toml")?; // Fallible
// input_path no longer needed - simplified API
let custom = AppPath::new(dir.join("file.txt")); // Use standard Path::join

// Global functions
let exe_dir = exe_dir(); // Infallible
// OR for error handling:
let exe_dir = try_exe_dir()?; // Fallible
```

### Added
- **Infallible API design** - `new()` constructor that panics on rare system failures
- **Fallible API design** - `try_new()` constructor and `try_exe_dir()` function for explicit error handling
- **Static executable directory caching** - Uses `OnceLock` for optimal performance  
- Comprehensive trait implementations: `Default`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`, `Deref<Target=Path>`, `Borrow<Path>`
- Enhanced collection integration - `AppPath` now works seamlessly in `HashMap`, `BTreeSet`, etc.
- `Default` implementation that points to the executable directory
- Direct `Path` method access via `Deref` (e.g., `app_path.extension()`)
- Efficient collection lookups via `Borrow<Path>`
- Comprehensive error handling examples and fallback patterns
- `AppPathError` enum with descriptive error messages for system failures

### Enhanced
- **Simplified API design** - Now focused on the core use case: "paths relative to executable"
- **Improved testing patterns** - Use standard `Path::join()` for custom test directories
- **Cleaner documentation** - Removed confusing examples and focused on practical usage
- **Zero-allocation optimization** - `#[inline]` attributes on all performance-critical methods
- **Better error handling examples** - Practical fallback patterns using `std::env::current_exe()`

### Performance
- Static caching of executable directory using `OnceLock` for optimal performance on stable Rust
- Aggressive inlining of trait implementations and core methods
- Zero-allocation design for `impl AsRef<Path>` parameters

### Fixed
- **MSRV Compatibility** - Replaced `std::sync::LazyLock` with `std::sync::OnceLock` for stable Rust support (≥1.70)

### Documentation
- Streamlined README focusing on API usage rather than compiler optimizations
- Updated all examples to remove `with_base()` references
- Added comprehensive trait implementation examples
- Improved cognitive load by removing overly technical explanations
- Clear migration guide for users of the removed `with_base()` method

### Internal
- Split tests into dedicated `src/tests.rs` module for better maintainability
- Improved test organization and coverage

## [0.1.2] - 2025-07-06

### Added
- Comprehensive documentation improvements across all files
- Generic `impl Into<PathBuf>` parameter for `try_new()` supporting any path-like type
- Smart path resolution behavior: relative paths resolve to executable directory, absolute paths used as-is
- Ownership transfer optimization for `String` and `PathBuf` types
- Enhanced examples showing different path types and ownership patterns
- Path behavior documentation explaining absolute vs relative path handling

### Enhanced
- README.md with complete feature overview, path behavior section, and ownership examples
- Crate-level documentation with absolute path examples and system integration use cases
- API documentation with detailed behavior explanations for different path types
- Test coverage for absolute path behavior and ownership transfer scenarios

### Performance
- Optimized ownership transfer when moving `String` or `PathBuf` into `AppPath`
- Zero-copy path handling where possible through generic `impl Into<PathBuf>` parameter

### Documentation
- Added comprehensive path resolution behavior documentation
- Enhanced examples showing portable vs system integration use cases
- Complete API coverage with ownership and conversion examples
- Improved testing guidelines and cross-platform compatibility notes

## [0.1.1] - 2025-07-05

### Added
- Initial stable release of `app-path` crate
- `AppPath::try_new()` - Create paths relative to executable location
- `AppPath::with_base()` - Override base directory for testing  
- `AppPath::path()` - Get the full resolved path (primary method)
- `AppPath::input()` - Get the original input path before resolution
- `AppPath::exists()` - Check if the path exists
- `AppPath::create_dir_all()` - Create parent directories if needed
- `TryFrom<&str>`, `TryFrom<String>`, and `TryFrom<&String>` implementations for ergonomic string conversions
- Implementation of `Display`, `From<AppPath>`, and `AsRef<Path>` traits
- Zero dependencies - uses only standard library
- Cross-platform support (Windows, Linux, macOS)
- Comprehensive documentation with examples
- Full test suite with CI/CD pipeline

### Fixed
- Improved API design over initial 0.1.0 release

### Notes
- Uses `try_new()` instead of `new()` to follow Rust conventions where `new()` implies infallible construction
- Methods `path()` and `input()` provide clear, intuitive naming
- Multiple ergonomic creation methods via `try_new()` and `TryFrom` traits

## [0.1.0] - 2025-07-05

### Added  
- Initial release (yanked - replaced by 0.1.1 with improved API)

[Unreleased]: https://github.com/DK26/app-path-rs/compare/v0.2.2...HEAD
[0.2.2]: https://github.com/DK26/app-path-rs/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/DK26/app-path-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/DK26/app-path-rs/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/DK26/app-path-rs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/DK26/app-path-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/DK26/app-path-rs/releases/tag/v0.1.0