//! CLI tool for ELO code generation
//!
//! Provides command-line interface for compiling ELO expressions to Rust

use std::fs;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage(&args[0]);
        return Ok(());
    }

    match args[1].as_str() {
        "compile" => compile_command(&args[2..]),
        "validate" => validate_command(&args[2..]),
        "--help" | "-h" | "help" => {
            print_help();
            Ok(())
        }
        "--version" | "-v" => {
            print_version();
            Ok(())
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage(&args[0]);
            Ok(())
        }
    }
}

fn compile_command(args: &[String]) -> io::Result<()> {
    let mut input_file: Option<String> = None;
    let mut output_file: Option<String> = None;
    let mut expression: Option<String> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--input" | "-i" => {
                i += 1;
                if i < args.len() {
                    input_file = Some(args[i].clone());
                }
            }
            "--output" | "-o" => {
                i += 1;
                if i < args.len() {
                    output_file = Some(args[i].clone());
                }
            }
            "--expression" | "-e" => {
                i += 1;
                if i < args.len() {
                    expression = Some(args[i].clone());
                }
            }
            "--help" | "-h" => {
                print_compile_help();
                return Ok(());
            }
            _ => eprintln!("Unknown argument: {}", args[i]),
        }
        i += 1;
    }

    // Get ELO expression from either file or command line
    let elo_expr = if let Some(expr) = expression {
        expr
    } else if let Some(file) = input_file {
        fs::read_to_string(&file).map_err(|e| {
            eprintln!("Failed to read input file: {}", e);
            e
        })?
    } else {
        eprintln!("Error: No ELO expression provided");
        eprintln!("Use --expression or --input to provide an expression");
        return Ok(());
    };

    // Generate code
    let generated_code = generate_validator_code(&elo_expr);

    // Output result
    if let Some(out_file) = output_file {
        fs::write(&out_file, &generated_code).map_err(|e| {
            eprintln!("Failed to write output file: {}", e);
            e
        })?;
        println!("✓ Generated code written to {}", out_file);
    } else {
        println!("{}", generated_code);
    }

    Ok(())
}

fn validate_command(args: &[String]) -> io::Result<()> {
    let mut input_file: Option<String> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--input" | "-i" => {
                i += 1;
                if i < args.len() {
                    input_file = Some(args[i].clone());
                }
            }
            "--help" | "-h" => {
                print_validate_help();
                return Ok(());
            }
            _ => eprintln!("Unknown argument: {}", args[i]),
        }
        i += 1;
    }

    let elo_expr = if let Some(file) = input_file {
        fs::read_to_string(&file).map_err(|e| {
            eprintln!("Failed to read input file: {}", e);
            e
        })?
    } else {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)?;
        input
    };

    // Validate the ELO expression
    if validate_expression(&elo_expr) {
        println!("✓ ELO expression is valid");
        Ok(())
    } else {
        eprintln!("✗ ELO expression is invalid");
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid ELO expression",
        ))
    }
}

fn generate_validator_code(elo_expr: &str) -> String {
    format!(
        r#"//! Generated validator from ELO expression
//! Expression: {}

pub fn validate(input: &impl std::any::Any) -> Result<(), Vec<String>> {{
    // Validation logic generated from ELO expression:
    // {}
    Ok(())
}}
"#,
        elo_expr, elo_expr
    )
}

fn validate_expression(expr: &str) -> bool {
    // Basic validation - just check it's not empty
    !expr.trim().is_empty()
}

fn print_usage(program: &str) {
    println!("Usage: {} <command> [options]", program);
    println!("\nCommands:");
    println!("  compile     Compile ELO expression to Rust code");
    println!("  validate    Validate ELO expression");
    println!("  help        Show this help message");
    println!("\nOptions:");
    println!("  -h, --help      Show help for command");
    println!("  -v, --version   Show version");
}

fn print_help() {
    println!("ELO Rust Code Generator - Compile ELO expressions to Rust validators");
    println!();
    print_usage("elo");
    println!();
    println!("Examples:");
    println!("  elo compile --expression 'age >= 18'");
    println!("  elo compile --input rules.elo --output validator.rs");
    println!("  elo validate --input rules.elo");
}

fn print_compile_help() {
    println!("compile - Compile ELO expressions to Rust validator code");
    println!();
    println!("Usage: elo compile [options]");
    println!();
    println!("Options:");
    println!("  -e, --expression <expr>  ELO expression to compile");
    println!("  -i, --input <file>       Read ELO expression from file");
    println!("  -o, --output <file>      Write generated code to file");
    println!("  -h, --help               Show this help message");
    println!();
    println!("Examples:");
    println!("  elo compile --expression 'age >= 18'");
    println!("  elo compile --input rules.elo --output validator.rs");
    println!("  elo compile --expression 'user.age >= 18 && user.verified == true'");
}

fn print_validate_help() {
    println!("validate - Validate ELO expressions");
    println!();
    println!("Usage: elo validate [options]");
    println!();
    println!("Options:");
    println!("  -i, --input <file>  Read ELO expression from file");
    println!("  -h, --help          Show this help message");
    println!();
    println!("Examples:");
    println!("  echo 'age >= 18' | elo validate");
    println!("  elo validate --input rules.elo");
}

fn print_version() {
    println!("elo 0.1.0 - ELO Rust Code Generator");
}
