//! Code generation errors
//!
//! Provides error types for code generation failures, including unsupported features,
//! type mismatches, and invalid expressions.

use std::fmt;

/// Errors that can occur during ELO-to-Rust code generation
///
/// This enum represents the different kinds of errors that can occur when compiling
/// ELO validation expressions to Rust code.
///
/// # Examples
///
/// ```ignore
/// use elo_rust::codegen::CodeGenError;
///
/// let err = CodeGenError::UnsupportedFeature("custom functions".to_string());
/// println!("{}", err); // "Unsupported feature: custom functions"
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeGenError {
    /// Feature is not yet supported in this implementation
    ///
    /// Contains the name of the unsupported feature (e.g., "async validators", "custom functions")
    UnsupportedFeature(String),

    /// Type mismatch detected during code generation
    ///
    /// This occurs when operands don't match expected types for an operation.
    /// Contains a descriptive message about the mismatch.
    TypeMismatch(String),

    /// Invalid or malformed ELO expression
    ///
    /// This occurs when the expression cannot be parsed or compiled.
    /// Contains details about what makes the expression invalid.
    InvalidExpression(String),
}

impl fmt::Display for CodeGenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedFeature(feature) => {
                write!(f, "Unsupported feature: {}", feature)
            }
            Self::TypeMismatch(msg) => {
                write!(f, "Type mismatch: {}", msg)
            }
            Self::InvalidExpression(msg) => {
                write!(f, "Invalid expression: {}", msg)
            }
        }
    }
}

impl std::error::Error for CodeGenError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsupported_feature_creation() {
        let err = CodeGenError::UnsupportedFeature("async".to_string());
        assert_eq!(err.to_string(), "Unsupported feature: async");
    }

    #[test]
    fn test_type_mismatch_creation() {
        let err = CodeGenError::TypeMismatch("expected string".to_string());
        assert_eq!(err.to_string(), "Type mismatch: expected string");
    }

    #[test]
    fn test_invalid_expression_creation() {
        let err = CodeGenError::InvalidExpression("malformed".to_string());
        assert_eq!(err.to_string(), "Invalid expression: malformed");
    }

    #[test]
    fn test_error_equality() {
        let err1 = CodeGenError::UnsupportedFeature("test".to_string());
        let err2 = CodeGenError::UnsupportedFeature("test".to_string());
        assert_eq!(err1, err2);
    }

    #[test]
    fn test_error_debug() {
        let err = CodeGenError::TypeMismatch("test".to_string());
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("TypeMismatch"));
    }

    #[test]
    fn test_error_is_error_trait() {
        use std::error::Error;
        let err: Box<dyn Error> = Box::new(CodeGenError::InvalidExpression("test".to_string()));
        assert!(!err.to_string().is_empty());
    }
}
