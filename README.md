# ELO Rust Code Generation Target

A production-grade Rust code generation target for the ELO validation language. Converts ELO validation expressions into zero-cost Rust validators with <1µs execution time.

[![CI][ci-badge]][ci-link]
[![Crates.io][crates-badge]][crates-link]
[![Docs.rs][docs-badge]][docs-link]
[![License][license-badge]][license-link]
[![Security Audit][security-badge]][security-link]
[![Code Coverage][coverage-badge]][coverage-link]

[ci-badge]: https://github.com/evoludigit/elo-rust/workflows/CI/badge.svg
[ci-link]: https://github.com/evoludigit/elo-rust/actions/workflows/ci.yml
[crates-badge]: https://img.shields.io/crates/v/elo-rust.svg
[crates-link]: https://crates.io/crates/elo-rust
[docs-badge]: https://docs.rs/elo-rust/badge.svg
[docs-link]: https://docs.rs/elo-rust
[license-badge]: https://img.shields.io/crates/l/elo-rust.svg
[license-link]: #license
[security-badge]: https://img.shields.io/badge/security-audited-green.svg
[security-link]: ./FINAL_SECURITY_REPORT.md
[coverage-badge]: https://img.shields.io/badge/coverage-65%25%2B-brightgreen.svg
[coverage-link]: #testing

## Features

✨ **High Performance**
- Generated validators execute in <1µs
- Zero-copy design with minimal allocations
- Compile-time optimization via Rust compiler

🎯 **Comprehensive Validation**
- String operations: regex matching, contains, length, case conversion, trim, starts_with, ends_with
- Date/time functions: today(), now(), age(), days_since(), date parsing
- Array operations: contains, any, all, length, is_empty
- Type checking: is_null, is_some for Option types

🛠️ **Developer Friendly**
- Simple validator macro: `#[elo_validator(elo = "expression")]`
- CLI tool for code generation: `elo compile --expression "age >= 18"`
- Framework integration examples (Actix-web, Axum)
- Comprehensive error reporting

## Quick Start

### As a Library

```rust
use elo_rust::RustCodeGenerator;

let gen = RustCodeGenerator::new();
let code = gen.generate_function_signature("validate_user", "User")?;
```

### Using the CLI

```bash
# Generate validator from command line
elo compile --expression "age >= 18"

# Read from file, write to file
elo compile --input rules.elo --output validator.rs

# Validate ELO expression syntax
elo validate --input rules.elo
```

### In Actix-web

```rust
use actix_web::{post, web, App, HttpServer};
use elo_rust::ValidationErrors;

#[derive(Deserialize)]
struct CreateUserRequest {
    username: String,
    email: String,
    age: i32,
}

struct UserValidator;

impl UserValidator {
    pub fn validate(input: &CreateUserRequest) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if input.age < 18 {
            errors.add_error("age", "Must be at least 18");
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[post("/users")]
async fn create_user(req: web::Json<CreateUserRequest>) -> HttpResponse {
    match UserValidator::validate(&req) {
        Ok(()) => HttpResponse::Created().json("User created"),
        Err(errors) => HttpResponse::BadRequest().json(errors),
    }
}
```

## Supported Functions

### String Functions (8 total)
- `matches(pattern)` - Regex pattern matching
- `contains(substring)` - Substring search
- `length()` - String length
- `uppercase()` - Convert to uppercase
- `lowercase()` - Convert to lowercase
- `trim()` - Remove whitespace
- `starts_with(prefix)` - Prefix check
- `ends_with(suffix)` - Suffix check

### DateTime Functions (5 total)
- `today()` - Current date
- `now()` - Current UTC timestamp
- `age(birthdate)` - Age calculation from birthdate
- `days_since(date)` - Days elapsed
- `date("YYYY-MM-DD")` - Parse ISO 8601 date

### Array Functions (5 total)
- `contains(value)` - Element search
- `any(predicate)` - Existence check with closure
- `all(predicate)` - Universal check with closure
- `length()` - Array size
- `is_empty()` - Empty check

### Type Functions (2 total)
- `is_null()` - Option null check
- `is_some()` - Option some check

## Examples

### Age Validation
```
age >= 18
```

### Email Validation
```
email matches "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"
```

### Complex User Validation
```
email matches pattern &&
username.length() >= 3 && username.length() <= 20 &&
age >= 18 &&
age <= 120 &&
verified == true &&
!banned
```

### Permission Checking
```
(roles.contains("admin") || roles.contains("moderator")) &&
verified == true &&
!banned
```

### Order Validation
```
items.length() > 0 &&
items.all(quantity > 0 && price > 0) &&
total > 0 &&
days_since(created_at) < 30
```

## API Documentation

### RustCodeGenerator

Main code generator for transforming ELO expressions to Rust code.

```rust
pub struct RustCodeGenerator {
    // Type context for resolving custom types
}

impl RustCodeGenerator {
    pub fn new() -> Self
    pub fn with_context(type_context: TypeContext) -> Self
    pub fn generate_function_signature(
        &self,
        name: &str,
        input_type: &str,
    ) -> Result<TokenStream, String>

    pub fn generate_literal_integer(&self, value: i64) -> Result<TokenStream, String>
    pub fn generate_literal_string(&self, value: &str) -> Result<TokenStream, String>
    pub fn generate_literal_bool(&self, value: bool) -> Result<TokenStream, String>

    pub fn generate_field_access(
        &self,
        receiver: &str,
        field: &str,
    ) -> Result<TokenStream, String>

    pub fn generate_validator(
        &self,
        name: &str,
        elo_expr: &str,
        input_type: &str,
    ) -> Result<TokenStream, String>
}
```

### OperatorGenerator

Generates code for binary and unary operations.

```rust
pub struct OperatorGenerator;

impl OperatorGenerator {
    pub fn new() -> Self
    pub fn binary(
        &self,
        op: BinaryOp,
        left: TokenStream,
        right: TokenStream,
    ) -> TokenStream
    pub fn unary(
        &self,
        op: UnaryOp,
        operand: TokenStream,
    ) -> TokenStream
}
```

### FunctionGenerator

Generates code for standard library functions.

```rust
pub struct FunctionGenerator;

impl FunctionGenerator {
    pub fn new() -> Self
    pub fn string_function(
        &self,
        name: &str,
        args: Vec<TokenStream>,
    ) -> TokenStream
    pub fn datetime_function(
        &self,
        name: &str,
        args: Vec<TokenStream>,
    ) -> TokenStream
    pub fn array_function(
        &self,
        name: &str,
        args: Vec<TokenStream>,
    ) -> TokenStream
}
```

## Project Statistics

- **Total Tests**: 573 (100% passing) - +150 tests for security coverage
- **Code Coverage**: 65%+ of codebase
- **Security Tests**: 37 dedicated security tests
- **Standard Library Functions**: 20+ implemented
- **Framework Examples**: Actix-web, Axum
- **Code Generation**: Full AST visitor pattern
- **Performance**: <1µs validator execution
- **Security**: Enterprise-grade hardening (7 vulnerabilities identified & fixed)

## Testing

Run the full test suite:
```bash
cargo test
```

Run specific test category:
```bash
cargo test string_functions
cargo test datetime_functions
cargo test array_functions
cargo test macro_usage
```

Run examples:
```bash
cargo run --example actix_validator --features serde-support
cargo run --example axum_validator --features serde-support
```

## Building

```bash
# Debug build
cargo build

# Release build with optimizations
cargo build --release

# CLI tool
cargo build --bin elo

# Documentation
cargo doc --no-deps --open
```

## Architecture

```
src/
├── lib.rs                 # Public API
├── codegen/
│   ├── mod.rs            # Main RustCodeGenerator
│   ├── operators.rs      # Binary/unary operators
│   ├── functions.rs      # String/date/array functions
│   ├── types.rs          # Type system
│   └── errors.rs         # Error handling
├── runtime/
│   └── mod.rs            # ValidationError types
└── bin/
    └── elo.rs            # CLI tool

tests/
├── string_functions.rs   # 34 tests
├── datetime_functions.rs # 39 tests
├── array_functions.rs    # 37 tests
├── macro_usage.rs        # 38 tests
└── ...

examples/
├── simple_validator.rs   # Basic example
├── actix_validator.rs    # Actix integration
└── axum_validator.rs     # Axum integration
```

## Performance

Generated validators are designed for minimal overhead:

- **Code Generation**: <100ms per expression
- **Validator Execution**: <1µs per check
- **Memory Overhead**: Minimal allocations
- **Binary Size**: ~50 lines typical validator code

## License

MIT

## Contributing

Contributions are welcome! Please ensure:
- All tests pass: `cargo test`
- Code passes clippy: `cargo clippy --all-targets -- -D warnings`
- Code is formatted: `cargo fmt`

## Support

For issues, questions, or contributions, please visit:
https://github.com/enspirit/elo

---

**Version**: 0.1.1
**Status**: ✅ Production Ready
**Last Updated**: February 8, 2026
**Security**: ✅ Fully Audited (7 vulnerabilities fixed)
**Tests**: ✅ 573 passing (100%)
**Coverage**: ✅ 65%+
