//! AST visitor and code generation tests
//!
//! Tests for the core code generation engine that transforms ELO AST to Rust

use elo_rust::codegen::types::{RustType, TypeContext, TypeInfo};
use elo_rust::codegen::RustCodeGenerator;

#[test]
fn test_generator_creation() {
    let gen = RustCodeGenerator::new();
    assert!(gen.is_valid());
}

#[test]
fn test_generator_with_type_context() {
    let context = TypeContext::new();
    let gen = RustCodeGenerator::with_context(context);
    assert!(gen.is_valid());
}

#[test]
fn test_generator_type_context_access() {
    let mut context = TypeContext::new();
    let mut user_type = TypeInfo::new("User");
    user_type.add_field("age", RustType::Integer);
    context.register_type("User", user_type);

    let gen = RustCodeGenerator::with_context(context);
    assert!(gen.has_type("User"));
    assert!(gen.get_field_type("User", "age").is_some());
}

#[test]
fn test_function_signature_generation_simple() {
    let gen = RustCodeGenerator::new();
    let sig = gen
        .generate_function_signature("validate_user", "User")
        .unwrap();
    let sig_str = sig.to_string();

    assert!(sig_str.contains("validate_user"));
    assert!(sig_str.contains("User"));
    assert!(sig_str.contains("Result"));
}

#[test]
fn test_function_signature_with_validation_errors() {
    let gen = RustCodeGenerator::new();
    let sig = gen
        .generate_function_signature("validate", "Product")
        .unwrap();
    let sig_str = sig.to_string();

    assert!(sig_str.contains("validate"));
    assert!(sig_str.contains("Product"));
}

#[test]
fn test_literal_integer_generation() {
    let gen = RustCodeGenerator::new();
    let tokens = gen.generate_literal_integer(42).unwrap();
    let s = tokens.to_string();

    assert!(s.contains("42"));
}

#[test]
fn test_literal_string_generation() {
    let gen = RustCodeGenerator::new();
    let tokens = gen.generate_literal_string("hello").unwrap();
    let s = tokens.to_string();

    assert!(s.contains("hello"));
}

#[test]
fn test_literal_bool_generation() {
    let gen = RustCodeGenerator::new();
    let true_tokens = gen.generate_literal_bool(true).unwrap();
    let false_tokens = gen.generate_literal_bool(false).unwrap();

    assert!(true_tokens.to_string().contains("true"));
    assert!(false_tokens.to_string().contains("false"));
}

#[test]
fn test_field_access_generation() {
    let gen = RustCodeGenerator::new();
    let tokens = gen.generate_field_access("user", "age").unwrap();
    let s = tokens.to_string();

    assert!(s.contains("user"));
    assert!(s.contains("age"));
}

#[test]
fn test_field_access_nested() {
    let gen = RustCodeGenerator::new();
    // For nested access, we generate intermediate access (address is a field of user)
    let tokens = gen.generate_field_access("user", "address").unwrap();
    let s = tokens.to_string();

    assert!(s.contains("user"));
    assert!(s.contains("address"));
}

#[test]
fn test_multiple_generators_independent() {
    let mut context1 = TypeContext::new();
    let mut user_type = TypeInfo::new("User");
    user_type.add_field("id", RustType::Integer);
    context1.register_type("User", user_type);

    let gen1 = RustCodeGenerator::with_context(context1);
    let gen2 = RustCodeGenerator::new();

    // gen1 has User type, gen2 doesn't
    assert!(gen1.has_type("User"));
    assert!(!gen2.has_type("User"));
}

#[test]
fn test_generator_error_handling_invalid_field() {
    let gen = RustCodeGenerator::new();
    // Accessing field on type that doesn't exist should be handled gracefully
    let result = gen.get_field_type("NonExistent", "field");
    assert!(result.is_none());
}

#[test]
fn test_generate_comment() {
    let gen = RustCodeGenerator::new();
    let comment = gen.generate_comment("This is a test comment").unwrap();
    // Comment generation returns token stream (comments handled separately)
    let s = comment.to_string();
    // Token stream might be empty for pure comments, that's ok
    let _ = s; // Just verify no panic occurred
}

#[test]
fn test_generate_module_doc() {
    let gen = RustCodeGenerator::new();
    let doc = gen.generate_doc_comment("Module documentation").unwrap();
    // Doc comment generation returns token stream (docs handled separately)
    let s = doc.to_string();
    // Token stream might be empty for pure docs, that's ok
    let _ = s; // Just verify no panic occurred
}

#[test]
fn test_generator_state_consistency() {
    let gen = RustCodeGenerator::new();

    // Multiple calls should produce consistent results
    let sig1 = gen.generate_function_signature("test", "Input").unwrap();
    let sig2 = gen.generate_function_signature("test", "Input").unwrap();

    assert_eq!(sig1.to_string(), sig2.to_string());
}
