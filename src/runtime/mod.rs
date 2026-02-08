//! Runtime support for validation
//!
//! Provides error types, utilities, and dynamic value representation for generated validators

pub mod value;
pub mod temporal;

pub use value::EloValue;
pub use temporal::TemporalValue;

use std::fmt;

/// A single validation error
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    /// The path to the field being validated (e.g., "user.email")
    pub path: String,
    /// Human-readable error message
    pub message: String,
    /// The rule that failed
    pub rule: String,
    /// Optional value for debugging
    pub value: Option<String>,
}

impl ValidationError {
    /// Create a new validation error
    pub fn new(
        path: impl Into<String>,
        message: impl Into<String>,
        rule: impl Into<String>,
    ) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
            rule: rule.into(),
            value: None,
        }
    }

    /// Add a value for debugging
    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.path, self.message)
    }
}

impl std::error::Error for ValidationError {}

/// Multiple validation errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationErrors {
    /// Collection of validation errors
    pub errors: Vec<ValidationError>,
}

impl ValidationErrors {
    /// Create a new error collection
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    /// Add an error to the collection
    pub fn push(&mut self, error: ValidationError) {
        self.errors.push(error);
    }

    /// Check if there are any errors
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Get the number of errors
    pub fn len(&self) -> usize {
        self.errors.len()
    }
}

impl Default for ValidationErrors {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, err) in self.errors.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "{}", err)?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationErrors {}

#[cfg(test)]
mod tests {
    use super::*;

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
        let err =
            ValidationError::new("email", "Invalid email", "email_pattern").with_value("invalid@");
        assert_eq!(err.value, Some("invalid@".to_string()));
    }

    #[test]
    fn test_validation_error_display() {
        let err = ValidationError::new("email", "Invalid email", "email_pattern");
        assert_eq!(err.to_string(), "email: Invalid email");
    }

    #[test]
    fn test_validation_errors_collection() {
        let mut errors = ValidationErrors::new();
        assert!(errors.is_empty());
        assert_eq!(errors.len(), 0);

        errors.push(ValidationError::new("email", "Invalid", "rule1"));
        assert!(!errors.is_empty());
        assert_eq!(errors.len(), 1);

        errors.push(ValidationError::new("age", "Too young", "rule2"));
        assert_eq!(errors.len(), 2);
    }
}
