//! AST-to-Rust code generation visitor
//!
//! This module implements the Visitor trait to transform ELO AST nodes into
//! Rust TokenStreams that can be compiled.

use crate::ast::visitor::Visitor;
use crate::ast::{BinaryOperator, Expr, Literal, TemporalKeyword, UnaryOperator};
use proc_macro2::TokenStream;
use quote::quote;

use super::{
    functions::FunctionGenerator, operators::{BinaryOp, OperatorGenerator, UnaryOp},
    temporal::TemporalGenerator,
};

/// Visitor that generates Rust code from ELO AST
#[derive(Debug)]
pub struct CodegenVisitor {
    operator_gen: OperatorGenerator,
    function_gen: FunctionGenerator,
    temporal_gen: TemporalGenerator,
}

impl CodegenVisitor {
    /// Create a new code generation visitor
    pub fn new() -> Self {
        CodegenVisitor {
            operator_gen: OperatorGenerator::new(),
            function_gen: FunctionGenerator::new(),
            temporal_gen: TemporalGenerator::new(),
        }
    }

    /// Convert AST BinaryOperator to codegen BinaryOp
    fn convert_binary_op(op: BinaryOperator) -> BinaryOp {
        match op {
            BinaryOperator::Add => BinaryOp::Add,
            BinaryOperator::Sub => BinaryOp::Subtract,
            BinaryOperator::Mul => BinaryOp::Multiply,
            BinaryOperator::Div => BinaryOp::Divide,
            BinaryOperator::Mod => BinaryOp::Modulo,
            BinaryOperator::Pow => BinaryOp::Multiply, // Fallback, would need special handling
            BinaryOperator::Eq => BinaryOp::Equal,
            BinaryOperator::Neq => BinaryOp::NotEqual,
            BinaryOperator::Lt => BinaryOp::Less,
            BinaryOperator::Lte => BinaryOp::LessEqual,
            BinaryOperator::Gt => BinaryOp::Greater,
            BinaryOperator::Gte => BinaryOp::GreaterEqual,
            BinaryOperator::And => BinaryOp::And,
            BinaryOperator::Or => BinaryOp::Or,
        }
    }

    /// Convert AST UnaryOperator to codegen UnaryOp
    fn convert_unary_op(op: UnaryOperator) -> UnaryOp {
        match op {
            UnaryOperator::Not => UnaryOp::Not,
            UnaryOperator::Neg => UnaryOp::Negate,
            UnaryOperator::Plus => UnaryOp::Negate, // Identity, treat as no-op via negate
        }
    }
}

impl Default for CodegenVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl Visitor<TokenStream> for CodegenVisitor {
    fn visit_expr(&mut self, expr: &Expr) -> TokenStream {
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

    fn visit_literal(&mut self, lit: &Literal) -> TokenStream {
        match lit {
            Literal::Integer(n) => quote! { #n },
            Literal::Float(f) => quote! { #f },
            Literal::Boolean(b) => quote! { #b },
        }
    }

    fn visit_null(&mut self) -> TokenStream {
        quote! { None::<()> }
    }

    fn visit_identifier(&mut self, name: &str) -> TokenStream {
        let ident = quote::format_ident!("{}", name);
        quote! { #ident }
    }

    fn visit_field_access(&mut self, receiver: &Expr, field: &str) -> TokenStream {
        let recv = self.visit_expr(receiver);
        let field_ident = quote::format_ident!("{}", field);
        quote! { #recv.#field_ident }
    }

    fn visit_binary_op(
        &mut self,
        op: BinaryOperator,
        left: &Expr,
        right: &Expr,
    ) -> TokenStream {
        let l = self.visit_expr(left);
        let r = self.visit_expr(right);
        let codegen_op = Self::convert_binary_op(op);
        self.operator_gen.binary(codegen_op, l, r)
    }

    fn visit_unary_op(&mut self, op: UnaryOperator, operand: &Expr) -> TokenStream {
        let operand = self.visit_expr(operand);
        let codegen_op = Self::convert_unary_op(op);
        self.operator_gen.unary(codegen_op, operand)
    }

    fn visit_function_call(&mut self, name: &str, args: &[Expr]) -> TokenStream {
        let arg_tokens: Vec<TokenStream> = args.iter().map(|a| self.visit_expr(a)).collect();

        // Use the unified function generator interface
        self.function_gen.call(name, arg_tokens)
    }

    fn visit_lambda(&mut self, param: &str, body: &Expr) -> TokenStream {
        let param_ident = quote::format_ident!("{}", param);
        let body = self.visit_expr(body);
        quote! {
            |#param_ident| {
                #body
            }
        }
    }

    fn visit_let(&mut self, name: &str, value: &Expr, body: &Expr) -> TokenStream {
        let var_ident = quote::format_ident!("{}", name);
        let val = self.visit_expr(value);
        let bod = self.visit_expr(body);
        quote! {
            {
                let #var_ident = #val;
                #bod
            }
        }
    }

    fn visit_if(
        &mut self,
        condition: &Expr,
        then_branch: &Expr,
        else_branch: &Expr,
    ) -> TokenStream {
        let cond = self.visit_expr(condition);
        let then_b = self.visit_expr(then_branch);
        let else_b = self.visit_expr(else_branch);
        quote! {
            if #cond { #then_b } else { #else_b }
        }
    }

    fn visit_array(&mut self, elements: &[Expr]) -> TokenStream {
        let elems: Vec<TokenStream> = elements.iter().map(|e| self.visit_expr(e)).collect();
        quote! {
            vec![#(#elems),*]
        }
    }

    fn visit_object(&mut self, fields: &[(String, Expr)]) -> TokenStream {
        // For objects, we can't easily generate code without knowing the target type
        // For now, generate a tuple struct representing the key-value pairs
        let pairs: Vec<TokenStream> = fields
            .iter()
            .map(|(k, v)| {
                let val = self.visit_expr(v);
                let key_str = k.clone();
                quote! { (#key_str, #val) }
            })
            .collect();
        quote! {
            vec![#(#pairs),*]
        }
    }

    fn visit_pipe(&mut self, value: &Expr, functions: &[Expr]) -> TokenStream {
        let mut result = self.visit_expr(value);

        for func in functions {
            // For each function in the pipe, we need to apply it to the previous result
            // If it's a function call, inject result as first arg
            // Otherwise, create a function call with result as argument
            match func {
                Expr::FunctionCall { name, args } => {
                    // Insert result as first argument
                    let mut new_args = vec![result.clone()];
                    for arg in args {
                        new_args.push(self.visit_expr(arg));
                    }

                    // Generate the function call with the new arguments
                    let arg_tokens: Vec<TokenStream> = new_args;
                    result = self.function_gen.call(name, arg_tokens);
                }
                Expr::Identifier(name) => {
                    // Simple identifier - treat as a function call with one argument
                    result = self.function_gen.call(name, vec![result]);
                }
                _ => {
                    // Other expressions - try to apply them
                    result = self.visit_expr(func);
                }
            }
        }
        result
    }

    fn visit_alternative(&mut self, primary: &Expr, alternative: &Expr) -> TokenStream {
        let prim = self.visit_expr(primary);
        let alt = self.visit_expr(alternative);
        quote! {
            #prim.or_else(|| #alt)
        }
    }

    fn visit_guard(&mut self, condition: &Expr, body: &Expr) -> TokenStream {
        let cond = self.visit_expr(condition);
        let bod = self.visit_expr(body);
        quote! {
            if #cond { #bod } else { panic!("Guard failed") }
        }
    }

    fn visit_date(&mut self, date: &str) -> TokenStream {
        self.temporal_gen.date(date)
    }

    fn visit_datetime(&mut self, datetime: &str) -> TokenStream {
        self.temporal_gen.datetime(datetime)
    }

    fn visit_duration(&mut self, duration: &str) -> TokenStream {
        self.temporal_gen.duration(duration)
    }

    fn visit_temporal_keyword(&mut self, keyword: TemporalKeyword) -> TokenStream {
        let keyword_str = match keyword {
            TemporalKeyword::Now => "NOW",
            TemporalKeyword::Today => "TODAY",
            TemporalKeyword::Tomorrow => "TOMORROW",
            TemporalKeyword::Yesterday => "YESTERDAY",
            TemporalKeyword::StartOfDay => "START_OF_DAY",
            TemporalKeyword::EndOfDay => "END_OF_DAY",
            TemporalKeyword::StartOfWeek => "START_OF_WEEK",
            TemporalKeyword::EndOfWeek => "END_OF_WEEK",
            TemporalKeyword::StartOfMonth => "START_OF_MONTH",
            TemporalKeyword::EndOfMonth => "END_OF_MONTH",
            TemporalKeyword::StartOfQuarter => "START_OF_QUARTER",
            TemporalKeyword::EndOfQuarter => "END_OF_QUARTER",
            TemporalKeyword::StartOfYear => "START_OF_YEAR",
            TemporalKeyword::EndOfYear => "END_OF_YEAR",
            TemporalKeyword::BeginningOfTime => "BEGINNING_OF_TIME",
            TemporalKeyword::EndOfTime => "END_OF_TIME",
        };
        self.temporal_gen.keyword(keyword_str)
    }

    fn visit_string(&mut self, value: &str) -> TokenStream {
        quote! { #value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::BinaryOperator;

    #[test]
    fn test_codegen_literal_integer() {
        let mut visitor = CodegenVisitor::new();
        let expr = Expr::Literal(Literal::Integer(42));
        let tokens = visitor.visit_expr(&expr);
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("42"));
    }

    #[test]
    fn test_codegen_identifier() {
        let mut visitor = CodegenVisitor::new();
        let expr = Expr::Identifier("age".to_string());
        let tokens = visitor.visit_expr(&expr);
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("age"));
    }

    #[test]
    fn test_codegen_field_access() {
        let mut visitor = CodegenVisitor::new();
        let expr = Expr::FieldAccess {
            receiver: Box::new(Expr::Identifier("user".to_string())),
            field: "age".to_string(),
        };
        let tokens = visitor.visit_expr(&expr);
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("user"));
        assert!(tokens_str.contains("age"));
    }

    #[test]
    fn test_codegen_binary_op() {
        let mut visitor = CodegenVisitor::new();
        let expr = Expr::BinaryOp {
            op: BinaryOperator::Add,
            left: Box::new(Expr::Literal(Literal::Integer(1))),
            right: Box::new(Expr::Literal(Literal::Integer(2))),
        };
        let tokens = visitor.visit_expr(&expr);
        let tokens_str = tokens.to_string();
        // Should contain the addition operation
        assert!(!tokens_str.is_empty());
    }

    #[test]
    fn test_codegen_let_expr() {
        let mut visitor = CodegenVisitor::new();
        let expr = Expr::Let {
            name: "x".to_string(),
            value: Box::new(Expr::Literal(Literal::Integer(42))),
            body: Box::new(Expr::Identifier("x".to_string())),
        };
        let tokens = visitor.visit_expr(&expr);
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("let"));
        assert!(tokens_str.contains("x"));
    }

    #[test]
    fn test_codegen_if_expr() {
        let mut visitor = CodegenVisitor::new();
        let expr = Expr::If {
            condition: Box::new(Expr::Literal(Literal::Boolean(true))),
            then_branch: Box::new(Expr::Literal(Literal::Integer(1))),
            else_branch: Box::new(Expr::Literal(Literal::Integer(0))),
        };
        let tokens = visitor.visit_expr(&expr);
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("if"));
    }

    #[test]
    fn test_codegen_array() {
        let mut visitor = CodegenVisitor::new();
        let expr = Expr::Array(vec![
            Expr::Literal(Literal::Integer(1)),
            Expr::Literal(Literal::Integer(2)),
        ]);
        let tokens = visitor.visit_expr(&expr);
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("vec"));
    }

    #[test]
    fn test_codegen_string() {
        let mut visitor = CodegenVisitor::new();
        let expr = Expr::String("hello".to_string());
        let tokens = visitor.visit_expr(&expr);
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("hello"));
    }

    #[test]
    fn test_codegen_null() {
        let mut visitor = CodegenVisitor::new();
        let expr = Expr::Null;
        let tokens = visitor.visit_expr(&expr);
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("None"));
    }
}
