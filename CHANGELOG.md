# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1] - 2025-07-08

### Added
- **Comprehensive fallible API** - Complete implementation of `try_new()` and `try_exe_dir()` for library use cases
- **Advanced override API** - `new_with_override()`, `new_with_override_fn()`, `try_new_with_override()`, `try_new_with_override_fn()` for flexible deployment
- **Complete trait ecosystem** - `Default`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`, `Deref<Target=Path>`, `Borrow<Path>` implementations
- **Modular architecture** - Split codebase into focused modules: `app_path`, `functions`, `traits`, `error` for better maintainability
- **Comprehensive error handling** - `AppPathError` with proper `std::error::Error` implementation for library integration

### Enhanced
- **Documentation overhaul** - Streamlined all documentation for clarity, practicality, and consistency
- **API consistency** - Fixed documentation to properly showcase override API instead of manual environment variable handling
- **Real-world examples** - All code examples now focus on practical usage patterns and compile correctly
- **Better override guidance** - Clear prioritization of `new_with_override()` over `try_new()` for environment variable handling

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
- **Comprehensive test coverage** - 58 unit tests + 31 documentation tests covering all features
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
- **Documentation accuracy** - All 31 documentation examples verified to compile and execute correctly
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

[Unreleased]: https://github.com/DK26/app-path-rs/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/DK26/app-path-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/DK26/app-path-rs/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/DK26/app-path-rs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/DK26/app-path-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/DK26/app-path-rs/releases/tag/v0.1.0