//! Parser for ELO expressions
//!
//! This module provides a recursive descent parser that converts ELO token streams
//! into an Abstract Syntax Tree (AST).
//!
//! The parser implements correct operator precedence through a precedence cascade:
//! pipe > logical_or > logical_and > equality > comparison > addition > multiplication > power > unary > postfix > primary

pub mod error;
pub mod lexer;

pub use error::ParseError;
pub use lexer::{LexError, Lexer, Token};

use crate::ast::{BinaryOperator, Expr, Literal, TemporalKeyword, UnaryOperator};

/// Parser for ELO expressions
///
/// Implements a recursive descent parser with correct operator precedence.
#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    /// Create a new parser from a token stream
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    /// Parse a complete ELO expression from a string
    ///
    /// # Example
    ///
    /// ```ignore
    /// let expr = Parser::parse("age >= 18")?;
    /// ```
    pub fn parse(input: &str) -> Result<Expr, ParseError> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer
            .tokenize()
            .map_err(|err| ParseError::new(err.message, err.line, err.column))?;
        let mut parser = Parser::new(tokens);
        parser.parse_expression()
    }

    /// Parse an expression
    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        self.parse_pipe()
    }

    /// Peek at the current token
    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or(&Token::Eof)
    }

    /// Advance to the next token
    fn advance(&mut self) -> Token {
        let token = self.peek().clone();
        if self.current < self.tokens.len() {
            self.current += 1;
        }
        token
    }

    /// Check if the current token matches a given token
    fn check(&self, token: &Token) -> bool {
        std::mem::discriminant(self.peek()) == std::mem::discriminant(token)
    }

    /// Consume a specific token or return an error
    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        if self.check(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::new(
                format!("Expected {}, found {}", expected, self.peek()),
                1,
                1,
            ))
        }
    }

    /// Parse pipe operator expressions: expr |> func() |> ...
    fn parse_pipe(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_logical_or()?;

        while self.check(&Token::Pipe) {
            self.advance();
            let func = self.parse_logical_or()?;
            expr = Expr::Pipe {
                value: Box::new(expr),
                functions: vec![func],
            };
        }

        Ok(expr)
    }

    /// Parse logical OR: left || right
    fn parse_logical_or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_logical_and()?;

        while self.check(&Token::OrOr) {
            self.advance();
            let right = self.parse_logical_and()?;
            expr = Expr::BinaryOp {
                op: BinaryOperator::Or,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parse logical AND: left && right
    fn parse_logical_and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_equality()?;

        while self.check(&Token::AndAnd) {
            self.advance();
            let right = self.parse_equality()?;
            expr = Expr::BinaryOp {
                op: BinaryOperator::And,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parse equality operators: == !=
    fn parse_equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_comparison()?;

        loop {
            let op = match self.peek() {
                Token::EqualEqual => BinaryOperator::Eq,
                Token::NotEqual => BinaryOperator::Neq,
                _ => break,
            };
            self.advance();
            let right = self.parse_comparison()?;
            expr = Expr::BinaryOp {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parse comparison operators: < > <= >=
    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_addition()?;

        loop {
            let op = match self.peek() {
                Token::Less => BinaryOperator::Lt,
                Token::LessEqual => BinaryOperator::Lte,
                Token::Greater => BinaryOperator::Gt,
                Token::GreaterEqual => BinaryOperator::Gte,
                _ => break,
            };
            self.advance();
            let right = self.parse_addition()?;
            expr = Expr::BinaryOp {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parse addition and subtraction: + -
    fn parse_addition(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_multiplication()?;

        loop {
            let op = match self.peek() {
                Token::Plus => BinaryOperator::Add,
                Token::Minus => BinaryOperator::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplication()?;
            expr = Expr::BinaryOp {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parse multiplication, division, modulo: * / %
    fn parse_multiplication(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_power()?;

        loop {
            let op = match self.peek() {
                Token::Star => BinaryOperator::Mul,
                Token::Slash => BinaryOperator::Div,
                Token::Percent => BinaryOperator::Mod,
                _ => break,
            };
            self.advance();
            let right = self.parse_power()?;
            expr = Expr::BinaryOp {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parse exponentiation: ^
    fn parse_power(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_unary()?;

        if self.check(&Token::Caret) {
            self.advance();
            let right = self.parse_power()?; // Right-associative
            expr = Expr::BinaryOp {
                op: BinaryOperator::Pow,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parse unary operators: ! - +
    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        match self.peek() {
            Token::Bang => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::UnaryOp {
                    op: UnaryOperator::Not,
                    operand: Box::new(operand),
                })
            }
            Token::Minus => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::UnaryOp {
                    op: UnaryOperator::Neg,
                    operand: Box::new(operand),
                })
            }
            Token::Plus => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::UnaryOp {
                    op: UnaryOperator::Plus,
                    operand: Box::new(operand),
                })
            }
            _ => self.parse_postfix(),
        }
    }

    /// Parse postfix expressions: field access, function calls, etc.
    fn parse_postfix(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_primary()?;

        loop {
            match self.peek() {
                Token::Dot => {
                    self.advance();
                    match self.advance() {
                        Token::Identifier(field) => {
                            expr = Expr::FieldAccess {
                                receiver: Box::new(expr),
                                field,
                            };
                        }
                        _ => {
                            return Err(ParseError::new("Expected field name after '.'", 1, 1));
                        }
                    }
                }
                Token::LeftBracket => {
                    // Array access (not fully implemented in MVP)
                    break;
                }
                Token::LeftParen if matches!(expr, Expr::Identifier(_)) => {
                    // This is a function call - handle it in primary instead
                    break;
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    /// Parse primary expressions: literals, identifiers, function calls, etc.
    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        match self.peek() {
            Token::Integer(n) => {
                let value = *n;
                self.advance();
                Ok(Expr::Literal(Literal::Integer(value)))
            }
            Token::Float(f) => {
                let value = *f;
                self.advance();
                Ok(Expr::Literal(Literal::Float(value)))
            }
            Token::True => {
                self.advance();
                Ok(Expr::Literal(Literal::Boolean(true)))
            }
            Token::False => {
                self.advance();
                Ok(Expr::Literal(Literal::Boolean(false)))
            }
            Token::Null => {
                self.advance();
                Ok(Expr::Null)
            }
            Token::String(s) => {
                let value = s.clone();
                self.advance();
                Ok(Expr::String(value))
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();

                // Check for function call
                if self.check(&Token::LeftParen) {
                    self.advance();
                    let args = self.parse_function_args()?;
                    self.expect(Token::RightParen)?;
                    Ok(Expr::FunctionCall { name, args })
                } else {
                    Ok(Expr::Identifier(name))
                }
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(Token::RightParen)?;
                Ok(expr)
            }
            Token::LeftBracket => {
                self.advance();
                let elements = self.parse_array_elements()?;
                self.expect(Token::RightBracket)?;
                Ok(Expr::Array(elements))
            }
            Token::LeftBrace => {
                self.advance();
                let fields = self.parse_object_fields()?;
                self.expect(Token::RightBrace)?;
                Ok(Expr::Object(fields))
            }
            Token::Let => self.parse_let(),
            Token::If => self.parse_if(),
            Token::Fn => self.parse_lambda(),
            Token::Guard => self.parse_guard(),
            Token::Now => {
                self.advance();
                Ok(Expr::TemporalKeyword(TemporalKeyword::Now))
            }
            Token::Today => {
                self.advance();
                Ok(Expr::TemporalKeyword(TemporalKeyword::Today))
            }
            Token::Tomorrow => {
                self.advance();
                Ok(Expr::TemporalKeyword(TemporalKeyword::Tomorrow))
            }
            Token::Yesterday => {
                self.advance();
                Ok(Expr::TemporalKeyword(TemporalKeyword::Yesterday))
            }
            Token::StartOfDay => {
                self.advance();
                Ok(Expr::TemporalKeyword(TemporalKeyword::StartOfDay))
            }
            Token::EndOfDay => {
                self.advance();
                Ok(Expr::TemporalKeyword(TemporalKeyword::EndOfDay))
            }
            Token::StartOfWeek => {
                self.advance();
                Ok(Expr::TemporalKeyword(TemporalKeyword::StartOfWeek))
            }
            Token::EndOfWeek => {
                self.advance();
                Ok(Expr::TemporalKeyword(TemporalKeyword::EndOfWeek))
            }
            Token::StartOfMonth => {
                self.advance();
                Ok(Expr::TemporalKeyword(TemporalKeyword::StartOfMonth))
            }
            Token::EndOfMonth => {
                self.advance();
                Ok(Expr::TemporalKeyword(TemporalKeyword::EndOfMonth))
            }
            Token::StartOfQuarter => {
                self.advance();
                Ok(Expr::TemporalKeyword(TemporalKeyword::StartOfQuarter))
            }
            Token::EndOfQuarter => {
                self.advance();
                Ok(Expr::TemporalKeyword(TemporalKeyword::EndOfQuarter))
            }
            Token::StartOfYear => {
                self.advance();
                Ok(Expr::TemporalKeyword(TemporalKeyword::StartOfYear))
            }
            Token::EndOfYear => {
                self.advance();
                Ok(Expr::TemporalKeyword(TemporalKeyword::EndOfYear))
            }
            Token::BeginningOfTime => {
                self.advance();
                Ok(Expr::TemporalKeyword(TemporalKeyword::BeginningOfTime))
            }
            Token::EndOfTime => {
                self.advance();
                Ok(Expr::TemporalKeyword(TemporalKeyword::EndOfTime))
            }
            _ => Err(ParseError::new(
                format!("Unexpected token: {}", self.peek()),
                1,
                1,
            )),
        }
    }

    /// Parse function call arguments
    fn parse_function_args(&mut self) -> Result<Vec<Expr>, ParseError> {
        let mut args = Vec::new();

        if !self.check(&Token::RightParen) {
            loop {
                args.push(self.parse_expression()?);
                if !self.check(&Token::Comma) {
                    break;
                }
                self.advance();
            }
        }

        Ok(args)
    }

    /// Parse array literal elements
    fn parse_array_elements(&mut self) -> Result<Vec<Expr>, ParseError> {
        let mut elements = Vec::new();

        if !self.check(&Token::RightBracket) {
            loop {
                elements.push(self.parse_expression()?);
                if !self.check(&Token::Comma) {
                    break;
                }
                self.advance();
            }
        }

        Ok(elements)
    }

    /// Parse object literal fields
    fn parse_object_fields(&mut self) -> Result<Vec<(String, Expr)>, ParseError> {
        let mut fields = Vec::new();

        if !self.check(&Token::RightBrace) {
            loop {
                let key = match self.advance() {
                    Token::Identifier(name) => name,
                    Token::String(s) => s,
                    _ => {
                        return Err(ParseError::new(
                            "Expected field name in object literal",
                            1,
                            1,
                        ))
                    }
                };

                self.expect(Token::Colon)?;
                let value = self.parse_expression()?;
                fields.push((key, value));

                if !self.check(&Token::Comma) {
                    break;
                }
                self.advance();
            }
        }

        Ok(fields)
    }

    /// Parse let expression: let name = value in body
    fn parse_let(&mut self) -> Result<Expr, ParseError> {
        self.expect(Token::Let)?;

        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => return Err(ParseError::new("Expected variable name after 'let'", 1, 1)),
        };

        self.expect(Token::Equal)?;
        let value = Box::new(self.parse_expression()?);

        self.expect(Token::In)?;
        let body = Box::new(self.parse_expression()?);

        Ok(Expr::Let { name, value, body })
    }

    /// Parse if expression: if condition then branch_a else branch_b
    fn parse_if(&mut self) -> Result<Expr, ParseError> {
        self.expect(Token::If)?;
        let condition = Box::new(self.parse_expression()?);
        self.expect(Token::Then)?;
        let then_branch = Box::new(self.parse_expression()?);
        self.expect(Token::Else)?;
        let else_branch = Box::new(self.parse_expression()?);

        Ok(Expr::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    /// Parse lambda expression: fn(param ~> body) or (param ~> body)
    fn parse_lambda(&mut self) -> Result<Expr, ParseError> {
        self.expect(Token::Fn)?;
        self.expect(Token::LeftParen)?;

        let param = match self.advance() {
            Token::Identifier(p) => p,
            _ => return Err(ParseError::new("Expected parameter name in lambda", 1, 1)),
        };

        self.expect(Token::LambdaArrow)?;
        let body = Box::new(self.parse_expression()?);
        self.expect(Token::RightParen)?;

        Ok(Expr::Lambda { param, body })
    }

    /// Parse guard expression: guard condition in body
    fn parse_guard(&mut self) -> Result<Expr, ParseError> {
        self.expect(Token::Guard)?;
        let condition = Box::new(self.parse_expression()?);
        self.expect(Token::In)?;
        let body = Box::new(self.parse_expression()?);

        Ok(Expr::Guard { condition, body })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_integer() {
        let expr = Parser::parse("42").unwrap();
        assert_eq!(expr, Expr::Literal(Literal::Integer(42)));
    }

    #[test]
    fn test_parse_float() {
        let expr = Parser::parse("3.15").unwrap();
        assert_eq!(expr, Expr::Literal(Literal::Float(3.15)));
    }

    #[test]
    fn test_parse_boolean() {
        let expr = Parser::parse("true").unwrap();
        assert_eq!(expr, Expr::Literal(Literal::Boolean(true)));
    }

    #[test]
    fn test_parse_identifier() {
        let expr = Parser::parse("age").unwrap();
        assert_eq!(expr, Expr::Identifier("age".to_string()));
    }

    #[test]
    fn test_parse_binary_op() {
        let expr = Parser::parse("1 + 2").unwrap();
        match expr {
            Expr::BinaryOp {
                op: BinaryOperator::Add,
                ..
            } => {}
            _ => panic!("Expected binary add operation"),
        }
    }

    #[test]
    fn test_parse_comparison() {
        let expr = Parser::parse("age >= 18").unwrap();
        match expr {
            Expr::BinaryOp {
                op: BinaryOperator::Gte,
                ..
            } => {}
            _ => panic!("Expected >= operator"),
        }
    }

    #[test]
    fn test_parse_logical_and() {
        let expr = Parser::parse("true && false").unwrap();
        match expr {
            Expr::BinaryOp {
                op: BinaryOperator::And,
                ..
            } => {}
            _ => panic!("Expected && operator"),
        }
    }

    #[test]
    fn test_parse_logical_or() {
        let expr = Parser::parse("true || false").unwrap();
        match expr {
            Expr::BinaryOp {
                op: BinaryOperator::Or,
                ..
            } => {}
            _ => panic!("Expected || operator"),
        }
    }

    #[test]
    fn test_parse_field_access() {
        let expr = Parser::parse("user.age").unwrap();
        match expr {
            Expr::FieldAccess { field, .. } => {
                assert_eq!(field, "age");
            }
            _ => panic!("Expected field access"),
        }
    }

    #[test]
    fn test_parse_function_call() {
        let expr = Parser::parse("length(name)").unwrap();
        match expr {
            Expr::FunctionCall { name, args } => {
                assert_eq!(name, "length");
                assert_eq!(args.len(), 1);
            }
            _ => panic!("Expected function call"),
        }
    }

    #[test]
    fn test_parse_function_multiple_args() {
        let expr = Parser::parse("substring(name, 0, 5)").unwrap();
        match expr {
            Expr::FunctionCall { args, .. } => {
                assert_eq!(args.len(), 3);
            }
            _ => panic!("Expected function call with 3 args"),
        }
    }

    #[test]
    fn test_parse_array_literal() {
        let expr = Parser::parse("[1, 2, 3]").unwrap();
        match expr {
            Expr::Array(elements) => {
                assert_eq!(elements.len(), 3);
            }
            _ => panic!("Expected array literal"),
        }
    }

    #[test]
    fn test_parse_object_literal() {
        let expr = Parser::parse("{x: 1, y: 2}").unwrap();
        match expr {
            Expr::Object(fields) => {
                assert_eq!(fields.len(), 2);
            }
            _ => panic!("Expected object literal"),
        }
    }

    #[test]
    fn test_parse_null() {
        let expr = Parser::parse("null").unwrap();
        assert_eq!(expr, Expr::Null);
    }

    #[test]
    fn test_parse_string_literal() {
        let expr = Parser::parse("'hello'").unwrap();
        assert_eq!(expr, Expr::String("hello".to_string()));
    }

    #[test]
    fn test_parse_temporal_keyword() {
        let expr = Parser::parse("NOW").unwrap();
        assert_eq!(expr, Expr::TemporalKeyword(TemporalKeyword::Now));
    }

    #[test]
    fn test_operator_precedence_add_mul() {
        let expr = Parser::parse("1 + 2 * 3").unwrap();
        // Should be 1 + (2 * 3), not (1 + 2) * 3
        match expr {
            Expr::BinaryOp {
                op: BinaryOperator::Add,
                right,
                ..
            } => {
                match *right {
                    Expr::BinaryOp {
                        op: BinaryOperator::Mul,
                        ..
                    } => {} // Correct
                    _ => panic!("Expected multiplication to have higher precedence"),
                }
            }
            _ => panic!("Expected addition at top level"),
        }
    }

    #[test]
    fn test_operator_precedence_pow() {
        let expr = Parser::parse("2 ^ 3 ^ 2").unwrap();
        // Power is right-associative: 2 ^ (3 ^ 2)
        match expr {
            Expr::BinaryOp {
                op: BinaryOperator::Pow,
                right,
                ..
            } => {
                match *right {
                    Expr::BinaryOp {
                        op: BinaryOperator::Pow,
                        ..
                    } => {} // Correct (right-associative)
                    _ => panic!("Expected power to be right-associative"),
                }
            }
            _ => panic!("Expected power operation"),
        }
    }

    #[test]
    fn test_unary_not() {
        let expr = Parser::parse("!true").unwrap();
        match expr {
            Expr::UnaryOp {
                op: UnaryOperator::Not,
                ..
            } => {}
            _ => panic!("Expected unary not"),
        }
    }

    #[test]
    fn test_unary_neg() {
        let expr = Parser::parse("-42").unwrap();
        match expr {
            Expr::UnaryOp {
                op: UnaryOperator::Neg,
                ..
            } => {}
            _ => panic!("Expected unary negation"),
        }
    }

    #[test]
    fn test_parse_if_expression() {
        let _expr = Parser::parse("if true then 1 else 0").unwrap();
        // Just check it parses without error
    }

    #[test]
    fn test_parse_temporal_keywords() {
        let expr = Parser::parse("TODAY").unwrap();
        assert_eq!(expr, Expr::TemporalKeyword(TemporalKeyword::Today));
    }

    #[test]
    fn test_error_unexpected_token() {
        let result = Parser::parse("@invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_parentheses() {
        let expr = Parser::parse("(1 + 2) * 3").unwrap();
        match expr {
            Expr::BinaryOp {
                op: BinaryOperator::Mul,
                left,
                ..
            } => {
                match *left {
                    Expr::BinaryOp {
                        op: BinaryOperator::Add,
                        ..
                    } => {} // Correct
                    _ => panic!("Expected addition inside parentheses"),
                }
            }
            _ => panic!("Expected multiplication at top level"),
        }
    }

    #[test]
    fn test_complex_expression() {
        let expr = Parser::parse("user.age >= 18 && isActive").unwrap();
        match expr {
            Expr::BinaryOp {
                op: BinaryOperator::And,
                ..
            } => {}
            _ => panic!("Expected logical AND at top level"),
        }
    }
}
