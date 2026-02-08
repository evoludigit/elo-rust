//! Extended function generator tests for edge cases and special functions

use elo_rust::codegen::functions::FunctionGenerator;

// ============================================================================
// TYPE CHECKING FUNCTIONS (is_null, is_some)
// ============================================================================

#[test]
fn test_is_null_function() {
    let gen = FunctionGenerator::new();
    let value = quote::quote!(user);
    let result = gen.array_function("is_null", vec![value]);
    let s = result.to_string();

    // Should generate is_none() for Option types
    assert!(s.contains("is_none"));
}

#[test]
fn test_is_null_with_no_arguments() {
    let gen = FunctionGenerator::new();
    let result = gen.array_function("is_null", vec![]);
    let s = result.to_string();

    assert!(s.is_empty());
}

#[test]
fn test_is_some_function() {
    let gen = FunctionGenerator::new();
    let value = quote::quote!(result);
    let result = gen.array_function("is_some", vec![value]);
    let s = result.to_string();

    // Should generate is_some()
    assert!(s.contains("is_some"));
}

#[test]
fn test_is_some_with_no_arguments() {
    let gen = FunctionGenerator::new();
    let result = gen.array_function("is_some", vec![]);
    let s = result.to_string();

    assert!(s.is_empty());
}

// ============================================================================
// CALL FUNCTION ROUTING WITH TYPE CHECKING
// ============================================================================

#[test]
fn test_call_is_null_not_routed() {
    let gen = FunctionGenerator::new();
    let value = quote::quote!(optional_value);
    let result = gen.call("is_null", vec![value]);
    let s = result.to_string();

    // is_null is not in the call routing, so it returns empty
    assert!(s.is_empty());
}

#[test]
fn test_call_is_some_not_routed() {
    let gen = FunctionGenerator::new();
    let value = quote::quote!(maybe_data);
    let result = gen.call("is_some", vec![value]);
    let s = result.to_string();

    // is_some is not in the call routing, so it returns empty
    assert!(s.is_empty());
}

// ============================================================================
// COMPREHENSIVE ARRAY FUNCTION COVERAGE
// ============================================================================

#[test]
fn test_array_contains_proper_return() {
    let gen = FunctionGenerator::new();
    let array = quote::quote!(tags);
    let item = quote::quote!("admin");
    let result = gen.array_function("contains", vec![array, item]);
    let s = result.to_string();

    assert!(s.contains("contains"));
}

#[test]
fn test_array_any_proper_return() {
    let gen = FunctionGenerator::new();
    let array = quote::quote!(conditions);
    let predicate = quote::quote!(|x| x > 5);
    let result = gen.array_function("any", vec![array, predicate]);
    let s = result.to_string();

    assert!(s.contains("any"));
}

#[test]
fn test_array_all_proper_return() {
    let gen = FunctionGenerator::new();
    let array = quote::quote!(requirements);
    let predicate = quote::quote!(|req| req.is_met());
    let result = gen.array_function("all", vec![array, predicate]);
    let s = result.to_string();

    assert!(s.contains("all"));
}

#[test]
fn test_array_length_proper_return() {
    let gen = FunctionGenerator::new();
    let array = quote::quote!(items);
    let result = gen.array_function("length", vec![array]);
    let s = result.to_string();

    assert!(s.contains("len"));
}

#[test]
fn test_array_is_empty_proper_return() {
    let gen = FunctionGenerator::new();
    let array = quote::quote!(collection);
    let result = gen.array_function("is_empty", vec![array]);
    let s = result.to_string();

    assert!(s.contains("is_empty"));
}

// ============================================================================
// STRING FUNCTION COMPREHENSIVE COVERAGE
// ============================================================================

#[test]
fn test_length_function_returns_code() {
    let gen = FunctionGenerator::new();
    let text = quote::quote!(name);
    let result = gen.string_function("length", vec![text]);
    let s = result.to_string();

    assert!(s.contains("len"));
}

#[test]
fn test_uppercase_function_returns_code() {
    let gen = FunctionGenerator::new();
    let text = quote::quote!(email);
    let result = gen.string_function("uppercase", vec![text]);
    let s = result.to_string();

    assert!(s.contains("to_uppercase"));
}

#[test]
fn test_lowercase_function_returns_code() {
    let gen = FunctionGenerator::new();
    let text = quote::quote!(input);
    let result = gen.string_function("lowercase", vec![text]);
    let s = result.to_string();

    assert!(s.contains("to_lowercase"));
}

#[test]
fn test_trim_function_returns_code() {
    let gen = FunctionGenerator::new();
    let text = quote::quote!(raw_input);
    let result = gen.string_function("trim", vec![text]);
    let s = result.to_string();

    assert!(s.contains("trim"));
}

#[test]
fn test_contains_function_with_pattern() {
    let gen = FunctionGenerator::new();
    let text = quote::quote!(email);
    let pattern = quote::quote!("@");
    let result = gen.string_function("contains", vec![text, pattern]);
    let s = result.to_string();

    assert!(s.contains("contains"));
}

#[test]
fn test_matches_function_with_regex() {
    let gen = FunctionGenerator::new();
    let text = quote::quote!(phone);
    let pattern = quote::quote!(r"\d+");
    let result = gen.string_function("matches", vec![text, pattern]);
    let s = result.to_string();

    // matches function generates regex code
    assert!(s.contains("Regex") || s.contains("regex"));
}

#[test]
fn test_starts_with_function() {
    let gen = FunctionGenerator::new();
    let text = quote::quote!(path);
    let prefix = quote::quote!("./");
    let result = gen.string_function("starts_with", vec![text, prefix]);
    let s = result.to_string();

    assert!(s.contains("starts_with"));
}

#[test]
fn test_ends_with_function() {
    let gen = FunctionGenerator::new();
    let text = quote::quote!(filename);
    let suffix = quote::quote!(".txt");
    let result = gen.string_function("ends_with", vec![text, suffix]);
    let s = result.to_string();

    assert!(s.contains("ends_with"));
}

// ============================================================================
// DATETIME FUNCTION COMPREHENSIVE COVERAGE
// ============================================================================

#[test]
fn test_today_function_returns_code() {
    let gen = FunctionGenerator::new();
    let result = gen.datetime_function("today", vec![]);
    let s = result.to_string();

    assert!(!s.is_empty());
}

#[test]
fn test_now_function_returns_code() {
    let gen = FunctionGenerator::new();
    let result = gen.datetime_function("now", vec![]);
    let s = result.to_string();

    assert!(!s.is_empty());
}

#[test]
fn test_age_function_with_date() {
    let gen = FunctionGenerator::new();
    let birth_date = quote::quote!(user.birth_date);
    let result = gen.datetime_function("age", vec![birth_date]);
    let s = result.to_string();

    assert!(!s.is_empty());
}

#[test]
fn test_days_since_function_with_date() {
    let gen = FunctionGenerator::new();
    let event_date = quote::quote!(created_at);
    let result = gen.datetime_function("days_since", vec![event_date]);
    let s = result.to_string();

    assert!(!s.is_empty());
}

#[test]
fn test_date_function_with_string() {
    let gen = FunctionGenerator::new();
    let date_str = quote::quote!("2026-02-08");
    let result = gen.datetime_function("date", vec![date_str]);
    let s = result.to_string();

    assert!(!s.is_empty());
}

// ============================================================================
// GENERATOR CLONING AND INDEPENDENCE
// ============================================================================

#[test]
fn test_generator_independent_is_null() {
    let gen1 = FunctionGenerator::new();
    let gen2 = FunctionGenerator::new();

    let value = quote::quote!(data);
    let result1 = gen1.array_function("is_null", vec![value.clone()]);
    let result2 = gen2.array_function("is_null", vec![value]);

    assert_eq!(result1.to_string(), result2.to_string());
}

#[test]
fn test_generator_independent_is_some() {
    let gen1 = FunctionGenerator::new();
    let gen2 = FunctionGenerator::new();

    let value = quote::quote!(option);
    let result1 = gen1.array_function("is_some", vec![value.clone()]);
    let result2 = gen2.array_function("is_some", vec![value]);

    assert_eq!(result1.to_string(), result2.to_string());
}

// ============================================================================
// EDGE CASES IN FUNCTION SELECTION
// ============================================================================

#[test]
fn test_unknown_function_via_call() {
    let gen = FunctionGenerator::new();
    let result = gen.call("nonexistent_fn", vec![quote::quote!(data)]);
    let s = result.to_string();

    assert!(s.is_empty());
}

#[test]
fn test_empty_function_name() {
    let gen = FunctionGenerator::new();
    let result = gen.call("", vec![quote::quote!(data)]);
    let s = result.to_string();

    assert!(s.is_empty());
}

#[test]
fn test_function_case_sensitivity() {
    let gen = FunctionGenerator::new();

    // lowercase should work
    let result_lower = gen.string_function("length", vec![quote::quote!(text)]);
    // UPPERCASE should not match
    let result_upper = gen.string_function("LENGTH", vec![quote::quote!(text)]);

    // lowercase should produce code, uppercase should not
    assert!(!result_lower.to_string().is_empty());
    assert!(result_upper.to_string().is_empty());
}

#[test]
fn test_mixed_case_function_name() {
    let gen = FunctionGenerator::new();
    // CamelCase should not match snake_case
    let result = gen.string_function("Length", vec![quote::quote!(text)]);
    assert!(result.to_string().is_empty());
}
