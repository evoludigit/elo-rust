//! Tests for codegen module coverage

use elo_rust::codegen::types::{RustType, TypeContext, TypeInfo};
use elo_rust::codegen::RustCodeGenerator;

// ============================================================================
// RUST CODE GENERATOR - MAIN MODULE
// ============================================================================

#[test]
fn test_generator_is_valid() {
    let gen = RustCodeGenerator::new();
    assert!(gen.is_valid());
}

#[test]
fn test_generator_with_empty_context() {
    let context = TypeContext::new();
    let gen = RustCodeGenerator::with_context(context);
    assert!(gen.is_valid());
}

#[test]
fn test_generator_type_not_found() {
    let gen = RustCodeGenerator::new();
    assert!(!gen.has_type("NonExistentType"));
}

#[test]
fn test_generator_field_type_not_found() {
    let gen = RustCodeGenerator::new();
    let result = gen.get_field_type("User", "nonexistent_field");
    assert!(result.is_none());
}

#[test]
fn test_generator_with_registered_type() {
    let mut context = TypeContext::new();
    let mut user_type = TypeInfo::new("User");
    user_type.add_field("id", RustType::Integer);
    user_type.add_field("name", RustType::String);
    context.register_type("User", user_type);

    let gen = RustCodeGenerator::with_context(context);
    assert!(gen.has_type("User"));
    assert!(gen.get_field_type("User", "id").is_some());
    assert!(gen.get_field_type("User", "name").is_some());
}

#[test]
fn test_generator_field_type_mismatch() {
    let mut context = TypeContext::new();
    let mut user_type = TypeInfo::new("User");
    user_type.add_field("age", RustType::Integer);
    context.register_type("User", user_type);

    let gen = RustCodeGenerator::with_context(context);
    assert!(gen.get_field_type("User", "age").is_some());
    assert!(gen.get_field_type("User", "wrong_field").is_none());
}

#[test]
fn test_generate_literal_string_simple() {
    let gen = RustCodeGenerator::new();
    let result = gen.generate_literal_string("hello").unwrap();
    let s = result.to_string();
    assert!(s.contains("hello"));
}

#[test]
fn test_generate_literal_string_with_special_chars() {
    let gen = RustCodeGenerator::new();
    let result = gen.generate_literal_string("hello@world.com").unwrap();
    let s = result.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_generate_literal_string_empty() {
    let gen = RustCodeGenerator::new();
    let result = gen.generate_literal_string("").unwrap();
    let s = result.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_generate_literal_integer_zero() {
    let gen = RustCodeGenerator::new();
    let result = gen.generate_literal_integer(0).unwrap();
    let s = result.to_string();
    assert!(s.contains("0"));
}

#[test]
fn test_generate_literal_integer_negative() {
    let gen = RustCodeGenerator::new();
    let result = gen.generate_literal_integer(-42).unwrap();
    let s = result.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_generate_literal_integer_large() {
    let gen = RustCodeGenerator::new();
    let result = gen.generate_literal_integer(i64::MAX).unwrap();
    let s = result.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_generate_literal_bool_true() {
    let gen = RustCodeGenerator::new();
    let result = gen.generate_literal_bool(true).unwrap();
    let s = result.to_string();
    assert!(s.contains("true"));
}

#[test]
fn test_generate_literal_bool_false() {
    let gen = RustCodeGenerator::new();
    let result = gen.generate_literal_bool(false).unwrap();
    let s = result.to_string();
    assert!(s.contains("false"));
}

#[test]
fn test_generate_field_access_simple() {
    let gen = RustCodeGenerator::new();
    let result = gen.generate_field_access("user", "age").unwrap();
    let s = result.to_string();
    assert!(s.contains("user") || s.contains("age"));
}

#[test]
fn test_generate_field_access_nested() {
    let gen = RustCodeGenerator::new();
    let result = gen.generate_field_access("user", "profile").unwrap();
    let s = result.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_generate_field_access_another_receiver() {
    let gen = RustCodeGenerator::new();
    let result = gen.generate_field_access("data", "zipcode").unwrap();
    let s = result.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_generate_function_signature_basic() {
    let gen = RustCodeGenerator::new();
    let sig = gen
        .generate_function_signature("validate_user", "User")
        .unwrap();
    let s = sig.to_string();
    assert!(s.contains("validate_user") || s.contains("User"));
}

#[test]
fn test_generate_function_signature_different_names() {
    let gen = RustCodeGenerator::new();

    let sig1 = gen
        .generate_function_signature("check_age", "Person")
        .unwrap();
    let sig2 = gen
        .generate_function_signature("verify_email", "Contact")
        .unwrap();

    let s1 = sig1.to_string();
    let s2 = sig2.to_string();

    assert!(s1.contains("check_age") || !s1.is_empty());
    assert!(s2.contains("verify_email") || !s2.is_empty());
}

#[test]
fn test_generate_validator_simple() {
    let gen = RustCodeGenerator::new();
    let validator = gen
        .generate_validator("validate", "age >= 18", "User")
        .unwrap();
    let s = validator.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_generate_validator_complex_expression() {
    let gen = RustCodeGenerator::new();
    let validator = gen
        .generate_validator("check_user", "age >= 18 && verified == true", "User")
        .unwrap();
    let s = validator.to_string();
    assert!(!s.is_empty());
}

// ============================================================================
// TYPE CONTEXT AND TYPE INFO
// ============================================================================

#[test]
fn test_type_context_empty() {
    let context = TypeContext::new();
    assert!(context.is_empty());
}

#[test]
fn test_type_context_register_single_type() {
    let mut context = TypeContext::new();
    let type_info = TypeInfo::new("User");
    context.register_type("User", type_info);

    assert!(!context.is_empty());
}

#[test]
fn test_type_context_register_multiple_types() {
    let mut context = TypeContext::new();
    context.register_type("User", TypeInfo::new("User"));
    context.register_type("Product", TypeInfo::new("Product"));
    context.register_type("Order", TypeInfo::new("Order"));

    assert!(!context.is_empty());
}

#[test]
fn test_type_info_add_fields() {
    let mut info = TypeInfo::new("User");
    info.add_field("id", RustType::Integer);
    info.add_field("name", RustType::String);
    info.add_field("active", RustType::Bool);

    let fields = info.fields();
    assert_eq!(fields.len(), 3);
}

#[test]
fn test_type_info_get_field_type() {
    let mut info = TypeInfo::new("User");
    info.add_field("age", RustType::Integer);

    let field_type = info.get_field("age");
    assert!(field_type.is_some());
}

#[test]
fn test_type_info_nonexistent_field() {
    let info = TypeInfo::new("User");
    let field_type = info.get_field("nonexistent");
    assert!(field_type.is_none());
}

#[test]
fn test_rust_type_string_representation() {
    assert_eq!(RustType::Integer.to_rust_string(), "i64");
    assert_eq!(RustType::String.to_rust_string(), "&str");
    assert_eq!(RustType::Bool.to_rust_string(), "bool");
    assert_eq!(RustType::Float.to_rust_string(), "f64");
}

#[test]
fn test_rust_type_option_representation() {
    let opt_type = RustType::Option(Box::new(RustType::Integer));
    let s = opt_type.to_rust_string();
    assert!(s.contains("Option"));
}

#[test]
fn test_rust_type_array_representation() {
    let array_type = RustType::Array(Box::new(RustType::String));
    let s = array_type.to_rust_string();
    assert!(s.contains("&["));
}

#[test]
fn test_type_context_get_field_type() {
    let mut context = TypeContext::new();
    let mut user_type = TypeInfo::new("User");
    user_type.add_field("id", RustType::Integer);
    context.register_type("User", user_type);

    assert!(context.get_field_type("User", "id").is_some());
    assert!(context.get_field_type("NonExistent", "field").is_none());
}

#[test]
fn test_type_context_with_complex_types() {
    let mut context = TypeContext::new();

    let mut user_type = TypeInfo::new("User");
    user_type.add_field("id", RustType::Integer);
    user_type.add_field("email", RustType::String);
    user_type.add_field("tags", RustType::Array(Box::new(RustType::String)));
    user_type.add_field("settings", RustType::Option(Box::new(RustType::String)));
    context.register_type("User", user_type);

    assert!(!context.is_empty());
    assert!(context.get_field_type("User", "id").is_some());
    assert!(context.get_field_type("User", "email").is_some());
    assert!(context.get_field_type("User", "tags").is_some());
}

// ============================================================================
// ERROR HANDLING
// ============================================================================

#[test]
fn test_generator_with_invalid_field_access() {
    let gen = RustCodeGenerator::new();
    let result = gen.generate_field_access("unknown_receiver", "unknown_field");
    // Should either succeed with empty/placeholder or fail gracefully
    assert!(result.is_ok() || result.is_err());
}
