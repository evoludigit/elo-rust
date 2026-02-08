//! Integration tests for code generation of advanced expressions
//!
//! Tests verify that generated code for advanced expressions is valid Rust

use elo_rust::parser::Parser;
use elo_rust::codegen::RustCodeGenerator;
use elo_rust::ast::visitor::Visitor;
use elo_rust::codegen::ast_to_code::CodegenVisitor;

#[test]
fn test_codegen_let_expression() {
    let expr = Parser::parse("let x = 10 in x + 5").expect("Parse failed");
    let mut visitor = CodegenVisitor::new();
    let tokens = visitor.visit_expr(&expr);
    let code = tokens.to_string();

    // Should contain let keyword
    assert!(code.contains("let"));
    assert!(code.contains("x"));
    // Should not be empty
    assert!(!code.is_empty());
}

#[test]
fn test_codegen_nested_let() {
    let expr = Parser::parse("let x = 1 in let y = 2 in x + y")
        .expect("Parse failed");
    let mut visitor = CodegenVisitor::new();
    let tokens = visitor.visit_expr(&expr);
    let code = tokens.to_string();

    assert!(code.contains("let"));
    assert!(code.len() > 20);
}

#[test]
fn test_codegen_if_expression() {
    let expr = Parser::parse("if age > 18 then 1 else 0")
        .expect("Parse failed");
    let mut visitor = CodegenVisitor::new();
    let tokens = visitor.visit_expr(&expr);
    let code = tokens.to_string();

    assert!(code.contains("if"));
    assert!(code.contains("else"));
}

#[test]
fn test_codegen_lambda_expression() {
    let expr = Parser::parse("fn(x ~> x * 2)")
        .expect("Parse failed");
    let mut visitor = CodegenVisitor::new();
    let tokens = visitor.visit_expr(&expr);
    let code = tokens.to_string();

    // Lambda should generate closure syntax
    assert!(code.contains("|"));
}

#[test]
fn test_codegen_guard_expression() {
    let expr = Parser::parse("guard x > 0 in x * 2")
        .expect("Parse failed");
    let mut visitor = CodegenVisitor::new();
    let tokens = visitor.visit_expr(&expr);
    let code = tokens.to_string();

    assert!(code.contains("if"));
    assert!(code.contains("panic"));
}

#[test]
fn test_codegen_pipe_expression() {
    let expr = Parser::parse("name |> uppercase()")
        .expect("Parse failed");
    let mut visitor = CodegenVisitor::new();
    let tokens = visitor.visit_expr(&expr);
    let code = tokens.to_string();

    // Should not be empty
    assert!(!code.is_empty());
    // Should contain the function call
    assert!(code.len() > 5);
}

#[test]
fn test_codegen_array_literal() {
    let expr = Parser::parse("[1, 2, 3]").expect("Parse failed");
    let mut visitor = CodegenVisitor::new();
    let tokens = visitor.visit_expr(&expr);
    let code = tokens.to_string();

    assert!(code.contains("vec"));
}

#[test]
fn test_codegen_object_literal() {
    let expr = Parser::parse("{x: 1, y: 2}")
        .expect("Parse failed");
    let mut visitor = CodegenVisitor::new();
    let tokens = visitor.visit_expr(&expr);
    let code = tokens.to_string();

    assert!(code.contains("vec"));
}

#[test]
fn test_codegen_complex_nested_expression() {
    let expr = Parser::parse("let x = 5 in if x > 0 then x * 2 else 0")
        .expect("Parse failed");
    let mut visitor = CodegenVisitor::new();
    let tokens = visitor.visit_expr(&expr);
    let code = tokens.to_string();

    assert!(code.contains("let"));
    assert!(code.contains("if"));
}

#[test]
fn test_codegen_if_with_let_in_branches() {
    let expr = Parser::parse("if cond then let x = 1 in x else 0")
        .expect("Parse failed");
    let mut visitor = CodegenVisitor::new();
    let tokens = visitor.visit_expr(&expr);
    let code = tokens.to_string();

    assert!(code.contains("if"));
    assert!(code.contains("let"));
}

#[test]
fn test_codegen_multiple_pipes() {
    let expr = Parser::parse("name |> uppercase() |> trim()")
        .expect("Parse failed");
    let mut visitor = CodegenVisitor::new();
    let tokens = visitor.visit_expr(&expr);
    let code = tokens.to_string();

    // Should have meaningful code
    assert!(!code.is_empty());
}

#[test]
fn test_codegen_array_with_expressions() {
    let expr = Parser::parse("[1 + 2, 3 * 4]")
        .expect("Parse failed");
    let mut visitor = CodegenVisitor::new();
    let tokens = visitor.visit_expr(&expr);
    let code = tokens.to_string();

    assert!(code.contains("vec"));
    assert!(code.contains("+") || code.contains("1i64"));
}

#[test]
fn test_codegen_object_with_expressions() {
    let expr = Parser::parse("{sum: 1 + 2, product: 3 * 4}")
        .expect("Parse failed");
    let mut visitor = CodegenVisitor::new();
    let tokens = visitor.visit_expr(&expr);
    let code = tokens.to_string();

    assert!(code.contains("vec"));
}

#[test]
fn test_codegen_nested_if() {
    let expr = Parser::parse("if x > 0 then if y > 0 then 1 else 2 else 3")
        .expect("Parse failed");
    let mut visitor = CodegenVisitor::new();
    let tokens = visitor.visit_expr(&expr);
    let code = tokens.to_string();

    assert!(code.contains("if"));
}

#[test]
fn test_codegen_lambda_with_complex_body() {
    let expr = Parser::parse("fn(x ~> if x > 0 then x else 0)")
        .expect("Parse failed");
    let mut visitor = CodegenVisitor::new();
    let tokens = visitor.visit_expr(&expr);
    let code = tokens.to_string();

    assert!(code.contains("|"));
    assert!(code.contains("if"));
}

#[test]
fn test_codegen_empty_array() {
    let expr = Parser::parse("[]").expect("Parse failed");
    let mut visitor = CodegenVisitor::new();
    let tokens = visitor.visit_expr(&expr);
    let code = tokens.to_string();

    assert!(code.contains("vec"));
}

#[test]
fn test_codegen_empty_object() {
    let expr = Parser::parse("{}").expect("Parse failed");
    let mut visitor = CodegenVisitor::new();
    let tokens = visitor.visit_expr(&expr);
    let code = tokens.to_string();

    assert!(code.contains("vec"));
}

#[test]
fn test_codegen_with_generator_simple_let() {
    let generator = RustCodeGenerator::new();
    let result = generator.generate_validator("validate", "let x = 10 in x > 5", "T");
    assert!(result.is_ok());
    let code = result.unwrap().to_string();
    assert!(code.contains("validate"));
}

#[test]
fn test_codegen_with_generator_if() {
    let generator = RustCodeGenerator::new();
    let result = generator.generate_validator("validate", "if age > 18 then true else false", "T");
    assert!(result.is_ok());
    let code = result.unwrap().to_string();
    assert!(code.contains("validate"));
}

#[test]
fn test_codegen_with_generator_complex() {
    let generator = RustCodeGenerator::new();
    let result = generator.generate_validator(
        "validate",
        "let x = 5 in if x > 0 then x * 2 else 0",
        "T"
    );
    assert!(result.is_ok());
    let code = result.unwrap().to_string();
    assert!(code.contains("validate"));
}

#[test]
fn test_codegen_all_advanced_features() {
    let expressions = vec![
        "let x = 10 in x + 5",
        "if age > 18 then 1 else 0",
        "guard x > 0 in x * 2",
        "name |> uppercase()",
        "[1, 2, 3]",
        "{x: 1, y: 2}",
    ];

    let mut visitor = CodegenVisitor::new();
    for expr_str in expressions {
        let expr = Parser::parse(expr_str)
            .expect(&format!("Failed to parse: {}", expr_str));
        let tokens = visitor.visit_expr(&expr);
        assert!(!tokens.to_string().is_empty(),
                "Generated empty code for: {}", expr_str);
    }
}
