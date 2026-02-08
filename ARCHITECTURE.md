# ELO Rust Target: Technical Architecture

**Document Version**: 1.0
**Date**: February 8, 2026
**Scope**: Detailed technical design for Rust code generation backend

---

## Overview

The ELO Rust target extends ELO's multi-target compilation framework by adding a native Rust code generator. This document describes the technical approach, design decisions, and implementation strategy.

---

## System Architecture

### Data Flow

```
┌──────────────────────────┐
│ ELO Source Expression    │
│ "user.age >= 18 &&       │
│  user.email matches ..." │
└────────────┬─────────────┘
             │
             ↓ (ELO Parser - EXISTING)
┌──────────────────────────────────────┐
│ ELO AST (Abstract Syntax Tree)       │
│ {                                    │
│   type: "And",                       │
│   left: { type: "GTE", ... },        │
│   right: { type: "Call", ... }       │
│ }                                    │
└────────────┬─────────────────────────┘
             │
             ↓ (Rust Code Generator - NEW)
┌──────────────────────────────────────┐
│ Rust Token Stream (via quote!)       │
│ TokenStream {                        │
│   ... generated token trees ...      │
│ }                                    │
└────────────┬─────────────────────────┘
             │
             ↓ (Rust Compiler)
┌──────────────────────────────────────┐
│ Machine Code (Optimized)             │
│ - Type-safe                          │
│ - Zero-cost abstraction              │
│ - Fully validated at compile-time    │
└──────────────────────────────────────┘
```

---

## Module Structure

```
elo-rust-target/
├── src/
│   ├── lib.rs                 # Public API
│   ├── codegen/
│   │   ├── mod.rs             # Code generator entry point
│   │   ├── types.rs           # Type mapping (ELO ↔ Rust)
│   │   ├── expressions.rs     # Expression code generation
│   │   ├── operators.rs       # Binary/unary operator handling
│   │   ├── functions.rs       # Stdlib function call generation
│   │   └── errors.rs          # Error type generation
│   ├── stdlib/
│   │   ├── mod.rs             # Stdlib function definitions
│   │   ├── string.rs          # String functions
│   │   ├── datetime.rs        # Date/time functions
│   │   ├── array.rs           # Collection functions
│   │   └── types.rs           # Type checking functions
│   ├── macro/
│   │   ├── mod.rs             # Derive macro implementation
│   │   ├── validator.rs       # #[elo_validator] macro
│   │   └── guard.rs           # #[elo_guard] macro
│   └── runtime/
│       ├── mod.rs             # Runtime utilities
│       ├── errors.rs          # Validation error types
│       └── context.rs         # Validation context
├── tests/
│   ├── codegen.rs             # Code generation tests
│   ├── integration.rs         # End-to-end tests
│   └── fixtures/              # Test data
└── Cargo.toml
```

---

## Type System Mapping

### ELO Types → Rust Types

| ELO Type | Rust Type | Notes |
|----------|-----------|-------|
| `string` | `&str` or `String` | Use references in validators |
| `number` | `i64` or `f64` | Inferred from context |
| `boolean` | `bool` | Direct mapping |
| `date` | `chrono::NaiveDate` | ISO 8601 handling |
| `time` | `chrono::NaiveTime` | Time of day |
| `duration` | `chrono::Duration` | Time spans (P30D, etc.) |
| `array<T>` | `&[T]` or `Vec<T>` | References preferred |
| `object` | User-defined `struct` | Via type annotations |
| `null` | `Option<T>` | Explicit nullability |

### Type Inference

```rust
// Input: user.age >= 18
// Type inference:
// - user: &User (context type)
// - user.age: i32 (from struct definition)
// - 18: i32 (inferred from comparison)
// - Result: bool

// Generated:
fn check_age(user: &User) -> bool {
    user.age >= 18i32
}
```

---

## Code Generation Strategy

### Expression Compilation Process

#### 1. **Simple Comparison**

Input ELO:
```elo
user.age >= 18
```

AST:
```rust
BinaryOp {
    op: GreaterEqual,
    left: FieldAccess { receiver: "user", field: "age" },
    right: Literal { value: 18 }
}
```

Generated Rust:
```rust
(user.age >= 18i32)
```

#### 2. **Logical Operators (Short-Circuit)**

Input ELO:
```elo
user.email matches pattern && user.age >= 18
```

Generated Rust:
```rust
email_matches(&user.email) && (user.age >= 18i32)
// Short-circuit: if first check fails, second doesn't execute
```

#### 3. **Function Calls**

Input ELO:
```elo
user.birthDate < today()
```

Generated Rust:
```rust
{
    use chrono::Local;
    user.birth_date < Local::now().date_naive()
}
```

#### 4. **Guard Statements**

Input ELO:
```elo
guard user.status != "banned" as "user must not be banned"
guard user.verified == true as "user must be verified"
```

Generated Rust:
```rust
{
    if user.status == "banned" {
        return Err("user must not be banned".into());
    }
    if !user.verified {
        return Err("user must be verified".into());
    }
    Ok(())
}
```

---

## Error Handling

### Validation Error Types

```rust
// Generated error type
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub path: String,           // "user.email", "items[0].price"
    pub message: String,        // "Email must match pattern"
    pub rule: String,           // "email_pattern"
    pub value: Option<String>,  // For debugging
}

// Or error collection
pub struct ValidationErrors {
    pub errors: Vec<ValidationError>,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.path, self.message)
    }
}
```

### Error Propagation

```rust
// For single validator:
pub fn validate_user(user: &User) -> Result<(), ValidationError> {
    // ...
}

// For multiple validators:
pub fn validate_user(user: &User) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    if !email_valid(&user.email) {
        errors.push(ValidationError {
            path: "user.email".into(),
            message: "Invalid email format".into(),
            rule: "email_pattern".into(),
            value: Some(user.email.clone()),
        });
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
```

---

## Derive Macro Implementation

### #[elo_validator] Macro

```rust
#[elo_validator(
    elo = r#"
        user.email matches email_pattern &&
        user.age >= 18
    "#
)]
pub struct UserValidator;

// Expands to:
impl UserValidator {
    pub fn validate(user: &User) -> Result<(), Vec<ValidationError>> {
        // Generated code from ELO
        // ...
    }
}
```

### Macro Processing

```rust
// Input attributes
#[elo_validator(elo = "...", error_type = "Vec<ValidationError>")]

// Processing steps:
1. Parse ELO attribute value
2. Compile ELO → AST
3. Generate Rust code
4. Emit tokens via quote!
5. Attach to impl block
```

---

## Standard Library Functions

### Function Categories

#### String Functions
```rust
fn matches(s: &str, pattern: &str) -> bool
fn contains(s: &str, substr: &str) -> bool
fn length(s: &str) -> usize
fn uppercase(s: &str) -> String
fn lowercase(s: &str) -> String
fn trim(s: &str) -> &str
fn starts_with(s: &str, prefix: &str) -> bool
fn ends_with(s: &str, suffix: &str) -> bool
```

#### Date/Time Functions
```rust
fn today() -> chrono::NaiveDate
fn now() -> chrono::DateTime<chrono::Utc>
fn age(birth_date: chrono::NaiveDate) -> u32
fn days_since(date: chrono::NaiveDate) -> i64
fn duration_days(days: i64) -> chrono::Duration
```

#### Collection Functions
```rust
fn contains<T: PartialEq>(slice: &[T], value: T) -> bool
fn any<T>(slice: &[T], predicate: fn(&T) -> bool) -> bool
fn all<T>(slice: &[T], predicate: fn(&T) -> bool) -> bool
fn length<T>(slice: &[T]) -> usize
fn is_empty<T>(slice: &[T]) -> bool
```

#### Type Checking
```rust
fn is_null<T>(option: &Option<T>) -> bool
fn is_some<T>(option: &Option<T>) -> bool
fn is_empty(s: &str) -> bool
```

---

## Performance Considerations

### Zero-Cost Abstractions

The generated code should compile to equivalent machine code as hand-written Rust:

```elo
user.age >= 18 && user.email matches pattern
```

Generates:

```rust
// Should compile to efficient assembly:
// - Branch prediction for && short-circuit
// - Inlined field access
// - No heap allocations
// - No runtime overhead vs. hand-written version
```

### Optimization Techniques

1. **Short-circuit evaluation** - `&&` and `||` don't evaluate right if left determines result
2. **Inlining** - Marked with `#[inline]` where appropriate
3. **Static dispatch** - No trait objects, no dynamic dispatch
4. **Zero allocations** - Prefer `&str` over `String`, `&[T]` over `Vec<T>`
5. **Lazy evaluation** - Don't compute values unless needed

### Benchmarking

```rust
#[bench]
fn bench_simple_comparison(b: &mut Bencher) {
    let user = User { age: 25, ... };
    b.iter(|| user.age >= 18);  // Should be <1ns
}

#[bench]
fn bench_email_validation(b: &mut Bencher) {
    let user = User { email: "test@example.com".to_string(), ... };
    b.iter(|| email_matches(&user.email));  // Should be <100ns
}
```

---

## Integration Points

### With ELO Core

1. **Parser**: Reuse existing ELO parser and AST
2. **Type System**: Respect ELO's type system, extend with Rust-specific mappings
3. **Stdlib**: Generate bindings to ELO stdlib definitions
4. **Error Messages**: Use ELO's error formatting when possible

### With Frameworks

#### Actix-web
```rust
#[post("/users")]
async fn create_user(Json(payload): Json<CreateUserRequest>) -> impl Responder {
    match validate_create_user(&payload) {
        Ok(()) => { /* process */ },
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}
```

#### Tokio
```rust
tokio::spawn(async move {
    validate_input(&input)?;
    process_async(input).await
});
```

#### Axum
```rust
async fn handler(
    Path(id): Path<u64>,
    Json(payload): Json<CreateRequest>,
) -> Result<Json<Response>> {
    validate_create_request(&payload)?;
    Ok(Json(process(id, payload)?))
}
```

---

## Testing Strategy

### Unit Tests
- Test individual code generation for each ELO construct
- Test type mapping for all supported types
- Test operator code generation
- Test function call generation

### Integration Tests
- End-to-end: ELO → Rust → Compilation → Execution
- Test with real data structures
- Test with all stdlib functions
- Test error cases

### Benchmark Tests
- Verify <1µs validation latency
- Compare against hand-written code
- Memory usage verification

---

## Deployment & Distribution

### Package Structure

```toml
[package]
name = "elo-rust"
version = "1.0.0"
edition = "2021"

[lib]
name = "elo_rust"

[dependencies]
elo = "1.0"  # ELO core dependency
chrono = "0.4"
regex = "1.10"
quote = "1.0"
proc-macro2 = "1.0"

[dev-dependencies]
criterion = "0.5"
tokio = { version = "1", features = ["full"] }
actix-web = "4"
axum = "0.7"
```

### Distribution

1. **Crates.io**: Primary distribution (published as `elo-rust`)
2. **GitHub**: Source repository (fork/PR into ELO)
3. **Documentation**: elo-lang.org pages
4. **Examples**: Template projects for common frameworks

---

## Future Extensions

### Phase 2+: Custom Functions
```rust
#[elo_function]
pub fn is_premium_user(user: &User) -> bool {
    user.account_type == AccountType::Premium
}

// Use in ELO:
// user.verified && is_premium_user(user)
```

### Phase 2+: Async Validators
```rust
#[elo_validator(async)]
pub async fn validate_email_unique(email: &str) -> Result<()> {
    // Call database to check uniqueness
    db.email_exists(email).await?
}
```

### Phase 3: Macro Integration
```rust
#[elo_guard(input: "input.quantity > 0")]
async fn place_order(input: OrderInput) -> Result<Order> {
    // Guard verified at compile-time
    // Guaranteed: input.quantity > 0
}
```

---

## Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | Feb 8, 2026 | FraiseQL | Initial architecture design |

