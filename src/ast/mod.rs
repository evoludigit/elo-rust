//! AST (Abstract Syntax Tree) module for ELO expressions
//!
//! Defines the complete set of ELO expression types matching the language specification.
//! Each expression variant represents a different construct in the ELO language.

use std::fmt;

pub mod visitor;

pub use visitor::Visitor;

/// Top-level ELO expression type
///
/// Represents any valid ELO expression that can be parsed and executed.
/// This is an exhaustive enum of all expression forms in ELO.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Literal values: numbers (int/float) or booleans
    Literal(Literal),

    /// Null literal
    Null,

    /// Variable reference (identifier)
    Identifier(String),

    /// Field access: receiver.field (e.g., user.age)
    FieldAccess {
        /// The expression being accessed
        receiver: Box<Expr>,
        /// The field name
        field: String,
    },

    /// Binary operation: left op right
    BinaryOp {
        /// The binary operator
        op: BinaryOperator,
        /// Left operand
        left: Box<Expr>,
        /// Right operand
        right: Box<Expr>,
    },

    /// Unary operation: op operand
    UnaryOp {
        /// The unary operator
        op: UnaryOperator,
        /// The operand
        operand: Box<Expr>,
    },

    /// Function call: name(args)
    FunctionCall {
        /// Function name
        name: String,
        /// Function arguments
        args: Vec<Expr>,
    },

    /// Lambda expression: param ~> body
    Lambda {
        /// Parameter name
        param: String,
        /// Lambda body expression
        body: Box<Expr>,
    },

    /// Let binding: let name = value in body
    Let {
        /// Variable name being bound
        name: String,
        /// Value expression
        value: Box<Expr>,
        /// Body expression (scope where binding is available)
        body: Box<Expr>,
    },

    /// If conditional: if condition then branch_a else branch_b
    If {
        /// Condition expression
        condition: Box<Expr>,
        /// Then branch
        then_branch: Box<Expr>,
        /// Else branch
        else_branch: Box<Expr>,
    },

    /// Array literal: [expr1, expr2, ...]
    Array(Vec<Expr>),

    /// Object literal: {key1: value1, key2: value2, ...}
    Object(Vec<(String, Expr)>),

    /// Pipe operator: expr |> func() |> ...
    Pipe {
        /// The value being piped
        value: Box<Expr>,
        /// Functions to pipe through (in order)
        functions: Vec<Expr>,
    },

    /// Alternative operator: expr ?| default
    Alternative {
        /// Primary expression
        primary: Box<Expr>,
        /// Alternative/default expression
        alternative: Box<Expr>,
    },

    /// Guard expression: guard condition in expr
    Guard {
        /// Condition that must be true
        condition: Box<Expr>,
        /// Expression to evaluate if guard passes
        body: Box<Expr>,
    },

    /// Date literal: @date(2024-01-15)
    Date(String), // ISO8601 date: YYYY-MM-DD

    /// DateTime literal: @datetime(2024-01-15T10:30:00Z)
    DateTime(String), // ISO8601 datetime

    /// Duration literal: @duration(P1D), @duration(PT1H30M)
    Duration(String), // ISO8601 duration

    /// Temporal keyword: NOW, TODAY, TOMORROW, etc.
    TemporalKeyword(TemporalKeyword),

    /// String literal (explicitly quoted with single quotes)
    String(String),
}

/// Literal value types
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    /// Integer literal
    Integer(i64),

    /// Float literal
    Float(f64),

    /// Boolean literal
    Boolean(bool),
}

/// Binary operators supported in ELO
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    // Arithmetic operators
    /// Addition: +
    Add,
    /// Subtraction: -
    Sub,
    /// Multiplication: *
    Mul,
    /// Division: /
    Div,
    /// Modulo: %
    Mod,
    /// Exponentiation: ^
    Pow,

    // Comparison operators
    /// Equality: ==
    Eq,
    /// Inequality: !=
    Neq,
    /// Less than: <
    Lt,
    /// Less than or equal: <=
    Lte,
    /// Greater than: >
    Gt,
    /// Greater than or equal: >=
    Gte,

    // Logical operators
    /// Logical AND: &&
    And,
    /// Logical OR: ||
    Or,
}

/// Unary operators supported in ELO
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    /// Logical NOT: !
    Not,
    /// Negation: -
    Neg,
    /// Unary plus: +
    Plus,
}

/// Temporal keywords for date/time operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemporalKeyword {
    /// Current date and time
    Now,
    /// Current date (start of day)
    Today,
    /// Next calendar day
    Tomorrow,
    /// Previous calendar day
    Yesterday,

    // Period boundaries (boundaries are inclusive start of period)
    /// Start of current day
    StartOfDay,
    /// End of current day
    EndOfDay,
    /// Start of current week
    StartOfWeek,
    /// End of current week
    EndOfWeek,
    /// Start of current month
    StartOfMonth,
    /// End of current month
    EndOfMonth,
    /// Start of current quarter
    StartOfQuarter,
    /// End of current quarter
    EndOfQuarter,
    /// Start of current year
    StartOfYear,
    /// End of current year
    EndOfYear,

    // Extremes
    /// Beginning of time (used for range checking)
    BeginningOfTime,
    /// End of time (used for range checking)
    EndOfTime,
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Mod => write!(f, "%"),
            Self::Pow => write!(f, "^"),
            Self::Eq => write!(f, "=="),
            Self::Neq => write!(f, "!="),
            Self::Lt => write!(f, "<"),
            Self::Lte => write!(f, "<="),
            Self::Gt => write!(f, ">"),
            Self::Gte => write!(f, ">="),
            Self::And => write!(f, "&&"),
            Self::Or => write!(f, "||"),
        }
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Not => write!(f, "!"),
            Self::Neg => write!(f, "-"),
            Self::Plus => write!(f, "+"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_integer() {
        let expr = Expr::Literal(Literal::Integer(42));
        assert_eq!(expr, Expr::Literal(Literal::Integer(42)));
    }

    #[test]
    fn test_literal_float() {
        let expr = Expr::Literal(Literal::Float(3.14));
        assert_eq!(expr, Expr::Literal(Literal::Float(3.14)));
    }

    #[test]
    fn test_literal_boolean() {
        let expr = Expr::Literal(Literal::Boolean(true));
        assert_eq!(expr, Expr::Literal(Literal::Boolean(true)));
    }

    #[test]
    fn test_identifier() {
        let expr = Expr::Identifier("age".to_string());
        assert_eq!(expr, Expr::Identifier("age".to_string()));
    }

    #[test]
    fn test_string_literal() {
        let expr = Expr::String("hello".to_string());
        assert_eq!(expr, Expr::String("hello".to_string()));
    }

    #[test]
    fn test_field_access() {
        let expr = Expr::FieldAccess {
            receiver: Box::new(Expr::Identifier("user".to_string())),
            field: "age".to_string(),
        };
        matches!(expr, Expr::FieldAccess { .. });
    }

    #[test]
    fn test_binary_op() {
        let expr = Expr::BinaryOp {
            op: BinaryOperator::Add,
            left: Box::new(Expr::Literal(Literal::Integer(1))),
            right: Box::new(Expr::Literal(Literal::Integer(2))),
        };
        matches!(expr, Expr::BinaryOp { .. });
    }

    #[test]
    fn test_unary_op() {
        let expr = Expr::UnaryOp {
            op: UnaryOperator::Not,
            operand: Box::new(Expr::Literal(Literal::Boolean(true))),
        };
        matches!(expr, Expr::UnaryOp { .. });
    }

    #[test]
    fn test_function_call() {
        let expr = Expr::FunctionCall {
            name: "length".to_string(),
            args: vec![Expr::Identifier("name".to_string())],
        };
        matches!(expr, Expr::FunctionCall { .. });
    }

    #[test]
    fn test_lambda() {
        let expr = Expr::Lambda {
            param: "x".to_string(),
            body: Box::new(Expr::BinaryOp {
                op: BinaryOperator::Mul,
                left: Box::new(Expr::Identifier("x".to_string())),
                right: Box::new(Expr::Literal(Literal::Integer(2))),
            }),
        };
        matches!(expr, Expr::Lambda { .. });
    }

    #[test]
    fn test_let_expr() {
        let expr = Expr::Let {
            name: "x".to_string(),
            value: Box::new(Expr::Literal(Literal::Integer(1))),
            body: Box::new(Expr::Identifier("x".to_string())),
        };
        matches!(expr, Expr::Let { .. });
    }

    #[test]
    fn test_if_expr() {
        let expr = Expr::If {
            condition: Box::new(Expr::Literal(Literal::Boolean(true))),
            then_branch: Box::new(Expr::Literal(Literal::Integer(1))),
            else_branch: Box::new(Expr::Literal(Literal::Integer(2))),
        };
        matches!(expr, Expr::If { .. });
    }

    #[test]
    fn test_array() {
        let expr = Expr::Array(vec![
            Expr::Literal(Literal::Integer(1)),
            Expr::Literal(Literal::Integer(2)),
        ]);
        matches!(expr, Expr::Array(_));
    }

    #[test]
    fn test_object() {
        let expr = Expr::Object(vec![(
            "key".to_string(),
            Expr::Literal(Literal::Integer(1)),
        )]);
        matches!(expr, Expr::Object(_));
    }

    #[test]
    fn test_null() {
        let expr = Expr::Null;
        assert_eq!(expr, Expr::Null);
    }

    #[test]
    fn test_date_literal() {
        let expr = Expr::Date("2024-01-15".to_string());
        assert_eq!(expr, Expr::Date("2024-01-15".to_string()));
    }

    #[test]
    fn test_datetime_literal() {
        let expr = Expr::DateTime("2024-01-15T10:30:00Z".to_string());
        assert_eq!(expr, Expr::DateTime("2024-01-15T10:30:00Z".to_string()));
    }

    #[test]
    fn test_duration_literal() {
        let expr = Expr::Duration("P1D".to_string());
        assert_eq!(expr, Expr::Duration("P1D".to_string()));
    }

    #[test]
    fn test_temporal_keyword() {
        let expr = Expr::TemporalKeyword(TemporalKeyword::Now);
        assert_eq!(expr, Expr::TemporalKeyword(TemporalKeyword::Now));
    }

    #[test]
    fn test_binary_operator_display() {
        assert_eq!(BinaryOperator::Add.to_string(), "+");
        assert_eq!(BinaryOperator::Eq.to_string(), "==");
        assert_eq!(BinaryOperator::And.to_string(), "&&");
    }

    #[test]
    fn test_unary_operator_display() {
        assert_eq!(UnaryOperator::Not.to_string(), "!");
        assert_eq!(UnaryOperator::Neg.to_string(), "-");
    }

    #[test]
    fn test_pipe() {
        let expr = Expr::Pipe {
            value: Box::new(Expr::Identifier("x".to_string())),
            functions: vec![Expr::FunctionCall {
                name: "double".to_string(),
                args: vec![],
            }],
        };
        matches!(expr, Expr::Pipe { .. });
    }

    #[test]
    fn test_guard() {
        let expr = Expr::Guard {
            condition: Box::new(Expr::BinaryOp {
                op: BinaryOperator::Gt,
                left: Box::new(Expr::Identifier("x".to_string())),
                right: Box::new(Expr::Literal(Literal::Integer(0))),
            }),
            body: Box::new(Expr::Identifier("x".to_string())),
        };
        matches!(expr, Expr::Guard { .. });
    }

    #[test]
    fn test_alternative() {
        let expr = Expr::Alternative {
            primary: Box::new(Expr::Identifier("maybe".to_string())),
            alternative: Box::new(Expr::Literal(Literal::Integer(0))),
        };
        matches!(expr, Expr::Alternative { .. });
    }
}
