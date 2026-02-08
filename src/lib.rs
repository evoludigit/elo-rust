#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    unreachable_pub
)]

//! ELO Rust Code Generation Target
//!
//! This crate provides a Rust code generation backend for the ELO validation language,
//! enabling developers to compile ELO validation expressions directly to type-safe,
//! zero-overhead Rust functions.

pub mod codegen;
pub mod runtime;
pub mod security;
pub mod stdlib;

pub use codegen::RustCodeGenerator;
pub use runtime::{ValidationError, ValidationErrors};

/// Result type for validation operations
pub type ValidationResult<T> = std::result::Result<T, ValidationError>;

/// Result type for validation operations that return multiple errors
pub type ValidationResults<T> = std::result::Result<T, ValidationErrors>;
