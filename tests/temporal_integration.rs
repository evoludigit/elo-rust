//! Integration tests for temporal types and operations

#[cfg(test)]
mod temporal_tests {
    use elo_rust::parser::Parser;
    use elo_rust::codegen::ast_to_code::CodegenVisitor;
    use elo_rust::ast::Visitor;

    #[test]
    fn test_parse_today_keyword() {
        let expr_str = "TODAY";
        let ast = Parser::parse(expr_str).expect("Failed to parse TODAY");
        let mut visitor = CodegenVisitor::new();
        let tokens = visitor.visit_expr(&ast);
        let token_str = tokens.to_string();

        assert!(!token_str.is_empty());
        assert!(token_str.contains("Local"));
    }

    #[test]
    fn test_parse_now_keyword() {
        let expr_str = "NOW";
        let ast = Parser::parse(expr_str).expect("Failed to parse NOW");
        let mut visitor = CodegenVisitor::new();
        let tokens = visitor.visit_expr(&ast);
        let token_str = tokens.to_string();

        assert!(!token_str.is_empty());
        assert!(token_str.contains("Utc"));
    }

    #[test]
    fn test_parse_temporal_keyword_tomorrow() {
        let expr_str = "TOMORROW";
        let ast = Parser::parse(expr_str).expect("Failed to parse TOMORROW");
        let mut visitor = CodegenVisitor::new();
        let tokens = visitor.visit_expr(&ast);
        let token_str = tokens.to_string();

        assert!(!token_str.is_empty());
        assert!(token_str.contains("days"));
    }

    #[test]
    fn test_parse_temporal_keyword_yesterday() {
        let expr_str = "YESTERDAY";
        let ast = Parser::parse(expr_str).expect("Failed to parse YESTERDAY");
        let mut visitor = CodegenVisitor::new();
        let tokens = visitor.visit_expr(&ast);
        let token_str = tokens.to_string();

        assert!(!token_str.is_empty());
    }

    #[test]
    fn test_parse_date_comparison() {
        let expr_str = "created_date >= TODAY";
        let ast = Parser::parse(expr_str).expect("Failed to parse date comparison");
        let mut visitor = CodegenVisitor::new();
        let tokens = visitor.visit_expr(&ast);
        let token_str = tokens.to_string();

        assert!(!token_str.is_empty());
        assert!(token_str.contains(">="));
    }

    #[test]
    fn test_parse_date_with_field_access() {
        let expr_str = "user.created_at > TODAY";
        let ast = Parser::parse(expr_str).expect("Failed to parse date with field access");
        let mut visitor = CodegenVisitor::new();
        let tokens = visitor.visit_expr(&ast);
        let token_str = tokens.to_string();

        assert!(!token_str.is_empty());
        assert!(token_str.contains("user"));
    }

    #[test]
    fn test_temporal_keyword_start_of_day() {
        let expr_str = "START_OF_DAY";
        let ast = Parser::parse(expr_str).expect("Failed to parse START_OF_DAY");
        let mut visitor = CodegenVisitor::new();
        let tokens = visitor.visit_expr(&ast);
        let token_str = tokens.to_string();

        assert!(!token_str.is_empty());
    }

    #[test]
    fn test_temporal_keyword_end_of_day() {
        let expr_str = "END_OF_DAY";
        let ast = Parser::parse(expr_str).expect("Failed to parse END_OF_DAY");
        let mut visitor = CodegenVisitor::new();
        let tokens = visitor.visit_expr(&ast);
        let token_str = tokens.to_string();

        assert!(!token_str.is_empty());
    }

    #[test]
    fn test_temporal_keyword_start_of_month() {
        let expr_str = "START_OF_MONTH";
        let ast = Parser::parse(expr_str).expect("Failed to parse START_OF_MONTH");
        let mut visitor = CodegenVisitor::new();
        let tokens = visitor.visit_expr(&ast);
        let token_str = tokens.to_string();

        assert!(!token_str.is_empty());
    }

    #[test]
    fn test_temporal_keyword_end_of_month() {
        let expr_str = "END_OF_MONTH";
        let ast = Parser::parse(expr_str).expect("Failed to parse END_OF_MONTH");
        let mut visitor = CodegenVisitor::new();
        let tokens = visitor.visit_expr(&ast);
        let token_str = tokens.to_string();

        assert!(!token_str.is_empty());
    }

    #[test]
    fn test_temporal_keyword_start_of_year() {
        let expr_str = "START_OF_YEAR";
        let ast = Parser::parse(expr_str).expect("Failed to parse START_OF_YEAR");
        let mut visitor = CodegenVisitor::new();
        let tokens = visitor.visit_expr(&ast);
        let token_str = tokens.to_string();

        assert!(!token_str.is_empty());
    }

    #[test]
    fn test_temporal_keyword_end_of_year() {
        let expr_str = "END_OF_YEAR";
        let ast = Parser::parse(expr_str).expect("Failed to parse END_OF_YEAR");
        let mut visitor = CodegenVisitor::new();
        let tokens = visitor.visit_expr(&ast);
        let token_str = tokens.to_string();

        assert!(!token_str.is_empty());
    }

    #[test]
    fn test_temporal_keyword_beginning_of_time() {
        let expr_str = "BEGINNING_OF_TIME";
        let ast = Parser::parse(expr_str).expect("Failed to parse BEGINNING_OF_TIME");
        let mut visitor = CodegenVisitor::new();
        let tokens = visitor.visit_expr(&ast);
        let token_str = tokens.to_string();

        assert!(!token_str.is_empty());
    }

    #[test]
    fn test_temporal_keyword_end_of_time() {
        let expr_str = "END_OF_TIME";
        let ast = Parser::parse(expr_str).expect("Failed to parse END_OF_TIME");
        let mut visitor = CodegenVisitor::new();
        let tokens = visitor.visit_expr(&ast);
        let token_str = tokens.to_string();

        assert!(!token_str.is_empty());
    }
}
