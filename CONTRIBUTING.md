# Contributing to app-path

Thanks for your interest in contributing! 🦀

## How to Contribute

- **🐛 Bug reports**: [Open an issue](https://github.com/DK26/app-path-rs/issues) with reproduction steps
- **💡 Features**: Discuss in an issue before implementing
- **📝 Docs**: Fix typos, add examples, improve clarity
- **🔧 Code**: Bug fixes and improvements welcome

## Development

```bash
cargo test          # Run tests
cargo fmt           # Format code  
cargo clippy        # Check for issues
```

## Guidelines

### Code Quality
- All tests must pass
- No clippy warnings
- Follow `cargo fmt` style
- Add tests for new features

### Project Philosophy
- **Simple API** - Easy to use, hard to misuse
- **Zero dependencies** - Keep it lightweight
- **Cross-platform** - Windows, Linux, macOS
- **Reliable** - Predictable behavior everywhere

## Pull Requests

1. Fork and create a feature branch
2. Write tests for your changes
3. Ensure `cargo test && cargo clippy` passes
4. Submit PR with clear description

**Note**: All PRs automatically run our CI pipeline which tests on Windows, Linux, and macOS with multiple Rust versions.

### What We Want ✅
- Bug fixes
- Performance improvements
- Better error handling
- Documentation improvements

### What We Don't Want ❌
- Complex APIs
- New dependencies (Unless presenting a strong case)
- Platform-specific features
- Breaking changes without discussion

## License

By contributing to this project, you agree that your contributions will be licensed under the same terms as the project: **MIT OR Apache-2.0**.

You confirm that:
- You have the right to submit your contribution
- Your contribution is your original work or properly attributed
- You understand your contribution may be redistributed under these licenses

## Getting Help

- **Issues**: Bug reports and feature requests
- **Email**: dikaveman@gmail.com

---

Every contribution matters! 🚀