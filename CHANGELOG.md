# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.3](https://github.com/evoludigit/elo-rust/compare/v0.3.2...v0.3.3) (2026-02-08)

### Bug Fixes

* resolve remaining security workflow issues ([25992ee](https://github.com/evoludigit/elo-rust/commit/25992ee252f63860433decee6e7b3ac16785863a))
* simplify cargo-deny to bans and sources checks ([c93ee92](https://github.com/evoludigit/elo-rust/commit/c93ee92541c2c14b1ede47e0a75f76944da389a9))

## [0.3.2](https://github.com/evoludigit/elo-rust/compare/v0.3.1...v0.3.2) (2026-02-08)

### Bug Fixes

* resolve security workflow failures ([335e499](https://github.com/evoludigit/elo-rust/commit/335e499f2912e772cb5fb5defd744db522c32643))

## [0.3.1](https://github.com/evoludigit/elo-rust/compare/v0.3.0...v0.3.1) (2026-02-08)

### Bug Fixes

* allow publish workflow to run on manual workflow_dispatch ([60e5157](https://github.com/evoludigit/elo-rust/commit/60e51576813120a52c72388a971f37f23691ca6d))
* trigger publish workflow on semantic release completion ([87e3e84](https://github.com/evoludigit/elo-rust/commit/87e3e84df53617e6cf452d0aa0243837d39eeb06))

## [0.3.0](https://github.com/evoludigit/elo-rust/compare/v0.2.1...v0.3.0) (2026-02-08)

### Features

* add comprehensive security scanning and SBOM generation ([4682805](https://github.com/evoludigit/elo-rust/commit/46828054c43a1c90a6fe5a42cd7c516d84f4ab1e))

### Bug Fixes

* make file not found error message check platform-agnostic ([5ce15f2](https://github.com/evoludigit/elo-rust/commit/5ce15f25ad681be2451f690c11a5907c01f868d5))
* mark Unix-specific path validation tests with cfg(unix) ([684da2a](https://github.com/evoludigit/elo-rust/commit/684da2a2aa2ef27199d2c36ad0aeb29e0bb9f3e9))
* resolve CI failures (artifacts API and Windows binary path) ([d5c0ae4](https://github.com/evoludigit/elo-rust/commit/d5c0ae46aec914bd1225b9c924ab8fa0efb65fba))
* resolve missing fs import on Windows builds ([31fc317](https://github.com/evoludigit/elo-rust/commit/31fc317a54ae48f6e1aea159b709cae424567382))
* skip Windows-specific file path tests on Windows platform ([66c88ad](https://github.com/evoludigit/elo-rust/commit/66c88adcc47612fe2ea152c085c76b8b19d636f0))

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
