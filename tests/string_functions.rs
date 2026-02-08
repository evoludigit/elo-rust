//! String function code generation tests
//!
//! Tests for string manipulation functions: matches, contains, length,
//! case conversion, trim, starts_with, ends_with

use elo_rust::codegen::functions::FunctionGenerator;

// ============================================================================
// MATCHES FUNCTION - REGEX PATTERN MATCHING
// ============================================================================

#[test]
fn test_matches_basic_pattern() {
    let gen = FunctionGenerator::new();
    let subject = quote::quote!(email);
    let pattern = quote::quote!("^[a-z]+@example\\.com$");

    let result = gen.string_function("matches", vec![subject, pattern]);
    let s = result.to_string();

    assert!(!s.is_empty());
    assert!(s.contains("regex") || s.contains("Regex"));
}

#[test]
fn test_matches_simple_pattern() {
    let gen = FunctionGenerator::new();
    let subject = quote::quote!(text);
    let pattern = quote::quote!("test");

    let result = gen.string_function("matches", vec![subject, pattern]);
    let s = result.to_string();

    assert!(!s.is_empty());
}

#[test]
fn test_matches_phone_pattern() {
    let gen = FunctionGenerator::new();
    let subject = quote::quote!(phone);
    let pattern = quote::quote!("^\\d{3}-\\d{3}-\\d{4}$");

    let result = gen.string_function("matches", vec![subject, pattern]);
    let s = result.to_string();

    assert!(!s.is_empty());
}

#[test]
fn test_matches_complex_email_pattern() {
    let gen = FunctionGenerator::new();
    let subject = quote::quote!(email);
    let pattern = quote::quote!(
        "^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"
    );

    let result = gen.string_function("matches", vec![subject, pattern]);
    let s = result.to_string();

    assert!(!s.is_empty());
}

// ============================================================================
// CONTAINS FUNCTION - SUBSTRING SEARCH
// ============================================================================

#[test]
fn test_contains_basic() {
    let gen = FunctionGenerator::new();
    let text = quote::quote!(description);
    let substring = quote::quote!("keyword");

    let result = gen.string_function("contains", vec![text, substring]);
    let s = result.to_string();

    assert!(s.contains("contains"));
}

#[test]
fn test_contains_with_string_literal() {
    let gen = FunctionGenerator::new();
    let text = quote::quote!(name);
    let substring = quote::quote!("admin");

    let result = gen.string_function("contains", vec![text, substring]);
    let s = result.to_string();

    assert!(!s.is_empty());
}

#[test]
fn test_contains_with_field_access() {
    let gen = FunctionGenerator::new();
    let text = quote::quote!(user.bio);
    let substring = quote::quote!("spam");

    let result = gen.string_function("contains", vec![text, substring]);
    let s = result.to_string();

    assert!(s.contains("user"));
}

// ============================================================================
// LENGTH FUNCTION - STRING LENGTH
// ============================================================================

#[test]
fn test_length_basic() {
    let gen = FunctionGenerator::new();
    let text = quote::quote!(password);

    let result = gen.string_function("length", vec![text]);
    let s = result.to_string();

    assert!(s.contains("len"));
}

#[test]
fn test_length_with_field_access() {
    let gen = FunctionGenerator::new();
    let text = quote::quote!(user.name);

    let result = gen.string_function("length", vec![text]);
    let s = result.to_string();

    assert!(s.contains("len"));
}

#[test]
fn test_length_comparison() {
    let gen = FunctionGenerator::new();
    let text = quote::quote!(input);

    let length_code = gen.string_function("length", vec![text]);
    let s = length_code.to_string();

    // Should generate something that can be compared
    assert!(!s.is_empty());
}

// ============================================================================
// UPPERCASE FUNCTION - CASE CONVERSION
// ============================================================================

#[test]
fn test_uppercase_basic() {
    let gen = FunctionGenerator::new();
    let text = quote::quote!(name);

    let result = gen.string_function("uppercase", vec![text]);
    let s = result.to_string();

    assert!(s.contains("to_uppercase") || s.contains("uppercase"));
}

#[test]
fn test_uppercase_with_field_access() {
    let gen = FunctionGenerator::new();
    let text = quote::quote!(user.email);

    let result = gen.string_function("uppercase", vec![text]);
    let s = result.to_string();

    assert!(!s.is_empty());
}

// ============================================================================
// LOWERCASE FUNCTION - CASE CONVERSION
// ============================================================================

#[test]
fn test_lowercase_basic() {
    let gen = FunctionGenerator::new();
    let text = quote::quote!(name);

    let result = gen.string_function("lowercase", vec![text]);
    let s = result.to_string();

    assert!(s.contains("to_lowercase") || s.contains("lowercase"));
}

#[test]
fn test_lowercase_with_field_access() {
    let gen = FunctionGenerator::new();
    let text = quote::quote!(user.email);

    let result = gen.string_function("lowercase", vec![text]);
    let s = result.to_string();

    assert!(!s.is_empty());
}

// ============================================================================
// TRIM FUNCTION - WHITESPACE REMOVAL
// ============================================================================

#[test]
fn test_trim_basic() {
    let gen = FunctionGenerator::new();
    let text = quote::quote!(input);

    let result = gen.string_function("trim", vec![text]);
    let s = result.to_string();

    assert!(s.contains("trim"));
}

#[test]
fn test_trim_with_field_access() {
    let gen = FunctionGenerator::new();
    let text = quote::quote!(user.name);

    let result = gen.string_function("trim", vec![text]);
    let s = result.to_string();

    assert!(s.contains("trim"));
}

// ============================================================================
// STARTS_WITH FUNCTION - PREFIX CHECK
// ============================================================================

#[test]
fn test_starts_with_basic() {
    let gen = FunctionGenerator::new();
    let text = quote::quote!(url);
    let prefix = quote::quote!("https://");

    let result = gen.string_function("starts_with", vec![text, prefix]);
    let s = result.to_string();

    assert!(s.contains("starts_with"));
}

#[test]
fn test_starts_with_with_field_access() {
    let gen = FunctionGenerator::new();
    let text = quote::quote!(user.email);
    let prefix = quote::quote!("admin@");

    let result = gen.string_function("starts_with", vec![text, prefix]);
    let s = result.to_string();

    assert!(s.contains("starts_with"));
}

#[test]
fn test_starts_with_protocol() {
    let gen = FunctionGenerator::new();
    let url = quote::quote!(link);
    let protocol = quote::quote!("http");

    let result = gen.string_function("starts_with", vec![url, protocol]);
    let s = result.to_string();

    assert!(!s.is_empty());
}

// ============================================================================
// ENDS_WITH FUNCTION - SUFFIX CHECK
// ============================================================================

#[test]
fn test_ends_with_basic() {
    let gen = FunctionGenerator::new();
    let text = quote::quote!(email);
    let suffix = quote::quote!("@example.com");

    let result = gen.string_function("ends_with", vec![text, suffix]);
    let s = result.to_string();

    assert!(s.contains("ends_with"));
}

#[test]
fn test_ends_with_with_field_access() {
    let gen = FunctionGenerator::new();
    let text = quote::quote!(user.email);
    let suffix = quote::quote!(".com");

    let result = gen.string_function("ends_with", vec![text, suffix]);
    let s = result.to_string();

    assert!(s.contains("ends_with"));
}

#[test]
fn test_ends_with_file_extension() {
    let gen = FunctionGenerator::new();
    let filename = quote::quote!(file_name);
    let extension = quote::quote!(".pdf");

    let result = gen.string_function("ends_with", vec![filename, extension]);
    let s = result.to_string();

    assert!(!s.is_empty());
}

// ============================================================================
// COMBINED STRING OPERATIONS
// ============================================================================

#[test]
fn test_email_validation_expression() {
    let gen = FunctionGenerator::new();

    // email.lowercase().matches(pattern)
    let email = quote::quote!(email);
    let lowercase = gen.string_function("lowercase", vec![email]);

    let s = lowercase.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_password_strength_check() {
    let gen = FunctionGenerator::new();

    // password.length() >= 8
    let password = quote::quote!(password);
    let length = gen.string_function("length", vec![password]);

    let s = length.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_username_validation() {
    let gen = FunctionGenerator::new();

    // username.length() >= 3 && username.matches("^[a-zA-Z0-9_]+$")
    let username = quote::quote!(username);
    let length = gen.string_function("length", vec![username.clone()]);
    let pattern = gen.string_function("matches", vec![username, quote::quote!("^[a-zA-Z0-9_]+$")]);

    let len_str = length.to_string();
    let pattern_str = pattern.to_string();

    assert!(!len_str.is_empty());
    assert!(!pattern_str.is_empty());
}

#[test]
fn test_url_validation() {
    let gen = FunctionGenerator::new();

    // url.starts_with("https://") && url.length() > 10
    let url = quote::quote!(url);
    let starts_with =
        gen.string_function("starts_with", vec![url.clone(), quote::quote!("https://")]);
    let length = gen.string_function("length", vec![url]);

    let starts_str = starts_with.to_string();
    let length_str = length.to_string();

    assert!(starts_str.contains("starts_with"));
    assert!(length_str.contains("len"));
}

#[test]
fn test_domain_check() {
    let gen = FunctionGenerator::new();

    // email.ends_with("@company.com")
    let email = quote::quote!(email);
    let ends_with = gen.string_function("ends_with", vec![email, quote::quote!("@company.com")]);

    let s = ends_with.to_string();
    assert!(s.contains("ends_with"));
}

#[test]
fn test_trimmed_length_check() {
    let gen = FunctionGenerator::new();

    // input.trim().length() > 0
    let input = quote::quote!(input);
    let trimmed = gen.string_function("trim", vec![input]);

    let s = trimmed.to_string();
    assert!(s.contains("trim"));
}

// ============================================================================
// GENERATOR CONSISTENCY
// ============================================================================

#[test]
fn test_string_function_consistency() {
    let gen = FunctionGenerator::new();

    let text = quote::quote!(value);
    let result1 = gen.string_function("length", vec![text.clone()]);
    let result2 = gen.string_function("length", vec![text]);

    assert_eq!(result1.to_string(), result2.to_string());
}

#[test]
fn test_multiple_string_functions() {
    let gen = FunctionGenerator::new();

    let text = quote::quote!(text);
    let contains_result =
        gen.string_function("contains", vec![text.clone(), quote::quote!("test")]);
    let length_result = gen.string_function("length", vec![text]);

    assert!(!contains_result.to_string().is_empty());
    assert!(!length_result.to_string().is_empty());
}

#[test]
fn test_case_conversion_chain() {
    let gen = FunctionGenerator::new();

    let text = quote::quote!(text);
    let uppercase = gen.string_function("uppercase", vec![text]);
    let lowercase = gen.string_function("lowercase", vec![uppercase]);

    let s = lowercase.to_string();
    assert!(!s.is_empty());
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_empty_string_operations() {
    let gen = FunctionGenerator::new();

    let empty = quote::quote!("");

    let contains_result = gen.string_function("contains", vec![empty.clone(), quote::quote!("x")]);
    let length_result = gen.string_function("length", vec![empty]);

    assert!(!contains_result.to_string().is_empty());
    assert!(!length_result.to_string().is_empty());
}

#[test]
fn test_special_characters_in_pattern() {
    let gen = FunctionGenerator::new();

    let text = quote::quote!(text);
    let special_pattern = quote::quote!(".*[!@#$%^&*].*");

    let result = gen.string_function("matches", vec![text, special_pattern]);
    let s = result.to_string();

    assert!(!s.is_empty());
}

#[test]
fn test_unicode_string_operations() {
    let gen = FunctionGenerator::new();

    let text = quote::quote!(unicode_text);

    let uppercase = gen.string_function("uppercase", vec![text.clone()]);
    let length = gen.string_function("length", vec![text]);

    assert!(!uppercase.to_string().is_empty());
    assert!(!length.to_string().is_empty());
}
