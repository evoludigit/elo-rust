//! Binary and unary operator code generation

use proc_macro2::TokenStream;
use quote::quote;

/// Represents a binary operator
///
/// Supports all common comparison, arithmetic, and logical operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    /// Equality (==)
    Equal,
    /// Inequality (!=)
    NotEqual,
    /// Less than (<)
    Less,
    /// Less than or equal (<=)
    LessEqual,
    /// Greater than (>)
    Greater,
    /// Greater than or equal (>=)
    GreaterEqual,
    /// Logical AND (&&)
    And,
    /// Logical OR (||)
    Or,
    /// Addition (+)
    Add,
    /// Subtraction (-)
    Subtract,
    /// Multiplication (*)
    Multiply,
    /// Division (/)
    Divide,
    /// Modulo (%)
    Modulo,
}

/// Represents a unary operator
///
/// Supports logical negation and numeric negation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    /// Logical NOT (!)
    Not,
    /// Negation (-)
    Negate,
}

/// Generates code for operators
///
/// Provides methods for generating Rust code for binary and unary operations.
#[derive(Debug, Clone)]
pub struct OperatorGenerator;

impl OperatorGenerator {
    /// Create a new operator generator
    pub fn new() -> Self {
        Self
    }

    /// Generate code for a binary operation
    ///
    /// # Arguments
    ///
    /// * `op` - The operator to apply
    /// * `left` - The left operand as a TokenStream
    /// * `right` - The right operand as a TokenStream
    ///
    /// # Returns
    ///
    /// A `TokenStream` representing the binary operation
    pub fn binary(&self, op: BinaryOp, left: TokenStream, right: TokenStream) -> TokenStream {
        match op {
            BinaryOp::Equal => quote! { #left == #right },
            BinaryOp::NotEqual => quote! { #left != #right },
            BinaryOp::Less => quote! { #left < #right },
            BinaryOp::LessEqual => quote! { #left <= #right },
            BinaryOp::Greater => quote! { #left > #right },
            BinaryOp::GreaterEqual => quote! { #left >= #right },
            BinaryOp::Add => quote! { #left + #right },
            BinaryOp::Subtract => quote! { #left - #right },
            BinaryOp::Multiply => quote! { #left * #right },
            BinaryOp::Divide => quote! { #left / #right },
            BinaryOp::Modulo => quote! { #left % #right },
            BinaryOp::And => quote! { #left && #right },
            BinaryOp::Or => quote! { #left || #right },
        }
    }

    /// Generate code for a unary operation
    ///
    /// # Arguments
    ///
    /// * `op` - The unary operator to apply
    /// * `operand` - The operand as a TokenStream
    ///
    /// # Returns
    ///
    /// A `TokenStream` representing the unary operation
    pub fn unary(&self, op: UnaryOp, operand: TokenStream) -> TokenStream {
        match op {
            UnaryOp::Not => quote! { !#operand },
            UnaryOp::Negate => quote! { -#operand },
        }
    }
}

impl Default for OperatorGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_op_equality() {
        assert_eq!(BinaryOp::Equal, BinaryOp::Equal);
        assert_ne!(BinaryOp::Equal, BinaryOp::NotEqual);
    }

    #[test]
    fn test_binary_op_comparison() {
        assert_ne!(BinaryOp::Less, BinaryOp::Greater);
        assert_eq!(BinaryOp::LessEqual, BinaryOp::LessEqual);
    }

    #[test]
    fn test_binary_op_logical() {
        assert_eq!(BinaryOp::And, BinaryOp::And);
        assert_ne!(BinaryOp::And, BinaryOp::Or);
    }

    #[test]
    fn test_unary_op_not() {
        assert_eq!(UnaryOp::Not, UnaryOp::Not);
        assert_ne!(UnaryOp::Not, UnaryOp::Negate);
    }

    #[test]
    fn test_operator_generator_creation() {
        let _gen = OperatorGenerator::new();
    }
}
