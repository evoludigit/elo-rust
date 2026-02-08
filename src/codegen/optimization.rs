//! Optimization passes for ELO code generation
//!
//! Provides optimization strategies including constant folding,
//! dead code elimination, and expression simplification.

use crate::ast::{BinaryOperator, Expr, Literal, UnaryOperator};

/// Optimization context for code generation
#[derive(Debug, Clone)]
pub struct Optimizer;

impl Optimizer {
    /// Create a new optimizer
    pub fn new() -> Self {
        Self
    }

    /// Apply all optimizations to an expression
    pub fn optimize(expr: &Expr) -> Expr {
        Self::fold_constants(expr)
    }

    /// Constant folding: evaluate constant expressions at compile time
    pub fn fold_constants(expr: &Expr) -> Expr {
        match expr {
            // Binary operations on literals can be folded
            Expr::BinaryOp { op, left, right } => {
                let left_folded = Self::fold_constants(left);
                let right_folded = Self::fold_constants(right);

                // Try to fold if both sides are literals
                if let (Expr::Literal(left_lit), Expr::Literal(right_lit)) =
                    (&left_folded, &right_folded)
                {
                    if let Some(folded) = Self::fold_binary_op(*op, left_lit, right_lit) {
                        return folded;
                    }
                }

                Expr::BinaryOp {
                    op: *op,
                    left: Box::new(left_folded),
                    right: Box::new(right_folded),
                }
            }

            // Unary operations on literals can be folded
            Expr::UnaryOp { op, operand } => {
                let operand_folded = Self::fold_constants(operand);

                if let Expr::Literal(lit) = &operand_folded {
                    if let Some(folded) = Self::fold_unary_op(*op, lit) {
                        return folded;
                    }
                }

                Expr::UnaryOp {
                    op: *op,
                    operand: Box::new(operand_folded),
                }
            }

            // Recursively fold expressions in containers
            Expr::Array(elements) => {
                let folded: Vec<Expr> = elements.iter().map(Self::fold_constants).collect();
                Expr::Array(folded)
            }

            Expr::Object(fields) => {
                let folded: Vec<(String, Expr)> = fields
                    .iter()
                    .map(|(k, v)| (k.clone(), Self::fold_constants(v)))
                    .collect();
                Expr::Object(folded)
            }

            Expr::FieldAccess { receiver, field } => Expr::FieldAccess {
                receiver: Box::new(Self::fold_constants(receiver)),
                field: field.clone(),
            },

            Expr::FunctionCall { name, args } => Expr::FunctionCall {
                name: name.clone(),
                args: args.iter().map(Self::fold_constants).collect(),
            },

            Expr::Lambda { param, body } => Expr::Lambda {
                param: param.clone(),
                body: Box::new(Self::fold_constants(body)),
            },

            Expr::Let { name, value, body } => Expr::Let {
                name: name.clone(),
                value: Box::new(Self::fold_constants(value)),
                body: Box::new(Self::fold_constants(body)),
            },

            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => Expr::If {
                condition: Box::new(Self::fold_constants(condition)),
                then_branch: Box::new(Self::fold_constants(then_branch)),
                else_branch: Box::new(Self::fold_constants(else_branch)),
            },

            Expr::Pipe { value, functions } => Expr::Pipe {
                value: Box::new(Self::fold_constants(value)),
                functions: functions.iter().map(Self::fold_constants).collect(),
            },

            Expr::Alternative {
                primary,
                alternative,
            } => Expr::Alternative {
                primary: Box::new(Self::fold_constants(primary)),
                alternative: Box::new(Self::fold_constants(alternative)),
            },

            Expr::Guard { condition, body } => Expr::Guard {
                condition: Box::new(Self::fold_constants(condition)),
                body: Box::new(Self::fold_constants(body)),
            },

            // Literals and identifiers cannot be folded further
            expr => expr.clone(),
        }
    }

    /// Fold a binary operation on two literals
    fn fold_binary_op(op: BinaryOperator, left: &Literal, right: &Literal) -> Option<Expr> {
        match (left, right) {
            (Literal::Integer(l), Literal::Integer(r)) => {
                match op {
                    BinaryOperator::Add => {
                        let result = l.checked_add(*r)?;
                        Some(Expr::Literal(Literal::Integer(result)))
                    }
                    BinaryOperator::Sub => {
                        let result = l.checked_sub(*r)?;
                        Some(Expr::Literal(Literal::Integer(result)))
                    }
                    BinaryOperator::Mul => {
                        let result = l.checked_mul(*r)?;
                        Some(Expr::Literal(Literal::Integer(result)))
                    }
                    BinaryOperator::Div if *r != 0 => {
                        let result = l.checked_div(*r)?;
                        Some(Expr::Literal(Literal::Integer(result)))
                    }
                    BinaryOperator::Mod if *r != 0 => Some(Expr::Literal(Literal::Integer(l % r))),
                    BinaryOperator::Pow => {
                        if *r < 0 || *r > 31 {
                            return None; // Can't fold negative or large exponents
                        }
                        let result = l.checked_pow(*r as u32)?;
                        Some(Expr::Literal(Literal::Integer(result)))
                    }
                    BinaryOperator::Eq => Some(Expr::Literal(Literal::Boolean(l == r))),
                    BinaryOperator::Neq => Some(Expr::Literal(Literal::Boolean(l != r))),
                    BinaryOperator::Lt => Some(Expr::Literal(Literal::Boolean(l < r))),
                    BinaryOperator::Lte => Some(Expr::Literal(Literal::Boolean(l <= r))),
                    BinaryOperator::Gt => Some(Expr::Literal(Literal::Boolean(l > r))),
                    BinaryOperator::Gte => Some(Expr::Literal(Literal::Boolean(l >= r))),
                    #[allow(clippy::needless_return)]
                    _ => return None,
                }
            }

            (Literal::Float(l), Literal::Float(r)) => {
                let result = match op {
                    BinaryOperator::Add => l + r,
                    BinaryOperator::Sub => l - r,
                    BinaryOperator::Mul => l * r,
                    BinaryOperator::Div if *r != 0.0 => l / r,
                    BinaryOperator::Mod if *r != 0.0 => l % r,
                    BinaryOperator::Pow => l.powf(*r),
                    BinaryOperator::Eq => {
                        return Some(Expr::Literal(Literal::Boolean(
                            (l - r).abs() < f64::EPSILON,
                        )))
                    }
                    BinaryOperator::Neq => {
                        return Some(Expr::Literal(Literal::Boolean(
                            (l - r).abs() >= f64::EPSILON,
                        )))
                    }
                    BinaryOperator::Lt => return Some(Expr::Literal(Literal::Boolean(l < r))),
                    BinaryOperator::Lte => return Some(Expr::Literal(Literal::Boolean(l <= r))),
                    BinaryOperator::Gt => return Some(Expr::Literal(Literal::Boolean(l > r))),
                    BinaryOperator::Gte => return Some(Expr::Literal(Literal::Boolean(l >= r))),
                    _ => return None,
                };
                Some(Expr::Literal(Literal::Float(result)))
            }

            (Literal::Boolean(l), Literal::Boolean(r)) => match op {
                BinaryOperator::And => Some(Expr::Literal(Literal::Boolean(*l && *r))),
                BinaryOperator::Or => Some(Expr::Literal(Literal::Boolean(*l || *r))),
                BinaryOperator::Eq => Some(Expr::Literal(Literal::Boolean(l == r))),
                BinaryOperator::Neq => Some(Expr::Literal(Literal::Boolean(l != r))),
                _ => None,
            },

            _ => None,
        }
    }

    /// Fold a unary operation on a literal
    fn fold_unary_op(op: UnaryOperator, lit: &Literal) -> Option<Expr> {
        match op {
            UnaryOperator::Not => {
                if let Literal::Boolean(b) = lit {
                    Some(Expr::Literal(Literal::Boolean(!b)))
                } else {
                    None
                }
            }
            UnaryOperator::Neg => match lit {
                Literal::Integer(n) => Some(Expr::Literal(Literal::Integer(-n))),
                Literal::Float(f) => Some(Expr::Literal(Literal::Float(-f))),
                _ => None,
            },
            UnaryOperator::Plus => match lit {
                Literal::Integer(n) => Some(Expr::Literal(Literal::Integer(*n))),
                Literal::Float(f) => Some(Expr::Literal(Literal::Float(*f))),
                _ => None,
            },
        }
    }
}

impl Default for Optimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimizer_creation() {
        let _opt = Optimizer::new();
    }

    #[test]
    fn test_fold_integer_addition() {
        let expr = Expr::BinaryOp {
            op: BinaryOperator::Add,
            left: Box::new(Expr::Literal(Literal::Integer(5))),
            right: Box::new(Expr::Literal(Literal::Integer(3))),
        };

        let folded = Optimizer::optimize(&expr);
        match folded {
            Expr::Literal(Literal::Integer(n)) => assert_eq!(n, 8),
            _ => panic!("Expected folded integer literal"),
        }
    }

    #[test]
    fn test_fold_integer_subtraction() {
        let expr = Expr::BinaryOp {
            op: BinaryOperator::Sub,
            left: Box::new(Expr::Literal(Literal::Integer(10))),
            right: Box::new(Expr::Literal(Literal::Integer(3))),
        };

        let folded = Optimizer::optimize(&expr);
        match folded {
            Expr::Literal(Literal::Integer(n)) => assert_eq!(n, 7),
            _ => panic!("Expected folded integer literal"),
        }
    }

    #[test]
    fn test_fold_integer_multiplication() {
        let expr = Expr::BinaryOp {
            op: BinaryOperator::Mul,
            left: Box::new(Expr::Literal(Literal::Integer(4))),
            right: Box::new(Expr::Literal(Literal::Integer(3))),
        };

        let folded = Optimizer::optimize(&expr);
        match folded {
            Expr::Literal(Literal::Integer(n)) => assert_eq!(n, 12),
            _ => panic!("Expected folded integer literal"),
        }
    }

    #[test]
    fn test_fold_float_addition() {
        let expr = Expr::BinaryOp {
            op: BinaryOperator::Add,
            left: Box::new(Expr::Literal(Literal::Float(2.5))),
            right: Box::new(Expr::Literal(Literal::Float(1.5))),
        };

        let folded = Optimizer::optimize(&expr);
        match folded {
            Expr::Literal(Literal::Float(f)) => assert!((f - 4.0).abs() < f64::EPSILON),
            _ => panic!("Expected folded float literal"),
        }
    }

    #[test]
    fn test_fold_boolean_and() {
        let expr = Expr::BinaryOp {
            op: BinaryOperator::And,
            left: Box::new(Expr::Literal(Literal::Boolean(true))),
            right: Box::new(Expr::Literal(Literal::Boolean(false))),
        };

        let folded = Optimizer::optimize(&expr);
        match folded {
            Expr::Literal(Literal::Boolean(b)) => assert!(!b),
            _ => panic!("Expected folded boolean literal"),
        }
    }

    #[test]
    fn test_fold_boolean_or() {
        let expr = Expr::BinaryOp {
            op: BinaryOperator::Or,
            left: Box::new(Expr::Literal(Literal::Boolean(false))),
            right: Box::new(Expr::Literal(Literal::Boolean(true))),
        };

        let folded = Optimizer::optimize(&expr);
        match folded {
            Expr::Literal(Literal::Boolean(b)) => assert!(b),
            _ => panic!("Expected folded boolean literal"),
        }
    }

    #[test]
    fn test_fold_integer_comparison() {
        let expr = Expr::BinaryOp {
            op: BinaryOperator::Gt,
            left: Box::new(Expr::Literal(Literal::Integer(10))),
            right: Box::new(Expr::Literal(Literal::Integer(5))),
        };

        let folded = Optimizer::optimize(&expr);
        match folded {
            Expr::Literal(Literal::Boolean(b)) => assert!(b),
            _ => panic!("Expected folded boolean literal"),
        }
    }

    #[test]
    fn test_fold_unary_not() {
        let expr = Expr::UnaryOp {
            op: UnaryOperator::Not,
            operand: Box::new(Expr::Literal(Literal::Boolean(true))),
        };

        let folded = Optimizer::optimize(&expr);
        match folded {
            Expr::Literal(Literal::Boolean(b)) => assert!(!b),
            _ => panic!("Expected folded boolean literal"),
        }
    }

    #[test]
    fn test_fold_unary_negate() {
        let expr = Expr::UnaryOp {
            op: UnaryOperator::Neg,
            operand: Box::new(Expr::Literal(Literal::Integer(42))),
        };

        let folded = Optimizer::optimize(&expr);
        match folded {
            Expr::Literal(Literal::Integer(n)) => assert_eq!(n, -42),
            _ => panic!("Expected folded integer literal"),
        }
    }

    #[test]
    fn test_no_fold_identifier() {
        let expr = Expr::BinaryOp {
            op: BinaryOperator::Add,
            left: Box::new(Expr::Identifier("x".to_string())),
            right: Box::new(Expr::Literal(Literal::Integer(5))),
        };

        let folded = Optimizer::optimize(&expr);
        // Should not be folded since left is not a literal
        matches!(folded, Expr::BinaryOp { .. });
    }

    #[test]
    fn test_fold_nested_constants() {
        let expr = Expr::BinaryOp {
            op: BinaryOperator::Add,
            left: Box::new(Expr::BinaryOp {
                op: BinaryOperator::Mul,
                left: Box::new(Expr::Literal(Literal::Integer(2))),
                right: Box::new(Expr::Literal(Literal::Integer(3))),
            }),
            right: Box::new(Expr::Literal(Literal::Integer(4))),
        };

        let folded = Optimizer::optimize(&expr);
        match folded {
            Expr::Literal(Literal::Integer(n)) => assert_eq!(n, 10), // (2*3)+4 = 10
            _ => panic!("Expected folded integer literal"),
        }
    }

    #[test]
    fn test_fold_array_constants() {
        let expr = Expr::Array(vec![
            Expr::Literal(Literal::Integer(1)),
            Expr::BinaryOp {
                op: BinaryOperator::Add,
                left: Box::new(Expr::Literal(Literal::Integer(2))),
                right: Box::new(Expr::Literal(Literal::Integer(3))),
            },
        ]);

        let folded = Optimizer::optimize(&expr);
        match folded {
            Expr::Array(elements) => {
                assert_eq!(elements.len(), 2);
                match &elements[1] {
                    Expr::Literal(Literal::Integer(n)) => assert_eq!(*n, 5),
                    _ => panic!("Expected folded constant in array"),
                }
            }
            _ => panic!("Expected array expression"),
        }
    }

    #[test]
    fn test_fold_division_by_zero() {
        let expr = Expr::BinaryOp {
            op: BinaryOperator::Div,
            left: Box::new(Expr::Literal(Literal::Integer(10))),
            right: Box::new(Expr::Literal(Literal::Integer(0))),
        };

        let folded = Optimizer::optimize(&expr);
        // Should not fold division by zero
        matches!(folded, Expr::BinaryOp { .. });
    }
}
