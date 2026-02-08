//! Error handling tests
//!
//! Comprehensive tests for validation errors and code generation errors

use elo_rust::codegen::CodeGenError;
use elo_rust::{ValidationError, ValidationErrors};

#[test]
fn test_validation_error_creation() {
    let err = ValidationError::new("email", "Invalid email", "email_pattern");
    assert_eq!(err.path, "email");
    assert_eq!(err.message, "Invalid email");
    assert_eq!(err.rule, "email_pattern");
    assert_eq!(err.value, None);
}

#[test]
fn test_validation_error_with_value() {
    let err = ValidationError::new("email", "Invalid email", "email_pattern")
        .with_value("invalid@example");
    assert_eq!(err.value, Some("invalid@example".to_string()));
}

#[test]
fn test_validation_error_display() {
    let err = ValidationError::new("email", "Invalid email", "email_pattern");
    assert_eq!(err.to_string(), "email: Invalid email");
}

#[test]
fn test_validation_error_debug() {
    let err = ValidationError::new("age", "Too young", "min_age");
    let debug_str = format!("{:?}", err);
    assert!(debug_str.contains("age"));
    assert!(debug_str.contains("Too young"));
}

#[test]
fn test_validation_error_clone() {
    let err1 = ValidationError::new("field", "message", "rule");
    let err2 = err1.clone();
    assert_eq!(err1, err2);
}

#[test]
fn test_validation_errors_empty() {
    let errors = ValidationErrors::new();
    assert!(errors.is_empty());
    assert_eq!(errors.len(), 0);
}

#[test]
fn test_validation_errors_push() {
    let mut errors = ValidationErrors::new();
    assert!(errors.is_empty());

    errors.push(ValidationError::new("email", "Invalid", "rule1"));
    assert!(!errors.is_empty());
    assert_eq!(errors.len(), 1);

    errors.push(ValidationError::new("age", "Too young", "rule2"));
    assert_eq!(errors.len(), 2);
}

#[test]
fn test_validation_errors_default() {
    let errors = ValidationErrors::default();
    assert!(errors.is_empty());
}

#[test]
fn test_validation_errors_display_single() {
    let mut errors = ValidationErrors::new();
    errors.push(ValidationError::new("field", "message", "rule"));
    let output = errors.to_string();
    assert_eq!(output, "field: message");
}

#[test]
fn test_validation_errors_display_multiple() {
    let mut errors = ValidationErrors::new();
    errors.push(ValidationError::new("email", "Invalid email", "rule1"));
    errors.push(ValidationError::new("age", "Too young", "rule2"));

    let output = errors.to_string();
    assert!(output.contains("email: Invalid email"));
    assert!(output.contains("age: Too young"));
}

#[test]
fn test_validation_errors_clone() {
    let mut errors1 = ValidationErrors::new();
    errors1.push(ValidationError::new("field", "message", "rule"));

    let errors2 = errors1.clone();
    assert_eq!(errors1, errors2);
    assert_eq!(errors1.len(), errors2.len());
}

#[test]
fn test_codegen_error_unsupported_feature() {
    let err = CodeGenError::UnsupportedFeature("custom functions".to_string());
    let msg = err.to_string();
    assert!(msg.contains("custom functions"));
}

#[test]
fn test_codegen_error_type_mismatch() {
    let err = CodeGenError::TypeMismatch("Expected string, found integer".to_string());
    let msg = err.to_string();
    assert!(msg.contains("Expected string"));
}

#[test]
fn test_codegen_error_invalid_expression() {
    let err = CodeGenError::InvalidExpression("malformed syntax".to_string());
    let msg = err.to_string();
    assert!(msg.contains("malformed syntax"));
}

#[test]
fn test_codegen_error_debug() {
    let err = CodeGenError::UnsupportedFeature("async".to_string());
    let debug_str = format!("{:?}", err);
    assert!(debug_str.contains("UnsupportedFeature"));
}

#[test]
fn test_codegen_error_clone() {
    let err1 = CodeGenError::InvalidExpression("test".to_string());
    let err2 = err1.clone();
    assert_eq!(err1.to_string(), err2.to_string());
}

#[test]
fn test_validation_error_is_error() {
    use std::error::Error;
    let err: Box<dyn Error> = Box::new(ValidationError::new("field", "msg", "rule"));
    assert!(!err.to_string().is_empty());
}

#[test]
fn test_validation_errors_is_error() {
    use std::error::Error;
    let mut errors = ValidationErrors::new();
    errors.push(ValidationError::new("field", "msg", "rule"));
    let err: Box<dyn Error> = Box::new(errors);
    assert!(!err.to_string().is_empty());
}

#[test]
fn test_error_with_multiple_fields() {
    let err1 = ValidationError::new("user.email", "Invalid email", "email_pattern");
    let err2 = ValidationError::new("user.age", "Too young", "min_age");
    let err3 = ValidationError::new("user.status", "Invalid status", "enum_check");

    assert_eq!(err1.path, "user.email");
    assert_eq!(err2.path, "user.age");
    assert_eq!(err3.path, "user.status");
}

#[test]
fn test_validation_errors_iteration() {
    let mut errors = ValidationErrors::new();
    errors.push(ValidationError::new("field1", "msg1", "rule1"));
    errors.push(ValidationError::new("field2", "msg2", "rule2"));
    errors.push(ValidationError::new("field3", "msg3", "rule3"));

    assert_eq!(errors.len(), 3);
    assert!(!errors.is_empty());

    // Verify all errors are in the collection
    let output = errors.to_string();
    assert!(output.contains("field1"));
    assert!(output.contains("field2"));
    assert!(output.contains("field3"));
}

#[test]
fn test_validation_error_with_complex_value() {
    let complex_value = r#"{"nested": {"field": "value"}}"#;
    let err = ValidationError::new("json", "Invalid JSON", "json_parse").with_value(complex_value);
    assert_eq!(err.value, Some(complex_value.to_string()));
}

#[test]
fn test_validation_error_nested_path() {
    let err = ValidationError::new("user.address.street.line1", "Street too long", "max_length");
    assert_eq!(err.path, "user.address.street.line1");
    let display = err.to_string();
    assert!(display.contains("user.address.street.line1"));
}
