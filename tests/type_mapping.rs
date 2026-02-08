//! Type mapping tests
//!
//! Comprehensive tests for ELO-to-Rust type system mapping

use elo_rust::codegen::types::{RustType, TypeContext, TypeInfo};

#[test]
fn test_basic_type_conversions() {
    assert_eq!(RustType::String.to_rust_string(), "&str");
    assert_eq!(RustType::Integer.to_rust_string(), "i64");
    assert_eq!(RustType::Float.to_rust_string(), "f64");
    assert_eq!(RustType::Bool.to_rust_string(), "bool");
    assert_eq!(RustType::Date.to_rust_string(), "chrono::NaiveDate");
    assert_eq!(RustType::Time.to_rust_string(), "chrono::NaiveTime");
    assert_eq!(RustType::Duration.to_rust_string(), "chrono::Duration");
}

#[test]
fn test_option_type_composition() {
    let opt_string = RustType::Option(Box::new(RustType::String));
    assert_eq!(opt_string.to_rust_string(), "Option<&str>");

    let opt_int = RustType::Option(Box::new(RustType::Integer));
    assert_eq!(opt_int.to_rust_string(), "Option<i64>");

    let opt_option_string =
        RustType::Option(Box::new(RustType::Option(Box::new(RustType::String))));
    assert_eq!(opt_option_string.to_rust_string(), "Option<Option<&str>>");
}

#[test]
fn test_array_type_composition() {
    let array_string = RustType::Array(Box::new(RustType::String));
    assert_eq!(array_string.to_rust_string(), "&[&str]");

    let array_int = RustType::Array(Box::new(RustType::Integer));
    assert_eq!(array_int.to_rust_string(), "&[i64]");

    let array_of_options = RustType::Array(Box::new(RustType::Option(Box::new(RustType::String))));
    assert_eq!(array_of_options.to_rust_string(), "&[Option<&str>]");
}

#[test]
fn test_custom_type_mapping() {
    let user_type = RustType::Custom("User".to_string());
    assert_eq!(user_type.to_rust_string(), "User");

    let custom_array = RustType::Array(Box::new(RustType::Custom("Item".to_string())));
    assert_eq!(custom_array.to_rust_string(), "&[Item]");
}

#[test]
fn test_type_context_creation() {
    let context = TypeContext::new();
    assert!(context.is_empty());
}

#[test]
fn test_type_context_register_struct() {
    let mut context = TypeContext::new();

    // Register a User struct with fields
    let mut user_type = TypeInfo::new("User");
    user_type.add_field("email", RustType::String);
    user_type.add_field("age", RustType::Integer);
    user_type.add_field("verified", RustType::Bool);

    context.register_type("User", user_type);
    assert!(!context.is_empty());
}

#[test]
fn test_type_context_field_lookup() {
    let mut context = TypeContext::new();

    let mut user_type = TypeInfo::new("User");
    user_type.add_field("email", RustType::String);
    user_type.add_field("age", RustType::Integer);
    user_type.add_field("verified", RustType::Bool);

    context.register_type("User", user_type);

    // Look up fields
    assert_eq!(
        context.get_field_type("User", "email"),
        Some(&RustType::String)
    );
    assert_eq!(
        context.get_field_type("User", "age"),
        Some(&RustType::Integer)
    );
    assert_eq!(
        context.get_field_type("User", "verified"),
        Some(&RustType::Bool)
    );

    // Non-existent field
    assert_eq!(context.get_field_type("User", "name"), None);

    // Non-existent type
    assert_eq!(context.get_field_type("Product", "email"), None);
}

#[test]
fn test_type_context_nested_types() {
    let mut context = TypeContext::new();

    // Register Address struct
    let mut address_type = TypeInfo::new("Address");
    address_type.add_field("street", RustType::String);
    address_type.add_field("city", RustType::String);

    // Register User struct with Address field
    let mut user_type = TypeInfo::new("User");
    user_type.add_field("email", RustType::String);
    user_type.add_field("address", RustType::Custom("Address".to_string()));

    context.register_type("Address", address_type);
    context.register_type("User", user_type);

    // Verify nested type resolution
    assert_eq!(
        context.get_field_type("User", "address"),
        Some(&RustType::Custom("Address".to_string()))
    );
}

#[test]
fn test_type_inference_basic() {
    let context = TypeContext::new();

    // Integer literal should infer to i64
    assert_eq!(context.infer_from_literal("42"), RustType::Integer);

    // String literal should infer to String
    assert_eq!(context.infer_from_literal("\"hello\""), RustType::String);

    // Boolean literal should infer to Bool
    assert_eq!(context.infer_from_literal("true"), RustType::Bool);
    assert_eq!(context.infer_from_literal("false"), RustType::Bool);
}

#[test]
fn test_type_inference_numeric() {
    let context = TypeContext::new();

    assert_eq!(context.infer_from_literal("0"), RustType::Integer);
    assert_eq!(context.infer_from_literal("123456"), RustType::Integer);
    assert_eq!(context.infer_from_literal("-789"), RustType::Integer);
}

#[test]
fn test_type_compatibility() {
    // Verify type compatibility rules
    assert!(RustType::Integer.is_compatible_with(&RustType::Integer));
    assert!(RustType::String.is_compatible_with(&RustType::String));

    // Option types
    assert!(RustType::Option(Box::new(RustType::String))
        .is_compatible_with(&RustType::Option(Box::new(RustType::String))));

    // Different types are not compatible
    assert!(!RustType::Integer.is_compatible_with(&RustType::String));
    assert!(!RustType::Bool.is_compatible_with(&RustType::Integer));
}

#[test]
fn test_type_context_multiple_structs() {
    let mut context = TypeContext::new();

    let mut user_type = TypeInfo::new("User");
    user_type.add_field("id", RustType::Integer);
    user_type.add_field("name", RustType::String);

    let mut product_type = TypeInfo::new("Product");
    product_type.add_field("id", RustType::Integer);
    product_type.add_field("price", RustType::Float);

    context.register_type("User", user_type);
    context.register_type("Product", product_type);

    // Both types should be independently queryable
    assert_eq!(
        context.get_field_type("User", "name"),
        Some(&RustType::String)
    );
    assert_eq!(
        context.get_field_type("Product", "price"),
        Some(&RustType::Float)
    );

    // Cross-queries should not interfere
    assert_eq!(context.get_field_type("User", "price"), None);
    assert_eq!(context.get_field_type("Product", "name"), None);
}

#[test]
fn test_type_list_all_types() {
    let context = TypeContext::new();
    let all_types = context.list_all_type_names();
    assert!(all_types.is_empty());

    // After registering types, should be able to list them
    let mut context = TypeContext::new();
    context.register_type("User", TypeInfo::new("User"));
    context.register_type("Product", TypeInfo::new("Product"));

    let names = context.list_all_type_names();
    assert_eq!(names.len(), 2);
    assert!(names.contains(&"User".to_string()));
    assert!(names.contains(&"Product".to_string()));
}
