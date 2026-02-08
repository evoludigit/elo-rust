/// Benchmarks for ELO parsing performance
///
/// These benchmarks measure the performance of the ELO parser
/// across various expression types and complexity levels.
#[cfg(test)]
mod benchmarks {
    use elo_rust::ast::Visitor;
    use elo_rust::codegen::ast_to_code::CodegenVisitor;
    use elo_rust::parser::Parser;

    #[test]
    fn bench_parse_simple_expression() {
        let expr = "age >= 18";
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _ = Parser::parse(expr);
        }
        let elapsed = start.elapsed();
        let per_expr = elapsed.as_micros() as f64 / 1000.0;
        println!("Simple parse: {:.2}µs per expression", per_expr);
    }

    #[test]
    fn bench_parse_field_access() {
        let expr = "user.email == 'john@example.com'";
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _ = Parser::parse(expr);
        }
        let elapsed = start.elapsed();
        let per_expr = elapsed.as_micros() as f64 / 1000.0;
        println!("Field access parse: {:.2}µs per expression", per_expr);
    }

    #[test]
    fn bench_parse_complex_expression() {
        let expr = "(age >= 18 && age < 65) || (is_admin && verified)";
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _ = Parser::parse(expr);
        }
        let elapsed = start.elapsed();
        let per_expr = elapsed.as_micros() as f64 / 1000.0;
        println!("Complex parse: {:.2}µs per expression", per_expr);
    }

    #[test]
    fn bench_codegen_simple() {
        let expr = "age >= 18";
        let ast = Parser::parse(expr).unwrap();
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let mut visitor = CodegenVisitor::new();
            let _ = visitor.visit_expr(&ast);
        }
        let elapsed = start.elapsed();
        let per_expr = elapsed.as_micros() as f64 / 1000.0;
        println!("Simple codegen: {:.2}µs per expression", per_expr);
    }

    #[test]
    fn bench_codegen_complex() {
        let expr = "(age >= 18 && age < 65) || (is_admin && verified)";
        let ast = Parser::parse(expr).unwrap();
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let mut visitor = CodegenVisitor::new();
            let _ = visitor.visit_expr(&ast);
        }
        let elapsed = start.elapsed();
        let per_expr = elapsed.as_micros() as f64 / 1000.0;
        println!("Complex codegen: {:.2}µs per expression", per_expr);
    }

    #[test]
    fn bench_parse_and_codegen() {
        let expr = "length(email) > 5 && contains(email, '@')";
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let ast = Parser::parse(expr).unwrap();
            let mut visitor = CodegenVisitor::new();
            let _ = visitor.visit_expr(&ast);
        }
        let elapsed = start.elapsed();
        let per_expr = elapsed.as_micros() as f64 / 1000.0;
        println!("Parse + codegen: {:.2}µs per expression", per_expr);
    }

    #[test]
    fn bench_string_function() {
        let expr = "matches(email, 'test@example.com')";
        let ast = Parser::parse(expr).unwrap();
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let mut visitor = CodegenVisitor::new();
            let _ = visitor.visit_expr(&ast);
        }
        let elapsed = start.elapsed();
        let per_expr = elapsed.as_micros() as f64 / 1000.0;
        println!("String function codegen: {:.2}µs per expression", per_expr);
    }

    #[test]
    fn bench_temporal_keyword() {
        let expr = "created_date >= TODAY";
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let ast = Parser::parse(expr).unwrap();
            let mut visitor = CodegenVisitor::new();
            let _ = visitor.visit_expr(&ast);
        }
        let elapsed = start.elapsed();
        let per_expr = elapsed.as_micros() as f64 / 1000.0;
        println!("Temporal keyword: {:.2}µs per expression", per_expr);
    }

    #[test]
    fn bench_pipe_expression() {
        let expr = "email |> lowercase() |> trim()";
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let ast = Parser::parse(expr).unwrap();
            let mut visitor = CodegenVisitor::new();
            let _ = visitor.visit_expr(&ast);
        }
        let elapsed = start.elapsed();
        let per_expr = elapsed.as_micros() as f64 / 1000.0;
        println!("Pipe expression: {:.2}µs per expression", per_expr);
    }
}
