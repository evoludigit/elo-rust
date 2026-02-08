//! Visitor trait for traversing and transforming ELO AST
//!
//! This module defines the Visitor trait, which implements the visitor pattern
//! for AST traversal. Implementors can transform or analyze AST nodes.

use super::{BinaryOperator, Expr, Literal, TemporalKeyword, UnaryOperator};

/// Visitor trait for AST traversal and transformation
///
/// Implement this trait to traverse and/or transform ELO AST nodes.
/// The generic type parameter `T` specifies the return type of visitor methods.
///
/// # Example
///
/// ```ignore
/// use elo_rust::ast::{Expr, Visitor};
/// use proc_macro2::TokenStream;
///
/// struct CodegenVisitor {
///     // ... fields
/// }
///
/// impl Visitor<TokenStream> for CodegenVisitor {
///     fn visit_expr(&mut self, expr: &Expr) -> TokenStream {
///         match expr {
///             Expr::Literal(lit) => self.visit_literal(lit),
///             // ... handle other cases
///         }
///     }
/// }
/// ```
pub trait Visitor<T> {
    /// Visit a generic expression, dispatching to specific methods
    fn visit_expr(&mut self, expr: &Expr) -> T;

    /// Visit a literal value (number or boolean)
    fn visit_literal(&mut self, lit: &Literal) -> T;

    /// Visit a null literal
    fn visit_null(&mut self) -> T;

    /// Visit an identifier (variable reference)
    fn visit_identifier(&mut self, name: &str) -> T;

    /// Visit a field access expression
    fn visit_field_access(&mut self, receiver: &Expr, field: &str) -> T;

    /// Visit a binary operation
    fn visit_binary_op(&mut self, op: BinaryOperator, left: &Expr, right: &Expr) -> T;

    /// Visit a unary operation
    fn visit_unary_op(&mut self, op: UnaryOperator, operand: &Expr) -> T;

    /// Visit a function call
    fn visit_function_call(&mut self, name: &str, args: &[Expr]) -> T;

    /// Visit a lambda expression
    fn visit_lambda(&mut self, param: &str, body: &Expr) -> T;

    /// Visit a let binding
    fn visit_let(&mut self, name: &str, value: &Expr, body: &Expr) -> T;

    /// Visit an if conditional
    fn visit_if(&mut self, condition: &Expr, then_branch: &Expr, else_branch: &Expr) -> T;

    /// Visit an array literal
    fn visit_array(&mut self, elements: &[Expr]) -> T;

    /// Visit an object literal
    fn visit_object(&mut self, fields: &[(String, Expr)]) -> T;

    /// Visit a pipe operator
    fn visit_pipe(&mut self, value: &Expr, functions: &[Expr]) -> T;

    /// Visit an alternative operator (?|)
    fn visit_alternative(&mut self, primary: &Expr, alternative: &Expr) -> T;

    /// Visit a guard expression
    fn visit_guard(&mut self, condition: &Expr, body: &Expr) -> T;

    /// Visit a date literal
    fn visit_date(&mut self, date: &str) -> T;

    /// Visit a datetime literal
    fn visit_datetime(&mut self, datetime: &str) -> T;

    /// Visit a duration literal
    fn visit_duration(&mut self, duration: &str) -> T;

    /// Visit a temporal keyword
    fn visit_temporal_keyword(&mut self, keyword: TemporalKeyword) -> T;

    /// Visit a string literal
    fn visit_string(&mut self, value: &str) -> T;
}

/// Default visitor implementation that dispatches to specific visitor methods
///
/// This provides the standard dispatch logic for visit_expr.
/// Most implementations will use the default visit_expr from this trait.
pub trait DefaultVisitor<T>: Visitor<T> {
    /// Default implementation of visit_expr that dispatches to specific methods
    fn default_visit_expr(&mut self, expr: &Expr) -> T {
        match expr {
            Expr::Literal(lit) => self.visit_literal(lit),
            Expr::Null => self.visit_null(),
            Expr::Identifier(name) => self.visit_identifier(name),
            Expr::String(value) => self.visit_string(value),
            Expr::FieldAccess { receiver, field } => {
                self.visit_field_access(receiver, field)
            }
            Expr::BinaryOp { op, left, right } => {
                self.visit_binary_op(*op, left, right)
            }
            Expr::UnaryOp { op, operand } => {
                self.visit_unary_op(*op, operand)
            }
            Expr::FunctionCall { name, args } => {
                self.visit_function_call(name, args)
            }
            Expr::Lambda { param, body } => {
                self.visit_lambda(param, body)
            }
            Expr::Let { name, value, body } => {
                self.visit_let(name, value, body)
            }
            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.visit_if(condition, then_branch, else_branch)
            }
            Expr::Array(elements) => {
                self.visit_array(elements)
            }
            Expr::Object(fields) => {
                self.visit_object(fields)
            }
            Expr::Pipe { value, functions } => {
                self.visit_pipe(value, functions)
            }
            Expr::Alternative {
                primary,
                alternative,
            } => {
                self.visit_alternative(primary, alternative)
            }
            Expr::Guard { condition, body } => {
                self.visit_guard(condition, body)
            }
            Expr::Date(date) => {
                self.visit_date(date)
            }
            Expr::DateTime(datetime) => {
                self.visit_datetime(datetime)
            }
            Expr::Duration(duration) => {
                self.visit_duration(duration)
            }
            Expr::TemporalKeyword(keyword) => {
                self.visit_temporal_keyword(*keyword)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    struct CountingVisitor {
        count: Rc<RefCell<usize>>,
    }

    impl Visitor<()> for CountingVisitor {
        fn visit_expr(&mut self, expr: &Expr) {
            *self.count.borrow_mut() += 1;
            match expr {
                Expr::BinaryOp { left, right, .. } => {
                    self.visit_expr(left);
                    self.visit_expr(right);
                }
                Expr::UnaryOp { operand, .. } => {
                    self.visit_expr(operand);
                }
                Expr::FieldAccess { receiver, .. } => {
                    self.visit_expr(receiver);
                }
                _ => {}
            }
        }

        fn visit_literal(&mut self, _lit: &Literal) {}
        fn visit_null(&mut self) {}
        fn visit_identifier(&mut self, _name: &str) {}
        fn visit_field_access(&mut self, _receiver: &Expr, _field: &str) {}
        fn visit_binary_op(&mut self, _op: BinaryOperator, _left: &Expr, _right: &Expr) {}
        fn visit_unary_op(&mut self, _op: UnaryOperator, _operand: &Expr) {}
        fn visit_function_call(&mut self, _name: &str, _args: &[Expr]) {}
        fn visit_lambda(&mut self, _param: &str, _body: &Expr) {}
        fn visit_let(&mut self, _name: &str, _value: &Expr, _body: &Expr) {}
        fn visit_if(&mut self, _condition: &Expr, _then_branch: &Expr, _else_branch: &Expr) {}
        fn visit_array(&mut self, _elements: &[Expr]) {}
        fn visit_object(&mut self, _fields: &[(String, Expr)]) {}
        fn visit_pipe(&mut self, _value: &Expr, _functions: &[Expr]) {}
        fn visit_alternative(&mut self, _primary: &Expr, _alternative: &Expr) {}
        fn visit_guard(&mut self, _condition: &Expr, _body: &Expr) {}
        fn visit_date(&mut self, _date: &str) {}
        fn visit_datetime(&mut self, _datetime: &str) {}
        fn visit_duration(&mut self, _duration: &str) {}
        fn visit_temporal_keyword(&mut self, _keyword: TemporalKeyword) {}
        fn visit_string(&mut self, _value: &str) {}
    }

    #[test]
    fn test_visitor_basic() {
        let expr = Expr::Literal(Literal::Integer(42));
        let mut visitor = CountingVisitor {
            count: Rc::new(RefCell::new(0)),
        };
        visitor.visit_expr(&expr);
        assert_eq!(*visitor.count.borrow(), 1);
    }

    #[test]
    fn test_visitor_traversal() {
        let expr = Expr::BinaryOp {
            op: BinaryOperator::Add,
            left: Box::new(Expr::Literal(Literal::Integer(1))),
            right: Box::new(Expr::Literal(Literal::Integer(2))),
        };
        let mut visitor = CountingVisitor {
            count: Rc::new(RefCell::new(0)),
        };
        visitor.visit_expr(&expr);
        assert_eq!(*visitor.count.borrow(), 3); // binary op + 2 literals
    }
}
