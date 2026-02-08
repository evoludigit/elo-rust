# Phase 1: Setup & Architecture

**Duration**: Weeks 1-2
**Objective**: Establish solid project foundation with testing framework, type system design, and CI/CD
**Team**: 1 engineer
**Status**: [ ] Not Started | [~] In Progress | [ ] Complete

---

## Success Criteria

- [ ] Project structure matches architecture design
- [ ] All module stubs compile without warnings
- [ ] Testing framework configured and working
- [ ] Type system design finalized and documented
- [ ] CI/CD pipeline set up and green
- [ ] 50+ unit tests passing
- [ ] Zero Clippy warnings
- [ ] Ready to implement core code generator (Phase 2)

---

## Cycle 1: Module Stubs & Testing Framework (Week 1, Days 1-2)

### Objective
Initialize project structure with all planned modules and basic testing infrastructure.

### RED Phase: Write Failing Tests
```bash
# Verify project structure exists
cargo test --lib 2>&1 | grep "test result"

# Expected: Some tests pass (stdlib modules), all modules compile
```

**Test files to create:**
- `tests/module_structure.rs` - Verify all modules load
- Unit tests in each module (`mod tests { }` blocks)

### GREEN Phase: Create Module Stubs
✅ Already done in setup! Modules created:
- `src/lib.rs` - Entry point
- `src/codegen/mod.rs`, `types.rs`, `operators.rs`, `expressions.rs`, `functions.rs`
- `src/runtime/mod.rs` - Error types
- `src/stdlib/mod.rs`, `string.rs`, `datetime.rs`, `array.rs`, `types.rs`

### REFACTOR Phase: Improve Module Organization
```bash
# Verify all modules are properly organized
cargo check
```

### CLEANUP Phase: Fix Lints
```bash
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
```

**Commit:**
```
feat(setup): initialize module structure and stub implementations [Phase 1, Cycle 1: CLEANUP]

## Changes
- Created core module hierarchy (codegen, runtime, stdlib)
- Added basic type system definitions
- Implemented ValidationError and ValidationErrors types
- Added placeholder functions for code generators

## Verification
✅ All modules compile without warnings
✅ Basic unit tests pass
✅ Zero Clippy warnings
```

---

## Cycle 2: Type System Design & Mapping (Week 1, Days 3-5)

### Objective
Define and implement comprehensive ELO-to-Rust type mapping system.

### RED Phase: Write Type Mapping Tests

**File**: `tests/type_mapping.rs`

```rust
#[test]
fn test_basic_type_conversions() {
    // Test each ELO type maps correctly to Rust type
}

#[test]
fn test_type_composition() {
    // Test Option<T>, Array<T>, custom types
}

#[test]
fn test_type_string_generation() {
    // Verify to_rust_string() produces valid Rust syntax
}
```

✅ Already partially done in `src/codegen/types.rs`!
- Basic type definitions
- Type string generation
- Unit tests for basic types

### GREEN Phase: Enhance Type Mapping

Enhance `src/codegen/types.rs` with:
1. Generic type support
2. Custom type handling
3. Type context for field resolution
4. Type inference helpers

**Add to `src/codegen/types.rs`:**
```rust
/// Type context for resolving field types
pub struct TypeContext {
    // Maps struct names to their fields and types
    custom_types: std::collections::HashMap<String, TypeInfo>,
}

pub struct TypeInfo {
    fields: std::collections::HashMap<String, RustType>,
}
```

### REFACTOR Phase: Simplify Type Resolution
- Extract field lookup into dedicated function
- Create helper for common conversions
- Improve error handling

### CLEANUP Phase: Verify and Document

```bash
cargo test type::
cargo clippy --lib -- -D warnings
cargo doc --no-deps --open
```

**Commit:**
```
feat(codegen): implement comprehensive type mapping system [Phase 1, Cycle 2: CLEANUP]

## Changes
- Extended RustType enum with generic support
- Created TypeContext for field resolution
- Added type inference helpers
- Documented ELO ↔ Rust type mapping
- Added 15+ type mapping tests

## Verification
✅ Type tests pass
✅ Type string generation correct
✅ Zero Clippy warnings
✅ Documentation generated
```

---

## Cycle 3: Error Handling Architecture (Week 2, Days 1-3)

### Objective
Design and implement comprehensive error handling for code generation and validation.

### RED Phase: Write Error Handling Tests

**File**: `tests/error_handling.rs`

```rust
#[test]
fn test_validation_error_creation() { }

#[test]
fn test_validation_error_formatting() { }

#[test]
fn test_validation_errors_collection() { }

#[test]
fn test_error_serialization() { }
```

✅ Already done in `src/runtime/mod.rs`!
- ValidationError with path, message, rule, value
- ValidationErrors collection
- Display implementations
- Unit tests

### GREEN Phase: Add Code Generation Errors

Extend error handling for code generation:

**Add to `src/codegen/mod.rs`:**
```rust
/// Code generation error
#[derive(Debug)]
pub enum CodeGenError {
    UnsupportedFeature(String),
    TypeMismatch(String),
    InvalidExpression(String),
}

impl std::fmt::Display for CodeGenError {
    // ...
}

impl std::error::Error for CodeGenError {}
```

### REFACTOR Phase: Unify Error Handling
- Consolidate validation and codegen errors
- Create error context for better messages
- Add source tracking for debugging

### CLEANUP Phase: Test and Document

```bash
cargo test error_handling
cargo clippy
```

**Commit:**
```
feat(runtime): implement comprehensive error handling [Phase 1, Cycle 3: CLEANUP]

## Changes
- Created ValidationError and ValidationErrors types
- Added CodeGenError for code generation issues
- Implemented Display and Error traits
- Added comprehensive error messages with context
- Added 10+ error handling tests

## Verification
✅ All error tests pass
✅ Error messages clear and actionable
✅ Zero Clippy warnings
```

---

## Cycle 4: CI/CD & Testing Infrastructure (Week 2, Days 4-5)

### Objective
Set up continuous integration, testing framework, and benchmarking infrastructure.

### RED Phase: Create CI Configuration

**File**: `.github/workflows/ci.yml`

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --all-features
      - run: cargo clippy -- -D warnings
      - run: cargo fmt --check
```

### GREEN Phase: Set Up Test Suites

Create test structure:
- `tests/unit/` - Module-specific tests
- `tests/integration/` - End-to-end tests
- `benches/` - Performance benchmarks

### REFACTOR Phase: Organize Testing
- Create test utilities and fixtures
- Add test macro helpers
- Improve test organization

### CLEANUP Phase: Verify All Passing

```bash
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
```

**Commit:**
```
chore(ci): set up ci/cd pipeline and testing infrastructure [Phase 1, Cycle 4: CLEANUP]

## Changes
- Created GitHub Actions CI workflow
- Set up test directory structure
- Added testing utilities
- Configured clippy and fmt checks
- Created benchmark framework

## Verification
✅ CI workflow passes locally
✅ All tests run and pass
✅ Format and lint checks pass
✅ Ready for Phase 2
```

---

## Dependencies

**Requires**:
- Nothing - this is the foundation phase

**Blocks**:
- Phase 2: Core Code Generator (depends on type system)
- Phase 3: Standard Library (depends on code generator)
- Phases 4-5: Depend on earlier phases

---

## Testing Strategy

### Unit Tests
- Each module has inline tests (`mod tests`)
- Test type conversions thoroughly
- Test error handling paths
- Test edge cases (null, empty, boundaries)

### Integration Tests
- Full compilation pipeline
- End-to-end scenarios
- Framework integration (Actix, Axum examples)

### Manual Verification
```bash
# Build project
cargo build --all-features

# Run tests
cargo test --all

# Check coverage
cargo tarpaulin --out Html

# Benchmark baseline
cargo bench --bench validation_performance
```

---

## Architecture Decisions

### Type System
- **Decision**: Map to common Rust types (not generics)
- **Rationale**: Simpler code generation, easier to optimize
- **Trade-off**: Less flexibility vs. better performance

### Error Collection
- **Decision**: Both single error and Vec<Error> support
- **Rationale**: Flexibility for different validation patterns
- **Trade-off**: Extra code vs. better ergonomics

### Module Organization
- **Decision**: Separate codegen, stdlib, runtime
- **Rationale**: Clear separation of concerns
- **Trade-off**: More modules vs. better maintainability

---

## Notes

- Focus on solid foundation; optimization comes later
- All tests should be fast (<100ms total)
- Avoid premature optimization
- Document all decisions in code comments

---

## Status

| Item | Status |
|------|--------|
| Project structure | ✅ Complete |
| Type system | 🔄 In Progress |
| Error handling | ✅ Complete |
| Testing framework | ⏳ Next |
| CI/CD | ⏳ Next |

---

**Next Phase**: [Phase 2: Core Code Generator](./phase-02-codegen.md)
