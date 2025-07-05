# Changelog

## [Unreleased]

## [0.1.0] - 2025-01-05

### Added
- Initial release of `app-path` crate
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

### Notes
- Uses `try_new()` instead of `new()` to follow Rust conventions where `new()` implies infallible construction
- Methods `path()` and `input()` provide clear, intuitive naming
- Multiple ergonomic creation methods via `try_new()` and `TryFrom` traits

[Unreleased]: https://github.com/DK26/app-path-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/DK26/app-path-rs/releases/tag/v0.1.0