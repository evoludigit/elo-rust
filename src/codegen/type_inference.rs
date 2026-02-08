//! Type inference system for ELO expressions
//!
//! Infers types for expressions to enable better error checking and code generation.
//! Uses a simple bidirectional type inference approach.

use crate::ast::{BinaryOperator, Expr, Literal, TemporalKeyword, UnaryOperator, Visitor};
use std::fmt;

/// Inferred type of an ELO expression
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InferredType {
    /// Integer type
    Integer,

    /// Float type
    Float,

    /// String type
    String,

    /// Boolean type
    Boolean,

    /// Null/None type
    Null,

    /// Array with element type
    Array(Box<InferredType>),

    /// Object with field types (simplified - just track it's an object)
    Object,

    /// Date type
    Date,

    /// DateTime type
    DateTime,

    /// Duration type
    Duration,

    /// Unknown type (when inference fails or type can't be determined)
    Unknown,

    /// Any numeric type (integer or float)
    Numeric,

    /// Type error - incompatible types
    Error(String),
}

impl fmt::Display for InferredType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer => write!(f, "integer"),
            Self::Float => write!(f, "float"),
            Self::String => write!(f, "string"),
            Self::Boolean => write!(f, "boolean"),
            Self::Null => write!(f, "null"),
            Self::Array(elem_type) => write!(f, "[{}]", elem_type),
            Self::Object => write!(f, "object"),
            Self::Date => write!(f, "date"),
            Self::DateTime => write!(f, "datetime"),
            Self::Duration => write!(f, "duration"),
            Self::Unknown => write!(f, "unknown"),
            Self::Numeric => write!(f, "number"),
            Self::Error(msg) => write!(f, "error({})", msg),
        }
    }
}

impl InferredType {
    /// Check if this is a numeric type
    pub fn is_numeric(&self) -> bool {
        matches!(self, Self::Integer | Self::Float | Self::Numeric)
    }

    /// Check if this is a scalar type
    pub fn is_scalar(&self) -> bool {
        matches!(
            self,
            Self::Integer | Self::Float | Self::String | Self::Boolean
        )
    }

    /// Check if this is an error type
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error(_))
    }

    /// Get a common type that's compatible with both types
    pub fn common_type(a: &InferredType, b: &InferredType) -> InferredType {
        match (a, b) {
            // Same types unify to themselves
            (t1, t2) if t1 == t2 => t1.clone(),

            // Numeric type unification
            (InferredType::Integer, InferredType::Float)
            | (InferredType::Float, InferredType::Integer) => InferredType::Float,
            (InferredType::Numeric, other) | (other, InferredType::Numeric) => {
                if other.is_numeric() {
                    other.clone()
                } else {
                    InferredType::Error(format!(
                        "Type mismatch: cannot unify numeric and {}",
                        other
                    ))
                }
            }

            // Unknown can unify with anything
            (InferredType::Unknown, t) | (t, InferredType::Unknown) => t.clone(),

            // Array types unify on element type
            (InferredType::Array(a), InferredType::Array(b)) => {
                let elem_type = Self::common_type(a, b);
                InferredType::Array(Box::new(elem_type))
            }

            // Otherwise, type mismatch
            (a, b) => InferredType::Error(format!("Type mismatch: cannot unify {} and {}", a, b)),
        }
    }
}

/// Type inference visitor
///
/// Analyzes expressions and infers their types.
/// Returns the inferred type for each expression.
#[derive(Debug)]
pub struct TypeInferenceVisitor;

impl TypeInferenceVisitor {
    /// Create a new type inference visitor
    pub fn new() -> Self {
        TypeInferenceVisitor
    }

    /// Infer the type of an expression
    pub fn infer(&self, expr: &Expr) -> InferredType {
        Self::infer_expr(expr)
    }

    /// Helper function to infer expression type without mut self
    fn infer_expr(expr: &Expr) -> InferredType {
        match expr {
            Expr::Literal(lit) => match lit {
                Literal::Integer(_) => InferredType::Integer,
                Literal::Float(_) => InferredType::Float,
                Literal::Boolean(_) => InferredType::Boolean,
            },
            Expr::Null => InferredType::Null,
            Expr::Identifier(_) => InferredType::Unknown,
            Expr::String(_) => InferredType::String,
            Expr::FieldAccess { .. } => InferredType::Unknown,
            Expr::BinaryOp { op, left, right } => Self::infer_binary_op(*op, left, right),
            Expr::UnaryOp { op, operand } => Self::infer_unary_op(*op, operand),
            Expr::FunctionCall { name, args } => Self::infer_function_call(name, args),
            Expr::Lambda { .. } => InferredType::Unknown,
            Expr::Let { body, .. } => Self::infer_expr(body),
            Expr::If {
                then_branch,
                else_branch,
                ..
            } => {
                let then_type = Self::infer_expr(then_branch);
                let else_type = Self::infer_expr(else_branch);
                InferredType::common_type(&then_type, &else_type)
            }
            Expr::Array(elements) => {
                if elements.is_empty() {
                    InferredType::Array(Box::new(InferredType::Unknown))
                } else {
                    let first_type = Self::infer_expr(&elements[0]);
                    let mut common = first_type;
                    for elem in &elements[1..] {
                        let elem_type = Self::infer_expr(elem);
                        common = InferredType::common_type(&common, &elem_type);
                        if common.is_error() {
                            break;
                        }
                    }
                    InferredType::Array(Box::new(common))
                }
            }
            Expr::Object(_) => InferredType::Object,
            Expr::Pipe { functions, .. } => {
                if functions.is_empty() {
                    InferredType::Unknown
                } else {
                    Self::infer_expr(functions.last().unwrap())
                }
            }
            Expr::Alternative {
                primary,
                alternative,
            } => {
                let primary_type = Self::infer_expr(primary);
                let alt_type = Self::infer_expr(alternative);
                InferredType::common_type(&primary_type, &alt_type)
            }
            Expr::Guard { body, .. } => Self::infer_expr(body),
            Expr::Date(_) => InferredType::Date,
            Expr::DateTime(_) => InferredType::DateTime,
            Expr::Duration(_) => InferredType::Duration,
            Expr::TemporalKeyword(keyword) => match keyword {
                TemporalKeyword::Now => InferredType::DateTime,
                TemporalKeyword::Today | TemporalKeyword::Tomorrow | TemporalKeyword::Yesterday => {
                    InferredType::Date
                }
                _ => InferredType::Date, // Boundary operations return dates
            },
        }
    }

    fn infer_binary_op(op: BinaryOperator, left: &Expr, right: &Expr) -> InferredType {
        let left_type = Self::infer_expr(left);
        let right_type = Self::infer_expr(right);

        match op {
            BinaryOperator::Add => match (&left_type, &right_type) {
                (InferredType::Integer, InferredType::Integer) => InferredType::Integer,
                (InferredType::Float, InferredType::Float) => InferredType::Float,
                (InferredType::Integer, InferredType::Float)
                | (InferredType::Float, InferredType::Integer) => InferredType::Float,
                (InferredType::String, InferredType::String) => InferredType::String,
                // Temporal arithmetic: date/datetime + duration
                (InferredType::Date, InferredType::Duration)
                | (InferredType::Duration, InferredType::Date) => InferredType::Date,
                (InferredType::DateTime, InferredType::Duration)
                | (InferredType::Duration, InferredType::DateTime) => InferredType::DateTime,
                (InferredType::Duration, InferredType::Duration) => InferredType::Duration,
                // Handle Unknown by returning the other type
                (InferredType::Unknown, t) | (t, InferredType::Unknown) => t.clone(),
                _ => InferredType::Error(format!("Cannot add {} and {}", left_type, right_type)),
            },
            BinaryOperator::Sub => match (&left_type, &right_type) {
                (InferredType::Integer, InferredType::Integer) => InferredType::Integer,
                (InferredType::Float, InferredType::Float) => InferredType::Float,
                (InferredType::Integer, InferredType::Float)
                | (InferredType::Float, InferredType::Integer) => InferredType::Float,
                // Temporal arithmetic: date/datetime - duration = date/datetime, date - date = duration
                (InferredType::Date, InferredType::Duration) => InferredType::Date,
                (InferredType::DateTime, InferredType::Duration) => InferredType::DateTime,
                (InferredType::Date, InferredType::Date) => InferredType::Duration,
                (InferredType::DateTime, InferredType::DateTime) => InferredType::Duration,
                (InferredType::Duration, InferredType::Duration) => InferredType::Duration,
                // Handle Unknown
                (InferredType::Unknown, t) | (t, InferredType::Unknown) => {
                    if t.is_numeric() {
                        t.clone()
                    } else {
                        InferredType::Error(format!(
                            "Cannot apply arithmetic to {} and {}",
                            left_type, right_type
                        ))
                    }
                }
                _ => InferredType::Error(format!(
                    "Cannot apply arithmetic to {} and {}",
                    left_type, right_type
                )),
            },
            BinaryOperator::Mul | BinaryOperator::Div => {
                match (&left_type, &right_type) {
                    (InferredType::Integer, InferredType::Integer) => InferredType::Integer,
                    (InferredType::Float, InferredType::Float) => InferredType::Float,
                    (InferredType::Integer, InferredType::Float)
                    | (InferredType::Float, InferredType::Integer) => InferredType::Float,
                    // Handle Unknown
                    (InferredType::Unknown, t) | (t, InferredType::Unknown) => {
                        if t.is_numeric() {
                            t.clone()
                        } else {
                            InferredType::Error(format!(
                                "Cannot apply arithmetic to {} and {}",
                                left_type, right_type
                            ))
                        }
                    }
                    _ => InferredType::Error(format!(
                        "Cannot apply arithmetic to {} and {}",
                        left_type, right_type
                    )),
                }
            }
            BinaryOperator::Mod | BinaryOperator::Pow => {
                if left_type.is_numeric() && right_type.is_numeric() {
                    InferredType::Integer
                } else if left_type == InferredType::Unknown || right_type == InferredType::Unknown
                {
                    // If either is Unknown but the other is numeric, assume Integer result
                    InferredType::Integer
                } else {
                    InferredType::Error(format!(
                        "Cannot apply operator to {} and {}",
                        left_type, right_type
                    ))
                }
            }
            BinaryOperator::Eq
            | BinaryOperator::Neq
            | BinaryOperator::Lt
            | BinaryOperator::Lte
            | BinaryOperator::Gt
            | BinaryOperator::Gte => InferredType::Boolean,
            BinaryOperator::And | BinaryOperator::Or => InferredType::Boolean,
        }
    }

    fn infer_unary_op(op: UnaryOperator, operand: &Expr) -> InferredType {
        let operand_type = Self::infer_expr(operand);
        match op {
            UnaryOperator::Not => InferredType::Boolean,
            UnaryOperator::Neg | UnaryOperator::Plus => operand_type,
        }
    }

    fn infer_function_call(name: &str, args: &[Expr]) -> InferredType {
        match name {
            "length" | "uppercase" | "lowercase" | "trim" | "contains" | "starts_with"
            | "ends_with" => InferredType::String,
            "map" | "filter" | "sort" => InferredType::Array(Box::new(InferredType::Unknown)),
            "abs" | "min" | "max" | "round" | "floor" | "ceil" => {
                if args.is_empty() {
                    InferredType::Unknown
                } else {
                    let arg_type = Self::infer_expr(&args[0]);
                    if arg_type.is_numeric() {
                        arg_type
                    } else {
                        InferredType::Error(format!("Expected numeric argument, got {}", arg_type))
                    }
                }
            }
            "all" | "any" => InferredType::Boolean,
            _ => InferredType::Unknown,
        }
    }
}

impl Default for TypeInferenceVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl Visitor<InferredType> for TypeInferenceVisitor {
    fn visit_expr(&mut self, expr: &Expr) -> InferredType {
        Self::infer_expr(expr)
    }

    fn visit_literal(&mut self, lit: &Literal) -> InferredType {
        match lit {
            Literal::Integer(_) => InferredType::Integer,
            Literal::Float(_) => InferredType::Float,
            Literal::Boolean(_) => InferredType::Boolean,
        }
    }

    fn visit_null(&mut self) -> InferredType {
        InferredType::Null
    }

    fn visit_identifier(&mut self, _name: &str) -> InferredType {
        InferredType::Unknown
    }

    fn visit_field_access(&mut self, _receiver: &Expr, _field: &str) -> InferredType {
        InferredType::Unknown
    }

    fn visit_binary_op(&mut self, op: BinaryOperator, left: &Expr, right: &Expr) -> InferredType {
        Self::infer_binary_op(op, left, right)
    }

    fn visit_unary_op(&mut self, op: UnaryOperator, operand: &Expr) -> InferredType {
        Self::infer_unary_op(op, operand)
    }

    fn visit_function_call(&mut self, name: &str, args: &[Expr]) -> InferredType {
        Self::infer_function_call(name, args)
    }

    fn visit_lambda(&mut self, _param: &str, _body: &Expr) -> InferredType {
        InferredType::Unknown
    }

    fn visit_let(&mut self, _name: &str, _value: &Expr, body: &Expr) -> InferredType {
        Self::infer_expr(body)
    }

    fn visit_if(
        &mut self,
        _condition: &Expr,
        then_branch: &Expr,
        else_branch: &Expr,
    ) -> InferredType {
        let then_type = Self::infer_expr(then_branch);
        let else_type = Self::infer_expr(else_branch);
        InferredType::common_type(&then_type, &else_type)
    }

    fn visit_array(&mut self, elements: &[Expr]) -> InferredType {
        if elements.is_empty() {
            InferredType::Array(Box::new(InferredType::Unknown))
        } else {
            let first_type = Self::infer_expr(&elements[0]);
            let mut common = first_type;
            for elem in &elements[1..] {
                let elem_type = Self::infer_expr(elem);
                common = InferredType::common_type(&common, &elem_type);
                if common.is_error() {
                    break;
                }
            }
            InferredType::Array(Box::new(common))
        }
    }

    fn visit_object(&mut self, _fields: &[(String, Expr)]) -> InferredType {
        InferredType::Object
    }

    fn visit_pipe(&mut self, value: &Expr, functions: &[Expr]) -> InferredType {
        if functions.is_empty() {
            Self::infer_expr(value)
        } else {
            Self::infer_expr(functions.last().unwrap())
        }
    }

    fn visit_alternative(&mut self, primary: &Expr, alternative: &Expr) -> InferredType {
        let primary_type = Self::infer_expr(primary);
        let alt_type = Self::infer_expr(alternative);
        InferredType::common_type(&primary_type, &alt_type)
    }

    fn visit_guard(&mut self, _condition: &Expr, body: &Expr) -> InferredType {
        Self::infer_expr(body)
    }

    fn visit_date(&mut self, _date: &str) -> InferredType {
        InferredType::String
    }

    fn visit_datetime(&mut self, _datetime: &str) -> InferredType {
        InferredType::String
    }

    fn visit_duration(&mut self, _duration: &str) -> InferredType {
        InferredType::String
    }

    fn visit_temporal_keyword(&mut self, _keyword: TemporalKeyword) -> InferredType {
        InferredType::String
    }

    fn visit_string(&mut self, _value: &str) -> InferredType {
        InferredType::String
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    #[test]
    fn test_infer_integer_literal() {
        let expr = Parser::parse("42").unwrap();
        let ty = TypeInferenceVisitor::infer_expr(&expr);
        assert_eq!(ty, InferredType::Integer);
    }

    #[test]
    fn test_infer_float_literal() {
        let expr = Parser::parse("3.14").unwrap();
        let ty = TypeInferenceVisitor::infer_expr(&expr);
        assert_eq!(ty, InferredType::Float);
    }

    #[test]
    fn test_infer_string_literal() {
        let expr = Parser::parse("'hello'").unwrap();
        let ty = TypeInferenceVisitor::infer_expr(&expr);
        assert_eq!(ty, InferredType::String);
    }

    #[test]
    fn test_infer_boolean_literal() {
        let expr = Parser::parse("true").unwrap();
        let ty = TypeInferenceVisitor::infer_expr(&expr);
        assert_eq!(ty, InferredType::Boolean);
    }

    #[test]
    fn test_infer_null_literal() {
        let expr = Parser::parse("null").unwrap();
        let ty = TypeInferenceVisitor::infer_expr(&expr);
        assert_eq!(ty, InferredType::Null);
    }

    #[test]
    fn test_infer_integer_addition() {
        let expr = Parser::parse("1 + 2").unwrap();
        let ty = TypeInferenceVisitor::infer_expr(&expr);
        assert_eq!(ty, InferredType::Integer);
    }

    #[test]
    fn test_infer_float_arithmetic() {
        let expr = Parser::parse("3.0 + 2.0").unwrap();
        let ty = TypeInferenceVisitor::infer_expr(&expr);
        assert_eq!(ty, InferredType::Float);
    }

    #[test]
    fn test_infer_mixed_numeric() {
        let expr = Parser::parse("1 + 2.0").unwrap();
        let ty = TypeInferenceVisitor::infer_expr(&expr);
        assert_eq!(ty, InferredType::Float);
    }

    #[test]
    fn test_infer_comparison() {
        let expr = Parser::parse("5 > 3").unwrap();
        let ty = TypeInferenceVisitor::infer_expr(&expr);
        assert_eq!(ty, InferredType::Boolean);
    }

    #[test]
    fn test_infer_logical_and() {
        let expr = Parser::parse("true && false").unwrap();
        let ty = TypeInferenceVisitor::infer_expr(&expr);
        assert_eq!(ty, InferredType::Boolean);
    }

    #[test]
    fn test_infer_array_integers() {
        let expr = Parser::parse("[1, 2, 3]").unwrap();
        let ty = TypeInferenceVisitor::infer_expr(&expr);
        assert_eq!(ty, InferredType::Array(Box::new(InferredType::Integer)));
    }

    #[test]
    fn test_infer_array_mixed_numeric() {
        let expr = Parser::parse("[1, 2.0, 3]").unwrap();
        let ty = TypeInferenceVisitor::infer_expr(&expr);
        assert_eq!(ty, InferredType::Array(Box::new(InferredType::Float)));
    }

    #[test]
    fn test_infer_empty_array() {
        let expr = Parser::parse("[]").unwrap();
        let ty = TypeInferenceVisitor::infer_expr(&expr);
        assert_eq!(ty, InferredType::Array(Box::new(InferredType::Unknown)));
    }

    #[test]
    fn test_infer_if_same_types() {
        let expr = Parser::parse("if true then 1 else 2").unwrap();
        let ty = TypeInferenceVisitor::infer_expr(&expr);
        assert_eq!(ty, InferredType::Integer);
    }

    #[test]
    fn test_infer_if_different_numeric_types() {
        let expr = Parser::parse("if true then 1 else 2.0").unwrap();
        let ty = TypeInferenceVisitor::infer_expr(&expr);
        assert_eq!(ty, InferredType::Float);
    }

    #[test]
    fn test_infer_let_expression() {
        let expr = Parser::parse("let x = 5 in x + 3").unwrap();
        let ty = TypeInferenceVisitor::infer_expr(&expr);
        assert_eq!(ty, InferredType::Integer);
    }

    #[test]
    fn test_infer_unary_not() {
        let expr = Parser::parse("!true").unwrap();
        let ty = TypeInferenceVisitor::infer_expr(&expr);
        assert_eq!(ty, InferredType::Boolean);
    }

    #[test]
    fn test_infer_string_concat() {
        let expr = Parser::parse("'hello' + ' world'").unwrap();
        let ty = TypeInferenceVisitor::infer_expr(&expr);
        assert_eq!(ty, InferredType::String);
    }

    #[test]
    fn test_type_common_type_same() {
        let t1 = InferredType::Integer;
        let t2 = InferredType::Integer;
        let common = InferredType::common_type(&t1, &t2);
        assert_eq!(common, InferredType::Integer);
    }

    #[test]
    fn test_type_common_type_numeric() {
        let t1 = InferredType::Integer;
        let t2 = InferredType::Float;
        let common = InferredType::common_type(&t1, &t2);
        assert_eq!(common, InferredType::Float);
    }

    #[test]
    fn test_type_common_type_unknown() {
        let t1 = InferredType::Unknown;
        let t2 = InferredType::Integer;
        let common = InferredType::common_type(&t1, &t2);
        assert_eq!(common, InferredType::Integer);
    }

    #[test]
    fn test_type_is_numeric() {
        assert!(InferredType::Integer.is_numeric());
        assert!(InferredType::Float.is_numeric());
        assert!(InferredType::Numeric.is_numeric());
        assert!(!InferredType::String.is_numeric());
    }

    #[test]
    fn test_type_is_scalar() {
        assert!(InferredType::Integer.is_scalar());
        assert!(InferredType::String.is_scalar());
        assert!(!InferredType::Array(Box::new(InferredType::Integer)).is_scalar());
    }
}
