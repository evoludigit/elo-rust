//! Parse error types

use std::fmt;

/// Parse error with location information and optional source context
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    /// Error message
    pub message: String,
    /// Line number (1-based)
    pub line: usize,
    /// Column number (1-based)
    pub column: usize,
    /// Optional source context showing the problematic line
    pub context: Option<String>,
}

impl ParseError {
    /// Create a new parse error
    pub fn new(message: impl Into<String>, line: usize, column: usize) -> Self {
        ParseError {
            message: message.into(),
            line,
            column,
            context: None,
        }
    }

    /// Create a parse error with context from an input string
    pub fn with_context(message: impl Into<String>, input: &str, position: usize) -> Self {
        let (line, column) = Self::position_to_line_col(input, position);
        let context = Self::extract_context(input, line, column);
        ParseError {
            message: message.into(),
            line,
            column,
            context,
        }
    }

    /// Create a parse error with explicit context
    pub fn with_explicit_context(
        message: impl Into<String>,
        line: usize,
        column: usize,
        context: impl Into<String>,
    ) -> Self {
        ParseError {
            message: message.into(),
            line,
            column,
            context: Some(context.into()),
        }
    }

    /// Convert a string position to line and column numbers
    fn position_to_line_col(input: &str, position: usize) -> (usize, usize) {
        let mut line = 1;
        let mut column = 1;

        for (idx, ch) in input.chars().enumerate() {
            if idx >= position {
                break;
            }
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }

        (line, column)
    }

    /// Extract source context for an error (the problematic line with a caret pointer)
    fn extract_context(input: &str, line: usize, column: usize) -> Option<String> {
        let lines: Vec<&str> = input.lines().collect();
        if line == 0 || line > lines.len() {
            return None;
        }

        let error_line = lines[line - 1];
        let pointer = " ".repeat(column.saturating_sub(1)) + "^";

        Some(format!(
            "  {} |\n  {} | {}\n  {} | {}",
            line, "|", error_line, "|", pointer
        ))
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Parse error at line {}, column {}: {}",
            self.line, self.column, self.message
        )?;

        if let Some(context) = &self.context {
            write!(f, "\n{}", context)?;
        }

        Ok(())
    }
}

impl std::error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error_creation() {
        let err = ParseError::new("test error", 1, 5);
        assert_eq!(err.message, "test error");
        assert_eq!(err.line, 1);
        assert_eq!(err.column, 5);
        assert_eq!(err.context, None);
    }

    #[test]
    fn test_parse_error_display() {
        let err = ParseError::new("unexpected token", 2, 10);
        assert_eq!(
            err.to_string(),
            "Parse error at line 2, column 10: unexpected token"
        );
    }

    #[test]
    fn test_parse_error_with_context() {
        let input = "age >= 18";
        let err = ParseError::with_context("unexpected character", input, 5);
        assert_eq!(err.line, 1);
        assert_eq!(err.column, 6);
        assert!(err.context.is_some());
        let context = err.context.unwrap();
        assert!(context.contains("age >= 18"));
        assert!(context.contains("^"));
    }

    #[test]
    fn test_parse_error_with_explicit_context() {
        let err =
            ParseError::with_explicit_context("invalid syntax", 1, 3, "  | age >= 18\n  | ^^");
        assert_eq!(err.message, "invalid syntax");
        assert!(err.context.is_some());
    }

    #[test]
    fn test_parse_error_multiline_context() {
        let input = "let x = 1\nin y = 2";
        let err = ParseError::with_context("unexpected token", input, 15);
        assert!(err.context.is_some());
    }

    #[test]
    fn test_position_to_line_col_first_line() {
        let input = "hello world";
        let (line, col) = ParseError::position_to_line_col(input, 5);
        assert_eq!(line, 1);
        assert_eq!(col, 6);
    }

    #[test]
    fn test_position_to_line_col_multiple_lines() {
        let input = "hello\nworld\ntest";
        let (line, col) = ParseError::position_to_line_col(input, 12);
        assert_eq!(line, 3);
        assert_eq!(col, 1);
    }

    #[test]
    fn test_extract_context_basic() {
        let input = "age >= 18";
        let context = ParseError::extract_context(input, 1, 5);
        assert!(context.is_some());
        let ctx = context.unwrap();
        assert!(ctx.contains("age >= 18"));
        assert!(ctx.contains("^"));
    }

    #[test]
    fn test_extract_context_multiple_lines() {
        let input = "age >= 18\nname == 'John'";
        let context = ParseError::extract_context(input, 2, 8);
        assert!(context.is_some());
        let ctx = context.unwrap();
        assert!(ctx.contains("name == 'John'"));
    }

    #[test]
    fn test_extract_context_invalid_line() {
        let input = "age >= 18";
        let context = ParseError::extract_context(input, 10, 1);
        assert!(context.is_none());
    }
}
