//! Type system mapping between ELO and Rust types

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
