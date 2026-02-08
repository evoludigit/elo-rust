//! Comprehensive error handling tests
//!
//! Tests for parse errors and error context display

#[cfg(test)]
mod error_tests {
    use elo_rust::parser::error::ParseError;
    use elo_rust::parser::Parser;

    #[test]
    fn test_parse_error_with_location() {
        let expr = "age >=";
        let err = Parser::parse(expr);
        assert!(err.is_err());
    }

    #[test]
    fn test_valid_expression() {
        let expr = "age >= 18";
        let result = Parser::parse(expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_unbalanced_parentheses_open() {
        let expr = "(age >= 18";
        let err = Parser::parse(expr);
        assert!(err.is_err());
    }

    #[test]
    fn test_empty_function_call() {
        let expr = "()";
        let err = Parser::parse(expr);
        assert!(err.is_err());
    }

    #[test]
    fn test_missing_operand() {
        let expr = "age >=";
        let err = Parser::parse(expr);
        assert!(err.is_err());
    }

    #[test]
    fn test_parse_error_context_first_line() {
        let input = "age >= 18";
        let err = ParseError::with_context("test error", input, 5);
        assert_eq!(err.line, 1);
        assert!(err.context.is_some());
        let context = err.context.unwrap();
        assert!(context.contains("age >= 18"));
    }

    #[test]
    fn test_parse_error_context_multiline() {
        let input = "name == 'John'\nage >= 18";
        let err = ParseError::with_context("test error", input, 20);
        assert_eq!(err.line, 2);
        assert!(err.context.is_some());
    }

    #[test]
    fn test_parse_error_display_format() {
        let err = ParseError::new("unexpected token", 1, 5);
        let display = err.to_string();
        assert!(display.contains("Parse error"));
        assert!(display.contains("line 1"));
        assert!(display.contains("column 5"));
    }

    #[test]
    fn test_parse_error_display_with_context() {
        let input = "age >= 18";
        let err = ParseError::with_context("error occurred", input, 0);
        let display = err.to_string();
        assert!(display.contains("Parse error"));
        assert!(display.contains("age >= 18"));
    }

    #[test]
    fn test_nested_function_calls() {
        let expr = "length(uppercase(name))";
        let result = Parser::parse(expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_valid_lambda_syntax() {
        let expr = "x ~> x * 2";
        let result = Parser::parse(expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_let_syntax() {
        let expr = "let x =";
        let err = Parser::parse(expr);
        assert!(err.is_err());
    }

    #[test]
    fn test_valid_let_expression() {
        let expr = "let x = 5 in x + 1";
        let result = Parser::parse(expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_incomplete_if_expression() {
        let expr = "if age >= 18";
        let err = Parser::parse(expr);
        assert!(err.is_err());
    }

    #[test]
    fn test_valid_if_expression() {
        let expr = "if age >= 18 then 'adult' else 'minor'";
        let result = Parser::parse(expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_pipe_syntax() {
        let expr = "email |>";
        let err = Parser::parse(expr);
        assert!(err.is_err());
    }

    #[test]
    fn test_valid_pipe_syntax() {
        let expr = "email |> uppercase()";
        let result = Parser::parse(expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_error_column_accuracy() {
        let input = "age >= 18";
        let err = ParseError::with_context("unexpected", input, 5);
        assert_eq!(err.column, 6);
    }

    #[test]
    fn test_parse_error_explicit_context() {
        let err = ParseError::with_explicit_context("test error", 1, 5, "test context");
        assert!(err.context.is_some());
        assert_eq!(err.message, "test error");
    }

    #[test]
    fn test_parse_error_clone() {
        let err1 = ParseError::new("test", 1, 5);
        let err2 = err1.clone();
        assert_eq!(err1, err2);
    }

    #[test]
    fn test_parse_error_equality() {
        let err1 = ParseError::new("same message", 1, 5);
        let err2 = ParseError::new("same message", 1, 5);
        assert_eq!(err1, err2);
    }

    #[test]
    fn test_parse_error_inequality() {
        let err1 = ParseError::new("message1", 1, 5);
        let err2 = ParseError::new("message2", 1, 5);
        assert_ne!(err1, err2);
    }

    #[test]
    fn test_parse_error_different_location() {
        let err1 = ParseError::new("same message", 1, 5);
        let err2 = ParseError::new("same message", 2, 10);
        assert_ne!(err1, err2);
    }

    #[test]
    fn test_multiple_errors_in_context() {
        let input = "age >= 18\nname ==\nemail > 5";
        let err = ParseError::with_context("line 2 error", input, 11);
        assert_eq!(err.line, 2);
    }

    #[test]
    fn test_error_at_end_of_input() {
        let input = "age >=";
        let err = ParseError::with_context("end of input", input, 6);
        assert!(err.context.is_some());
    }

    #[test]
    fn test_error_position_to_line_col() {
        let input = "first line\nsecond line\nthird line";
        let err = ParseError::with_context("test", input, 22);
        assert_eq!(err.line, 2);
    }
}
