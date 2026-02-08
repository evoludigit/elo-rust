//! Lexer for tokenizing ELO expressions
//!
//! Converts a string of ELO code into a stream of tokens.
//! Handles all ELO token types including literals, operators, keywords, and punctuation.

use std::fmt;

/// A single token in an ELO expression
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    /// Integer literal (e.g., 42, -100)
    Integer(i64),
    /// Float literal (e.g., 3.14, -2.5)
    Float(f64),
    /// String literal (single-quoted, e.g., 'hello')
    String(String),
    /// Boolean true
    True,
    /// Boolean false
    False,
    /// Null literal
    Null,

    // Operators - Arithmetic
    /// Plus operator: +
    Plus,
    /// Minus operator: -
    Minus,
    /// Star operator (multiplication): *
    Star,
    /// Slash operator (division): /
    Slash,
    /// Percent operator (modulo): %
    Percent,
    /// Caret operator (exponentiation): ^
    Caret,

    // Operators - Assignment/Comparison
    /// Single equals operator: =
    Equal,
    /// Equal equal operator: ==
    EqualEqual,
    /// Not equal operator: !=
    NotEqual,
    /// Less than operator: <
    Less,
    /// Less or equal operator: <=
    LessEqual,
    /// Greater than operator: >
    Greater,
    /// Greater or equal operator: >=
    GreaterEqual,

    // Operators - Logical
    /// Logical AND operator: &&
    AndAnd,
    /// Logical OR operator: ||
    OrOr,
    /// Logical NOT operator: !
    Bang,

    // Keywords
    /// let keyword
    Let,
    /// in keyword
    In,
    /// if keyword
    If,
    /// then keyword
    Then,
    /// else keyword
    Else,
    /// fn keyword
    Fn,
    /// guard keyword
    Guard,

    // Temporal keywords
    /// NOW keyword
    Now,
    /// TODAY keyword
    Today,
    /// TOMORROW keyword
    Tomorrow,
    /// YESTERDAY keyword
    Yesterday,
    /// SOD keyword (start of day)
    StartOfDay,
    /// EOD keyword (end of day)
    EndOfDay,
    /// SOW keyword (start of week)
    StartOfWeek,
    /// EOW keyword (end of week)
    EndOfWeek,
    /// SOM keyword (start of month)
    StartOfMonth,
    /// EOM keyword (end of month)
    EndOfMonth,
    /// SOQ keyword (start of quarter)
    StartOfQuarter,
    /// EOQ keyword (end of quarter)
    EndOfQuarter,
    /// SOY keyword (start of year)
    StartOfYear,
    /// EOY keyword (end of year)
    EndOfYear,
    /// BOT keyword (beginning of time)
    BeginningOfTime,
    /// EOT keyword (end of time)
    EndOfTime,

    // Punctuation
    /// Identifier or function name
    Identifier(String),
    /// Dot operator: .
    Dot,
    /// Comma: ,
    Comma,
    /// Left parenthesis: (
    LeftParen,
    /// Right parenthesis: )
    RightParen,
    /// Left bracket: [
    LeftBracket,
    /// Right bracket: ]
    RightBracket,
    /// Left brace: {
    LeftBrace,
    /// Right brace: }
    RightBrace,
    /// Colon: :
    Colon,
    /// Semicolon: ;
    Semicolon,

    // Special operators
    /// Arrow operator: =>
    Arrow,
    /// Pipe operator: |>
    Pipe,
    /// Lambda arrow: ~>
    LambdaArrow,
    /// Alternative operator: ?|
    Alternative,
    /// Pipe union: ||
    // (Note: OrOr handles this dual-purpose token)

    // End of file
    /// End of input
    Eof,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Integer(n) => write!(f, "{}", n),
            Token::Float(x) => write!(f, "{}", x),
            Token::String(s) => write!(f, "'{}'", s),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::Null => write!(f, "null"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Percent => write!(f, "%"),
            Token::Caret => write!(f, "^"),
            Token::Equal => write!(f, "="),
            Token::EqualEqual => write!(f, "=="),
            Token::NotEqual => write!(f, "!="),
            Token::Less => write!(f, "<"),
            Token::LessEqual => write!(f, "<="),
            Token::Greater => write!(f, ">"),
            Token::GreaterEqual => write!(f, ">="),
            Token::AndAnd => write!(f, "&&"),
            Token::OrOr => write!(f, "||"),
            Token::Bang => write!(f, "!"),
            Token::Let => write!(f, "let"),
            Token::In => write!(f, "in"),
            Token::If => write!(f, "if"),
            Token::Then => write!(f, "then"),
            Token::Else => write!(f, "else"),
            Token::Fn => write!(f, "fn"),
            Token::Guard => write!(f, "guard"),
            Token::Now => write!(f, "NOW"),
            Token::Today => write!(f, "TODAY"),
            Token::Tomorrow => write!(f, "TOMORROW"),
            Token::Yesterday => write!(f, "YESTERDAY"),
            Token::StartOfDay => write!(f, "SOD"),
            Token::EndOfDay => write!(f, "EOD"),
            Token::StartOfWeek => write!(f, "SOW"),
            Token::EndOfWeek => write!(f, "EOW"),
            Token::StartOfMonth => write!(f, "SOM"),
            Token::EndOfMonth => write!(f, "EOM"),
            Token::StartOfQuarter => write!(f, "SOQ"),
            Token::EndOfQuarter => write!(f, "EOQ"),
            Token::StartOfYear => write!(f, "SOY"),
            Token::EndOfYear => write!(f, "EOY"),
            Token::BeginningOfTime => write!(f, "BOT"),
            Token::EndOfTime => write!(f, "EOT"),
            Token::Identifier(name) => write!(f, "{}", name),
            Token::Dot => write!(f, "."),
            Token::Comma => write!(f, ","),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftBracket => write!(f, "["),
            Token::RightBracket => write!(f, "]"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::Colon => write!(f, ":"),
            Token::Semicolon => write!(f, ";"),
            Token::Arrow => write!(f, "=>"),
            Token::Pipe => write!(f, "|>"),
            Token::LambdaArrow => write!(f, "~>"),
            Token::Alternative => write!(f, "?|"),
            Token::Eof => write!(f, "EOF"),
        }
    }
}

/// Parse error with location information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexError {
    /// Error message
    pub message: String,
    /// Line number (1-based)
    pub line: usize,
    /// Column number (1-based)
    pub column: usize,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Lex error at line {}, column {}: {}",
            self.line, self.column, self.message
        )
    }
}

impl std::error::Error for LexError {}

/// Lexer for ELO expressions
#[derive(Debug)]
pub struct Lexer<'a> {
    #[allow(dead_code)]
    input: &'a str,
    position: usize,
    line: usize,
    column: usize,
    chars: std::str::Chars<'a>,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer from input string
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            line: 1,
            column: 1,
            chars: input.chars(),
            current_char: None,
        };
        lexer.current_char = lexer.chars.next();
        lexer
    }

    /// Advance to next character
    fn advance(&mut self) {
        if let Some('\n') = self.current_char {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        self.position += 1;
        self.current_char = self.chars.next();
    }

    /// Skip whitespace
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Read a number (integer or float)
    fn read_number(&mut self) -> Result<Token, LexError> {
        let start_line = self.line;
        let start_col = self.column;
        let mut num_str = String::new();
        let mut is_float = false;

        // Read digits (no sign handling at lexer level)
        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() {
                num_str.push(ch);
                self.advance();
            } else if ch == '.' && !is_float {
                // Check if next char is a digit
                is_float = true;
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        if is_float {
            num_str.parse::<f64>()
                .map(Token::Float)
                .map_err(|_| LexError {
                    message: format!("Invalid float: {}", num_str),
                    line: start_line,
                    column: start_col,
                })
        } else {
            num_str.parse::<i64>()
                .map(Token::Integer)
                .map_err(|_| LexError {
                    message: format!("Invalid integer: {}", num_str),
                    line: start_line,
                    column: start_col,
                })
        }
    }

    /// Read a string literal (single-quoted)
    fn read_string(&mut self) -> Result<Token, LexError> {
        let start_line = self.line;
        let start_col = self.column;
        let mut result = String::new();

        // Skip opening quote
        self.advance();

        while let Some(ch) = self.current_char {
            match ch {
                '\'' => {
                    self.advance();
                    return Ok(Token::String(result));
                }
                '\\' => {
                    self.advance();
                    match self.current_char {
                        Some('n') => {
                            result.push('\n');
                            self.advance();
                        }
                        Some('t') => {
                            result.push('\t');
                            self.advance();
                        }
                        Some('r') => {
                            result.push('\r');
                            self.advance();
                        }
                        Some('\\') => {
                            result.push('\\');
                            self.advance();
                        }
                        Some('\'') => {
                            result.push('\'');
                            self.advance();
                        }
                        _ => {
                            return Err(LexError {
                                message: "Invalid escape sequence".to_string(),
                                line: self.line,
                                column: self.column,
                            });
                        }
                    }
                }
                _ => {
                    result.push(ch);
                    self.advance();
                }
            }
        }

        Err(LexError {
            message: "Unterminated string literal".to_string(),
            line: start_line,
            column: start_col,
        })
    }

    /// Read an identifier or keyword
    fn read_identifier(&mut self) -> Token {
        let mut ident = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        match ident.as_str() {
            "let" => Token::Let,
            "in" => Token::In,
            "if" => Token::If,
            "then" => Token::Then,
            "else" => Token::Else,
            "fn" => Token::Fn,
            "guard" => Token::Guard,
            "true" => Token::True,
            "false" => Token::False,
            "null" => Token::Null,
            "NOW" => Token::Now,
            "TODAY" => Token::Today,
            "TOMORROW" => Token::Tomorrow,
            "YESTERDAY" => Token::Yesterday,
            "SOD" => Token::StartOfDay,
            "EOD" => Token::EndOfDay,
            "SOW" => Token::StartOfWeek,
            "EOW" => Token::EndOfWeek,
            "SOM" => Token::StartOfMonth,
            "EOM" => Token::EndOfMonth,
            "SOQ" => Token::StartOfQuarter,
            "EOQ" => Token::EndOfQuarter,
            "SOY" => Token::StartOfYear,
            "EOY" => Token::EndOfYear,
            "BOT" => Token::BeginningOfTime,
            "EOT" => Token::EndOfTime,
            _ => Token::Identifier(ident),
        }
    }

    /// Get next token
    pub fn next_token(&mut self) -> Result<Token, LexError> {
        self.skip_whitespace();

        match self.current_char {
            None => Ok(Token::Eof),
            Some(ch) => {
                match ch {
                    '+' => {
                        self.advance();
                        Ok(Token::Plus)
                    }
                    '-' => {
                        self.advance();
                        // Always treat - as minus operator
                        // Negative numbers are handled via unary operators in the parser
                        Ok(Token::Minus)
                    }
                    '*' => {
                        self.advance();
                        Ok(Token::Star)
                    }
                    '/' => {
                        self.advance();
                        Ok(Token::Slash)
                    }
                    '%' => {
                        self.advance();
                        Ok(Token::Percent)
                    }
                    '^' => {
                        self.advance();
                        Ok(Token::Caret)
                    }
                    '=' => {
                        self.advance();
                        if self.current_char == Some('=') {
                            self.advance();
                            Ok(Token::EqualEqual)
                        } else if self.current_char == Some('>') {
                            self.advance();
                            Ok(Token::Arrow)
                        } else {
                            Ok(Token::Equal)
                        }
                    }
                    '!' => {
                        self.advance();
                        if self.current_char == Some('=') {
                            self.advance();
                            Ok(Token::NotEqual)
                        } else {
                            Ok(Token::Bang)
                        }
                    }
                    '<' => {
                        self.advance();
                        if self.current_char == Some('=') {
                            self.advance();
                            Ok(Token::LessEqual)
                        } else {
                            Ok(Token::Less)
                        }
                    }
                    '>' => {
                        self.advance();
                        if self.current_char == Some('=') {
                            self.advance();
                            Ok(Token::GreaterEqual)
                        } else {
                            Ok(Token::Greater)
                        }
                    }
                    '&' => {
                        self.advance();
                        if self.current_char == Some('&') {
                            self.advance();
                            Ok(Token::AndAnd)
                        } else {
                            Err(LexError {
                                message: "Unexpected '&', did you mean '&&'?".to_string(),
                                line: self.line,
                                column: self.column - 1,
                            })
                        }
                    }
                    '|' => {
                        self.advance();
                        if self.current_char == Some('|') {
                            self.advance();
                            Ok(Token::OrOr)
                        } else if self.current_char == Some('>') {
                            self.advance();
                            Ok(Token::Pipe)
                        } else {
                            Err(LexError {
                                message: "Unexpected '|', did you mean '||' or '|>'?".to_string(),
                                line: self.line,
                                column: self.column - 1,
                            })
                        }
                    }
                    '?' => {
                        self.advance();
                        if self.current_char == Some('|') {
                            self.advance();
                            Ok(Token::Alternative)
                        } else {
                            Err(LexError {
                                message: "Unexpected '?', did you mean '?|'?".to_string(),
                                line: self.line,
                                column: self.column - 1,
                            })
                        }
                    }
                    '~' => {
                        self.advance();
                        if self.current_char == Some('>') {
                            self.advance();
                            Ok(Token::LambdaArrow)
                        } else {
                            Err(LexError {
                                message: "Unexpected '~', did you mean '~>'?".to_string(),
                                line: self.line,
                                column: self.column - 1,
                            })
                        }
                    }
                    '.' => {
                        self.advance();
                        Ok(Token::Dot)
                    }
                    ',' => {
                        self.advance();
                        Ok(Token::Comma)
                    }
                    '(' => {
                        self.advance();
                        Ok(Token::LeftParen)
                    }
                    ')' => {
                        self.advance();
                        Ok(Token::RightParen)
                    }
                    '[' => {
                        self.advance();
                        Ok(Token::LeftBracket)
                    }
                    ']' => {
                        self.advance();
                        Ok(Token::RightBracket)
                    }
                    '{' => {
                        self.advance();
                        Ok(Token::LeftBrace)
                    }
                    '}' => {
                        self.advance();
                        Ok(Token::RightBrace)
                    }
                    ':' => {
                        self.advance();
                        Ok(Token::Colon)
                    }
                    ';' => {
                        self.advance();
                        Ok(Token::Semicolon)
                    }
                    '\'' => self.read_string(),
                    _ if ch.is_ascii_digit() => self.read_number(),
                    _ if ch.is_alphabetic() => Ok(self.read_identifier()),
                    _ => Err(LexError {
                        message: format!("Unexpected character: '{}'", ch),
                        line: self.line,
                        column: self.column,
                    }),
                }
            }
        }
    }

    /// Tokenize entire input into a vector of tokens
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token()?;
            let is_eof = token == Token::Eof;
            tokens.push(token);
            if is_eof {
                break;
            }
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_literal() {
        let mut lexer = Lexer::new("42");
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Integer(42));
    }

    #[test]
    fn test_float_literal() {
        let mut lexer = Lexer::new("3.14");
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Float(3.14));
    }

    #[test]
    fn test_string_literal() {
        let mut lexer = Lexer::new("'hello'");
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::String("hello".to_string()));
    }

    #[test]
    fn test_boolean_true() {
        let mut lexer = Lexer::new("true");
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::True);
    }

    #[test]
    fn test_boolean_false() {
        let mut lexer = Lexer::new("false");
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::False);
    }

    #[test]
    fn test_keyword_let() {
        let mut lexer = Lexer::new("let");
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Let);
    }

    #[test]
    fn test_keyword_if() {
        let mut lexer = Lexer::new("if");
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::If);
    }

    #[test]
    fn test_identifier() {
        let mut lexer = Lexer::new("myVar");
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Identifier("myVar".to_string()));
    }

    #[test]
    fn test_plus_operator() {
        let mut lexer = Lexer::new("+");
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Plus);
    }

    #[test]
    fn test_equal_equal() {
        let mut lexer = Lexer::new("==");
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::EqualEqual);
    }

    #[test]
    fn test_and_and() {
        let mut lexer = Lexer::new("&&");
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::AndAnd);
    }

    #[test]
    fn test_pipe() {
        let mut lexer = Lexer::new("|>");
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Pipe);
    }

    #[test]
    fn test_lambda_arrow() {
        let mut lexer = Lexer::new("~>");
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::LambdaArrow);
    }

    #[test]
    fn test_tokenize_simple_expr() {
        let mut lexer = Lexer::new("age >= 18");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("age".to_string()),
                Token::GreaterEqual,
                Token::Integer(18),
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_tokenize_function_call() {
        let mut lexer = Lexer::new("length(name)");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("length".to_string()),
                Token::LeftParen,
                Token::Identifier("name".to_string()),
                Token::RightParen,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_whitespace_ignored() {
        let mut lexer = Lexer::new("  42  +  3  ");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            tokens,
            vec![Token::Integer(42), Token::Plus, Token::Integer(3), Token::Eof]
        );
    }

    #[test]
    fn test_temporal_keywords() {
        let mut lexer = Lexer::new("NOW TODAY TOMORROW");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            tokens,
            vec![Token::Now, Token::Today, Token::Tomorrow, Token::Eof]
        );
    }

    #[test]
    fn test_negative_number_lexing() {
        // Negative numbers are represented as minus token followed by number
        let mut lexer = Lexer::new("-42");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Minus, Token::Integer(42), Token::Eof]);
    }

    #[test]
    fn test_minus_operator() {
        let mut lexer = Lexer::new("10 - 5");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Integer(10),
                Token::Minus,
                Token::Integer(5),
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_dot_operator() {
        let mut lexer = Lexer::new("user.age");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("user".to_string()),
                Token::Dot,
                Token::Identifier("age".to_string()),
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_string_escape_sequences() {
        let mut lexer = Lexer::new("'hello\\nworld'");
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::String("hello\nworld".to_string()));
    }

    #[test]
    fn test_lex_error_unterminated_string() {
        let mut lexer = Lexer::new("'hello");
        let result = lexer.next_token();
        assert!(result.is_err());
    }

    #[test]
    fn test_lex_error_invalid_char() {
        let mut lexer = Lexer::new("@#$");
        let result = lexer.next_token();
        assert!(result.is_err());
    }

    #[test]
    fn test_complex_expression() {
        let mut lexer = Lexer::new("let x = 42 in x + 1");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 9); // let, x, =, 42, in, x, +, 1, eof
        assert_eq!(tokens[0], Token::Let);
        assert_eq!(tokens[1], Token::Identifier("x".to_string()));
        assert_eq!(tokens[2], Token::Equal);
        assert_eq!(tokens[3], Token::Integer(42));
        assert_eq!(tokens[4], Token::In);
    }

    #[test]
    fn test_arrow_operator() {
        let mut lexer = Lexer::new("=>");
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Arrow);
    }

    #[test]
    fn test_null_literal() {
        let mut lexer = Lexer::new("null");
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Null);
    }
}
