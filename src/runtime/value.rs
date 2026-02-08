//! ELO runtime value representation
//!
//! Provides the EloValue enum for dynamic typing and runtime value handling.
//! This allows the compiler to track and validate types at both compile-time
//! and runtime.

use std::collections::BTreeMap;
use std::fmt;

/// Represents a runtime value in ELO
///
/// EloValue supports dynamic typing with support for all ELO data types:
/// scalars (int, float, string, bool, null) and collections (arrays, objects).
#[derive(Debug, Clone, PartialEq)]
pub enum EloValue {
    /// Integer value (64-bit signed)
    Integer(i64),

    /// Float value (64-bit IEEE 754)
    Float(f64),

    /// String value
    String(String),

    /// Boolean value
    Boolean(bool),

    /// Null/None value
    Null,

    /// Array of values (homogeneous or heterogeneous)
    Array(Vec<EloValue>),

    /// Object as key-value pairs (sorted by key for consistency)
    Object(BTreeMap<String, EloValue>),
}

impl EloValue {
    /// Get the type name as a string
    pub fn type_name(&self) -> &'static str {
        match self {
            EloValue::Integer(_) => "integer",
            EloValue::Float(_) => "float",
            EloValue::String(_) => "string",
            EloValue::Boolean(_) => "boolean",
            EloValue::Null => "null",
            EloValue::Array(_) => "array",
            EloValue::Object(_) => "object",
        }
    }

    /// Check if this value is truthy
    pub fn is_truthy(&self) -> bool {
        match self {
            EloValue::Null => false,
            EloValue::Boolean(b) => *b,
            EloValue::Integer(n) => *n != 0,
            EloValue::Float(f) => *f != 0.0,
            EloValue::String(s) => !s.is_empty(),
            EloValue::Array(a) => !a.is_empty(),
            EloValue::Object(o) => !o.is_empty(),
        }
    }

    /// Convert to integer if possible
    pub fn to_integer(&self) -> Option<i64> {
        match self {
            EloValue::Integer(n) => Some(*n),
            EloValue::Float(f) => Some(*f as i64),
            EloValue::Boolean(b) => Some(if *b { 1 } else { 0 }),
            EloValue::String(s) => s.parse().ok(),
            _ => None,
        }
    }

    /// Convert to float if possible
    pub fn to_float(&self) -> Option<f64> {
        match self {
            EloValue::Integer(n) => Some(*n as f64),
            EloValue::Float(f) => Some(*f),
            EloValue::String(s) => s.parse().ok(),
            _ => None,
        }
    }

    /// Convert to string
    pub fn to_string_value(&self) -> String {
        match self {
            EloValue::Integer(n) => n.to_string(),
            EloValue::Float(f) => {
                // Format floats nicely, avoiding trailing zeros
                if f.fract() == 0.0 {
                    format!("{:.1}", f)
                } else {
                    f.to_string()
                }
            }
            EloValue::String(s) => s.clone(),
            EloValue::Boolean(b) => b.to_string(),
            EloValue::Null => "null".to_string(),
            EloValue::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|v| v.to_string_value()).collect();
                format!("[{}]", elements.join(", "))
            }
            EloValue::Object(obj) => {
                let pairs: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string_value()))
                    .collect();
                format!("{{{}}}", pairs.join(", "))
            }
        }
    }

    /// Convert to boolean
    pub fn to_boolean(&self) -> bool {
        self.is_truthy()
    }

    /// Check if this value is numeric (integer or float)
    pub fn is_numeric(&self) -> bool {
        matches!(self, EloValue::Integer(_) | EloValue::Float(_))
    }

    /// Check if this value is a string
    pub fn is_string(&self) -> bool {
        matches!(self, EloValue::String(_))
    }

    /// Check if this value is an array
    pub fn is_array(&self) -> bool {
        matches!(self, EloValue::Array(_))
    }

    /// Check if this value is an object
    pub fn is_object(&self) -> bool {
        matches!(self, EloValue::Object(_))
    }

    /// Get array length if this is an array
    pub fn array_len(&self) -> Option<usize> {
        match self {
            EloValue::Array(arr) => Some(arr.len()),
            EloValue::String(s) => Some(s.len()),
            _ => None,
        }
    }

    /// Get value at array index
    pub fn array_get(&self, index: usize) -> Option<EloValue> {
        match self {
            EloValue::Array(arr) => arr.get(index).cloned(),
            _ => None,
        }
    }

    /// Get object field value
    pub fn object_get(&self, key: &str) -> Option<EloValue> {
        match self {
            EloValue::Object(obj) => obj.get(key).cloned(),
            _ => None,
        }
    }

    /// Add two values (numeric addition or string concatenation)
    pub fn add(&self, other: &EloValue) -> Result<EloValue, String> {
        match (self, other) {
            (EloValue::Integer(a), EloValue::Integer(b)) => Ok(EloValue::Integer(a + b)),
            (EloValue::Float(a), EloValue::Float(b)) => Ok(EloValue::Float(a + b)),
            (EloValue::Integer(a), EloValue::Float(b)) => Ok(EloValue::Float(*a as f64 + b)),
            (EloValue::Float(a), EloValue::Integer(b)) => Ok(EloValue::Float(a + *b as f64)),
            (EloValue::String(a), EloValue::String(b)) => {
                Ok(EloValue::String(format!("{}{}", a, b)))
            }
            _ => Err(format!(
                "Cannot add {} and {}",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    /// Subtract two values
    pub fn subtract(&self, other: &EloValue) -> Result<EloValue, String> {
        match (self, other) {
            (EloValue::Integer(a), EloValue::Integer(b)) => Ok(EloValue::Integer(a - b)),
            (EloValue::Float(a), EloValue::Float(b)) => Ok(EloValue::Float(a - b)),
            (EloValue::Integer(a), EloValue::Float(b)) => Ok(EloValue::Float(*a as f64 - b)),
            (EloValue::Float(a), EloValue::Integer(b)) => Ok(EloValue::Float(a - *b as f64)),
            _ => Err(format!(
                "Cannot subtract {} from {}",
                other.type_name(),
                self.type_name()
            )),
        }
    }

    /// Multiply two values
    pub fn multiply(&self, other: &EloValue) -> Result<EloValue, String> {
        match (self, other) {
            (EloValue::Integer(a), EloValue::Integer(b)) => Ok(EloValue::Integer(a * b)),
            (EloValue::Float(a), EloValue::Float(b)) => Ok(EloValue::Float(a * b)),
            (EloValue::Integer(a), EloValue::Float(b)) => Ok(EloValue::Float(*a as f64 * b)),
            (EloValue::Float(a), EloValue::Integer(b)) => Ok(EloValue::Float(a * *b as f64)),
            // String repetition
            (EloValue::String(s), EloValue::Integer(n)) => {
                if *n < 0 {
                    Err("Cannot repeat string negative times".to_string())
                } else {
                    Ok(EloValue::String(s.repeat(*n as usize)))
                }
            }
            _ => Err(format!(
                "Cannot multiply {} and {}",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    /// Divide two values
    pub fn divide(&self, other: &EloValue) -> Result<EloValue, String> {
        match (self, other) {
            (EloValue::Integer(a), EloValue::Integer(b)) => {
                if *b == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(EloValue::Integer(a / b))
                }
            }
            (EloValue::Float(a), EloValue::Float(b)) => {
                if *b == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(EloValue::Float(a / b))
                }
            }
            (EloValue::Integer(a), EloValue::Float(b)) => {
                if *b == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(EloValue::Float(*a as f64 / b))
                }
            }
            (EloValue::Float(a), EloValue::Integer(b)) => {
                if *b == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(EloValue::Float(a / *b as f64))
                }
            }
            _ => Err(format!(
                "Cannot divide {} by {}",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    /// Modulo operation
    pub fn modulo(&self, other: &EloValue) -> Result<EloValue, String> {
        match (self, other) {
            (EloValue::Integer(a), EloValue::Integer(b)) => {
                if *b == 0 {
                    Err("Modulo by zero".to_string())
                } else {
                    Ok(EloValue::Integer(a % b))
                }
            }
            _ => Err(format!(
                "Modulo requires integers, got {} and {}",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    /// Power operation
    pub fn power(&self, other: &EloValue) -> Result<EloValue, String> {
        match (self, other) {
            (EloValue::Integer(a), EloValue::Integer(b)) => {
                if *b < 0 {
                    // Negative power returns float
                    Ok(EloValue::Float((*a as f64).powf(*b as f64)))
                } else {
                    Ok(EloValue::Integer(a.pow(*b as u32)))
                }
            }
            (EloValue::Float(a), EloValue::Float(b)) => Ok(EloValue::Float(a.powf(*b))),
            (EloValue::Integer(a), EloValue::Float(b)) => Ok(EloValue::Float((*a as f64).powf(*b))),
            (EloValue::Float(a), EloValue::Integer(b)) => Ok(EloValue::Float(a.powf(*b as f64))),
            _ => Err(format!(
                "Cannot raise {} to power of {}",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    /// Equality comparison
    pub fn equals(&self, other: &EloValue) -> bool {
        match (self, other) {
            (EloValue::Integer(a), EloValue::Integer(b)) => a == b,
            (EloValue::Float(a), EloValue::Float(b)) => a == b,
            (EloValue::Integer(a), EloValue::Float(b)) => (*a as f64) == *b,
            (EloValue::Float(a), EloValue::Integer(b)) => *a == (*b as f64),
            (EloValue::String(a), EloValue::String(b)) => a == b,
            (EloValue::Boolean(a), EloValue::Boolean(b)) => a == b,
            (EloValue::Null, EloValue::Null) => true,
            _ => false,
        }
    }

    /// Less than comparison
    pub fn less_than(&self, other: &EloValue) -> Result<bool, String> {
        match (self, other) {
            (EloValue::Integer(a), EloValue::Integer(b)) => Ok(a < b),
            (EloValue::Float(a), EloValue::Float(b)) => Ok(a < b),
            (EloValue::Integer(a), EloValue::Float(b)) => Ok((*a as f64) < *b),
            (EloValue::Float(a), EloValue::Integer(b)) => Ok(*a < (*b as f64)),
            (EloValue::String(a), EloValue::String(b)) => Ok(a < b),
            _ => Err(format!(
                "Cannot compare {} and {}",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    /// Logical AND
    pub fn logical_and(&self, other: &EloValue) -> EloValue {
        if self.is_truthy() {
            other.clone()
        } else {
            self.clone()
        }
    }

    /// Logical OR
    pub fn logical_or(&self, other: &EloValue) -> EloValue {
        if self.is_truthy() {
            self.clone()
        } else {
            other.clone()
        }
    }

    /// Logical NOT
    pub fn logical_not(&self) -> EloValue {
        EloValue::Boolean(!self.is_truthy())
    }
}

impl fmt::Display for EloValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_operations() {
        let a = EloValue::Integer(10);
        let b = EloValue::Integer(3);

        assert_eq!(a.add(&b).unwrap(), EloValue::Integer(13));
        assert_eq!(a.subtract(&b).unwrap(), EloValue::Integer(7));
        assert_eq!(a.multiply(&b).unwrap(), EloValue::Integer(30));
        assert_eq!(a.divide(&b).unwrap(), EloValue::Integer(3));
        assert_eq!(a.modulo(&b).unwrap(), EloValue::Integer(1));
    }

    #[test]
    fn test_float_operations() {
        let a = EloValue::Float(10.5);
        let b = EloValue::Float(2.5);

        assert_eq!(a.add(&b).unwrap(), EloValue::Float(13.0));
        assert_eq!(a.multiply(&b).unwrap(), EloValue::Float(26.25));
    }

    #[test]
    fn test_mixed_numeric_operations() {
        let i = EloValue::Integer(10);
        let f = EloValue::Float(2.5);

        assert_eq!(i.add(&f).unwrap(), EloValue::Float(12.5));
        assert_eq!(i.multiply(&f).unwrap(), EloValue::Float(25.0));
    }

    #[test]
    fn test_string_operations() {
        let a = EloValue::String("hello".to_string());
        let b = EloValue::String(" world".to_string());
        let n = EloValue::Integer(3);

        assert_eq!(
            a.add(&b).unwrap(),
            EloValue::String("hello world".to_string())
        );
        assert_eq!(
            a.multiply(&n).unwrap(),
            EloValue::String("hellohellohello".to_string())
        );
    }

    #[test]
    fn test_comparison() {
        let a = EloValue::Integer(5);
        let b = EloValue::Integer(10);

        assert!(a.less_than(&b).unwrap());
        assert!(!b.less_than(&a).unwrap());
        assert!(a.equals(&EloValue::Integer(5)));
    }

    #[test]
    fn test_boolean_logic() {
        let t = EloValue::Boolean(true);
        let f = EloValue::Boolean(false);

        assert_eq!(t.logical_and(&f), EloValue::Boolean(false));
        assert_eq!(t.logical_or(&f), EloValue::Boolean(true));
        assert_eq!(t.logical_not(), EloValue::Boolean(false));
    }

    #[test]
    fn test_truthiness() {
        assert!(EloValue::Boolean(true).is_truthy());
        assert!(!EloValue::Boolean(false).is_truthy());
        assert!(EloValue::Integer(1).is_truthy());
        assert!(!EloValue::Integer(0).is_truthy());
        assert!(EloValue::String("x".to_string()).is_truthy());
        assert!(!EloValue::String("".to_string()).is_truthy());
        assert!(!EloValue::Null.is_truthy());
    }

    #[test]
    fn test_type_conversion() {
        let i = EloValue::Integer(42);
        let f = EloValue::Float(3.15);
        let s = EloValue::String("123".to_string());

        assert_eq!(i.to_integer(), Some(42));
        assert_eq!(f.to_float(), Some(3.15));
        assert_eq!(s.to_integer(), Some(123));
    }

    #[test]
    fn test_type_names() {
        assert_eq!(EloValue::Integer(1).type_name(), "integer");
        assert_eq!(EloValue::Float(1.0).type_name(), "float");
        assert_eq!(EloValue::String("x".to_string()).type_name(), "string");
        assert_eq!(EloValue::Boolean(true).type_name(), "boolean");
        assert_eq!(EloValue::Null.type_name(), "null");
        assert_eq!(EloValue::Array(vec![]).type_name(), "array");
        assert_eq!(EloValue::Object(BTreeMap::new()).type_name(), "object");
    }

    #[test]
    fn test_array_operations() {
        let arr = EloValue::Array(vec![
            EloValue::Integer(1),
            EloValue::Integer(2),
            EloValue::Integer(3),
        ]);

        assert_eq!(arr.array_len(), Some(3));
        assert_eq!(arr.array_get(0), Some(EloValue::Integer(1)));
        assert_eq!(arr.array_get(10), None);
    }

    #[test]
    fn test_object_operations() {
        let mut obj_map = BTreeMap::new();
        obj_map.insert("x".to_string(), EloValue::Integer(1));
        obj_map.insert("y".to_string(), EloValue::Integer(2));
        let obj = EloValue::Object(obj_map);

        assert_eq!(obj.object_get("x"), Some(EloValue::Integer(1)));
        assert_eq!(obj.object_get("z"), None);
    }

    #[test]
    fn test_division_by_zero() {
        let a = EloValue::Integer(10);
        let zero = EloValue::Integer(0);

        assert!(a.divide(&zero).is_err());
    }

    #[test]
    fn test_power_operation() {
        let base = EloValue::Integer(2);
        let exp = EloValue::Integer(3);

        assert_eq!(base.power(&exp).unwrap(), EloValue::Integer(8));
    }

    #[test]
    fn test_type_checks() {
        let i = EloValue::Integer(1);
        let s = EloValue::String("x".to_string());
        let arr = EloValue::Array(vec![]);

        assert!(i.is_numeric());
        assert!(!s.is_numeric());
        assert!(s.is_string());
        assert!(arr.is_array());
    }
}
