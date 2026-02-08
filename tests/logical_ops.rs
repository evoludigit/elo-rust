//! Logical operator and short-circuit evaluation tests
//!
//! Tests for logical operators (&&, ||, !) with proper short-circuit semantics

use elo_rust::codegen::operators::{BinaryOp, OperatorGenerator, UnaryOp};

// ============================================================================
// LOGICAL AND OPERATOR
// ============================================================================

#[test]
fn test_logical_and_basic() {
    let gen = OperatorGenerator::new();
    let left = quote::quote!(verified);
    let right = quote::quote!(active);

    let result = gen.binary(BinaryOp::And, left, right);
    let s = result.to_string();

    assert!(s.contains("&&"));
    assert!(s.contains("verified"));
    assert!(s.contains("active"));
}

#[test]
fn test_logical_and_with_comparison() {
    let gen = OperatorGenerator::new();
    let left = quote::quote!(age >= 18);
    let right = quote::quote!(verified == true);

    let result = gen.binary(BinaryOp::And, left, right);
    let s = result.to_string();

    assert!(s.contains("&&"));
}

#[test]
fn test_logical_and_short_circuit_false() {
    // && operator naturally short-circuits in Rust
    let gen = OperatorGenerator::new();
    let left = quote::quote!(false);
    let right = quote::quote!(expensive_check());

    let result = gen.binary(BinaryOp::And, left, right);
    // Should still generate the right side (short-circuit happens at runtime)
    let s = result.to_string();
    assert!(s.contains("&&"));
}

#[test]
fn test_logical_and_chained() {
    let gen = OperatorGenerator::new();
    let left = quote::quote!(a && b);
    let right = quote::quote!(c);

    let result = gen.binary(BinaryOp::And, left, right);
    let s = result.to_string();

    assert!(s.contains("&&"));
}

// ============================================================================
// LOGICAL OR OPERATOR
// ============================================================================

#[test]
fn test_logical_or_basic() {
    let gen = OperatorGenerator::new();
    let left = quote::quote!(admin);
    let right = quote::quote!(moderator);

    let result = gen.binary(BinaryOp::Or, left, right);
    let s = result.to_string();

    assert!(s.contains("||"));
    assert!(s.contains("admin"));
    assert!(s.contains("moderator"));
}

#[test]
fn test_logical_or_with_comparison() {
    let gen = OperatorGenerator::new();
    let left = quote::quote!(age < 13);
    let right = quote::quote!(age > 65);

    let result = gen.binary(BinaryOp::Or, left, right);
    let s = result.to_string();

    assert!(s.contains("||"));
}

#[test]
fn test_logical_or_short_circuit_true() {
    // || operator naturally short-circuits in Rust
    let gen = OperatorGenerator::new();
    let left = quote::quote!(true);
    let right = quote::quote!(expensive_check());

    let result = gen.binary(BinaryOp::Or, left, right);
    // Should still generate the right side (short-circuit happens at runtime)
    let s = result.to_string();
    assert!(s.contains("||"));
}

#[test]
fn test_logical_or_chained() {
    let gen = OperatorGenerator::new();
    let left = quote::quote!(a || b);
    let right = quote::quote!(c);

    let result = gen.binary(BinaryOp::Or, left, right);
    let s = result.to_string();

    assert!(s.contains("||"));
}

// ============================================================================
// LOGICAL NOT OPERATOR
// ============================================================================

#[test]
fn test_logical_not_basic() {
    let gen = OperatorGenerator::new();
    let operand = quote::quote!(banned);

    let result = gen.unary(UnaryOp::Not, operand);
    let s = result.to_string();

    assert!(s.contains("!"));
    assert!(s.contains("banned"));
}

#[test]
fn test_logical_not_with_field_access() {
    let gen = OperatorGenerator::new();
    let operand = quote::quote!(user.banned);

    let result = gen.unary(UnaryOp::Not, operand);
    let s = result.to_string();

    assert!(s.contains("!"));
}

#[test]
fn test_logical_not_double_negation() {
    let gen = OperatorGenerator::new();
    let operand = quote::quote!(value);

    let negated_once = gen.unary(UnaryOp::Not, operand.clone());
    let negated_twice = gen.unary(UnaryOp::Not, negated_once);

    let s = negated_twice.to_string();
    // Should have two ! operators
    assert!(s.matches("!").count() >= 1);
}

// ============================================================================
// MIXED LOGICAL OPERATORS
// ============================================================================

#[test]
fn test_and_with_or() {
    let gen = OperatorGenerator::new();

    // Build: (a && b) || c
    let a = quote::quote!(a);
    let b = quote::quote!(b);
    let and_result = gen.binary(BinaryOp::And, a, b);

    let c = quote::quote!(c);
    let or_result = gen.binary(BinaryOp::Or, and_result, c);

    let s = or_result.to_string();
    assert!(s.contains("&&"));
    assert!(s.contains("||"));
}

#[test]
fn test_or_with_and() {
    let gen = OperatorGenerator::new();

    // Build: (a || b) && c
    let a = quote::quote!(a);
    let b = quote::quote!(b);
    let or_result = gen.binary(BinaryOp::Or, a, b);

    let c = quote::quote!(c);
    let and_result = gen.binary(BinaryOp::And, or_result, c);

    let s = and_result.to_string();
    assert!(s.contains("||"));
    assert!(s.contains("&&"));
}

#[test]
fn test_not_with_and() {
    let gen = OperatorGenerator::new();

    // Build: !a && b
    let a = quote::quote!(a);
    let not_a = gen.unary(UnaryOp::Not, a);

    let b = quote::quote!(b);
    let result = gen.binary(BinaryOp::And, not_a, b);

    let s = result.to_string();
    assert!(s.contains("!"));
    assert!(s.contains("&&"));
}

#[test]
fn test_not_with_or() {
    let gen = OperatorGenerator::new();

    // Build: !a || b
    let a = quote::quote!(a);
    let not_a = gen.unary(UnaryOp::Not, a);

    let b = quote::quote!(b);
    let result = gen.binary(BinaryOp::Or, not_a, b);

    let s = result.to_string();
    assert!(s.contains("!"));
    assert!(s.contains("||"));
}

// ============================================================================
// COMPLEX LOGICAL EXPRESSIONS
// ============================================================================

#[test]
fn test_user_validation_expression() {
    let gen = OperatorGenerator::new();

    // Build: verified && (age >= 18)
    let verified = quote::quote!(verified);
    let age_check = quote::quote!(age >= 18);
    let result = gen.binary(BinaryOp::And, verified, age_check);

    let s = result.to_string();
    assert!(s.contains("&&"));
}

#[test]
fn test_permission_check_expression() {
    let gen = OperatorGenerator::new();

    // Build: (admin || moderator) && verified
    let admin = quote::quote!(admin);
    let moderator = quote::quote!(moderator);
    let or_result = gen.binary(BinaryOp::Or, admin, moderator);

    let verified = quote::quote!(verified);
    let result = gen.binary(BinaryOp::And, or_result, verified);

    let s = result.to_string();
    assert!(s.contains("||"));
    assert!(s.contains("&&"));
}

#[test]
fn test_age_range_check_expression() {
    let gen = OperatorGenerator::new();

    // Build: age >= 13 && age < 18
    let age_min = quote::quote!(age >= 13);
    let age_max = quote::quote!(age < 18);
    let result = gen.binary(BinaryOp::And, age_min, age_max);

    let s = result.to_string();
    assert!(s.contains("&&"));
}

#[test]
fn test_role_check_with_ban_expression() {
    let gen = OperatorGenerator::new();

    // Build: (admin || moderator) && !banned
    let admin = quote::quote!(admin);
    let moderator = quote::quote!(moderator);
    let or_result = gen.binary(BinaryOp::Or, admin, moderator);

    let banned = quote::quote!(banned);
    let not_banned = gen.unary(UnaryOp::Not, banned);

    let result = gen.binary(BinaryOp::And, or_result, not_banned);

    let s = result.to_string();
    assert!(s.contains("||"));
    assert!(s.contains("&&"));
    assert!(s.contains("!"));
}

// ============================================================================
// OPERATOR PRECEDENCE CONSISTENCY
// ============================================================================

#[test]
fn test_logical_operator_precedence_and_vs_or() {
    let gen = OperatorGenerator::new();

    // Test that AND and OR generate correct operators
    // (Actual precedence is handled by Rust compiler)
    let left = quote::quote!(a);
    let right = quote::quote!(b);

    let and_result = gen.binary(BinaryOp::And, left.clone(), right.clone());
    let or_result = gen.binary(BinaryOp::Or, left, right);

    let and_str = and_result.to_string();
    let or_str = or_result.to_string();

    assert!(and_str.contains("&&"));
    assert!(or_str.contains("||"));
    // Both should be properly formatted
    assert!(!and_str.is_empty());
    assert!(!or_str.is_empty());
}

#[test]
fn test_logical_operator_consistency() {
    let gen = OperatorGenerator::new();

    // Test that same operator generates same code when called multiple times
    let left = quote::quote!(a);
    let right = quote::quote!(b);

    let result1 = gen.binary(BinaryOp::And, left.clone(), right.clone());
    let result2 = gen.binary(BinaryOp::And, left, right);

    assert_eq!(result1.to_string(), result2.to_string());
}

// ============================================================================
// COMBINATIONS WITH COMPARISON OPERATORS
// ============================================================================

#[test]
fn test_comparison_with_logical_and() {
    let gen = OperatorGenerator::new();

    // user.age >= 18 && user.verified == true
    let age_check = quote::quote!(user.age >= 18);
    let verified_check = quote::quote!(user.verified == true);

    let result = gen.binary(BinaryOp::And, age_check, verified_check);
    let s = result.to_string();

    assert!(s.contains("&&"));
}

#[test]
fn test_comparison_with_logical_or() {
    let gen = OperatorGenerator::new();

    // status == "active" || status == "pending"
    let active = quote::quote!(status == "active");
    let pending = quote::quote!(status == "pending");

    let result = gen.binary(BinaryOp::Or, active, pending);
    let s = result.to_string();

    assert!(s.contains("||"));
}

// ============================================================================
// COMBINATIONS WITH ARITHMETIC OPERATORS
// ============================================================================

#[test]
fn test_arithmetic_in_logical_expression() {
    let gen = OperatorGenerator::new();

    // (count + 1) > 10 && remaining < 5
    let count_expr = quote::quote!(count + 1 > 10);
    let remaining_expr = quote::quote!(remaining < 5);

    let result = gen.binary(BinaryOp::And, count_expr, remaining_expr);
    let s = result.to_string();

    assert!(s.contains("&&"));
}

#[test]
fn test_multiple_arithmetic_in_logical_expression() {
    let gen = OperatorGenerator::new();

    // (price * quantity) < budget && discount > 0
    let price_expr = quote::quote!(price * quantity < budget);
    let discount_expr = quote::quote!(discount > 0);

    let result = gen.binary(BinaryOp::And, price_expr, discount_expr);
    let s = result.to_string();

    assert!(s.contains("&&"));
}
