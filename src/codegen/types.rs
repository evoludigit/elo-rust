//! Type system mapping between ELO and Rust types

use std::collections::HashMap;

/// Represents the Rust type equivalent of an ELO type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RustType {
    /// String type (&str or String)
    String,
    /// Integer type (i64 or i32)
    Integer,
    /// Float type (f64 or f32)
    Float,
    /// Boolean type
    Bool,
    /// Date type (chrono::NaiveDate)
    Date,
    /// Time type (chrono::NaiveTime)
    Time,
    /// Duration type (chrono::Duration)
    Duration,
    /// Option type
    Option(Box<RustType>),
    /// Array/slice type
    Array(Box<RustType>),
    /// Custom user-defined type
    Custom(String),
    /// Unknown/unresolved type
    Unknown,
}

impl RustType {
    /// Get the Rust type string representation
    pub fn to_rust_string(&self) -> String {
        match self {
            Self::String => "&str".to_string(),
            Self::Integer => "i64".to_string(),
            Self::Float => "f64".to_string(),
            Self::Bool => "bool".to_string(),
            Self::Date => "chrono::NaiveDate".to_string(),
            Self::Time => "chrono::NaiveTime".to_string(),
            Self::Duration => "chrono::Duration".to_string(),
            Self::Option(inner) => format!("Option<{}>", inner.to_rust_string()),
            Self::Array(inner) => format!("&[{}]", inner.to_rust_string()),
            Self::Custom(name) => name.clone(),
            Self::Unknown => "()".to_string(),
        }
    }

    /// Check if this type is compatible with another type
    pub fn is_compatible_with(&self, other: &RustType) -> bool {
        match (self, other) {
            // Same types are compatible
            (Self::String, Self::String)
            | (Self::Integer, Self::Integer)
            | (Self::Float, Self::Float)
            | (Self::Bool, Self::Bool)
            | (Self::Date, Self::Date)
            | (Self::Time, Self::Time)
            | (Self::Duration, Self::Duration)
            | (Self::Unknown, _)
            | (_, Self::Unknown) => true,

            // Option types are compatible if inner types are
            (Self::Option(a), Self::Option(b)) => a.is_compatible_with(b),

            // Array types are compatible if element types are
            (Self::Array(a), Self::Array(b)) => a.is_compatible_with(b),

            // Custom types are compatible if names match
            (Self::Custom(a), Self::Custom(b)) => a == b,

            // Different types are not compatible
            _ => false,
        }
    }
}

/// Information about a custom type (struct/enum)
#[derive(Debug, Clone)]
pub struct TypeInfo {
    /// The name of the type
    pub name: String,
    /// Field names and their types
    fields: HashMap<String, RustType>,
}

impl TypeInfo {
    /// Create a new type info with the given name
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            fields: HashMap::new(),
        }
    }

    /// Add a field to the type
    pub fn add_field(&mut self, name: &str, field_type: RustType) {
        self.fields.insert(name.to_string(), field_type);
    }

    /// Get the type of a field
    pub fn get_field(&self, name: &str) -> Option<&RustType> {
        self.fields.get(name)
    }

    /// Get all fields
    pub fn fields(&self) -> &HashMap<String, RustType> {
        &self.fields
    }
}

/// Context for type resolution and type checking
///
/// Maintains a registry of user-defined types and provides field lookup,
/// type compatibility checking, and type inference from literal values.
///
/// # Example
///
/// ```ignore
/// use elo_rust::codegen::types::{TypeContext, TypeInfo, RustType};
///
/// let mut context = TypeContext::new();
///
/// let mut user_type = TypeInfo::new("User");
/// user_type.add_field("email", RustType::String);
/// user_type.add_field("age", RustType::Integer);
///
/// context.register_type("User", user_type);
///
/// // Look up field types
/// assert_eq!(
///     context.get_field_type("User", "email"),
///     Some(&RustType::String)
/// );
/// ```
#[derive(Debug, Clone, Default)]
pub struct TypeContext {
    /// Registered user-defined types (name -> type info)
    types: HashMap<String, TypeInfo>,
}

impl TypeContext {
    /// Create a new empty type context
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
        }
    }

    /// Register a new custom type in the context
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the type (e.g., "User", "Product")
    /// * `type_info` - The type information including field definitions
    pub fn register_type(&mut self, name: &str, type_info: TypeInfo) {
        self.types.insert(name.to_string(), type_info);
    }

    /// Look up the type of a field in a registered type
    ///
    /// # Arguments
    ///
    /// * `type_name` - The name of the type (e.g., "User")
    /// * `field_name` - The name of the field (e.g., "email")
    ///
    /// # Returns
    ///
    /// `Some(&RustType)` if the type and field exist, `None` otherwise
    pub fn get_field_type(&self, type_name: &str, field_name: &str) -> Option<&RustType> {
        self.types
            .get(type_name)
            .and_then(|t| t.get_field(field_name))
    }

    /// Check if this context has any registered types
    pub fn is_empty(&self) -> bool {
        self.types.is_empty()
    }

    /// Get all registered type names
    pub fn list_all_type_names(&self) -> Vec<String> {
        self.types.keys().cloned().collect()
    }

    /// Infer the type from a literal value
    ///
    /// Attempts to determine the Rust type of a literal string by:
    /// 1. Trying to parse as integer
    /// 2. Trying to parse as float
    /// 3. Checking for boolean keywords (true/false)
    /// 4. Checking for quoted strings
    ///
    /// # Arguments
    ///
    /// * `literal` - The literal value as a string
    ///
    /// # Returns
    ///
    /// The inferred `RustType`, or `RustType::Unknown` if unable to infer
    pub fn infer_from_literal(&self, literal: &str) -> RustType {
        // Try to parse as integer
        if literal.parse::<i64>().is_ok() {
            return RustType::Integer;
        }

        // Try to parse as float
        if literal.parse::<f64>().is_ok() {
            return RustType::Float;
        }

        // Check for boolean literals
        if literal == "true" || literal == "false" {
            return RustType::Bool;
        }

        // Check for string literals (quoted)
        if (literal.starts_with('"') && literal.ends_with('"'))
            || (literal.starts_with('\'') && literal.ends_with('\''))
        {
            return RustType::String;
        }

        RustType::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_type_strings() {
        assert_eq!(RustType::String.to_rust_string(), "&str");
        assert_eq!(RustType::Integer.to_rust_string(), "i64");
        assert_eq!(RustType::Bool.to_rust_string(), "bool");
    }

    #[test]
    fn test_option_type_string() {
        let opt_string = RustType::Option(Box::new(RustType::String));
        assert_eq!(opt_string.to_rust_string(), "Option<&str>");
    }

    #[test]
    fn test_array_type_string() {
        let array_int = RustType::Array(Box::new(RustType::Integer));
        assert_eq!(array_int.to_rust_string(), "&[i64]");
    }
}
