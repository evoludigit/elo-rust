//! CLI integration tests
//!
//! Tests for the command-line interface functionality

use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Helper function to get the compiled binary path
fn get_binary_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target/debug/elo");
    path
}

/// Helper function to run the elo command
fn run_elo(args: &[&str]) -> std::process::Output {
    Command::new(get_binary_path())
        .args(args)
        .output()
        .expect("Failed to execute elo command")
}

// ============================================================================
// HELP AND VERSION COMMANDS
// ============================================================================

#[test]
fn test_cli_help_command() {
    let output = run_elo(&["help"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ELO Rust Code Generator"));
}

#[test]
fn test_cli_help_flag() {
    let output = run_elo(&["--help"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage"));
}

#[test]
fn test_cli_help_short_flag() {
    let output = run_elo(&["-h"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage") || stdout.contains("ELO"));
}

#[test]
fn test_cli_version_command() {
    let output = run_elo(&["--version"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("0.1.0") || stdout.contains("version"));
}

#[test]
fn test_cli_version_short_flag() {
    let output = run_elo(&["-v"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("0.1.0") || stdout.contains("version"));
}

#[test]
fn test_cli_no_arguments() {
    let output = run_elo(&[]);
    assert!(output.status.code().is_some());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage"));
}

#[test]
fn test_cli_unknown_command() {
    let output = run_elo(&["unknown-command"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown command"));
}

// ============================================================================
// COMPILE COMMAND
// ============================================================================

#[test]
fn test_compile_with_expression() {
    let output = run_elo(&["compile", "--expression", "age >= 18"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("pub fn validate"));
    assert!(stdout.contains("Generated validator"));
}

#[test]
fn test_compile_with_short_expression_flag() {
    let output = run_elo(&["compile", "-e", "age >= 18"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("pub fn validate"));
}

#[test]
fn test_compile_to_file() {
    let output_file = "test_output.rs";

    let output = run_elo(&[
        "compile",
        "--expression",
        "email matches pattern",
        "--output",
        output_file,
    ]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Generated code written"));

    // Verify file was created
    if PathBuf::from(output_file).exists() {
        let contents = fs::read_to_string(output_file).unwrap();
        assert!(contents.contains("pub fn validate"));
        let _ = fs::remove_file(output_file);
    }
}

#[test]
fn test_compile_with_short_output_flag() {
    let output_file = "test_output2.rs";

    let output = run_elo(&["compile", "-e", "verified == true", "-o", output_file]);

    assert!(output.status.success());

    if PathBuf::from(output_file).exists() {
        let _ = fs::remove_file(output_file);
    }
}

#[test]
fn test_compile_from_file() {
    let input_file = "test_input.elo";

    fs::write(input_file, "age >= 18 && verified == true").unwrap();

    let output = run_elo(&["compile", "--input", input_file]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Generated validator"));

    let _ = fs::remove_file(input_file);
}

#[test]
fn test_compile_short_input_flag() {
    let input_file = "test_input2.elo";

    fs::write(input_file, "username.length() >= 3").unwrap();

    let output = run_elo(&["compile", "-i", input_file]);

    assert!(output.status.success());

    let _ = fs::remove_file(input_file);
}

#[test]
fn test_compile_no_expression() {
    let output = run_elo(&["compile"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("No ELO expression"));
}

#[test]
fn test_compile_nonexistent_input_file() {
    let output = run_elo(&["compile", "--input", "nonexistent_file.elo"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Failed to read"));
}

#[test]
fn test_compile_help() {
    let output = run_elo(&["compile", "--help"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("compile"));
}

#[test]
fn test_compile_short_help() {
    let output = run_elo(&["compile", "-h"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("compile") || stdout.contains("Usage"));
}

#[test]
fn test_compile_unknown_argument() {
    let output = run_elo(&["compile", "--unknown-arg", "value"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown argument"));
}

// ============================================================================
// VALIDATE COMMAND
// ============================================================================

#[test]
fn test_validate_from_file() {
    let input_file = "test_validate.elo";

    fs::write(input_file, "age >= 18").unwrap();

    let output = run_elo(&["validate", "--input", input_file]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("valid"));

    let _ = fs::remove_file(input_file);
}

#[test]
fn test_validate_short_input_flag() {
    let input_file = "test_validate2.elo";

    fs::write(input_file, "email contains at-sign").unwrap();

    let output = run_elo(&["validate", "-i", input_file]);

    assert!(output.status.success());

    let _ = fs::remove_file(input_file);
}

#[test]
fn test_validate_help() {
    let output = run_elo(&["validate", "--help"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("validate"));
}

#[test]
fn test_validate_short_help() {
    let output = run_elo(&["validate", "-h"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("validate") || stdout.contains("Usage"));
}

#[test]
fn test_validate_nonexistent_file() {
    let output = run_elo(&["validate", "--input", "/nonexistent/validate.elo"]);
    assert!(!output.status.success());
}

#[test]
fn test_validate_unknown_argument() {
    let output = run_elo(&["validate", "--unknown", "value"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown argument"));
}

// ============================================================================
// COMPLEX EXPRESSION COMPILATION
// ============================================================================

#[test]
fn test_compile_complex_expression() {
    let expr = "age >= 18 && verified == true && !banned";
    let output = run_elo(&["compile", "--expression", expr]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("pub fn validate"));
}

#[test]
fn test_compile_string_functions() {
    let expr = "email matches pattern && username.length() >= 3";
    let output = run_elo(&["compile", "--expression", expr]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("pub fn validate"));
}

#[test]
fn test_compile_array_functions() {
    let expr = "roles.contains(admin) || permissions.any(elevated)";
    let output = run_elo(&["compile", "--expression", expr]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("pub fn validate"));
}
