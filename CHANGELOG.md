# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 1.0.0 (2026-02-08)

### Features

* add comprehensive security scanning and SBOM generation ([0372603](https://github.com/evoludigit/elo-rust/commit/0372603a5ac2fde84443bbbbb734f87da7b290b3))
* **cli:** implement ELO code generation CLI tool ([7aef09d](https://github.com/evoludigit/elo-rust/commit/7aef09def8c345aaff2d045c7214a06b8a144de7))
* **codegen:** comprehensive integration tests for complex expressions ([f612a85](https://github.com/evoludigit/elo-rust/commit/f612a858a27a329f093c49684a064dc9676936e9))
* **codegen:** implement AST visitor and code generation skeleton [Phase 2, Cycle 1: CLEANUP] ([bb1e5ba](https://github.com/evoludigit/elo-rust/commit/bb1e5baad8de0ae98cee8f367d8846b25b6513f2))
* **codegen:** implement comprehensive logical operator tests ([d1764a5](https://github.com/evoludigit/elo-rust/commit/d1764a59a9233263789c10df3035e6b6597c1531))
* **codegen:** implement comprehensive type mapping system [Phase 1, Cycle 2: CLEANUP] ([cae7056](https://github.com/evoludigit/elo-rust/commit/cae7056d8906ff750ece42febff4406aa1debf08))
* **codegen:** implement operator code generation ([bb78576](https://github.com/evoludigit/elo-rust/commit/bb78576284008bad8f63c1da9b5370934fbb9d1a))
* **examples:** add Actix-web and Axum framework integration examples ([d887cdf](https://github.com/evoludigit/elo-rust/commit/d887cdf0610b603ec57174967f68bcb8c3c5a53d))
* **macro:** implement validator generation and macro tests ([6551518](https://github.com/evoludigit/elo-rust/commit/65515182dafca3bcc11cf27eadf930230e85e9b1))
* **runtime:** implement comprehensive error handling [Phase 1, Cycle 3: CLEANUP] ([12824bb](https://github.com/evoludigit/elo-rust/commit/12824bbeb14dd25ab6d681fb59789d428901c97c))
* **setup:** initialize module structure and stub implementations [Phase 1, Cycle 1: CLEANUP] ([9c0a5c3](https://github.com/evoludigit/elo-rust/commit/9c0a5c3234fb35f9b1399a290092a6715a9d734f))
* **stdlib:** implement array and type checking functions ([14238dc](https://github.com/evoludigit/elo-rust/commit/14238dc265bd0f83ca2dba6acef43632a35acc39))
* **stdlib:** implement date/time manipulation functions ([d78e98c](https://github.com/evoludigit/elo-rust/commit/d78e98c03196199a1f8c6e618099ffb3a343ef5d))
* **stdlib:** implement string manipulation functions ([be509d9](https://github.com/evoludigit/elo-rust/commit/be509d9b1d517906c68a9a22b291a971452fba9e))

### Bug Fixes

* allow publish workflow to run on manual workflow_dispatch ([f943fc7](https://github.com/evoludigit/elo-rust/commit/f943fc7d144cff632ce91e5c51eca451d9f7e199))
* **ci:** add release workflow to path triggers ([6822d20](https://github.com/evoludigit/elo-rust/commit/6822d208ddf9a8e29378b221004c1f3a08b1faac))
* **ci:** replace deprecated rustsec action with cargo audit ([c97376d](https://github.com/evoludigit/elo-rust/commit/c97376d12fd57a94db57c8cf3ff131cb62cef790))
* **docs:** correct upstream repository reference from blambeau to enspirit ([7af8ec0](https://github.com/evoludigit/elo-rust/commit/7af8ec03b3aa2e673d94bd5d6b416193f122a438))
* make file not found error message check platform-agnostic ([59395bb](https://github.com/evoludigit/elo-rust/commit/59395bbc1ea76b137f4f8bcc1f663ca1d3d00f74))
* mark Unix-specific path validation tests with cfg(unix) ([484b3e6](https://github.com/evoludigit/elo-rust/commit/484b3e6d8a243f7b2e061c81eb97b68a64f83164))
* **package:** exclude large files from crates.io publication ([aff30a0](https://github.com/evoludigit/elo-rust/commit/aff30a001b937f2c30609c8a2cf7a2cdfe64e1a8))
* resolve CI failures (artifacts API and Windows binary path) ([b67f997](https://github.com/evoludigit/elo-rust/commit/b67f99783aed1a004eb6832e172b881bca42342c))
* resolve missing fs import on Windows builds ([c760c55](https://github.com/evoludigit/elo-rust/commit/c760c5557c4c55fb087701c236d70e1ce09e5565))
* resolve remaining security workflow issues ([8bc70ac](https://github.com/evoludigit/elo-rust/commit/8bc70ac63b6dbaab683529a22cddd28c63cb971e))
* resolve rustfmt and clippy issues in test files ([e8d53ed](https://github.com/evoludigit/elo-rust/commit/e8d53ed92d1f443eb4ff6bf6db51907d4c62c5b6))
* resolve security workflow failures ([75d7634](https://github.com/evoludigit/elo-rust/commit/75d763441c78ebc6b96f03858807f049aa004296))
* simplify cargo-deny to bans and sources checks ([1327194](https://github.com/evoludigit/elo-rust/commit/13271949cf5f97d3970ec38149eda5581dd5d6ee))
* skip Windows-specific file path tests on Windows platform ([57987cf](https://github.com/evoludigit/elo-rust/commit/57987cfae02526f0efac4d69620e31290f41e95f))
* trigger publish workflow on semantic release completion ([1d7c9ad](https://github.com/evoludigit/elo-rust/commit/1d7c9adc41d5dace5a1db8c9cbdcda9c4ce65068))
* **workflow:** add missing conventional-changelog dependency ([24db106](https://github.com/evoludigit/elo-rust/commit/24db106ec309733d55209fd270e594b4c8dbb615))

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
