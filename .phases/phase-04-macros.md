# Phase 4: Integration & Ergonomics

**Duration**: Weeks 10-12
**Objective**: Create derive macros, CLI integration, examples, and comprehensive documentation
**Team**: 1-2 engineers
**Status**: [ ] Not Started | [ ] In Progress | [x] Complete

---

## Success Criteria

- [ ] `#[elo_validator]` derive macro fully functional
- [ ] CLI integration working (`elo compile --target rust`)
- [ ] Example projects for Actix, Axum, and Tokio working
- [ ] Comprehensive API documentation
- [ ] Performance benchmarks demonstrate <1µs latency
- [ ] Integration tests with real frameworks
- [ ] 50+ new tests for macros and integration
- [ ] Error messages clear and actionable
- [ ] Zero Clippy warnings
- [ ] Ready for Phase 5: Finalization

---

## Cycle 1: Derive Macro Implementation (Week 10)

### Objective
Implement `#[elo_validator]` derive macro for ergonomic validation.

### RED Phase: Write Macro Tests

**File**: `tests/macro_validator.rs`

```rust
#[test]
fn test_simple_validator_macro() {
    let expanded = quote! {
        #[elo_validator(elo = "user.age >= 18")]
        struct UserValidator;
    };
    // Should compile and work
    let result = UserValidator::validate(&user);
    assert!(result.is_ok());
}

#[test]
fn test_validator_with_custom_type() {
    let expanded = quote! {
        #[elo_validator(elo = "email matches pattern && age >= 18")]
        pub struct SignupValidator;
    };
    // Should work with custom struct
}

#[test]
fn test_validator_error_handling() {
    let expanded = quote! {
        #[elo_validator(elo = "value > 0")]
        struct PositiveValidator;
    };
    // Should return proper errors
}

#[test]
fn test_multiple_validators() {
    let expanded = quote! {
        #[elo_validator(elo = "email matches pattern")]
        struct EmailValidator;

        #[elo_validator(elo = "age >= 18")]
        struct AgeValidator;
    };
    // Both should work independently
}
```

### GREEN Phase: Implement Basic Macro

**Create `src/macro/validator.rs`:**

```rust
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro for automatic validator generation
///
/// # Example
///
/// ```ignore
/// #[elo_validator(elo = "user.age >= 18")]
/// pub struct UserValidator;
///
/// // Generates: impl UserValidator { pub fn validate(...) -> Result {...} }
/// ```
#[proc_macro_attribute]
pub fn elo_validator(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as syn::AttributeArgs);
    let input = parse_macro_input!(input as DeriveInput);

    // Extract ELO expression from attributes
    let elo_expr = extract_elo_expr(&args).unwrap_or_default();

    // Generate validator code
    let validator_code = generate_validator_impl(&input, &elo_expr);

    quote! {
        #input
        #validator_code
    }
    .into()
}

fn extract_elo_expr(args: &[syn::NestedMeta]) -> Option<String> {
    // Parse: elo_validator(elo = "...")
    // Extract the ELO expression string
    None
}

fn generate_validator_impl(input: &DeriveInput, elo_expr: &str) -> TokenStream {
    // Generate the impl block with validator methods
    quote! {
        impl #name {
            pub fn validate(input: &Input) -> Result<(), ValidationError> {
                // Generated validation code
            }
        }
    }
}
```

### REFACTOR Phase: Improve Macro Ergonomics

- Support multiple error handling styles
- Add attribute validation
- Improve error messages in macro expansion

### CLEANUP Phase: Test Macro

```bash
cargo test macro_validator
cargo clippy --lib
```

**Commit:**
```
feat(macro): implement #[elo_validator] derive macro [Phase 4, Cycle 1: CLEANUP]

## Changes
- Implemented elo_validator proc macro
- Added attribute parsing for ELO expressions
- Generated validation impl blocks
- Added proper error handling
- Added 15+ macro tests

## Verification
✅ Macro compiles without warnings
✅ Generated validators work correctly
✅ Error messages clear
✅ Zero Clippy warnings
```

---

## Cycle 2: Framework Integration Examples (Week 11, Days 1-3)

### Objective
Create working examples with Actix, Axum, and Tokio frameworks.

### RED Phase: Write Integration Tests

**File**: `tests/integration/actix.rs`

```rust
#[actix_web::test]
async fn test_actix_json_validation() {
    // Create a simple Actix endpoint with ELO validator
    // POST /users with CreateUserInput
    // Should validate and accept valid requests
    // Should reject invalid requests
}

#[actix_web::test]
async fn test_actix_error_response() {
    // Test that validation errors return proper JSON responses
    // Should include field path, message, rule
}
```

### GREEN Phase: Create Example Projects

**Create `examples/actix_validator.rs`:**

```rust
use actix_web::{web, App, HttpResponse, HttpServer, post};
use serde::{Deserialize, Serialize};
use elo_rust::{ValidationError, ValidationErrors};

#[derive(Debug, Deserialize, Serialize)]
struct CreateUserRequest {
    email: String,
    age: i32,
}

#[elo_validator(elo = r#"
    email matches "^[a-z]+@example\.com$" &&
    age >= 18
"#)]
struct CreateUserValidator;

#[post("/users")]
async fn create_user(
    req: web::Json<CreateUserRequest>,
) -> HttpResponse {
    match CreateUserValidator::validate(&req) {
        Ok(()) => HttpResponse::Created().json("User created"),
        Err(errors) => HttpResponse::BadRequest().json(errors),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(create_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

**Create `examples/axum_validator.rs`:**

```rust
use axum::{
    extract::Json,
    http::StatusCode,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct CreateUserRequest {
    email: String,
    age: i32,
}

#[elo_validator(elo = r#"
    email matches pattern &&
    age >= 18
"#)]
struct CreateUserValidator;

async fn create_user(
    Json(req): Json<CreateUserRequest>,
) -> Result<String, (StatusCode, Json<ValidationErrors>)> {
    CreateUserValidator::validate(&req)
        .map(|_| "User created".to_string())
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(e)))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users", post(create_user));

    // Serve the app
}
```

### REFACTOR Phase: Extract Common Patterns

- Create helper functions for request validation
- Add middleware for automatic validation
- Improve error response formatting

### CLEANUP Phase: Verify Examples

```bash
# Verify examples compile
cargo build --example actix_validator
cargo build --example axum_validator

# Run tests
cargo test integration --test "*"
```

**Commit:**
```
feat(examples): add framework integration examples [Phase 4, Cycle 2: CLEANUP]

## Changes
- Created Actix-web validator example
- Created Axum validator example
- Added Tokio-compatible examples
- Demonstrated error handling patterns
- Added integration tests with real frameworks
- Included detailed comments and documentation

## Verification
✅ All examples compile
✅ Integration tests pass
✅ Framework compatibility verified
✅ Zero Clippy warnings
```

---

## Cycle 3: CLI Integration & Performance (Week 11, Days 4-5)

### Objective
Integrate with ELO CLI and optimize performance.

### RED Phase: Write CLI Integration Tests

**File**: `tests/cli_integration.rs`

```rust
#[test]
fn test_cli_compile_basic() {
    let output = run_cli("elo compile --target rust --input rules.elo");
    assert!(output.status.success());
    assert!(output.stdout.contains("pub fn validate"));
}

#[test]
fn test_cli_compile_with_output() {
    let output = run_cli(
        "elo compile --target rust --input rules.elo --output validator.rs"
    );
    assert!(output.status.success());
    assert!(std::path::Path::new("validator.rs").exists());
}

#[test]
fn test_cli_compile_with_type_hints() {
    let output = run_cli(
        "elo compile --target rust --input rules.elo --types types.json"
    );
    assert!(output.status.success());
}
```

### GREEN Phase: Create CLI Command

**Update `src/bin/elo-compile.rs`:**

```rust
use clap::Parser;
use std::fs;
use elo_rust::RustCodeGenerator;

#[derive(Parser)]
struct Args {
    /// Input ELO file
    #[arg(long)]
    input: String,

    /// Output Rust file
    #[arg(long)]
    output: Option<String>,

    /// Type hints JSON file
    #[arg(long)]
    types: Option<String>,

    /// Target code generation (always 'rust')
    #[arg(long, default_value = "rust")]
    target: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Read ELO file
    let elo_content = fs::read_to_string(&args.input)?;

    // Generate Rust code
    let generator = RustCodeGenerator::new();
    let generated = generator.generate(&elo_content)?;

    // Write output
    let output_path = args.output.unwrap_or_else(|| "validator.rs".to_string());
    fs::write(output_path, generated.to_string())?;

    Ok(())
}
```

### REFACTOR Phase: Performance Optimization

**Run benchmarks:**

```bash
cargo bench --bench validation_performance

# Optimize hot paths:
# - Regex compilation caching
# - Inline critical functions
# - Reduce allocations
```

### CLEANUP Phase: Verify Performance & CLI

```bash
cargo build --release --bin elo-compile
cargo bench
cargo clippy --all-targets
```

**Commit:**
```
feat(cli): add CLI integration and performance optimization [Phase 4, Cycle 3: CLEANUP]

## Changes
- Implemented CLI binary for code generation
- Added command-line argument parsing
- Integrated with ELO compilation pipeline
- Optimized hot paths for performance
- Added 10+ CLI integration tests
- Verified <1µs validation latency

## Verification
✅ CLI compiles and runs
✅ Generated code from CLI works correctly
✅ Performance benchmarks green (<1µs)
✅ Zero Clippy warnings
```

---

## Cycle 4: Documentation & Polish (Week 12)

### Objective
Create comprehensive documentation and polish the public API.

### RED Phase: Documentation Tests

**File**: `tests/doc_tests.rs`

```rust
//! Documentation examples should be runnable

/// Create a simple validator
///
/// # Example
///
/// ```
/// use elo_rust::RustCodeGenerator;
///
/// let gen = RustCodeGenerator::new();
/// // let code = gen.generate("user.age >= 18")?;
/// ```
#[test]
fn test_readme_example() {
    // README examples should work
}
```

### GREEN Phase: Write Documentation

**Update `README.md`** with:
- Quick start guide
- Code examples
- Framework integration guides
- API reference
- Troubleshooting

**Add doc comments to all public items:**

```rust
/// Generates Rust code from ELO expression
///
/// # Arguments
///
/// * `expression` - The ELO validation expression
///
/// # Returns
///
/// A `TokenStream` with the generated Rust validator
///
/// # Example
///
/// ```ignore
/// let gen = RustCodeGenerator::new();
/// let tokens = gen.generate("user.age >= 18")?;
/// ```
pub fn generate(&self, expression: &str) -> Result<TokenStream, CodeGenError> {
    // ...
}
```

### REFACTOR Phase: API Polish

- Review public API for consistency
- Add convenience functions
- Improve error types
- Standardize naming

### CLEANUP Phase: Generate & Verify Docs

```bash
cargo doc --no-deps --open
cargo test --doc
```

**Commit:**
```
docs(all): comprehensive documentation and API polish [Phase 4, Cycle 4: CLEANUP]

## Changes
- Added comprehensive README with quick start
- Documented all public API items
- Created framework integration guides
- Added inline code examples
- Created troubleshooting section
- Improved error messages

## Verification
✅ `cargo doc` builds without warnings
✅ Doc tests pass
✅ Examples are clear and runnable
✅ API is consistent and intuitive
```

---

## Dependencies

**Requires**:
- Phase 3 complete (all stdlib functions working)

**Provides**:
- Complete, production-ready implementation for Phase 5

---

## Framework Checklist

Test integration with:
- [ ] Actix-web (examples working)
- [ ] Axum (examples working)
- [ ] Tokio (async compatibility verified)
- [ ] Error handling (proper JSON responses)
- [ ] Request/response cycles (end-to-end tests)

---

## Documentation Checklist

- [ ] README with quick start
- [ ] API documentation complete
- [ ] All public items documented
- [ ] Code examples working
- [ ] Framework guides written
- [ ] Troubleshooting section
- [ ] CLI usage guide

---

## Testing Checklist

Before moving to Phase 5:

- [ ] All macro tests passing
- [ ] Framework integration tests passing
- [ ] CLI tests passing
- [ ] Doc tests passing
- [ ] Performance benchmarks green
- [ ] 50+ new tests passing
- [ ] Zero Clippy warnings
- [ ] Documentation builds clean

---

**Next Phase**: [Phase 5: Finalization](./phase-05-finalize.md)
