//! Standard library function implementations
//!
//! Defines all supported ELO standard library functions that can be called
//! from generated validators

pub mod array;
pub mod datetime;
pub mod string;
pub mod types;

/// Standard library function metadata
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionSignature {
    /// Function name
    pub name: String,
    /// Parameter types (as strings)
    pub params: Vec<String>,
    /// Return type
    pub return_type: String,
    /// Function category
    pub category: FunctionCategory,
}

/// Categories of standard library functions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionCategory {
    /// String manipulation
    String,
    /// Date and time operations
    DateTime,
    /// Array/collection operations
    Array,
    /// Type checking and conversion
    Type,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_signature_creation() {
        let sig = FunctionSignature {
            name: "matches".to_string(),
            params: vec!["&str".to_string(), "&str".to_string()],
            return_type: "bool".to_string(),
            category: FunctionCategory::String,
        };
        assert_eq!(sig.name, "matches");
        assert_eq!(sig.params.len(), 2);
    }
}
