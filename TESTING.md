# Testing Strategy for ELO Rust Target

**Version**: 1.0
**Date**: February 8, 2026

---

## Overview

The ELO Rust Target uses comprehensive testing across three tiers:
1. **Unit Tests** - Individual modules and functions
2. **Integration Tests** - End-to-end scenarios
3. **CI/CD Tests** - Automated quality checks

---

## Running Tests

### Run All Tests
```bash
cargo test
```

### Run Specific Test Suite
```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test --test type_mapping
cargo test --test error_handling

# Single test
cargo test test_validation_error_creation
```

### Run Tests with Output
```bash
cargo test -- --nocapture
```

### Run Tests in Release Mode
```bash
cargo test --release
```

---

## Test Structure

### Unit Tests (in `src/`)
- Located inline in modules using `#[cfg(test)] mod tests`
- Fast execution (<100ms total)
- Test individual functions and types
- Examples:
  - `src/runtime/mod.rs` - ValidationError tests
  - `src/codegen/types.rs` - Type system tests
  - `src/codegen/operators.rs` - Operator tests

### Integration Tests (in `tests/`)
- Located in separate test files
- Test cross-module interactions
- Test complete workflows
- Files:
  - `tests/type_mapping.rs` - Type system integration
  - `tests/error_handling.rs` - Error handling
  - `tests/common/mod.rs` - Shared test utilities

### Test Utilities
- `tests/lib.rs` - Test module organization
- `tests/common/mod.rs` - Helper functions and fixtures
  - `test_type_context()` - Pre-populated type context
  - `simple_user_type_context()` - Minimal test context
  - Assertion macros for error testing

---

## Test Coverage by Module

### Type System (13 + 10 tests)
- Basic type conversions
- Type composition (Option, Array)
- Custom type support
- Type context management
- Field lookup and resolution
- Type inference from literals
- Type compatibility checking

### Error Handling (22 + 6 tests)
- ValidationError creation and display
- ValidationErrors collection
- CodeGenError variants
- Error trait implementation
- Complex error scenarios
- Nested field paths

### Operators (8 tests)
- Binary operator definitions
- Unary operator definitions
- Operator equality checks
- Logical operator precedence

### Expression Generation (1 test)
- Expression generator creation

### Function Support (1 test)
- Function generator creation

### Runtime (4 tests)
- ValidationError operations
- ValidationErrors collection
- Error display formatting
- Default implementations

### Standard Library (11 tests)
- String function metadata
- DateTime function metadata
- Array function metadata
- Type checking function metadata

---

## CI/CD Pipeline

### GitHub Actions Workflow (`.github/workflows/ci.yml`)

The CI pipeline runs on every push and pull request:

1. **Test (Stable & Beta)**
   - Runs `cargo test --all-features` on stable and beta toolchains
   - Ensures compatibility with current and next Rust versions

2. **Clippy**
   - Runs `cargo clippy --all-targets --all-features -- -D warnings`
   - Zero warnings policy enforced
   - Catches common Rust mistakes

3. **Rustfmt**
   - Runs `cargo fmt --check`
   - Ensures code formatting consistency

4. **Documentation**
   - Runs `cargo doc --no-deps`
   - Verifies documentation builds without warnings

5. **Code Coverage**
   - Uses `cargo-tarpaulin` for coverage reporting
   - Uploads to Codecov for tracking

6. **Security Audit**
   - Runs `cargo audit` to check dependencies
   - Alerts on known vulnerabilities

### Running CI Locally
```bash
# Test all
cargo test --all-features

# Clippy check
cargo clippy --all-targets --all-features -- -D warnings

# Format check
cargo fmt --check

# Documentation
cargo doc --no-deps --document-private-items
```

---

## Test Naming Convention

All tests follow this pattern:
- **Unit tests**: `test_<what_is_being_tested>`
- **Integration tests**: `test_<feature>_<scenario>`

Examples:
- ✅ `test_validation_error_creation`
- ✅ `test_validation_error_with_value`
- ✅ `test_type_context_field_lookup`
- ✅ `test_codegen_error_unsupported_feature`

---

## Writing New Tests

### Unit Test Template
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_name() {
        // Arrange
        let input = setup();

        // Act
        let result = function_under_test(input);

        // Assert
        assert_eq!(result, expected);
    }
}
```

### Integration Test Template
```rust
#[test]
fn test_complete_workflow() {
    // Use common test utilities
    let context = common::test_type_context();

    // Test complete interaction
    let result = /* complex operation */;

    // Verify results
    assert!(result.is_ok());
}
```

### Using Test Utilities
```rust
use elo_rust_tests::common::*;

#[test]
fn test_with_fixtures() {
    let ctx = test_type_context(); // Pre-populated context
    assert_eq!(
        ctx.get_field_type("User", "email"),
        Some(&RustType::String)
    );
}
```

---

## Test Performance

### Current Metrics
- **Total tests**: 73
- **Execution time**: ~0.1 seconds
- **Memory usage**: Minimal (<10MB)

### Performance Goals
- **Unit test execution**: <100ms
- **Full suite execution**: <1 second
- **CI/CD total time**: <5 minutes

---

## Known Test Limitations

1. **No async tests yet** - Phase 2 will add async validator tests
2. **No benchmarks yet** - Phase 2 will add performance benchmarks
3. **No property-based tests yet** - Future enhancement
4. **No fuzzing yet** - Future enhancement

---

## Debugging Tests

### Verbose Output
```bash
cargo test -- --nocapture --test-threads=1
```

### Run Single Test in Debug Mode
```bash
cargo test test_name -- --nocapture --exact
```

### Generate Backtrace
```bash
RUST_BACKTRACE=1 cargo test
```

---

## Continuous Integration Checks

Every commit automatically triggers:

1. ✅ Compilation check
2. ✅ All test suites
3. ✅ Clippy linting
4. ✅ Format verification
5. ✅ Documentation build
6. ✅ Code coverage
7. ✅ Dependency audit

**Success requires**: All checks pass

---

## Test Dependencies

All testing uses only the standard library and existing dev dependencies:
- `criterion` - Benchmarking (Phase 2+)
- No additional testing frameworks

---

## Future Testing Plans

### Phase 2
- [ ] Benchmarking suite for performance validation
- [ ] Integration tests with real ELO expressions
- [ ] Async validator tests

### Phase 3
- [ ] Framework integration tests (Actix, Axum)
- [ ] End-to-end compilation tests
- [ ] Performance regression tests

### Phase 4
- [ ] Property-based testing with quickcheck
- [ ] Fuzzing for edge cases
- [ ] Load testing for validators

---

## Quick Reference

```bash
# All tests
cargo test

# Lib tests only
cargo test --lib

# Integration tests
cargo test --test error_handling

# With output
cargo test -- --nocapture

# Single test
cargo test test_validation_error_creation

# Check lints
cargo clippy -- -D warnings

# Check format
cargo fmt --check

# Check docs
cargo doc --no-deps
```

---

## Contributing Tests

When adding new functionality:

1. **Write test first** (RED phase)
2. **Make it fail** - Verify test catches the missing feature
3. **Implement feature** (GREEN phase)
4. **Make tests pass** - Verify feature works
5. **Refactor** - Improve design
6. **Cleanup** - Fix lints and format

All new code must have corresponding tests.

---

**Last Updated**: February 8, 2026
**Next Review**: After Phase 2 completion
