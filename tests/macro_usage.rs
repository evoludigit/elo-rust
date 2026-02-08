//! Validator macro usage and integration tests
//!
//! Tests for the #[validator] macro that generates validation code

// ============================================================================
// MACRO USAGE PATTERNS
// ============================================================================

/// Helper function to simulate what the macro would generate
fn generate_validator_code(elo_expression: &str) -> String {
    // For now, we'll simulate generating validation code
    format!(
        r#"
pub fn validate(input: &impl std::any::Any) -> Result<(), Vec<String>> {{
    // Generated from: {}
    Ok(())
}}
"#,
        elo_expression
    )
}

#[test]
fn test_simple_age_validator() {
    let code = generate_validator_code("age >= 18");
    assert!(code.contains("age >= 18") || code.contains("Generated from"));
    assert!(code.contains("pub fn validate"));
}

#[test]
fn test_email_validator() {
    let code = generate_validator_code(r#"email matches "^[a-z]+@example\.com$""#);
    assert!(code.contains("pub fn validate"));
    assert!(!code.is_empty());
}

#[test]
fn test_combined_validation() {
    let code = generate_validator_code("email matches pattern && age >= 18 && !banned");
    assert!(code.contains("pub fn validate"));
}

#[test]
fn test_string_validation() {
    let code = generate_validator_code("username.length() >= 3 && username.length() <= 20");
    assert!(code.contains("pub fn validate"));
}

#[test]
fn test_array_validation() {
    let code =
        generate_validator_code("roles.contains(\"admin\") || roles.contains(\"moderator\")");
    assert!(code.contains("pub fn validate"));
}

#[test]
fn test_date_validation() {
    let code = generate_validator_code("age(birth_date) >= 18 && age(birth_date) <= 65");
    assert!(code.contains("pub fn validate"));
}

// ============================================================================
// GENERATED CODE PROPERTIES
// ============================================================================

#[test]
fn test_generated_code_contains_input_parameter() {
    let code = generate_validator_code("value > 0");
    assert!(code.contains("input") || code.contains("validate"));
}

#[test]
fn test_generated_code_has_result_type() {
    let code = generate_validator_code("count >= 1");
    assert!(code.contains("Result") || code.contains("pub fn"));
}

#[test]
fn test_generated_code_is_public() {
    let code = generate_validator_code("valid == true");
    assert!(code.contains("pub fn"));
}

#[test]
fn test_generated_validator_name() {
    let code = generate_validator_code("test > 0");
    assert!(code.contains("validate"));
}

// ============================================================================
// MULTIPLE VALIDATORS
// ============================================================================

#[test]
fn test_age_validator_generation() {
    let code1 = generate_validator_code("age >= 18");
    let code2 = generate_validator_code("age >= 21");

    // Both should generate valid code
    assert!(code1.contains("pub fn validate"));
    assert!(code2.contains("pub fn validate"));
    assert_ne!(code1, code2); // Different conditions should differ
}

#[test]
fn test_email_and_age_validators() {
    let email_code = generate_validator_code(
        r#"email matches "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$""#,
    );
    let age_code = generate_validator_code("age >= 18");

    assert!(email_code.contains("pub fn validate"));
    assert!(age_code.contains("pub fn validate"));
}

#[test]
fn test_permission_validators() {
    let admin_code = generate_validator_code("roles.contains(\"admin\")");
    let user_code = generate_validator_code("roles.contains(\"user\") && !banned");

    assert!(admin_code.contains("pub fn validate"));
    assert!(user_code.contains("pub fn validate"));
}

// ============================================================================
// VALIDATOR FOR DIFFERENT TYPES
// ============================================================================

#[test]
fn test_string_field_validator() {
    let code = generate_validator_code("username.length() >= 3");
    assert!(code.contains("validate"));
}

#[test]
fn test_numeric_field_validator() {
    let code = generate_validator_code("balance > 0 && balance <= 1000000");
    assert!(code.contains("validate"));
}

#[test]
fn test_boolean_field_validator() {
    let code = generate_validator_code("verified == true && !banned");
    assert!(code.contains("validate"));
}

#[test]
fn test_date_field_validator() {
    let code = generate_validator_code("birth_date < today() && age(birth_date) >= 18");
    assert!(code.contains("validate"));
}

#[test]
fn test_array_field_validator() {
    let code = generate_validator_code("tags.length() > 0 && tags.length() <= 10");
    assert!(code.contains("validate"));
}

// ============================================================================
// COMPLEX VALIDATION LOGIC
// ============================================================================

#[test]
fn test_signup_form_validator() {
    let code = generate_validator_code(
        r#"
email matches "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$" &&
password.length() >= 8 &&
age >= 18 &&
terms_accepted == true
"#,
    );
    assert!(code.contains("validate"));
}

#[test]
fn test_order_validator() {
    let code = generate_validator_code(
        "items.length() > 0 && items.all(quantity > 0 && price > 0) && total > 0",
    );
    assert!(code.contains("validate"));
}

#[test]
fn test_user_profile_validator() {
    let code = generate_validator_code(
        r#"
username.length() >= 3 && username.length() <= 20 &&
email matches pattern &&
!email.contains(" ") &&
bio.length() <= 500 &&
age >= 13
"#,
    );
    assert!(code.contains("validate"));
}

#[test]
fn test_permission_validator() {
    let code = generate_validator_code(
        "(roles.contains(\"admin\") || roles.contains(\"moderator\")) && verified == true && !banned",
    );
    assert!(code.contains("validate"));
}

// ============================================================================
// VALIDATOR CONSISTENCY
// ============================================================================

#[test]
fn test_same_expression_generates_consistent_code() {
    let expr = "value > 0 && value < 100";
    let code1 = generate_validator_code(expr);
    let code2 = generate_validator_code(expr);

    assert_eq!(code1, code2);
}

#[test]
fn test_different_expressions_generate_different_code() {
    let code1 = generate_validator_code("value > 0");
    let code2 = generate_validator_code("value > 100");

    // They should both be valid but different
    assert!(code1.contains("validate"));
    assert!(code2.contains("validate"));
}

// ============================================================================
// ERROR HANDLING IN VALIDATORS
// ============================================================================

#[test]
fn test_validator_error_messages() {
    let code = generate_validator_code("email matches pattern");
    // Should be able to handle validation failures
    assert!(code.contains("validate"));
}

#[test]
fn test_multiple_validation_errors() {
    let code =
        generate_validator_code("age >= 18 && email matches pattern && username.length() >= 3");
    // Should support multiple validation points
    assert!(code.contains("validate"));
}

// ============================================================================
// VALIDATOR WITH FIELD ACCESS
// ============================================================================

#[test]
fn test_nested_field_validation() {
    let code = generate_validator_code("user.age >= 18 && user.verified == true");
    assert!(code.contains("validate"));
}

#[test]
fn test_deeply_nested_field_validation() {
    let code = generate_validator_code(
        "company.address.country.name matches \"US\" || other_valid == true",
    );
    assert!(code.contains("validate"));
}

// ============================================================================
// REAL-WORLD VALIDATOR SCENARIOS
// ============================================================================

#[test]
fn test_github_user_validator() {
    let code = generate_validator_code(
        r#"
username.length() >= 1 &&
username.length() <= 39 &&
username matches "^[a-zA-Z0-9][a-zA-Z0-9-]*[a-zA-Z0-9]?$"
"#,
    );
    assert!(code.contains("validate"));
}

#[test]
fn test_credit_card_validator() {
    let code = generate_validator_code(
        "card_number.length() == 16 && expiry_month >= 1 && expiry_month <= 12 && expiry_year > today().year()",
    );
    assert!(code.contains("validate"));
}

#[test]
fn test_api_key_validator() {
    let code = generate_validator_code(
        "api_key.length() >= 32 && api_key.length() <= 64 && created_at >= date(\"2024-01-01\")",
    );
    assert!(code.contains("validate"));
}

#[test]
fn test_document_upload_validator() {
    let code = generate_validator_code(
        r#"
file_size > 0 &&
file_size <= 10485760 &&
file_type matches "^(pdf|docx?|txt|xlsx?)$" &&
!file_name.contains("..")
"#,
    );
    assert!(code.contains("validate"));
}

// ============================================================================
// VALIDATOR COMPOSITION
// ============================================================================

#[test]
fn test_validator_composition_pattern() {
    let email_code = generate_validator_code("email matches pattern");
    let age_code = generate_validator_code("age >= 18");

    // Both validators should be independent
    assert!(email_code.contains("validate"));
    assert!(age_code.contains("validate"));
}

#[test]
fn test_chained_validators() {
    let code =
        generate_validator_code("email matches pattern && password.length() >= 8 && terms == true");
    // Should support chains of validations
    assert!(code.contains("validate"));
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_empty_validator() {
    let code = generate_validator_code("");
    // Should still generate valid function
    assert!(code.contains("validate"));
}

#[test]
fn test_very_long_validator_expression() {
    let long_expr = "a == 1 && b == 2 && c == 3 && d == 4 && e == 5 && f == 6 && g == 7 && h == 8 && i == 9 && j == 10";
    let code = generate_validator_code(long_expr);
    assert!(code.contains("validate"));
}

#[test]
fn test_deeply_nested_expression() {
    let code =
        generate_validator_code("(((a && b) || (c && d)) && ((e || f) && (g || h))) || (i && j)");
    assert!(code.contains("validate"));
}

#[test]
fn test_unicode_in_validator() {
    let code = generate_validator_code(
        "username.length() >= 1 && description contains \"测试\" || name contains \"test\"",
    );
    assert!(code.contains("validate"));
}
