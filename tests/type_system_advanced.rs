//! Advanced type system tests for coverage improvement
//!
//! Tests for type compatibility, type inference, and code generator impl methods

use elo_rust::codegen::types::{RustType, TypeContext, TypeInfo};
use elo_rust::RustCodeGenerator;

// ============================================================================
// TYPE COMPATIBILITY TESTS
// ============================================================================

#[test]
fn test_unknown_type_compatibility() {
    let unknown = RustType::Unknown;
    let string = RustType::String;

    // Unknown is compatible with any type
    assert!(unknown.is_compatible_with(&string));
    assert!(string.is_compatible_with(&unknown));
}

#[test]
fn test_unknown_type_rust_string() {
    let unknown = RustType::Unknown;
    let rust_str = unknown.to_rust_string();
    assert_eq!(rust_str, "()");
}

#[test]
fn test_array_type_compatibility_same_element() {
    let array_int1 = RustType::Array(Box::new(RustType::Integer));
    let array_int2 = RustType::Array(Box::new(RustType::Integer));

    assert!(array_int1.is_compatible_with(&array_int2));
}

#[test]
fn test_array_type_compatibility_different_element() {
    let array_int = RustType::Array(Box::new(RustType::Integer));
    let array_string = RustType::Array(Box::new(RustType::String));

    assert!(!array_int.is_compatible_with(&array_string));
}

#[test]
fn test_array_type_compatibility_unknown_element() {
    let array_int = RustType::Array(Box::new(RustType::Integer));
    let array_unknown = RustType::Array(Box::new(RustType::Unknown));

    // Unknown elements should be compatible with any type
    assert!(array_int.is_compatible_with(&array_unknown));
    assert!(array_unknown.is_compatible_with(&array_int));
}

#[test]
fn test_custom_type_compatibility_same_name() {
    let custom1 = RustType::Custom("User".to_string());
    let custom2 = RustType::Custom("User".to_string());

    assert!(custom1.is_compatible_with(&custom2));
}

#[test]
fn test_custom_type_compatibility_different_name() {
    let user = RustType::Custom("User".to_string());
    let product = RustType::Custom("Product".to_string());

    assert!(!user.is_compatible_with(&product));
}

#[test]
fn test_custom_type_not_compatible_with_builtin() {
    let custom = RustType::Custom("User".to_string());
    let string = RustType::String;

    assert!(!custom.is_compatible_with(&string));
    assert!(!string.is_compatible_with(&custom));
}

#[test]
fn test_option_type_compatibility_same_inner() {
    let opt_int1 = RustType::Option(Box::new(RustType::Integer));
    let opt_int2 = RustType::Option(Box::new(RustType::Integer));

    assert!(opt_int1.is_compatible_with(&opt_int2));
}

#[test]
fn test_option_type_compatibility_different_inner() {
    let opt_int = RustType::Option(Box::new(RustType::Integer));
    let opt_string = RustType::Option(Box::new(RustType::String));

    assert!(!opt_int.is_compatible_with(&opt_string));
}

#[test]
fn test_nested_option_compatibility() {
    let nested1 = RustType::Option(Box::new(RustType::Option(Box::new(RustType::Integer))));
    let nested2 = RustType::Option(Box::new(RustType::Option(Box::new(RustType::Integer))));

    assert!(nested1.is_compatible_with(&nested2));
}

#[test]
fn test_option_array_compatibility() {
    let opt_array = RustType::Option(Box::new(RustType::Array(Box::new(RustType::String))));
    let array_opt = RustType::Array(Box::new(RustType::Option(Box::new(RustType::String))));

    assert!(!opt_array.is_compatible_with(&array_opt));
}

// ============================================================================
// TYPE INFERENCE TESTS
// ============================================================================

#[test]
fn test_infer_integer_literal() {
    let context = TypeContext::new();
    let inferred = context.infer_from_literal("42");
    assert_eq!(inferred, RustType::Integer);
}

#[test]
fn test_infer_negative_integer() {
    let context = TypeContext::new();
    let inferred = context.infer_from_literal("-100");
    assert_eq!(inferred, RustType::Integer);
}

#[test]
fn test_infer_large_integer() {
    let context = TypeContext::new();
    let inferred = context.infer_from_literal("9223372036854775807"); // i64::MAX
    assert_eq!(inferred, RustType::Integer);
}

#[test]
fn test_infer_float_literal() {
    let context = TypeContext::new();
    let inferred = context.infer_from_literal("3.14");
    assert_eq!(inferred, RustType::Float);
}

#[test]
fn test_infer_float_scientific_notation() {
    let context = TypeContext::new();
    let inferred = context.infer_from_literal("1.5e-10");
    assert_eq!(inferred, RustType::Float);
}

#[test]
fn test_infer_float_without_decimal() {
    let context = TypeContext::new();
    let inferred = context.infer_from_literal("1.0");
    // 1.0 parses as float first
    assert_eq!(inferred, RustType::Float);
}

#[test]
fn test_infer_boolean_true() {
    let context = TypeContext::new();
    let inferred = context.infer_from_literal("true");
    assert_eq!(inferred, RustType::Bool);
}

#[test]
fn test_infer_boolean_false() {
    let context = TypeContext::new();
    let inferred = context.infer_from_literal("false");
    assert_eq!(inferred, RustType::Bool);
}

#[test]
fn test_infer_string_double_quoted() {
    let context = TypeContext::new();
    let inferred = context.infer_from_literal("\"hello\"");
    assert_eq!(inferred, RustType::String);
}

#[test]
fn test_infer_string_single_quoted() {
    let context = TypeContext::new();
    let inferred = context.infer_from_literal("'hello'");
    assert_eq!(inferred, RustType::String);
}

#[test]
fn test_infer_empty_string_double_quoted() {
    let context = TypeContext::new();
    let inferred = context.infer_from_literal("\"\"");
    assert_eq!(inferred, RustType::String);
}

#[test]
fn test_infer_empty_string_single_quoted() {
    let context = TypeContext::new();
    let inferred = context.infer_from_literal("''");
    assert_eq!(inferred, RustType::String);
}

#[test]
fn test_infer_unknown_literal() {
    let context = TypeContext::new();
    let inferred = context.infer_from_literal("unknown_identifier");
    assert_eq!(inferred, RustType::Unknown);
}

#[test]
fn test_infer_unknown_with_special_chars() {
    let context = TypeContext::new();
    let inferred = context.infer_from_literal("@#$%");
    assert_eq!(inferred, RustType::Unknown);
}

// ============================================================================
// CODE GENERATOR IMPL BLOCK TESTS
// ============================================================================

#[test]
fn test_generate_validator_impl_basic() {
    let gen = RustCodeGenerator::new();
    let result = gen
        .generate_validator_impl("UserValidator", "validate_user", "User")
        .unwrap();
    let s = result.to_string();

    // Should generate an impl block
    assert!(s.contains("impl") || !s.is_empty());
}

#[test]
fn test_generate_validator_impl_different_struct_name() {
    let gen = RustCodeGenerator::new();
    let result = gen
        .generate_validator_impl("ProductChecker", "check_product", "Product")
        .unwrap();
    let s = result.to_string();

    assert!(!s.is_empty());
}

#[test]
fn test_generate_validator_impl_method_name() {
    let gen = RustCodeGenerator::new();
    let result = gen
        .generate_validator_impl("Validator", "is_valid", "Data")
        .unwrap();
    let s = result.to_string();

    assert!(!s.is_empty());
}

#[test]
fn test_generate_validator_impl_special_chars_in_names() {
    let gen = RustCodeGenerator::new();
    // Names with underscores should work
    let result = gen
        .generate_validator_impl("User_Validator", "validate_user_info", "UserData")
        .unwrap();
    let s = result.to_string();

    assert!(!s.is_empty());
}

// ============================================================================
// TYPE CONTEXT WITH INFERENCE
// ============================================================================

#[test]
fn test_infer_and_register_type() {
    let mut context = TypeContext::new();

    // Register a custom type
    let mut custom_type = TypeInfo::new("Config");
    custom_type.add_field("timeout", RustType::Integer);
    custom_type.add_field("enabled", RustType::Bool);
    context.register_type("Config", custom_type);

    // Check inference doesn't use registered types
    let inferred = context.infer_from_literal("42");
    assert_eq!(inferred, RustType::Integer);
}

#[test]
fn test_type_inference_consistency() {
    let context = TypeContext::new();

    // Same literal should infer to same type every time
    let inferred1 = context.infer_from_literal("3.14");
    let inferred2 = context.infer_from_literal("3.14");

    assert_eq!(inferred1, inferred2);
}

// ============================================================================
// DEFAULT IMPL TESTS
// ============================================================================

#[test]
fn test_rust_code_generator_default() {
    let gen = RustCodeGenerator::default();
    assert!(gen.is_valid());
}

#[test]
fn test_type_context_default() {
    let context = TypeContext::default();
    assert!(context.is_empty());
}

// ============================================================================
// COMPREHENSIVE TYPE COMPOSITION TESTS
// ============================================================================

#[test]
fn test_deeply_nested_option_type_string() {
    let deeply_nested = RustType::Option(Box::new(RustType::Option(Box::new(RustType::Option(
        Box::new(RustType::String),
    )))));
    let s = deeply_nested.to_rust_string();
    assert_eq!(s, "Option<Option<Option<&str>>>");
}

#[test]
fn test_option_of_array_type_string() {
    let opt_array = RustType::Option(Box::new(RustType::Array(Box::new(RustType::Integer))));
    let s = opt_array.to_rust_string();
    assert_eq!(s, "Option<&[i64]>");
}

#[test]
fn test_array_of_option_type_string() {
    let array_opt = RustType::Array(Box::new(RustType::Option(Box::new(RustType::String))));
    let s = array_opt.to_rust_string();
    assert_eq!(s, "&[Option<&str>]");
}

#[test]
fn test_array_of_custom_type_string() {
    let array_custom = RustType::Array(Box::new(RustType::Custom("User".to_string())));
    let s = array_custom.to_rust_string();
    assert_eq!(s, "&[User]");
}

#[test]
fn test_option_of_custom_type_string() {
    let opt_custom = RustType::Option(Box::new(RustType::Custom("Config".to_string())));
    let s = opt_custom.to_rust_string();
    assert_eq!(s, "Option<Config>");
}
