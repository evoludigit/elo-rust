# Phase 2: Core Code Generator

**Duration**: Weeks 3-6
**Objective**: Implement the core ELO-to-Rust code generation engine handling expressions, operators, and basic function calls
**Team**: 1-2 engineers
**Status**: [ ] Not Started | [ ] In Progress | [ ] Complete

---

## Success Criteria

- [ ] Code generator traverses ELO AST correctly
- [ ] All comparison operators work (<, >, ==, !=, <=, >=)
- [ ] All logical operators work (&&, ||, !)
- [ ] Field access generates correct code (user.age)
- [ ] Short-circuit evaluation implemented
- [ ] Literal values (numbers, strings, booleans, dates) compile correctly
- [ ] Generated code is valid, executable Rust
- [ ] 150+ unit tests passing
- [ ] <1µs validation latency achieved
- [ ] Zero Clippy warnings
- [ ] Ready for Phase 3: Standard Library

---

## Cycle 1: AST Visitor & Code Generation Skeleton (Week 3)

### Objective
Build the foundation for traversing ELO AST and emitting Rust tokens.

### RED Phase: Write Failing Tests

**File**: `tests/codegen/ast_visitor.rs`

```rust
#[test]
fn test_generator_creation() {
    let gen = RustCodeGenerator::new();
    assert!(gen.is_valid());
}

#[test]
fn test_simple_literal_generation() {
    let gen = RustCodeGenerator::new();
    let tokens = gen.generate_literal(42).unwrap();
    assert!(!tokens.is_empty());
}

#[test]
fn test_function_signature_generation() {
    let gen = RustCodeGenerator::new();
    let sig = gen.function_signature("validate_user", "User").unwrap();
    assert!(sig.contains("fn validate_user"));
}
```

### GREEN Phase: Implement AST Visitor

**Update `src/codegen/mod.rs`:**

```rust
pub struct RustCodeGenerator {
    type_context: TypeContext,
}

impl RustCodeGenerator {
    pub fn generate(&self, expr: &EloExpression) -> Result<TokenStream, CodeGenError> {
        match expr {
            EloExpression::Literal(val) => self.literal(val),
            EloExpression::FieldAccess(recv, field) => self.field_access(recv, field),
            EloExpression::BinaryOp(left, op, right) => {
                self.binary_op(left, op, right)
            }
            _ => Err(CodeGenError::Unsupported),
        }
    }

    fn literal(&self, value: &LiteralValue) -> Result<TokenStream, CodeGenError> {
        Ok(match value {
            LiteralValue::String(s) => quote!(#s),
            LiteralValue::Number(n) => quote!(#n),
            LiteralValue::Bool(b) => quote!(#b),
        })
    }

    fn field_access(&self, receiver: &str, field: &str) -> Result<TokenStream, CodeGenError> {
        let receiver_ident = syn::parse_str::<syn::Ident>(receiver)?;
        let field_ident = syn::parse_str::<syn::Ident>(field)?;
        Ok(quote!(#receiver_ident.#field_ident))
    }
}
```

### REFACTOR Phase: Improve Visitor Pattern

- Extract visitor methods into separate impl blocks
- Create trait for expression generation
- Improve error handling and reporting

### CLEANUP Phase: Verify and Test

```bash
cargo test codegen::ast_visitor
cargo clippy
```

**Commit:**
```
feat(codegen): implement basic AST visitor and literal generation [Phase 2, Cycle 1: CLEANUP]

## Changes
- Created RustCodeGenerator with visitor pattern
- Implemented literal value generation
- Added field access code generation
- Added 20+ AST visitor tests
- Improved error messages

## Verification
✅ Literal tests pass
✅ Field access works
✅ AST visitor pattern clean
✅ Zero Clippy warnings
```

---

## Cycle 2: Comparison & Arithmetic Operators (Week 4, Days 1-3)

### Objective
Generate code for comparison and arithmetic operators with proper type handling.

### RED Phase: Write Operator Tests

**File**: `tests/codegen/operators.rs`

```rust
#[test]
fn test_equal_operator() {
    let result = generate("user.age == 18").unwrap();
    assert!(result.to_string().contains("=="));
}

#[test]
fn test_less_than_operator() {
    let result = generate("user.age < 18").unwrap();
    assert!(result.to_string().contains("<"));
}

#[test]
fn test_greater_equal_operator() {
    let result = generate("user.age >= 21").unwrap();
    assert!(result.to_string().contains(">="));
}

#[test]
fn test_arithmetic_addition() {
    let result = generate("age + 5").unwrap();
    assert!(result.to_string().contains("+"));
}

#[test]
fn test_arithmetic_operators() {
    generate("value - 10").unwrap();
    generate("count * 2").unwrap();
    generate("total / 3").unwrap();
    generate("remainder % 4").unwrap();
}
```

### GREEN Phase: Implement Operators

**Update `src/codegen/operators.rs`:**

```rust
impl OperatorGenerator {
    pub fn binary(&self, op: BinaryOp, left: TokenStream, right: TokenStream) -> TokenStream {
        match op {
            BinaryOp::Equal => quote!(#left == #right),
            BinaryOp::NotEqual => quote!(#left != #right),
            BinaryOp::Less => quote!(#left < #right),
            BinaryOp::Greater => quote!(#left > #right),
            BinaryOp::LessEqual => quote!(#left <= #right),
            BinaryOp::GreaterEqual => quote!(#left >= #right),
            BinaryOp::Add => quote!(#left + #right),
            BinaryOp::Subtract => quote!(#left - #right),
            BinaryOp::Multiply => quote!(#left * #right),
            BinaryOp::Divide => quote!(#left / #right),
            BinaryOp::Modulo => quote!(#left % #right),
            _ => quote!(()),
        }
    }

    pub fn unary(&self, op: UnaryOp, operand: TokenStream) -> TokenStream {
        match op {
            UnaryOp::Not => quote!(!#operand),
            UnaryOp::Negate => quote!(-#operand),
        }
    }
}
```

### REFACTOR Phase: Add Type Checking

- Verify operand types match operator requirements
- Add compile-time checks for type compatibility
- Improve error messages for type mismatches

### CLEANUP Phase: Test All Operators

```bash
cargo test codegen::operators
cargo clippy
```

**Commit:**
```
feat(codegen): implement comparison and arithmetic operators [Phase 2, Cycle 2: CLEANUP]

## Changes
- Implemented all comparison operators (==, !=, <, >, <=, >=)
- Implemented arithmetic operators (+, -, *, /, %)
- Implemented unary operators (!, -)
- Added operator type validation
- Added 25+ operator tests

## Verification
✅ All operator tests pass
✅ Generated code is valid Rust
✅ Type checking works
✅ Zero Clippy warnings
```

---

## Cycle 3: Logical Operators & Short-Circuit Evaluation (Week 4, Days 4-5)

### Objective
Implement logical operators (&&, ||) with proper short-circuit semantics.

### RED Phase: Write Logical Operator Tests

**File**: `tests/codegen/logical_ops.rs`

```rust
#[test]
fn test_logical_and() {
    let result = generate("user.verified && user.age >= 18").unwrap();
    assert!(result.contains("&&"));
}

#[test]
fn test_logical_or() {
    let result = generate("admin || moderator").unwrap();
    assert!(result.contains("||"));
}

#[test]
fn test_logical_not() {
    let result = generate("!user.banned").unwrap();
    assert!(result.contains("!"));
}

#[test]
fn test_short_circuit_and() {
    // Verify that && short-circuits on first false
    let code = generate("false && expensive_check()").unwrap();
    // Should generate: false && expensive_check()
    // Rust automatically short-circuits
}

#[test]
fn test_complex_logical_expression() {
    let result = generate(
        "user.verified && (user.age >= 18 || user.age < 13)"
    ).unwrap();
    assert!(result.contains("&&"));
    assert!(result.contains("||"));
}
```

### GREEN Phase: Implement Logical Operators

**Update `src/codegen/operators.rs`:**

```rust
impl OperatorGenerator {
    pub fn binary(&self, op: BinaryOp, left: TokenStream, right: TokenStream) -> TokenStream {
        match op {
            // ... existing operators ...
            BinaryOp::And => quote!(#left && #right),
            BinaryOp::Or => quote!(#left || #right),
        }
    }
}
```

### REFACTOR Phase: Add Operator Precedence

- Handle parenthesization correctly
- Ensure operator precedence matches ELO spec
- Document precedence rules

### CLEANUP Phase: Verify Short-Circuit

```bash
cargo test codegen::logical_ops
cargo clippy
# Test that short-circuit evaluation works correctly
```

**Commit:**
```
feat(codegen): implement logical operators with short-circuit evaluation [Phase 2, Cycle 3: CLEANUP]

## Changes
- Implemented logical AND (&&) with short-circuit
- Implemented logical OR (||) with short-circuit
- Added operator precedence handling
- Added 15+ logical operator tests
- Verified Rust compiler handles short-circuiting

## Verification
✅ All logical operator tests pass
✅ Short-circuit evaluation verified
✅ Precedence correct
✅ Zero Clippy warnings
```

---

## Cycle 4: Complex Expressions & Full Integration (Week 5-6)

### Objective
Test and optimize complete expression generation with real-world scenarios.

### RED Phase: Write Integration Tests

**File**: `tests/codegen/integration.rs`

```rust
#[test]
fn test_user_validation_expression() {
    let elo = r#"
        user.email matches pattern &&
        user.age >= 18 &&
        user.verified == true
    "#;
    let code = generate(elo).unwrap();
    assert_compiles(&code);
}

#[test]
fn test_complex_nested_expression() {
    let elo = r#"
        (admin && verified) ||
        (user && !banned && age >= 21)
    "#;
    let code = generate(elo).unwrap();
    assert_compiles(&code);
}

#[test]
fn test_mixed_operators() {
    let elo = "count >= 1 && count <= 100";
    let code = generate(elo).unwrap();
    assert_compiles(&code);
}
```

### GREEN Phase: Implement Full Expression Generation

Complete `src/codegen/mod.rs`:

```rust
pub fn generate(&self, expr: &EloExpression) -> Result<TokenStream, CodeGenError> {
    // Handle all expression types
    // Return valid Rust tokens
}
```

### REFACTOR Phase: Optimize & Clean Up

- Remove unnecessary parentheses where safe
- Inline simple expressions
- Improve code readability of generated code

### CLEANUP Phase: Full Integration Test

```bash
cargo test codegen::integration --lib
cargo clippy --all-targets --all-features -- -D warnings

# Create simple example and verify it compiles
cargo check --example simple_validator
```

**Commit:**
```
feat(codegen): complete expression generation and integration [Phase 2, Cycle 4: CLEANUP]

## Changes
- Implemented full expression tree traversal
- Added support for nested expressions
- Integrated all operator types
- Added 30+ integration tests
- Verified generated code compiles and runs
- Optimized token stream generation

## Verification
✅ All expression tests pass
✅ Generated code is valid Rust
✅ Real-world expressions work
✅ Performance meets <1µs target
✅ Zero Clippy warnings
✅ Ready for Phase 3
```

---

## Dependencies

**Requires**:
- Phase 1 complete (type system, error handling, testing infrastructure)

**Provides**:
- Working code generator for Phase 3 (stdlib functions)
- Foundation for Phase 4 (derive macros)

---

## Performance Targets

- **Code Generation**: <100ms per expression
- **Compiled Code Execution**: <1µs per validation
- **Generated Code Size**: <50 lines for typical validator

### Benchmarking

```bash
cargo bench --bench validation_performance

# Expected results:
# simple_comparison: < 10ns
# complex_expression: < 100ns
# regex_matching: < 1µs
```

---

## Key Implementation Notes

1. **Use quote!** - Leverage proc-macro2::quote! for clean token generation
2. **Type Safety** - Verify types at code generation time when possible
3. **Short-Circuit** - Rust's `&&` and `||` automatically short-circuit
4. **No Runtime** - Generated code should have zero runtime overhead
5. **Error Messages** - Report clear errors for unsupported constructs

---

## Testing Checklist

Before moving to Phase 3:

- [ ] All operator tests pass
- [ ] Integration tests pass
- [ ] Generated code is valid Rust
- [ ] Performance benchmarks green
- [ ] Clippy warnings zero
- [ ] 150+ tests passing
- [ ] Documentation complete
- [ ] Example code runs successfully

---

**Next Phase**: [Phase 3: Standard Library Functions](./phase-03-stdlib.md)
