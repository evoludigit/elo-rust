# Release Guide

This document describes how to release new versions of the `elo-rust` crate with automatic semantic versioning, changelog generation, and publishing to crates.io.

## Table of Contents

1. [Overview](#overview)
2. [Conventional Commits](#conventional-commits)
3. [Release Workflow](#release-workflow)
4. [Manual Release](#manual-release)
5. [Troubleshooting](#troubleshooting)

---

## Overview

The `elo-rust` project uses:

- **Conventional Commits** for commit message format
- **Semantic Versioning** for version numbering
- **GitHub Actions** for automated testing, versioning, and publishing
- **Semantic Release** for intelligent version bumping
- **Cargo** for Rust package management

### Version Numbering

This project follows [Semantic Versioning](https://semver.org/):

- **MAJOR** (e.g., 1.0.0): Breaking changes to the public API
- **MINOR** (e.g., 0.2.0): New features (backward compatible)
- **PATCH** (e.g., 0.1.1): Bug fixes (backward compatible)

### Automatic Versioning

The version is automatically determined based on commit messages:

- `feat(...)` → MINOR version bump (e.g., 0.1.0 → 0.2.0)
- `fix(...)` → PATCH version bump (e.g., 0.1.0 → 0.1.1)
- `BREAKING CHANGE:` in body → MAJOR version bump (e.g., 0.1.0 → 1.0.0)

---

## Conventional Commits

All commit messages must follow the [Conventional Commits](https://www.conventionalcommits.org/) specification.

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation only changes
- **style**: Changes that don't affect code meaning (formatting, semicolons)
- **refactor**: Code change that neither fixes a bug nor adds a feature
- **perf**: Code change that improves performance
- **test**: Adding missing tests or correcting existing tests
- **chore**: Changes to build process, dependencies, tooling
- **ci**: Changes to CI configuration files and scripts
- **revert**: Revert of a previous commit
- **build**: Changes to build system

### Scope (Optional)

The scope specifies what part of the codebase is affected:

- `codegen` - Code generation module
- `cli` - Command-line tool
- `runtime` - Runtime types
- `docs` - Documentation
- `deps` - Dependencies
- `ci` - CI/CD infrastructure

### Subject

- Use imperative, present tense: "add" not "added" or "adds"
- Don't capitalize first letter
- No period (.) at the end
- Maximum 50 characters

### Examples

```
feat(codegen): add custom function support

feat(cli): add --output flag for code generation

fix(runtime): handle null values correctly

docs: update README with new examples

chore(deps): update regex to 1.11

ci(gh-actions): enhance test matrix coverage
```

### Breaking Changes

For breaking changes, add `BREAKING CHANGE:` in the commit body:

```
feat(api): redesign type context API

BREAKING CHANGE: The TypeContext API has been redesigned.
Old code: TypeContext::new()
New code: TypeContext::with_defaults()
```

---

## Release Workflow

### Automatic Release (Recommended)

Releases are triggered automatically when:

1. A commit with `feat(...)` or `fix(...)` is pushed to `main`
2. GitHub Actions analyzes the commits since the last release
3. The version is automatically bumped according to semantic versioning
4. A new tag `v*.*.*` is created
5. The publish workflow is triggered
6. The crate is automatically published to crates.io

### Prerequisites

Before making commits, ensure:

- [ ] All tests pass locally: `cargo test --all`
- [ ] Code is properly formatted: `cargo fmt`
- [ ] Clippy passes: `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] Documentation builds: `cargo doc --no-deps`

### Release Steps

1. **Make commits with conventional messages**

   ```bash
   git commit -m "feat(codegen): add new operator support"
   git commit -m "fix(cli): handle empty input correctly"
   git commit -m "docs: update operator documentation"
   ```

2. **Push to main branch**

   ```bash
   git push origin main
   ```

3. **GitHub Actions automatically:**
   - Runs all tests
   - Checks formatting and lints
   - Analyzes commits since last release
   - Determines version bump
   - Updates CHANGELOG.md
   - Creates tag (e.g., `v0.2.0`)
   - Publishes to crates.io
   - Creates GitHub Release

4. **Verify the release**
   - Check [releases page](https://github.com/evoludigit/elo-rust/releases)
   - Verify on [crates.io](https://crates.io/crates/elo-rust)
   - Check docs on [docs.rs](https://docs.rs/elo-rust)

---

## Manual Release

If you need to create a release manually:

### Option 1: Using Cargo Release (Recommended)

```bash
# Install cargo-release if not already installed
cargo install cargo-release

# Create a new release
cargo release 0.2.0

# Or let it auto-detect the version
cargo release
```

### Option 2: Manual Steps

```bash
# 1. Update version in Cargo.toml
# Change: version = "0.1.0"
# To:     version = "0.2.0"

# 2. Update CHANGELOG.md with new version entry
# Add section like:
# ## [0.2.0] - 2026-02-15
# ### Added
# - New feature description

# 3. Commit changes
git add Cargo.toml Cargo.lock CHANGELOG.md
git commit -m "chore(release): bump version to 0.2.0"

# 4. Create and push tag
git tag v0.2.0
git push origin main
git push origin v0.2.0

# The publish workflow will be triggered automatically
```

### Option 3: Direct Publishing

```bash
# If you need to publish immediately:
cargo publish --token $YOUR_CRATES_IO_TOKEN
```

---

## Changelog Generation

The CHANGELOG.md file is automatically updated with each release. It includes:

- **Unreleased** section tracking development
- **Semantic version headers** for each release
- **Grouped commits** by type (Features, Bug Fixes, Performance)
- **Issue links** and cross-references

### Manual Changelog Updates

To manually add entries:

1. Edit `CHANGELOG.md`
2. Add new version section under `[Unreleased]`
3. Categorize changes:
   - `### Added` - New features
   - `### Changed` - Changes to existing features
   - `### Fixed` - Bug fixes
   - `### Deprecated` - Deprecations
   - `### Removed` - Removed features
   - `### Security` - Security fixes

Example:

```markdown
## [Unreleased]

### Added
- Custom function support in code generation
- Performance optimization for large expressions

### Fixed
- Edge case handling in type inference

### Changed
- Enhanced error messages with suggestions
```

---

## CI/CD Pipelines

### Continuous Integration (ci.yml)

Runs on every push and pull request:

- ✅ Tests on Linux, macOS, Windows
- ✅ Rust stable and beta
- ✅ Code formatting check
- ✅ Clippy lints
- ✅ Documentation build
- ✅ Code coverage
- ✅ Security audit

All must pass before merging.

### Publishing (publish.yml)

Runs when a tag matching `v*.*.*` is pushed:

- ✅ Final verification of all tests
- ✅ Publish to crates.io
- ✅ Create GitHub Release
- ✅ Upload release binaries
- ✅ Update documentation on docs.rs

### Dependency Updates (dependabot)

Weekly automated PRs for:

- ✅ Cargo dependencies
- ✅ GitHub Actions versions
- Reviewers are notified
- Tests run automatically

---

## Environment Variables

To enable automatic publishing, ensure these are configured in GitHub:

### Required Secrets

Add to **Settings → Secrets and variables → Actions**:

- `CARGO_REGISTRY_TOKEN`: Your crates.io API token
  - Get from: https://crates.io/me
  - Type: Personal Access Token with "Publish" scope

### Optional Secrets

- `GITHUB_TOKEN`: Automatically provided by GitHub Actions
- `GIT_AUTHOR_NAME`: For semantic-release (default: github-actions)
- `GIT_AUTHOR_EMAIL`: For semantic-release (default: github-actions@github.com)

---

## Troubleshooting

### Release Failed: Tests Didn't Pass

The publish workflow requires all tests to pass. To fix:

```bash
# Run all tests locally
cargo test --all --all-features

# Fix any failures
# Commit fix
git push origin main
# Tagging attempt will trigger tests again
```

### Release Failed: Clippy Warnings

The publish workflow has zero tolerance for Clippy warnings. To fix:

```bash
# Check Clippy
cargo clippy --all-targets --all-features -- -D warnings

# Fix warnings (usually automatically)
cargo fix --allow-dirty

# Run Clippy again to verify
cargo clippy --all-targets --all-features -- -D warnings

# Commit and push
git commit -am "fix: resolve Clippy warnings"
git push origin main
```

### Release Failed: Documentation Warnings

Documentation build must pass with zero warnings:

```bash
# Check docs
cargo doc --no-deps --all-features

# Fix any broken links or missing docs
# Build and check again
cargo doc --no-deps --all-features
```

### Crates.io Token Expired

If publishing fails with authentication error:

1. Visit https://crates.io/me
2. Regenerate API token
3. Update GitHub secret `CARGO_REGISTRY_TOKEN`
4. Retry the release

### Manual Re-trigger

To manually re-trigger publishing if automation failed:

```bash
# Ensure tag exists
git tag v0.2.0 (if not already created)

# Push tag to trigger publish workflow
git push origin v0.2.0

# Or push existing tag (may need --force)
git push origin v0.2.0 --force
```

---

## Best Practices

### Commit Practices

- ✅ Make small, focused commits
- ✅ Use conventional commit messages
- ✅ Group related changes together
- ✅ One feature or fix per commit
- ✅ Never force-push to main

### Version Practices

- ✅ Follow semantic versioning strictly
- ✅ Document breaking changes clearly
- ✅ Keep changelog up-to-date
- ✅ Tag releases consistently
- ✅ Never re-release same version

### Testing Practices

- ✅ Write tests for new features
- ✅ Ensure all tests pass before pushing
- ✅ Keep test coverage high
- ✅ Test on multiple platforms locally
- ✅ Check performance impact

### Documentation Practices

- ✅ Document breaking changes
- ✅ Update CHANGELOG.md promptly
- ✅ Keep README accurate
- ✅ Document new CLI flags
- ✅ Add examples for new features

---

## Example Release Sequence

### Scenario: Adding a New Feature

```bash
# 1. Create feature branch (optional)
git checkout -b feature/custom-functions

# 2. Implement feature
# ... edit code ...

# 3. Write tests
# ... edit tests ...

# 4. Verify locally
cargo test --all
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo doc --no-deps

# 5. Commit with conventional message
git add .
git commit -m "feat(codegen): add custom function support

- Implemented CustomFunction trait
- Added 10 new builtin custom functions
- Updated documentation with examples"

# 6. Push to main
git push origin main

# 7. GitHub Actions automatically:
# - Runs CI
# - Detects 'feat' commit
# - Bumps version 0.1.0 → 0.2.0
# - Updates CHANGELOG.md
# - Creates tag v0.2.0
# - Publishes to crates.io
# - Creates GitHub Release

# 8. Users can now:
cargo add elo-rust@0.2.0
```

### Scenario: Fixing a Bug

```bash
# 1. Create fix
git commit -m "fix(runtime): handle null values in type checking"

# 2. Push to main
git push origin main

# 3. GitHub Actions automatically:
# - Runs CI
# - Detects 'fix' commit
# - Bumps version 0.1.0 → 0.1.1
# - Updates CHANGELOG.md
# - Creates tag v0.1.1
# - Publishes to crates.io
```

---

## References

- [Conventional Commits](https://www.conventionalcommits.org/)
- [Semantic Versioning](https://semver.org/)
- [Keep a Changelog](https://keepachangelog.com/)
- [Cargo Publishing](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Semantic Release](https://semantic-release.gitbook.io/)

---

**Last Updated**: February 8, 2026
**Maintainer**: evoludigit
**Status**: Active
