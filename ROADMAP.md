# ELO Rust Target: Implementation Roadmap

**Status**: Ready for ELO team review and discussion
**Timeline**: 13 weeks (13 sprints, 1 week each)
**Team**: 1-2 engineers + feedback from ELO core team

---

## Phase 1: Planning & Design (Weeks 1-2)

### Week 1: Architecture Finalization
**Goal**: Design agreement with ELO team

**Deliverables:**
- [ ] Design review session with Bernard & ELO core
- [ ] Finalize type system mapping (ELO ↔ Rust)
- [ ] Agree on error handling approach
- [ ] Create architecture ADR (Architecture Decision Records)
- [ ] Define success metrics

**Artifacts:**
- ADR: Type System Mapping
- ADR: Error Handling Strategy
- Type compatibility matrix

**Key Questions:**
- How do we handle user-defined types?
- What's the error serialization format?
- Should we generate documentation from ELO rules?

### Week 2: Infrastructure & Testing Strategy
**Goal**: Set up development environment

**Deliverables:**
- [ ] Create GitHub repository
- [ ] Set up CI/CD pipeline (GitHub Actions)
- [ ] Create test framework structure
- [ ] Design testing strategy document
- [ ] Create development build process

**Artifacts:**
- Repository layout (finalized)
- CI/CD configuration
- Testing harness
- Build script

**Key Decisions:**
- Code generation approach (quote! vs. string building)
- Test file organization
- Performance benchmark framework

---

## Phase 2: Core Implementation (Weeks 3-6)

### Week 3: Code Generator Skeleton
**Goal**: Basic infrastructure for code generation

**Deliverables:**
- [ ] Implement basic code generator struct
- [ ] Create simple visitor pattern for AST traversal
- [ ] Generate function signature from ELO rule
- [ ] Handle literal values (strings, numbers, booleans)
- [ ] 20+ unit tests

**Artifacts:**
```rust
pub struct RustCodeGenerator {
    type_context: TypeContext,
    // ...
}

impl RustCodeGenerator {
    pub fn generate(&self, rule: &Rule) -> TokenStream {
        // Generated code
    }
}
```

**Tests:**
- Literal generation (strings, numbers, dates)
- Function signature generation
- Basic module structure

### Week 4: Binary Operators & Simple Expressions
**Goal**: Handle basic comparisons and logical operators

**Deliverables:**
- [ ] Implement comparison operators (<, >, ==, !=, <=, >=)
- [ ] Implement logical operators (&&, ||, !)
- [ ] Handle short-circuit evaluation
- [ ] Implement field access (user.age)
- [ ] 40+ unit tests

**Artifacts:**
```rust
// Generated for: user.age >= 18 && user.verified == true
let result = (user.age >= 18i32) && (user.verified == true);
```

**Tests:**
- All comparison operators
- Logical operators
- Operator precedence
- Field access on nested objects
- Type coercion

### Week 5: Function Calls & Type Handling
**Goal**: Support stdlib function calls

**Deliverables:**
- [ ] Implement function call code generation
- [ ] Add type annotations/hints
- [ ] Generate type conversions where needed
- [ ] Implement basic string functions (matches, contains)
- [ ] 50+ unit tests

**Artifacts:**
```rust
// Generated for: user.email matches pattern
let email_regex = regex::Regex::new(pattern)?;
email_regex.is_match(&user.email)
```

**Tests:**
- Function call generation
- Type conversions
- Regex pattern generation
- String function calls
- Error handling for invalid functions

### Week 6: Error Type Generation & Result Wrapping
**Goal**: Generate proper error handling

**Deliverables:**
- [ ] Generate ValidationError struct
- [ ] Implement Result-based error handling
- [ ] Support error message interpolation
- [ ] Handle error aggregation
- [ ] 30+ unit tests

**Artifacts:**
```rust
pub struct ValidationError {
    pub path: String,
    pub message: String,
    pub rule: String,
}

// Generated validator signature:
pub fn validate_user(user: &User) -> Result<(), Vec<ValidationError>>
```

**Tests:**
- Error struct generation
- Result type wrapping
- Multiple error collection
- Error message formatting

---

## Phase 3: Standard Library (Weeks 7-9)

### Week 7: String & Array Functions
**Goal**: Implement stdlib functions for strings and arrays

**Deliverables:**
- [ ] String functions: matches, contains, length, uppercase, lowercase
- [ ] Array functions: contains, any, all, length, is_empty
- [ ] Test string regex patterns
- [ ] Test array predicates
- [ ] 40+ unit tests

**Artifacts:**
```rust
// In crates/fraiseql-core/src/validation/stdlib/string.rs
pub fn string_matches(s: &str, pattern: &str) -> Result<bool>
pub fn string_contains(s: &str, substr: &str) -> bool
pub fn string_length(s: &str) -> usize

// Generated usage:
if string_matches(&user.email, email_pattern)? {
    // valid
}
```

**Tests:**
- String matching (regex)
- String operations (contains, length)
- Array operations (contains, filter)
- Type-specific tests

### Week 8: Date/Time Functions
**Goal**: Support temporal validators

**Deliverables:**
- [ ] Date functions: today(), now(), age(), date_range()
- [ ] Duration parsing (P30D format)
- [ ] Timezone handling
- [ ] Age calculations
- [ ] 40+ unit tests

**Artifacts:**
```rust
// Generated for: user.birthDate < today()
{
    use chrono::{Local, NaiveDate};
    user.birth_date < Local::now().date_naive()
}

// Generated for: age(user.birthDate) >= 18
{
    use chrono::Local;
    let age = calculate_age(user.birth_date, Local::now().date_naive());
    age >= 18u32
}
```

**Tests:**
- Date parsing and comparison
- Age calculations
- Duration parsing
- Timezone edge cases
- Leap year handling

### Week 9: Type Checking & Coercion
**Goal**: Handle optional values and type conversions

**Deliverables:**
- [ ] Functions: is_null, is_some, is_empty
- [ ] Option<T> unwrapping
- [ ] Type-safe conversions
- [ ] Default value handling
- [ ] 30+ unit tests

**Artifacts:**
```rust
// Generated for: user.middleName != null && user.middleName matches pattern
if let Some(middle_name) = &user.middle_name {
    if !middle_name.is_empty() {
        regex.is_match(middle_name)
    } else {
        false
    }
} else {
    false
}
```

**Tests:**
- Optional value handling
- Type coercion
- Null/None semantics
- Default values

---

## Phase 4: Integration & Ergonomics (Weeks 10-12)

### Week 10: Derive Macro Implementation
**Goal**: Provide ergonomic macro interface

**Deliverables:**
- [ ] Implement #[elo_validator] derive macro
- [ ] Implement #[elo_guard] macro
- [ ] Support attribute parsing
- [ ] Generate impl blocks
- [ ] 20+ macro tests

**Artifacts:**
```rust
#[elo_validator(
    elo = r#"
        user.email matches email_pattern &&
        user.age >= 18
    "#,
    error_type = "ValidationError"
)]
pub struct UserValidator;

// Expands to impl with validate() method
```

**Tests:**
- Macro attribute parsing
- Code generation from macro
- Multiple validator definitions
- Error handling in macros

### Week 11: CLI Integration & Documentation
**Goal**: Make it usable from command line

**Deliverables:**
- [ ] Implement `elo compile --target rust` subcommand
- [ ] Support .elo file compilation
- [ ] Output to .rs file
- [ ] Generate documentation from rules
- [ ] Create example projects

**Artifacts:**
```bash
# Usage:
elo compile --target rust validation.elo --output validation.rs

# Or with schema:
elo compile --target rust --with-types schema.graphql validation.elo
```

**Tests:**
- CLI argument parsing
- File I/O
- Error reporting
- Output validation

### Week 12: Performance Optimization & Benchmarking
**Goal**: Ensure <1µs validation latency

**Deliverables:**
- [ ] Create comprehensive benchmarks
- [ ] Profile generated code
- [ ] Optimize hot paths
- [ ] Verify zero-cost abstractions
- [ ] Document performance characteristics
- [ ] 10+ benchmark tests

**Artifacts:**
```rust
#[bench]
fn bench_simple_comparison(b: &mut Bencher) {
    let user = User { age: 25, ... };
    b.iter(|| validate_age(&user));
    // Expected: <1µs
}

#[bench]
fn bench_email_validation(b: &mut Bencher) {
    let user = User { email: "test@example.com", ... };
    b.iter(|| validate_email(&user));
    // Expected: <100ns (regex compiled once)
}
```

**Tests:**
- Latency benchmarks
- Memory usage tests
- Compilation time tests
- Comparison with hand-written validators

---

## Phase 5: Upstream Integration (Week 13)

### Week 13: Merge & Release Preparation
**Goal**: Prepare for official ELO integration

**Deliverables:**
- [ ] Code review with ELO team
- [ ] Address feedback & iterate
- [ ] Finalize documentation
- [ ] Create examples & templates
- [ ] Prepare for crates.io publication
- [ ] Write announcement

**Artifacts:**
- GitHub PR to ELO repository
- crates.io package (elo-rust)
- Documentation pages
- 3-5 example projects
- Announcement blog post

**Examples to Create:**
1. **Simple API Validator** (actix-web)
   - Input validation before database
   - Error response formatting

2. **GraphQL Validator** (async-graphql)
   - Mutation input validation
   - Integration with resolvers

3. **Data Pipeline** (tokio)
   - Batch validation
   - Error aggregation

4. **Derive Macro** (procedural macro example)
   - Using #[elo_validator]
   - Custom error types

5. **Two-Layer Validation** (FraiseQL)
   - Framework-level validation (Rust)
   - Database-level validation (SQL)

---

## Parallel Work Streams

### Code Generation Engine (Weeks 3-12)
- Expression handling
- Type system
- Error generation
- Function calls

### Standard Library (Weeks 7-9)
- String operations
- Date/time functions
- Collection functions
- Type checking

### Integration (Weeks 10-12)
- Derive macros
- CLI tools
- Documentation
- Performance

---

## Milestones & Go/No-Go Gates

### Milestone 1: Core Generation (End of Week 6)
**Gate Criteria:**
- [ ] All operators generate valid code
- [ ] Type system working
- [ ] 150+ unit tests passing
- [ ] Simple validators compiling
- [ ] **Go/No-Go Decision**: Can generate basic validators?

### Milestone 2: Stdlib Complete (End of Week 9)
**Gate Criteria:**
- [ ] All stdlib functions working
- [ ] 200+ unit tests passing
- [ ] Integration tests with frameworks
- [ ] Performance benchmarks show <1µs
- [ ] **Go/No-Go Decision**: Ready for ergonomic layer?

### Milestone 3: Ergonomic Interface (End of Week 12)
**Gate Criteria:**
- [ ] Macros working smoothly
- [ ] CLI fully functional
- [ ] 250+ tests passing
- [ ] Example projects verified
- [ ] **Go/No-Go Decision**: Ready for upstream merge?

### Milestone 4: Upstream Release (End of Week 13)
**Gate Criteria:**
- [ ] Merged to ELO repository
- [ ] Published to crates.io
- [ ] Documentation complete
- [ ] Examples working
- [ ] Community feedback incorporated
- [ ] **Success**: Rust target is official ELO feature

---

## Resource Allocation

### FraiseQL Contribution
- 1-2 engineers (primary implementation)
- Code review capacity
- Example project creation
- Documentation writing

### ELO Team
- Architecture review
- Design feedback
- Type system guidance
- Upstream merge review
- Marketing/announcement

### Community
- Early testing
- Feedback
- Example contributions
- Real-world usage

---

## Risk Management

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|-----------|
| Type system complexity | Medium | High | Start with simple types, iterate |
| Performance regression | Low | High | Benchmark frequently |
| Async/await challenges | Medium | Medium | Collaborate with async community |
| Maintenance burden | Low | Medium | Share with ELO team |
| Adoption friction | Low | Medium | Create templates & docs |
| Upstream merge delays | Low | Medium | Start PR early, iterate quickly |

---

## Success Metrics

### By Week 13
- [ ] 300+ unit tests passing
- [ ] 50+ integration tests passing
- [ ] <1µs validation latency (benchmarks)
- [ ] Zero Clippy warnings
- [ ] 100% documentation coverage
- [ ] 5+ example projects working
- [ ] Merged to ELO repository
- [ ] Published to crates.io

### By Month 3
- [ ] 50+ downloads/month
- [ ] 3+ public projects using it
- [ ] Zero critical bugs
- [ ] Positive community feedback

### By Month 6
- [ ] 200+ downloads/month
- [ ] 10+ public projects adopting it
- [ ] Contributing examples from community
- [ ] Feature requests for Phase 2

---

## Next Steps

1. **This Week**: Share PRD + Architecture with Bernard Lambeau
2. **Next Week**: Schedule design discussion with ELO team
3. **Week 2**: Finalize architecture, create ADRs
4. **Week 3**: Begin Phase 2 implementation
5. **Week 13**: Merge to ELO, publish to crates.io

---

## Questions for ELO Team

1. Should we fork ELO or create separate repository initially?
2. What's your preferred contribution process (PR flow)?
3. Are there existing type system utilities we should use?
4. Should we optimize for compile time or runtime?
5. What's the preferred error serialization format?
6. Any preferences on macro implementation approach?

---

## Document History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | Feb 8, 2026 | Initial roadmap for ELO team |

