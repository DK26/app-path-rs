# Changelog

## [Unreleased]

## [0.1.0] - 2025-01-05

### Added
- Initial release of `app-path` crate
- `AppPath::new()` - Create paths relative to executable location
- `AppPath::with_base()` - Override base directory for testing
- `AppPath::path()` - Get the full resolved path (primary method)
- `AppPath::input()` - Get the original input path before resolution
- `AppPath::exists()` - Check if the path exists
- `AppPath::create_dir_all()` - Create parent directories if needed
- Implementation of `Display`, `From<AppPath>`, and `AsRef<Path>` traits
- Zero dependencies - uses only standard library
- Cross-platform support (Windows, Linux, macOS)
- Comprehensive documentation with examples
- Full test suite with CI/CD pipeline

### Notes
- This version replaces the initial 0.1.0 release with improved API design
- Methods `path()` and `input()` provide clearer, more intuitive naming

[Unreleased]: https://github.com/DK26/app-path-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/DK26/app-path-rs/releases/tag/v0.1.0