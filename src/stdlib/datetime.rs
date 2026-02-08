//! Date and time functions

/// DateTime function signatures
pub const DATETIME_FUNCTIONS: &[&str] = &["today", "now", "age", "days_since", "duration_days"];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datetime_functions_count() {
        assert_eq!(DATETIME_FUNCTIONS.len(), 5);
    }

    #[test]
    fn test_today_function_exists() {
        assert!(DATETIME_FUNCTIONS.contains(&"today"));
    }

    #[test]
    fn test_all_datetime_functions_exist() {
        assert!(DATETIME_FUNCTIONS.contains(&"now"));
        assert!(DATETIME_FUNCTIONS.contains(&"age"));
        assert!(DATETIME_FUNCTIONS.contains(&"days_since"));
    }
}
