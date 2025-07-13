# Contributing to app-path

Thanks for your interest in contributing! 🦀

## Quick Start

1. **Fork** the repository
2. **Clone** your fork: `git clone https://github.com/YOUR_USERNAME/app-path-rs.git`
3. **Test** locally: `./ci-local.sh` (runs all CI checks)
4. **Submit** a pull request

## How to Contribute

- **🐛 Bug reports**: [Open an issue](https://github.com/DK26/app-path-rs/issues) with reproduction steps
- **💡 Features**: Discuss in an issue before implementing
- **📝 Docs**: Fix typos, add examples, improve clarity
- **🔧 Code**: Bug fixes and improvements welcome

## Development

### Local Testing

```bash
# Run all CI checks locally (recommended)
./ci-local.sh

# Or run individual checks
cargo fmt --check    # Format check
cargo clippy         # Linting
cargo test           # Unit tests
cargo test --doc     # Documentation tests
cargo doc            # Build docs
```

### Guidelines

**Code Quality:**
- All tests must pass (`./ci-local.sh`)
- No clippy warnings
- Follow `cargo fmt` style
- Add tests for new features

**Project Philosophy:**
- **Simple API** - Easy to use, hard to misuse
- **Zero dependencies** - Keep it lightweight  
- **Cross-platform** - Windows, Linux, macOS
- **Reliable** - Predictable behavior everywhere

## AI Prompt

Copy-paste this when working with AI on this project:

```
Rules: Always run `bash ci-local.sh` before committing. Use modern string formatting in ALL string-building macros - put variables directly in braces like format!("Hello, {name}") instead of format!("Hello, {}", name). Never remove existing APIs or tests. Preserve all public interfaces.
```

## Pull Requests

1. **Fork** and create a feature branch
2. **Write tests** for your changes
3. **Run CI locally**: `./ci-local.sh`
4. **Submit PR** with clear description

All PRs automatically run CI on Windows, Linux, and macOS.

## What We Want ✅

- Bug fixes and performance improvements
- Better error handling and documentation
- Cross-platform compatibility fixes
- Additional tests and examples

## What We Don't Want ❌

- Complex APIs or breaking changes (discuss first)
- New dependencies (unless strongly justified)
- Platform-specific features

## License

By contributing, you agree that your contributions will be licensed under **MIT OR Apache-2.0**.

You confirm that:
- You have the right to submit your contribution
- Your contribution is your original work or properly attributed

## Getting Help

- **Issues**: Bug reports and feature requests
- **Email**: dikaveman@gmail.com

---

Every contribution matters! 🚀