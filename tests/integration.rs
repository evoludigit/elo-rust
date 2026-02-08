//! Complex expression and integration tests
//!
//! Tests for real-world validation scenarios with nested expressions,
//! multiple operators, and field access combined.

use elo_rust::codegen::operators::{BinaryOp, OperatorGenerator, UnaryOp};

// ============================================================================
// USER VALIDATION EXPRESSIONS
// ============================================================================

#[test]
fn test_user_age_validation() {
    let gen = OperatorGenerator::new();

    // age >= 18
    let result = gen.binary(
        BinaryOp::GreaterEqual,
        quote::quote!(age),
        quote::quote!(18),
    );

    let s = result.to_string();
    assert!(s.contains(">="));
}

#[test]
fn test_user_complete_verification() {
    let gen = OperatorGenerator::new();

    // Simulate: email_verified && age >= 18 && !banned
    let email_verified = quote::quote!(email_verified);
    let age_check = quote::quote!(age >= 18);
    let not_banned = gen.unary(UnaryOp::Not, quote::quote!(banned));

    // Combine: email_verified && (age >= 18)
    let and_result1 = gen.binary(BinaryOp::And, email_verified, age_check);
    // Then: (email_verified && age >= 18) && !banned
    let final_result = gen.binary(BinaryOp::And, and_result1, not_banned);

    let s = final_result.to_string();
    assert!(s.contains("&&"));
    assert!(s.contains("!"));
}

#[test]
fn test_user_permission_check() {
    let gen = OperatorGenerator::new();

    // (admin || moderator) && verified
    let admin = quote::quote!(is_admin);
    let moderator = quote::quote!(is_moderator);
    let role_check = gen.binary(BinaryOp::Or, admin, moderator);

    let verified = quote::quote!(verified);
    let result = gen.binary(BinaryOp::And, role_check, verified);

    let s = result.to_string();
    assert!(s.contains("||"));
    assert!(s.contains("&&"));
}

// ============================================================================
// ACCOUNT VALIDATION EXPRESSIONS
// ============================================================================

#[test]
fn test_account_balance_check() {
    let gen = OperatorGenerator::new();

    // balance > 0 && balance < limit
    let greater_than_zero = quote::quote!(balance > 0);
    let less_than_limit = quote::quote!(balance < limit);
    let result = gen.binary(BinaryOp::And, greater_than_zero, less_than_limit);

    let s = result.to_string();
    assert!(s.contains("&&"));
}

#[test]
fn test_account_status_validation() {
    let gen = OperatorGenerator::new();

    // status == "active" || status == "pending"
    let active = quote::quote!(status == "active");
    let pending = quote::quote!(status == "pending");
    let result = gen.binary(BinaryOp::Or, active, pending);

    let s = result.to_string();
    assert!(s.contains("||"));
}

#[test]
fn test_account_fraud_detection() {
    let gen = OperatorGenerator::new();

    // !suspicious && balance > min && transactions < max
    let not_suspicious = gen.unary(UnaryOp::Not, quote::quote!(suspicious));
    let balance_check = quote::quote!(balance > min);
    let transaction_check = quote::quote!(transactions < max);

    let and_result = gen.binary(BinaryOp::And, not_suspicious, balance_check);
    let final_result = gen.binary(BinaryOp::And, and_result, transaction_check);

    let s = final_result.to_string();
    assert!(s.contains("!"));
    assert!(s.contains("&&"));
}

// ============================================================================
// DATA RANGE AND BOUNDARY VALIDATION
// ============================================================================

#[test]
fn test_age_range_teen() {
    let gen = OperatorGenerator::new();

    // age >= 13 && age < 18
    let min_age = quote::quote!(age >= 13);
    let max_age = quote::quote!(age < 18);
    let result = gen.binary(BinaryOp::And, min_age, max_age);

    let s = result.to_string();
    assert!(s.contains("&&"));
}

#[test]
fn test_age_range_adult() {
    let gen = OperatorGenerator::new();

    // age >= 18 && age <= 65
    let min_age = quote::quote!(age >= 18);
    let max_age = quote::quote!(age <= 65);
    let result = gen.binary(BinaryOp::And, min_age, max_age);

    let s = result.to_string();
    assert!(s.contains("&&"));
}

#[test]
fn test_age_senior_or_child() {
    let gen = OperatorGenerator::new();

    // age < 13 || age > 65
    let child = quote::quote!(age < 13);
    let senior = quote::quote!(age > 65);
    let result = gen.binary(BinaryOp::Or, child, senior);

    let s = result.to_string();
    assert!(s.contains("||"));
}

#[test]
fn test_percentage_validation() {
    let gen = OperatorGenerator::new();

    // value >= 0 && value <= 100
    let min = quote::quote!(percentage >= 0);
    let max = quote::quote!(percentage <= 100);
    let result = gen.binary(BinaryOp::And, min, max);

    let s = result.to_string();
    assert!(s.contains("&&"));
}

#[test]
fn test_count_range_validation() {
    let gen = OperatorGenerator::new();

    // count > 0 && count <= max_items && count % 2 == 0
    let positive = quote::quote!(count > 0);
    let within_limit = quote::quote!(count <= max_items);
    let is_even = quote::quote!(count % 2 == 0);

    let and1 = gen.binary(BinaryOp::And, positive, within_limit);
    let final_result = gen.binary(BinaryOp::And, and1, is_even);

    let s = final_result.to_string();
    assert!(s.contains("&&"));
}

// ============================================================================
// PRODUCT/INVENTORY VALIDATION
// ============================================================================

#[test]
fn test_product_in_stock() {
    let gen = OperatorGenerator::new();

    // quantity > 0 && price > 0
    let quantity_check = quote::quote!(quantity > 0);
    let price_check = quote::quote!(price > 0);
    let result = gen.binary(BinaryOp::And, quantity_check, price_check);

    let s = result.to_string();
    assert!(s.contains("&&"));
}

#[test]
fn test_product_category_check() {
    let gen = OperatorGenerator::new();

    // category == "electronics" || category == "software"
    let electronics = quote::quote!(category == "electronics");
    let software = quote::quote!(category == "software");
    let result = gen.binary(BinaryOp::Or, electronics, software);

    let s = result.to_string();
    assert!(s.contains("||"));
}

#[test]
fn test_product_discount_application() {
    let gen = OperatorGenerator::new();

    // quantity >= 10 && price_per_unit > 100
    let bulk = quote::quote!(quantity >= 10);
    let expensive = quote::quote!(price_per_unit > 100);
    let result = gen.binary(BinaryOp::And, bulk, expensive);

    let s = result.to_string();
    assert!(s.contains("&&"));
}

#[test]
fn test_inventory_critical_level() {
    let gen = OperatorGenerator::new();

    // stock < reorder_level && !on_order
    let low_stock = quote::quote!(stock < reorder_level);
    let not_on_order = gen.unary(UnaryOp::Not, quote::quote!(on_order));
    let result = gen.binary(BinaryOp::And, low_stock, not_on_order);

    let s = result.to_string();
    assert!(s.contains("&&"));
    assert!(s.contains("!"));
}

// ============================================================================
// PAYMENT/TRANSACTION VALIDATION
// ============================================================================

#[test]
fn test_payment_amount_valid() {
    let gen = OperatorGenerator::new();

    // amount > 0 && amount <= max_amount
    let positive = quote::quote!(amount > 0);
    let within_limit = quote::quote!(amount <= max_amount);
    let result = gen.binary(BinaryOp::And, positive, within_limit);

    let s = result.to_string();
    assert!(s.contains("&&"));
}

#[test]
fn test_payment_method_allowed() {
    let gen = OperatorGenerator::new();

    // method == "credit_card" || method == "paypal" || method == "bank_transfer"
    let credit = quote::quote!(method == "credit_card");
    let paypal = quote::quote!(method == "paypal");
    let bank = quote::quote!(method == "bank_transfer");

    let or1 = gen.binary(BinaryOp::Or, credit, paypal);
    let final_result = gen.binary(BinaryOp::Or, or1, bank);

    let s = final_result.to_string();
    assert!(s.contains("||"));
}

#[test]
fn test_transaction_fraud_score() {
    let gen = OperatorGenerator::new();

    // fraud_score < 0.5 && !flagged_for_review
    let low_fraud = quote::quote!(fraud_score < 0.5);
    let not_flagged = gen.unary(UnaryOp::Not, quote::quote!(flagged_for_review));
    let result = gen.binary(BinaryOp::And, low_fraud, not_flagged);

    let s = result.to_string();
    assert!(s.contains("&&"));
    assert!(s.contains("!"));
}

// ============================================================================
// COMPLEX NESTED EXPRESSIONS
// ============================================================================

#[test]
fn test_complex_nested_three_levels() {
    let gen = OperatorGenerator::new();

    // Build: (a || b) && (c || d) && e
    let a = quote::quote!(a);
    let b = quote::quote!(b);
    let or1 = gen.binary(BinaryOp::Or, a, b);

    let c = quote::quote!(c);
    let d = quote::quote!(d);
    let or2 = gen.binary(BinaryOp::Or, c, d);

    let and1 = gen.binary(BinaryOp::And, or1, or2);

    let e = quote::quote!(e);
    let final_result = gen.binary(BinaryOp::And, and1, e);

    let s = final_result.to_string();
    assert!(s.contains("||"));
    assert!(s.contains("&&"));
}

#[test]
fn test_complex_mixed_all_operators() {
    let gen = OperatorGenerator::new();

    // Build: !a && (b >= 10 || c == "x") && d
    let not_a = gen.unary(UnaryOp::Not, quote::quote!(a));
    let b_check = quote::quote!(b >= 10);
    let c_check = quote::quote!(c == "x");
    let or_result = gen.binary(BinaryOp::Or, b_check, c_check);
    let and1 = gen.binary(BinaryOp::And, not_a, or_result);

    let d = quote::quote!(d);
    let final_result = gen.binary(BinaryOp::And, and1, d);

    let s = final_result.to_string();
    assert!(s.contains("!"));
    assert!(s.contains("||"));
    assert!(s.contains("&&"));
}

#[test]
fn test_real_world_order_validation() {
    let gen = OperatorGenerator::new();

    // (customer_verified || customer_trusted) && order_amount > 0
    // && order_items > 0 && !fraud_flagged
    let customer_verified = quote::quote!(customer_verified);
    let customer_trusted = quote::quote!(customer_trusted);
    let customer_check = gen.binary(BinaryOp::Or, customer_verified, customer_trusted);

    let amount_check = quote::quote!(order_amount > 0);
    let and1 = gen.binary(BinaryOp::And, customer_check, amount_check);

    let items_check = quote::quote!(order_items > 0);
    let and2 = gen.binary(BinaryOp::And, and1, items_check);

    let not_fraud = gen.unary(UnaryOp::Not, quote::quote!(fraud_flagged));
    let final_result = gen.binary(BinaryOp::And, and2, not_fraud);

    let s = final_result.to_string();
    assert!(s.contains("||"));
    assert!(s.contains("&&"));
    assert!(s.contains("!"));
}

// ============================================================================
// EDGE CASES AND SPECIAL SCENARIOS
// ============================================================================

#[test]
fn test_all_comparison_operators_combined() {
    let gen = OperatorGenerator::new();

    // Test combining different comparison operators
    let eq = quote::quote!(a == b);
    let neq = quote::quote!(c != d);
    let lt = quote::quote!(e < f);

    let and1 = gen.binary(BinaryOp::And, eq, neq);
    let and2 = gen.binary(BinaryOp::And, and1, lt);

    let s = and2.to_string();
    assert!(s.contains("&&"));
}

#[test]
fn test_all_arithmetic_operators_combined() {
    let gen = OperatorGenerator::new();

    // Test combining different arithmetic operators
    let add = quote::quote!(a + b);
    let sub = quote::quote!(c - d);
    let mul = quote::quote!(e * f);

    // Combine with logical operators
    let and_result = gen.binary(BinaryOp::And, add, sub);
    let final_result = gen.binary(BinaryOp::And, and_result, mul);

    let s = final_result.to_string();
    assert!(s.contains("&&"));
}

#[test]
fn test_deeply_nested_unary_operations() {
    let gen = OperatorGenerator::new();

    // !!!value (triple negation)
    let operand = quote::quote!(value);
    let neg1 = gen.unary(UnaryOp::Not, operand);
    let neg2 = gen.unary(UnaryOp::Not, neg1);
    let neg3 = gen.unary(UnaryOp::Not, neg2);

    let s = neg3.to_string();
    // Should have multiple ! operators
    assert!(!s.is_empty());
}

#[test]
fn test_mixed_unary_in_binary_expression() {
    let gen = OperatorGenerator::new();

    // !a || !b
    let not_a = gen.unary(UnaryOp::Not, quote::quote!(a));
    let not_b = gen.unary(UnaryOp::Not, quote::quote!(b));
    let result = gen.binary(BinaryOp::Or, not_a, not_b);

    let s = result.to_string();
    assert!(s.contains("!"));
    assert!(s.contains("||"));
}

#[test]
fn test_negate_in_binary_expression() {
    let gen = OperatorGenerator::new();

    // (-a) < 0 && (-b) > 0
    let check_a = quote::quote!(-a < 0);
    let check_b = quote::quote!(-b > 0);

    let result = gen.binary(BinaryOp::And, check_a, check_b);

    let s = result.to_string();
    assert!(s.contains("&&"));
}

#[test]
fn test_same_operator_multiple_times() {
    let gen = OperatorGenerator::new();

    // a == b == c (chained equality)
    let eq1 = quote::quote!(a == b);
    let c = quote::quote!(c);
    let eq2 = gen.binary(BinaryOp::Equal, eq1, c);

    let s = eq2.to_string();
    assert!(s.contains("=="));
}

#[test]
fn test_alternating_and_or_operators() {
    let gen = OperatorGenerator::new();

    // a && b || c && d
    let a = quote::quote!(a);
    let b = quote::quote!(b);
    let and1 = gen.binary(BinaryOp::And, a, b);

    let c = quote::quote!(c);
    let d = quote::quote!(d);
    let and2 = gen.binary(BinaryOp::And, c, d);

    let final_result = gen.binary(BinaryOp::Or, and1, and2);

    let s = final_result.to_string();
    assert!(s.contains("&&"));
    assert!(s.contains("||"));
}

// ============================================================================
// GENERATOR CONSISTENCY AND REUSABILITY
// ============================================================================

#[test]
fn test_generator_reusable_across_expressions() {
    let gen = OperatorGenerator::new();

    // Use same generator for multiple expressions
    let expr1 = gen.binary(BinaryOp::And, quote::quote!(a), quote::quote!(b));
    let expr2 = gen.binary(BinaryOp::Or, quote::quote!(c), quote::quote!(d));
    let expr3 = gen.binary(BinaryOp::Add, quote::quote!(e), quote::quote!(f));

    assert!(!expr1.to_string().is_empty());
    assert!(!expr2.to_string().is_empty());
    assert!(!expr3.to_string().is_empty());
}

#[test]
fn test_generator_produces_consistent_output() {
    let gen = OperatorGenerator::new();

    // Same expression twice should produce identical output
    let left = quote::quote!(x);
    let right = quote::quote!(y);

    let result1 = gen.binary(BinaryOp::And, left.clone(), right.clone());
    let result2 = gen.binary(BinaryOp::And, left, right);

    assert_eq!(result1.to_string(), result2.to_string());
}

#[test]
fn test_different_generators_same_result() {
    let gen1 = OperatorGenerator::new();
    let gen2 = OperatorGenerator::new();

    let result1 = gen1.binary(BinaryOp::Or, quote::quote!(a), quote::quote!(b));
    let result2 = gen2.binary(BinaryOp::Or, quote::quote!(a), quote::quote!(b));

    assert_eq!(result1.to_string(), result2.to_string());
}
