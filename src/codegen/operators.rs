//! Binary and unary operator code generation

use proc_macro2::TokenStream;

/// Represents a binary operator
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    /// Logical NOT (!)
    Not,
    /// Negation (-)
    Negate,
}

/// Generates code for operators
#[derive(Debug)]
pub struct OperatorGenerator;

impl OperatorGenerator {
    /// Create a new operator generator
    pub fn new() -> Self {
        Self
    }

    /// Generate code for a binary operation
    pub fn binary(&self, _op: BinaryOp, _left: TokenStream, _right: TokenStream) -> TokenStream {
        // Phase 2 implementation
        quote::quote!()
    }

    /// Generate code for a unary operation
    pub fn unary(&self, _op: UnaryOp, _operand: TokenStream) -> TokenStream {
        // Phase 2 implementation
        quote::quote!()
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
