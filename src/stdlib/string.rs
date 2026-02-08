//! String manipulation functions

/// String function signatures
pub const STRING_FUNCTIONS: &[&str] = &[
    "matches",
    "contains",
    "length",
    "uppercase",
    "lowercase",
    "trim",
    "starts_with",
    "ends_with",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_functions_count() {
        assert_eq!(STRING_FUNCTIONS.len(), 8);
    }

    #[test]
    fn test_matches_function_exists() {
        assert!(STRING_FUNCTIONS.contains(&"matches"));
    }

    #[test]
    fn test_all_string_functions_exist() {
        assert!(STRING_FUNCTIONS.contains(&"contains"));
        assert!(STRING_FUNCTIONS.contains(&"length"));
        assert!(STRING_FUNCTIONS.contains(&"uppercase"));
    }
}
