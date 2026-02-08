//! Common test utilities

use elo_rust::codegen::types::{RustType, TypeContext, TypeInfo};

/// Create a test type context with predefined types
pub fn test_type_context() -> TypeContext {
    let mut context = TypeContext::new();

    // User type
    let mut user_type = TypeInfo::new("User");
    user_type.add_field("id", RustType::Integer);
    user_type.add_field("email", RustType::String);
    user_type.add_field("age", RustType::Integer);
    user_type.add_field("verified", RustType::Bool);
    user_type.add_field("created_at", RustType::Date);
    user_type.add_field("tags", RustType::Array(Box::new(RustType::String)));

    context.register_type("User", user_type);

    // Product type
    let mut product_type = TypeInfo::new("Product");
    product_type.add_field("id", RustType::Integer);
    product_type.add_field("name", RustType::String);
    product_type.add_field("price", RustType::Float);
    product_type.add_field("in_stock", RustType::Bool);
    product_type.add_field("description", RustType::Option(Box::new(RustType::String)));

    context.register_type("Product", product_type);

    context
}

/// Create a test type context with a single type
pub fn simple_user_type_context() -> TypeContext {
    let mut context = TypeContext::new();

    let mut user_type = TypeInfo::new("User");
    user_type.add_field("email", RustType::String);
    user_type.add_field("age", RustType::Integer);

    context.register_type("User", user_type);

    context
}

/// Helper to assert error messages contain expected text
#[macro_export]
macro_rules! assert_error_contains {
    ($error:expr, $needle:expr) => {
        assert!(
            $error.to_string().contains($needle),
            "Error message '{}' should contain '{}'",
            $error.to_string(),
            $needle
        )
    };
}

/// Helper to assert validation error fields
#[macro_export]
macro_rules! assert_validation_error {
    ($error:expr, path: $path:expr, message: $message:expr) => {
        assert_eq!($error.path, $path, "Error path mismatch");
        assert_eq!($error.message, $message, "Error message mismatch");
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_context_creation() {
        let ctx = test_type_context();
        assert!(!ctx.is_empty());
        let types = ctx.list_all_type_names();
        assert!(types.contains(&"User".to_string()));
        assert!(types.contains(&"Product".to_string()));
    }

    #[test]
    fn test_simple_user_type_context() {
        let ctx = simple_user_type_context();
        assert!(!ctx.is_empty());
        assert_eq!(ctx.get_field_type("User", "email"), Some(&RustType::String));
        assert_eq!(ctx.get_field_type("User", "age"), Some(&RustType::Integer));
    }

    #[test]
    fn test_test_type_context_has_user_fields() {
        let ctx = test_type_context();
        assert_eq!(ctx.get_field_type("User", "email"), Some(&RustType::String));
        assert_eq!(ctx.get_field_type("User", "age"), Some(&RustType::Integer));
        assert_eq!(
            ctx.get_field_type("User", "verified"),
            Some(&RustType::Bool)
        );
    }

    #[test]
    fn test_test_type_context_has_product_fields() {
        let ctx = test_type_context();
        assert_eq!(
            ctx.get_field_type("Product", "name"),
            Some(&RustType::String)
        );
        assert_eq!(
            ctx.get_field_type("Product", "price"),
            Some(&RustType::Float)
        );
        assert_eq!(
            ctx.get_field_type("Product", "in_stock"),
            Some(&RustType::Bool)
        );
    }
}
