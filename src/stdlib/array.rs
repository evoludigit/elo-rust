//! Array and collection functions

/// Array function signatures
pub const ARRAY_FUNCTIONS: &[&str] = &["contains", "any", "all", "length", "is_empty"];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_functions_count() {
        assert_eq!(ARRAY_FUNCTIONS.len(), 5);
    }

    #[test]
    fn test_contains_function_exists() {
        assert!(ARRAY_FUNCTIONS.contains(&"contains"));
    }

    #[test]
    fn test_all_array_functions_exist() {
        assert!(ARRAY_FUNCTIONS.contains(&"any"));
        assert!(ARRAY_FUNCTIONS.contains(&"all"));
        assert!(ARRAY_FUNCTIONS.contains(&"length"));
    }
}
