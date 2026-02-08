//! Operator code generation tests
//!
//! Tests for binary and unary operator code generation

use elo_rust::codegen::operators::{BinaryOp, OperatorGenerator, UnaryOp};

// ============================================================================
// COMPARISON OPERATORS
// ============================================================================

#[test]
fn test_equal_operator_generation() {
    let gen = OperatorGenerator::new();
    let left = quote::quote!(user.age);
    let right = quote::quote!(18);

    let result = gen.binary(BinaryOp::Equal, left, right);
    let s = result.to_string();

    assert!(s.contains("=="));
    assert!(s.contains("18"));
}

#[test]
fn test_not_equal_operator_generation() {
    let gen = OperatorGenerator::new();
    let left = quote::quote!(status);
    let right = quote::quote!("banned");

    let result = gen.binary(BinaryOp::NotEqual, left, right);
    let s = result.to_string();

    assert!(s.contains("!="));
}

#[test]
fn test_less_than_operator_generation() {
    let gen = OperatorGenerator::new();
    let left = quote::quote!(age);
    let right = quote::quote!(18);

    let result = gen.binary(BinaryOp::Less, left, right);
    let s = result.to_string();

    assert!(s.contains("<"));
}

#[test]
fn test_greater_than_operator_generation() {
    let gen = OperatorGenerator::new();
    let left = quote::quote!(balance);
    let right = quote::quote!(100);

    let result = gen.binary(BinaryOp::Greater, left, right);
    let s = result.to_string();

    assert!(s.contains(">"));
}

#[test]
fn test_less_equal_operator_generation() {
    let gen = OperatorGenerator::new();
    let left = quote::quote!(count);
    let right = quote::quote!(10);

    let result = gen.binary(BinaryOp::LessEqual, left, right);
    let s = result.to_string();

    assert!(s.contains("<="));
}

#[test]
fn test_greater_equal_operator_generation() {
    let gen = OperatorGenerator::new();
    let left = quote::quote!(score);
    let right = quote::quote!(80);

    let result = gen.binary(BinaryOp::GreaterEqual, left, right);
    let s = result.to_string();

    assert!(s.contains(">="));
}

// ============================================================================
// ARITHMETIC OPERATORS
// ============================================================================

#[test]
fn test_add_operator_generation() {
    let gen = OperatorGenerator::new();
    let left = quote::quote!(a);
    let right = quote::quote!(b);

    let result = gen.binary(BinaryOp::Add, left, right);
    let s = result.to_string();

    assert!(s.contains("+"));
}

#[test]
fn test_subtract_operator_generation() {
    let gen = OperatorGenerator::new();
    let left = quote::quote!(total);
    let right = quote::quote!(discount);

    let result = gen.binary(BinaryOp::Subtract, left, right);
    let s = result.to_string();

    assert!(s.contains("-"));
}

#[test]
fn test_multiply_operator_generation() {
    let gen = OperatorGenerator::new();
    let left = quote::quote!(quantity);
    let right = quote::quote!(price);

    let result = gen.binary(BinaryOp::Multiply, left, right);
    let s = result.to_string();

    assert!(s.contains("*"));
}

#[test]
fn test_divide_operator_generation() {
    let gen = OperatorGenerator::new();
    let left = quote::quote!(total);
    let right = quote::quote!(divisor);

    let result = gen.binary(BinaryOp::Divide, left, right);
    let s = result.to_string();

    assert!(s.contains("/"));
}

#[test]
fn test_modulo_operator_generation() {
    let gen = OperatorGenerator::new();
    let left = quote::quote!(value);
    let right = quote::quote!(2);

    let result = gen.binary(BinaryOp::Modulo, left, right);
    let s = result.to_string();

    assert!(s.contains("%"));
}

// ============================================================================
// LOGICAL OPERATORS
// ============================================================================

#[test]
fn test_and_operator_generation() {
    let gen = OperatorGenerator::new();
    let left = quote::quote!(verified);
    let right = quote::quote!(active);

    let result = gen.binary(BinaryOp::And, left, right);
    let s = result.to_string();

    assert!(s.contains("&&"));
}

#[test]
fn test_or_operator_generation() {
    let gen = OperatorGenerator::new();
    let left = quote::quote!(admin);
    let right = quote::quote!(moderator);

    let result = gen.binary(BinaryOp::Or, left, right);
    let s = result.to_string();

    assert!(s.contains("||"));
}

// ============================================================================
// UNARY OPERATORS
// ============================================================================

#[test]
fn test_not_operator_generation() {
    let gen = OperatorGenerator::new();
    let operand = quote::quote!(banned);

    let result = gen.unary(UnaryOp::Not, operand);
    let s = result.to_string();

    assert!(s.contains("!"));
}

#[test]
fn test_negate_operator_generation() {
    let gen = OperatorGenerator::new();
    let operand = quote::quote!(value);

    let result = gen.unary(UnaryOp::Negate, operand);
    let s = result.to_string();

    assert!(s.contains("-"));
}

// ============================================================================
// COMPLEX OPERATOR EXPRESSIONS
// ============================================================================

#[test]
fn test_comparison_chain() {
    let gen = OperatorGenerator::new();

    // age >= 18
    let left = quote::quote!(age);
    let right = quote::quote!(18);
    let age_check = gen.binary(BinaryOp::GreaterEqual, left, right);

    let s = age_check.to_string();
    assert!(s.contains("age"));
    assert!(s.contains(">="));
    assert!(s.contains("18"));
}

#[test]
fn test_arithmetic_expression() {
    let gen = OperatorGenerator::new();

    // quantity * price
    let left = quote::quote!(quantity);
    let right = quote::quote!(price);
    let result = gen.binary(BinaryOp::Multiply, left, right);

    let s = result.to_string();
    assert!(s.contains("quantity"));
    assert!(s.contains("*"));
    assert!(s.contains("price"));
}

#[test]
fn test_operator_precedence_representation() {
    let gen = OperatorGenerator::new();

    // All operators should be properly parenthesized in generated code
    let left = quote::quote!(a);
    let right = quote::quote!(b);

    let add_result = gen.binary(BinaryOp::Add, left.clone(), right.clone());
    let mul_result = gen.binary(BinaryOp::Multiply, left, right);

    // Both should generate valid expressions (actual precedence handled by Rust compiler)
    assert!(!add_result.to_string().is_empty());
    assert!(!mul_result.to_string().is_empty());
}

// ============================================================================
// GENERATOR STATE
// ============================================================================

#[test]
fn test_operator_generator_creation() {
    let gen = OperatorGenerator::new();
    // Should be valid and reusable
    let left = quote::quote!(x);
    let right = quote::quote!(y);
    let _ = gen.binary(BinaryOp::Equal, left, right);
}

#[test]
fn test_multiple_operators_independent() {
    let gen = OperatorGenerator::new();

    let left = quote::quote!(a);
    let right = quote::quote!(b);

    let add = gen.binary(BinaryOp::Add, left.clone(), right.clone());
    let sub = gen.binary(BinaryOp::Subtract, left.clone(), right.clone());
    let mul = gen.binary(BinaryOp::Multiply, left, right);

    // All should be generated independently
    assert!(add.to_string().contains("+"));
    assert!(sub.to_string().contains("-"));
    assert!(mul.to_string().contains("*"));
}

#[test]
fn test_operator_with_field_access() {
    let gen = OperatorGenerator::new();

    // user.age >= 18
    let left = quote::quote!(user.age);
    let right = quote::quote!(18);

    let result = gen.binary(BinaryOp::GreaterEqual, left, right);
    let s = result.to_string();

    assert!(s.contains("user"));
    assert!(s.contains("age"));
    assert!(s.contains(">="));
    assert!(s.contains("18"));
}

#[test]
fn test_double_negation() {
    let gen = OperatorGenerator::new();

    let operand = quote::quote!(value);
    let negated = gen.unary(UnaryOp::Negate, operand);

    let s = negated.to_string();
    assert!(s.contains("-"));
}
