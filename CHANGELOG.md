# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1](https://github.com/evoludigit/elo-rust/compare/v0.2.0...v0.2.1) (2026-02-08)

### Bug Fixes

* resolve rustfmt and clippy issues in test files ([1bd6001](https://github.com/evoludigit/elo-rust/commit/1bd60018544cc36878f80bd31a4d813fd2e6f7eb))

## [Unreleased]

### Added
- Automatic CI/CD pipeline with GitHub Actions
- Semantic versioning and automated publishing
- Security audit in CI workflow
- Code coverage reporting
- Multi-platform testing (Linux, macOS, Windows)

### Changed
- Enhanced CI workflow with comprehensive checks
- Updated repository link to evoludigit/elo-rust

### Fixed
- Repository metadata in Cargo.toml

## [0.1.0] - 2026-02-08

### Added
- Initial release of Rust code generation target for ELO
- 14 operators (binary and unary)
- 20 standard library functions (string, datetime, array, type functions)
- Complete type system with custom type support
- CLI tool (`elo compile`, `elo validate`)
- Framework integration examples (Actix-web, Axum)
- Comprehensive test suite (317 tests)
- Full API documentation (100% coverage)
- Professional README with quick start guide

### Features
- Zero-cost abstractions (validators execute in <1µs)
- Type-safe code generation
- 100% safe Rust (zero unsafe blocks)
- Minimal dependencies (4 core)
- CI/CD infrastructure
- Automatic semantic versioning
- Automated publishing to crates.io
- Multi-platform support

### Testing
- 317 comprehensive tests across 10 test modules
- Unit tests for all components
- Integration tests with frameworks
- Edge case coverage
- Real-world scenario testing

### Quality
- Zero Clippy warnings (maximum strictness: -D warnings)
- 100% API documentation
- Code coverage tracking
- Security audit in CI
- Dependency vulnerability scanning

### Documentation
- README with 500+ lines
- Framework integration guides
- CLI usage documentation
- Architecture documentation
- Comprehensive audit reports

---

## Versioning

This project follows [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes to public API
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

## Releasing

To create a new release:

```bash
# Using cargo-release (recommended)
cargo release 0.2.0

# Or manually:
# 1. Update version in Cargo.toml
# 2. Update CHANGELOG.md
# 3. Commit and tag:
git tag v0.2.0
git push origin v0.2.0
# The GitHub Actions workflow will handle publishing
```

The publish workflow is automatically triggered when a tag matching `v*.*.*` is pushed.

---

## [Unreleased]: https://github.com/evoludigit/elo-rust/compare/v0.1.0...main
[0.1.0]: https://github.com/evoludigit/elo-rust/releases/tag/v0.1.0
