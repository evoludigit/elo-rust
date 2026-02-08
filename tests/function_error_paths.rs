//! Tests for function generator error paths and edge cases

use elo_rust::codegen::functions::FunctionGenerator;

// ============================================================================
// STRING FUNCTION ERROR PATHS
// ============================================================================

#[test]
fn test_matches_with_no_arguments() {
    let gen = FunctionGenerator::new();
    let result = gen.string_function("matches", vec![]);
    let s = result.to_string();
    assert!(s.is_empty());
}

#[test]
fn test_matches_with_one_argument() {
    let gen = FunctionGenerator::new();
    let subject = quote::quote!(email);
    let result = gen.string_function("matches", vec![subject]);
    let s = result.to_string();
    assert!(s.is_empty());
}

#[test]
fn test_contains_with_no_arguments() {
    let gen = FunctionGenerator::new();
    let result = gen.string_function("contains", vec![]);
    let s = result.to_string();
    assert!(s.is_empty());
}

#[test]
fn test_contains_with_one_argument() {
    let gen = FunctionGenerator::new();
    let subject = quote::quote!(text);
    let result = gen.string_function("contains", vec![subject]);
    let s = result.to_string();
    assert!(s.is_empty());
}

#[test]
fn test_length_with_no_arguments() {
    let gen = FunctionGenerator::new();
    let result = gen.string_function("length", vec![]);
    let s = result.to_string();
    assert!(s.is_empty());
}

#[test]
fn test_uppercase_with_no_arguments() {
    let gen = FunctionGenerator::new();
    let result = gen.string_function("uppercase", vec![]);
    let s = result.to_string();
    assert!(s.is_empty());
}

#[test]
fn test_lowercase_with_no_arguments() {
    let gen = FunctionGenerator::new();
    let result = gen.string_function("lowercase", vec![]);
    let s = result.to_string();
    assert!(s.is_empty());
}

#[test]
fn test_trim_with_no_arguments() {
    let gen = FunctionGenerator::new();
    let result = gen.string_function("trim", vec![]);
    let s = result.to_string();
    assert!(s.is_empty());
}

#[test]
fn test_starts_with_with_no_arguments() {
    let gen = FunctionGenerator::new();
    let result = gen.string_function("starts_with", vec![]);
    let s = result.to_string();
    assert!(s.is_empty());
}

#[test]
fn test_starts_with_with_one_argument() {
    let gen = FunctionGenerator::new();
    let subject = quote::quote!(path);
    let result = gen.string_function("starts_with", vec![subject]);
    let s = result.to_string();
    assert!(s.is_empty());
}

#[test]
fn test_ends_with_with_no_arguments() {
    let gen = FunctionGenerator::new();
    let result = gen.string_function("ends_with", vec![]);
    let s = result.to_string();
    assert!(s.is_empty());
}

#[test]
fn test_ends_with_with_one_argument() {
    let gen = FunctionGenerator::new();
    let subject = quote::quote!(filename);
    let result = gen.string_function("ends_with", vec![subject]);
    let s = result.to_string();
    assert!(s.is_empty());
}

#[test]
fn test_unknown_string_function() {
    let gen = FunctionGenerator::new();
    let subject = quote::quote!(text);
    let result = gen.string_function("unknown_function", vec![subject]);
    let s = result.to_string();
    assert!(s.is_empty());
}

// ============================================================================
// DATETIME FUNCTION ERROR PATHS
// ============================================================================

#[test]
fn test_today_generation() {
    let gen = FunctionGenerator::new();
    let result = gen.datetime_function("today", vec![]);
    let s = result.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_today_ignores_arguments() {
    let gen = FunctionGenerator::new();
    let result = gen.datetime_function("today", vec![quote::quote!(ignored)]);
    let s = result.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_now_generation() {
    let gen = FunctionGenerator::new();
    let result = gen.datetime_function("now", vec![]);
    let s = result.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_now_ignores_arguments() {
    let gen = FunctionGenerator::new();
    let result = gen.datetime_function("now", vec![quote::quote!(ignored)]);
    let s = result.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_age_with_no_arguments() {
    let gen = FunctionGenerator::new();
    let result = gen.datetime_function("age", vec![]);
    let s = result.to_string();
    assert!(s.is_empty());
}

#[test]
fn test_age_with_one_argument() {
    let gen = FunctionGenerator::new();
    let date = quote::quote!(birth_date);
    let result = gen.datetime_function("age", vec![date]);
    let s = result.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_days_since_with_no_arguments() {
    let gen = FunctionGenerator::new();
    let result = gen.datetime_function("days_since", vec![]);
    let s = result.to_string();
    assert!(s.is_empty());
}

#[test]
fn test_days_since_with_one_argument() {
    let gen = FunctionGenerator::new();
    let date = quote::quote!(event_date);
    let result = gen.datetime_function("days_since", vec![date]);
    let s = result.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_date_with_no_arguments() {
    let gen = FunctionGenerator::new();
    let result = gen.datetime_function("date", vec![]);
    let s = result.to_string();
    assert!(s.is_empty());
}

#[test]
fn test_date_with_one_argument() {
    let gen = FunctionGenerator::new();
    let date_str = quote::quote!("2020-01-01");
    let result = gen.datetime_function("date", vec![date_str]);
    let s = result.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_unknown_datetime_function() {
    let gen = FunctionGenerator::new();
    let result = gen.datetime_function("unknown_datetime_fn", vec![]);
    let s = result.to_string();
    assert!(s.is_empty());
}

// ============================================================================
// ARRAY FUNCTION ERROR PATHS
// ============================================================================

#[test]
fn test_array_contains_with_no_arguments() {
    let gen = FunctionGenerator::new();
    let result = gen.array_function("contains", vec![]);
    let s = result.to_string();
    assert!(s.is_empty());
}

#[test]
fn test_array_contains_with_one_argument() {
    let gen = FunctionGenerator::new();
    let array = quote::quote!(items);
    let result = gen.array_function("contains", vec![array]);
    let s = result.to_string();
    assert!(s.is_empty());
}

#[test]
fn test_array_any_with_no_arguments() {
    let gen = FunctionGenerator::new();
    let result = gen.array_function("any", vec![]);
    let s = result.to_string();
    assert!(s.is_empty());
}

#[test]
fn test_array_any_with_one_argument() {
    let gen = FunctionGenerator::new();
    let array = quote::quote!(items);
    let result = gen.array_function("any", vec![array]);
    let s = result.to_string();
    assert!(s.is_empty());
}

#[test]
fn test_array_all_with_no_arguments() {
    let gen = FunctionGenerator::new();
    let result = gen.array_function("all", vec![]);
    let s = result.to_string();
    assert!(s.is_empty());
}

#[test]
fn test_array_all_with_one_argument() {
    let gen = FunctionGenerator::new();
    let array = quote::quote!(items);
    let result = gen.array_function("all", vec![array]);
    let s = result.to_string();
    assert!(s.is_empty());
}

#[test]
fn test_array_length_with_no_arguments() {
    let gen = FunctionGenerator::new();
    let result = gen.array_function("length", vec![]);
    let s = result.to_string();
    assert!(s.is_empty());
}

#[test]
fn test_array_length_with_one_argument() {
    let gen = FunctionGenerator::new();
    let array = quote::quote!(items);
    let result = gen.array_function("length", vec![array]);
    let s = result.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_array_is_empty_with_no_arguments() {
    let gen = FunctionGenerator::new();
    let result = gen.array_function("is_empty", vec![]);
    let s = result.to_string();
    assert!(s.is_empty());
}

#[test]
fn test_array_is_empty_with_one_argument() {
    let gen = FunctionGenerator::new();
    let array = quote::quote!(items);
    let result = gen.array_function("is_empty", vec![array]);
    let s = result.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_unknown_array_function() {
    let gen = FunctionGenerator::new();
    let array = quote::quote!(items);
    let result = gen.array_function("unknown_array_fn", vec![array]);
    let s = result.to_string();
    assert!(s.is_empty());
}

// ============================================================================
// GENERIC CALL FUNCTION ROUTING
// ============================================================================

#[test]
fn test_call_routes_to_string_function() {
    let gen = FunctionGenerator::new();
    let text = quote::quote!(name);
    let pattern = quote::quote!("John");
    let result = gen.call("contains", vec![text, pattern]);
    let s = result.to_string();
    assert!(s.contains("contains"));
}

#[test]
fn test_call_routes_to_datetime_function() {
    let gen = FunctionGenerator::new();
    let result = gen.call("today", vec![]);
    let s = result.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_call_routes_to_array_function() {
    let gen = FunctionGenerator::new();
    let array = quote::quote!(roles);
    let value = quote::quote!("admin");
    let result = gen.call("contains", vec![array, value]);
    let s = result.to_string();
    assert!(s.contains("contains"));
}

#[test]
fn test_call_unknown_function() {
    let gen = FunctionGenerator::new();
    let result = gen.call("nonexistent_function", vec![]);
    let s = result.to_string();
    assert!(s.is_empty());
}

#[test]
fn test_call_with_no_arguments() {
    let gen = FunctionGenerator::new();
    let result = gen.call("today", vec![]);
    let s = result.to_string();
    assert!(!s.is_empty());
}

// ============================================================================
// GENERATOR CONSISTENCY
// ============================================================================

#[test]
fn test_multiple_generator_instances_are_independent() {
    let gen1 = FunctionGenerator::new();
    let gen2 = FunctionGenerator::new();

    let text = quote::quote!(value);
    let result1 = gen1.string_function("length", vec![text.clone()]);
    let result2 = gen2.string_function("length", vec![text]);

    assert_eq!(result1.to_string(), result2.to_string());
}

#[test]
fn test_default_instance() {
    let gen = FunctionGenerator::default();
    let text = quote::quote!(data);
    let result = gen.string_function("trim", vec![text]);
    let s = result.to_string();
    assert!(s.contains("trim"));
}
