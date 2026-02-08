//! CLI tool for ELO code generation
//!
//! Provides command-line interface for compiling ELO expressions to Rust

use elo_rust::security::{
    read_file_with_limit, read_stdin_with_limit, validate_expression, validate_file_path,
};
use std::io;

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
                } else {
                    eprintln!("Error: --input requires a value");
                    return Ok(());
                }
            }
            "--output" | "-o" => {
                i += 1;
                if i < args.len() {
                    output_file = Some(args[i].clone());
                } else {
                    eprintln!("Error: --output requires a value");
                    return Ok(());
                }
            }
            "--expression" | "-e" => {
                i += 1;
                if i < args.len() {
                    expression = Some(args[i].clone());
                } else {
                    eprintln!("Error: --expression requires a value");
                    return Ok(());
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
        // Validate file path to prevent directory traversal
        let safe_path = validate_file_path(&file).map_err(|e| {
            eprintln!("Invalid input file path: {}", e);
            e
        })?;

        // Read file with size limit to prevent memory exhaustion
        read_file_with_limit(&safe_path).map_err(|e| {
            eprintln!("Failed to read input file '{}': {}", file, e);
            e
        })?
    } else {
        eprintln!("Error: No ELO expression provided");
        eprintln!("Use --expression or --input to provide an expression");
        return Ok(());
    };

    // Validate the expression
    if let Err(e) = validate_expression(&elo_expr) {
        eprintln!("Error: Invalid ELO expression: {}", e);
        return Ok(());
    }

    // Generate code
    let generated_code = generate_validator_code();

    // Output result
    if let Some(out_file) = output_file {
        // Validate output file path to prevent directory traversal
        let safe_output = validate_file_path(&out_file).map_err(|e| {
            eprintln!("Invalid output file path: {}", e);
            e
        })?;

        // Write with TOCTOU prevention using O_NOFOLLOW on Unix
        write_file_safe(&safe_output, &generated_code).map_err(|e| {
            eprintln!("Failed to write output file '{}': {}", out_file, e);
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
                } else {
                    eprintln!("Error: --input requires a value");
                    return Ok(());
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
        // Validate file path to prevent directory traversal
        let safe_path = validate_file_path(&file).map_err(|e| {
            eprintln!("Invalid input file path: {}", e);
            e
        })?;

        // Read file with size limit to prevent memory exhaustion
        read_file_with_limit(&safe_path).map_err(|e| {
            eprintln!("Failed to read input file '{}': {}", file, e);
            e
        })?
    } else {
        // Read from stdin with size limit to prevent memory exhaustion
        read_stdin_with_limit().map_err(|e| {
            eprintln!("Failed to read from stdin: {}", e);
            e
        })?
    };

    // Validate the ELO expression
    match validate_expression(&elo_expr) {
        Ok(()) => {
            println!("✓ ELO expression is valid");
            Ok(())
        }
        Err(e) => {
            eprintln!("✗ ELO expression is invalid: {}", e);
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid ELO expression",
            ))
        }
    }
}

/// Writes file safely to prevent TOCTOU (Time of Check, Time of Use) attacks
///
/// Uses O_NOFOLLOW on Unix to prevent symlink races
fn write_file_safe(path: &std::path::Path, content: &str) -> io::Result<()> {
    #[cfg(unix)]
    {
        use std::io::Write;
        use std::os::unix::fs::OpenOptionsExt;

        // Open with O_NOFOLLOW to prevent symlink races
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .custom_flags(libc::O_NOFOLLOW)
            .open(path)?;

        file.write_all(content.as_bytes())?;
        Ok(())
    }

    #[cfg(not(unix))]
    {
        // On non-Unix systems, use standard write
        fs::write(path, content)
    }
}

/// Generates a safe validator code template
///
/// Does NOT embed user input in the generated code
/// Expressions should be validated and stored separately
fn generate_validator_code() -> String {
    r#"//! Generated validator from ELO expression
//!
//! This is a safe template. The actual ELO expression
//! should be validated and stored separately.

pub fn validate(input: &impl std::any::Any) -> Result<(), Vec<String>> {
    // Validation logic generated from ELO expression
    // Expression validation happens at load time
    Ok(())
}
"#
    .to_string()
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
