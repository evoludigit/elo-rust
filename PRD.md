# Product Requirements Document: ELO Rust Code Generation Target

**Date**: February 8, 2026
**Status**: Proposal
**Audience**: ELO Core Team, Rust Ecosystem Stakeholders
**Champion**: FraiseQL Project

---

## Executive Summary

This document proposes extending ELO with a **Rust code generation target**, enabling developers to compile ELO validation expressions directly to type-safe, zero-overhead Rust functions. This addition would:

1. **Eliminate FFI barriers** - No JavaScript/Ruby bridging needed for Rust projects
2. **Serve validation-critical systems** - Framework-level validation before database execution
3. **Expand ELO's ecosystem** - Rust projects (actix, tokio, axum, tonic, etc.) gain ELO benefits
4. **Maintain ELO philosophy** - Multi-target compilation with semantically equivalent output

**Proposed Status**: Professional feature, production-ready, community-contributed

---

## Problem Statement

### Current State

ELO successfully compiles to JavaScript, Ruby, Python, and SQL. However, **Rust projects have no direct path**:

- ❌ Cannot use ELO for framework-level validation without FFI overhead
- ❌ Must choose between custom parsers or runtime evaluation
- ❌ High-performance Rust services (GraphQL, APIs) lose ELO benefits
- ❌ Validation logic duplicated across languages

### Impact

- Rust ecosystem (growing rapidly, increasingly used for systems programming)
- Teams building high-performance APIs/services
- Projects needing framework-level validation (before database)
- Organizations wanting single validation language across services

### Example: FraiseQL Use Case

A GraphQL engine needs to validate mutations before database execution:

```rust
// WITHOUT Rust target (current):
pub async fn create_user(input: CreateUserInput) -> Result<User> {
    // Hand-written validation logic
    if input.email.is_empty() || !input.email.contains('@') {
        return Err(ValidationError::InvalidEmail);
    }
    if input.age < 18 {
        return Err(ValidationError::UnderageUser);
    }
    if input.start_date >= input.end_date {
        return Err(ValidationError::InvalidDateRange);
    }
    // ... more validators
    db.create_user(input).await
}

// WITH Rust target (proposed):
#[elo_validator]
pub async fn create_user(input: CreateUserInput) -> Result<User> {
    // ELO rule: user.email matches email_pattern && user.age >= 18 && user.startDate < user.endDate
    // ↓ Compiles to optimized Rust validators
    create_user_validator(&input)?;  // Zero-cost abstraction
    db.create_user(input).await
}
```

---

## Vision & Strategic Value

### What This Enables

#### 1. **Framework-Level Validation (Performance)**
```
GraphQL Request
    ↓ [LAYER 1: ELO-compiled Rust validators]
    ↓ Fast rejection of invalid data at API boundary
    ↓ Never hits database
Validated Data
    ↓ [LAYER 2: SQL constraints from same ELO rules]
    ↓ Defense-in-depth
Database (Double-validated data only)
```

#### 2. **Ecosystem Expansion**
- **Actix-web** projects gain built-in validation
- **Tokio** async services validate efficiently
- **Axum** routers protect with ELO guards
- **Tonic** gRPC services enforce contracts
- **Rocket** APIs use type-safe validators

#### 3. **Single Source of Truth**
```elo
// One ELO rule... compiles to all targets:
user.email matches email_pattern &&
user.birthDate >= date("1900-01-01") &&
user.birthDate <= today() &&
user.age >= 18
```

Generates:
- ✅ Rust: Type-safe, compile-time checked validators
- ✅ JavaScript: Frontend validation
- ✅ Python: Data pipeline validation
- ✅ Ruby: Backend validation
- ✅ SQL: Database constraints

---

## Scope: What We're Building

### In Scope (Rust Target)

#### 1. **Core Language Support**
- [ ] Literals: Numbers, strings, dates, times, durations
- [ ] Operators: Comparison (==, !=, <, >, <=, >=), logical (&&, ||, !)
- [ ] Functions: String (matches, contains, length), date (today(), now()), arithmetic
- [ ] Collections: Array operations (contains, any, all), object field access
- [ ] Control flow: Guard statements, if/then/else conditionals
- [ ] Pipeline operators: Forward piping with lambdas
- [ ] Type coercion: Automatic safe conversions

#### 2. **Code Generation**
- [ ] Parse ELO expressions → Rust AST
- [ ] Generate idiomatic Rust code (not bytecode/eval)
- [ ] Type checking: Match Rust's type system
- [ ] Error handling: Return `Result<T, ValidationError>`
- [ ] Performance: Zero-cost abstractions

#### 3. **Integration Features**
- [ ] Derive macros: `#[elo_validator]` for automatic generation
- [ ] Custom type support: User-defined structs/enums
- [ ] Error messages: Localized, actionable feedback
- [ ] Async support: Compatible with tokio/async-std
- [ ] Lazy evaluation: Optimize unnecessary computations

#### 4. **Standard Library**
- [ ] String: `matches()`, `contains()`, `length()`, `uppercase()`, `lowercase()`
- [ ] Date/Time: `today()`, `now()`, `duration()`, comparisons
- [ ] Array: `contains()`, `any()`, `all()`, `filter()`, `map()`
- [ ] Object: Field access, method calls
- [ ] Type checking: `is_null()`, `is_empty()`, `is_string()`

### Out of Scope (Future)

- Custom user-defined functions (Phase 2)
- Mutable state (by design, out of scope forever)
- Side effects (by design, out of scope forever)
- Turing completeness (by design, intentional limitation)

---

## Technical Architecture

### High-Level Design

```
┌─────────────────────────────────┐
│ ELO Source Code                 │
│ user.email matches pattern &&   │
│ user.age >= 18                  │
└────────────┬────────────────────┘
             │
             ↓ (Existing ELO Parser)
┌─────────────────────────────────┐
│ ELO AST                         │
│ (Abstract Syntax Tree)          │
└────────────┬────────────────────┘
             │
             ↓ (NEW: Rust Code Generator)
┌─────────────────────────────────┐
│ Rust Code (Generated)           │
│                                 │
│ pub fn validate(user: &User)    │
│   -> Result<(), ValidationError> │
│ {                               │
│   check_email(&user.email)?;    │
│   check_age(user.age)?;         │
│   Ok(())                        │
│ }                               │
└─────────────────────────────────┘
```

### Implementation Strategy

#### Phase 1: Code Generation Engine
- [ ] Traverse ELO AST
- [ ] Emit Rust tokens via `quote!` macro
- [ ] Handle type mapping (ELO types → Rust types)
- [ ] Generate Result-based error handling

#### Phase 2: Type System Integration
- [ ] Map ELO types to Rust types
- [ ] Support user-defined struct fields
- [ ] Handle Option<T> for nullable fields
- [ ] Type-safe generic validators

#### Phase 3: Standard Library Functions
- [ ] String functions (regex matching, containment)
- [ ] Date/time calculations (age, date ranges)
- [ ] Collection operations (any, all, filter)
- [ ] Custom error messages

#### Phase 4: Integration & Ergonomics
- [ ] Derive macro support (`#[elo_validator]`)
- [ ] CLI integration: `elo compile --target rust`
- [ ] Error formatting
- [ ] Documentation generation

### Code Generation Example

**Input (ELO):**
```elo
user.email matches "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$" &&
user.age >= 18 &&
user.birthDate < today()
```

**Output (Rust):**
```rust
use regex::Regex;
use chrono::Local;

pub fn validate_user(user: &User) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    // Email validation
    let email_pattern = Regex::new(
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    ).expect("Invalid regex");

    if !email_pattern.is_match(&user.email) {
        errors.push("Email must match pattern".to_string());
    }

    // Age validation
    if user.age < 18 {
        errors.push("User must be at least 18 years old".to_string());
    }

    // Date validation
    let today = Local::now().date_naive();
    if user.birth_date >= today {
        errors.push("Birth date must be in the past".to_string());
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
```

---

## Use Cases & Applications

### 1. **GraphQL Input Validation (Primary)**
```rust
// FraiseQL use case: Validate mutations before database
#[elo_validator]
pub async fn create_user(
    input: CreateUserInput,
) -> Result<User, ValidationError> {
    validate_create_user_input(&input)?;
    db.create_user(input).await
}
```

### 2. **Form Validation (APIs)**
```rust
// Actix-web endpoint with ELO validation
#[post("/users")]
async fn create_user(
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>> {
    validate_user_request(&payload)?;
    // Process validated data
}
```

### 3. **Data Pipeline Validation**
```rust
// Process CSV/JSON data with ELO validators
for record in records {
    validate_record(&record)?;  // ELO-generated
    process(record)?;
}
```

### 4. **Type Guards & Assertions**
```rust
// Pre/post conditions using ELO
#[elo_guard(input: "input.quantity > 0")]
pub fn place_order(input: OrderInput) -> Result<Order> {
    // Guaranteed: input.quantity > 0
    // Compiler enforces the guard
}
```

### 5. **Cross-Service Contracts**
```rust
// tonic gRPC with validated inputs
#[elo_validator]
pub async fn process_payment(
    request: PaymentRequest,
) -> Result<PaymentResponse> {
    validate_payment_request(&request)?;
    // Process with confidence
}
```

---

## Dependencies & Requirements

### Language Features
- Rust 1.70+ (stable)
- Standard library (std::result::Result, std::option::Option)
- No unsafe code required
- No external runtime needed

### External Crates (Optional)
- `regex` - For pattern matching (already common)
- `chrono` - For date/time (already common)
- `quote` & `proc-macro2` - For derive macro support
- `serde` - For serialization (optional)

### ELO Dependencies
- Access to ELO parser & AST
- Type system definitions
- Standard library function signatures

---

## Success Criteria

### Functional
- [ ] Compiles ELO expressions to valid, executable Rust code
- [ ] Type safety: Compiler catches errors
- [ ] Performance: <1µs per validation (Rust's zero-cost abstractions)
- [ ] All ELO stdlib functions work in Rust
- [ ] Error messages are actionable and user-friendly

### Quality
- [ ] 100+ unit tests for code generation
- [ ] Integration tests with real Rust projects (actix, tokio)
- [ ] Zero Clippy warnings
- [ ] Comprehensive documentation
- [ ] Example applications

### Ecosystem
- [ ] Merged into official ELO repository
- [ ] Documented on elo-lang.org
- [ ] Published to crates.io
- [ ] Adopted by at least 2-3 major Rust projects
- [ ] Community contributions accepted

### Performance
- [ ] Validation latency <1µs (comparable to hand-written)
- [ ] No runtime overhead vs. native Rust
- [ ] Compile time reasonable (< 5s for typical validators)
- [ ] Memory efficient (no allocations for simple checks)

---

## Timeline & Milestones

### Phase 1: Planning & Design (2 weeks)
- [ ] Finalize architecture with ELO team
- [ ] Design type mapping (ELO ↔ Rust)
- [ ] Plan testing strategy
- [ ] Create architecture decision records

### Phase 2: Core Implementation (4 weeks)
- [ ] Implement code generator skeleton
- [ ] Handle literals and basic operators
- [ ] Generate simple validators
- [ ] Build test suite

### Phase 3: Standard Library (3 weeks)
- [ ] Implement stdlib functions
- [ ] String functions (regex, contains, length)
- [ ] Date/time functions
- [ ] Collection operations
- [ ] Error message formatting

### Phase 4: Integration & Polish (3 weeks)
- [ ] Derive macro support
- [ ] CLI integration
- [ ] Documentation
- [ ] Example projects
- [ ] Performance optimization

### Phase 5: Upstream Merge (1 week)
- [ ] Code review with ELO team
- [ ] Address feedback
- [ ] Merge to main ELO repository

**Total: ~13 weeks**

---

## Collaboration Model

### How This Works With ELO

**FraiseQL proposes:**
1. Design Rust target architecture together
2. Implement Rust code generator (upstream contribution)
3. Maintain as professional feature (not side project)
4. Support community Rust projects using it

**ELO maintains:**
1. Overall vision and direction
2. Code review and quality gates
3. Integration with other targets
4. Official documentation

**Shared benefits:**
- ✅ Rust ecosystem gets professional validation tool
- ✅ ELO becomes genuinely universal (all major languages)
- ✅ FraiseQL gets production-grade validation framework
- ✅ Community has sustainable ecosystem feature

---

## Risk Mitigation

| Risk | Mitigation |
|------|-----------|
| Complexity of Rust codegen | Start with simple expressions, iterate |
| Type system mismatch | Design mapping carefully upfront |
| Async/await challenges | Partner with tokio/async-std teams |
| Performance expectations | Benchmark frequently, optimize early |
| Maintenance burden | Share maintenance with ELO team |
| Adoption friction | Provide templates, examples, docs |

---

## Success Metrics

### Adoption
- [ ] Downloads from crates.io (target: 1000+ in Year 1)
- [ ] GitHub stars on ELO repo grow 50%+
- [ ] At least 5 public projects using Rust target
- [ ] Community PRs and contributions

### Technical
- [ ] Zero security vulnerabilities
- [ ] 100% test coverage for code generation
- [ ] Benchmarks show <1µs validation latency
- [ ] No regression in other ELO targets

### Ecosystem Impact
- [ ] Featured in Rust community (TWIR, etc.)
- [ ] Conference talks about validation approaches
- [ ] Integration examples with popular frameworks
- [ ] Positive community feedback

---

## Next Steps

### Immediate (Week 1)
1. [ ] Share this PRD with Bernard Lambeau & ELO team
2. [ ] Schedule design discussion
3. [ ] Gather feedback on approach
4. [ ] Finalize technical architecture

### Short-term (Weeks 2-4)
5. [ ] Create detailed design specs
6. [ ] Set up development repository
7. [ ] Begin architecture work
8. [ ] Start community engagement

### Medium-term (Weeks 5-13)
9. [ ] Execute implementation phases
10. [ ] Gather feedback from early users
11. [ ] Iterate on design based on learnings
12. [ ] Prepare for upstream merge

---

## FAQ

### Q: Why not use WASM for FFI?
**A:** WASM adds complexity and overhead. Native Rust code is faster, simpler, and idiomatic.

### Q: Will this slow down ELO development?
**A:** No. This is an additive feature. FraiseQL contributes the implementation and maintenance.

### Q: What about existing validation libraries?
**A:** ELO is unique—it compiles to multiple targets. This Rust target makes that promise complete.

### Q: Can users define custom functions?
**A:** Phase 2 feature. Start with stdlib, then add extensibility.

### Q: Is this only for FraiseQL?
**A:** No. This is for the entire Rust ecosystem. FraiseQL is just the initial use case.

---

## Appendix: Example Integration (FraiseQL)

### How FraiseQL Would Use This

**Cycle 7: ELO Rust Target Integration**

```graphql
input CreateUserInput {
  email: String!
  birthDate: DateTime!
  phone: PhoneNumber
  role: UserRole
}

# ELO rule in schema:
@validate(
  elo: """
    email matches email_pattern &&
    birthDate >= date("1900-01-01") &&
    birthDate <= today() &&
    age(birthDate) >= 18 &&
    (role == "user" || (role == "admin" && has_permission("grant_admin")))
  """
)
```

**Compiles to:**

1. **Rust validator** (framework-level, runs before DB):
```rust
pub fn validate_create_user(input: &CreateUserInput) -> Result<()> {
    // ELO-generated validation
}
```

2. **SQL constraint** (database-level, safety net):
```sql
ALTER TABLE users ADD CONSTRAINT
CHECK (
  email ~ '^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$' AND
  birth_date >= '1900-01-01' AND
  birth_date <= CURRENT_DATE AND
  EXTRACT(YEAR FROM AGE(birth_date)) >= 18
);
```

**Result:** Framework-level validation + database constraints from same ELO source.

---

## Document History

| Version | Date | Status | Notes |
|---------|------|--------|-------|
| 1.0 | Feb 8, 2026 | Draft | Initial proposal for ELO team review |

---

**For questions or discussion, contact: Lionel Hamayon (FraiseQL Project)**
