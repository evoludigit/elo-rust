//! Type checking functions

/// Type checking function signatures
pub const TYPE_FUNCTIONS: &[&str] = &["is_null", "is_some", "is_empty", "is_string", "is_number"];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_functions_count() {
        assert_eq!(TYPE_FUNCTIONS.len(), 5);
    }

    #[test]
    fn test_is_null_function_exists() {
        assert!(TYPE_FUNCTIONS.contains(&"is_null"));
    }

    #[test]
    fn test_all_type_functions_exist() {
        assert!(TYPE_FUNCTIONS.contains(&"is_some"));
        assert!(TYPE_FUNCTIONS.contains(&"is_empty"));
        assert!(TYPE_FUNCTIONS.contains(&"is_string"));
    }
}
