# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/DK26/app-path-rs/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/DK26/app-path-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/DK26/app-path-rs/releases/tag/v0.1.0