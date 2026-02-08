//! Code generation module for compiling ELO expressions to Rust
//!
//! This module provides the core code generation engine that transforms ELO AST
//! into idiomatic Rust code via the `quote!` macro.

pub mod errors;
pub mod expressions;
pub mod functions;
pub mod operators;
pub mod types;

pub use errors::CodeGenError;
pub use operators::{BinaryOp, OperatorGenerator, UnaryOp};

use proc_macro2::TokenStream;
use quote::quote;

pub use types::TypeContext;

/// Main code generator for transforming ELO AST to Rust code
///
/// Provides methods for generating Rust code from ELO expressions,
/// including literal values, field access, operators, and more.
///
/// # Example
///
/// ```no_run
/// use elo_rust::RustCodeGenerator;
///
/// let generator = RustCodeGenerator::new();
/// let int_literal = generator.generate_literal_integer(42).unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct RustCodeGenerator {
    /// Type context for resolving custom types
    type_context: TypeContext,
}

impl RustCodeGenerator {
    /// Create a new code generator instance with empty type context
    pub fn new() -> Self {
        Self {
            type_context: TypeContext::new(),
        }
    }

    /// Create a new code generator with a populated type context
    ///
    /// # Arguments
    ///
    /// * `type_context` - Pre-configured type context with custom types
    pub fn with_context(type_context: TypeContext) -> Self {
        Self { type_context }
    }

    /// Check if the generator is in a valid state
    pub fn is_valid(&self) -> bool {
        true
    }

    /// Check if a type is registered in the context
    pub fn has_type(&self, type_name: &str) -> bool {
        self.type_context
            .list_all_type_names()
            .iter()
            .any(|n| n == type_name)
    }

    /// Get the type of a field in a registered type
    pub fn get_field_type(&self, type_name: &str, field_name: &str) -> Option<&types::RustType> {
        self.type_context.get_field_type(type_name, field_name)
    }

    /// Generate function signature for a validator
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the validation function
    /// * `input_type` - The type being validated
    ///
    /// # Returns
    ///
    /// A `TokenStream` representing the function signature
    pub fn generate_function_signature(
        &self,
        name: &str,
        input_type: &str,
    ) -> Result<TokenStream, String> {
        let fn_name = quote::format_ident!("{}", name);
        let input_ident = quote::format_ident!("{}", input_type);

        Ok(quote! {
            pub fn #fn_name(input: &#input_ident) -> Result<(), Vec<String>>
        })
    }

    /// Generate code for an integer literal
    pub fn generate_literal_integer(&self, value: i64) -> Result<TokenStream, String> {
        Ok(quote! {
            #value
        })
    }

    /// Generate code for a string literal
    pub fn generate_literal_string(&self, value: &str) -> Result<TokenStream, String> {
        Ok(quote! {
            #value
        })
    }

    /// Generate code for a boolean literal
    pub fn generate_literal_bool(&self, value: bool) -> Result<TokenStream, String> {
        Ok(quote! {
            #value
        })
    }

    /// Generate code for field access (e.g., user.age)
    ///
    /// This generates the Rust code for accessing a field on a value.
    /// The receiver should be a valid Rust identifier (e.g., "user", "input").
    ///
    /// # Arguments
    ///
    /// * `receiver` - The expression being accessed (e.g., "user")
    /// * `field` - The field name (e.g., "age")
    ///
    /// # Returns
    ///
    /// A `TokenStream` representing `receiver.field`
    ///
    /// # Example
    ///
    /// ```ignore
    /// let gen = RustCodeGenerator::new();
    /// let tokens = gen.generate_field_access("user", "age")?;
    /// // Generates: user.age
    /// ```
    pub fn generate_field_access(
        &self,
        receiver: &str,
        field: &str,
    ) -> Result<TokenStream, String> {
        let receiver_ident = quote::format_ident!("{}", receiver);
        let field_ident = quote::format_ident!("{}", field);

        Ok(quote! {
            #receiver_ident.#field_ident
        })
    }

    /// Generate a single-line comment
    ///
    /// Note: Comments are handled at the token manipulation level, not in token streams.
    /// This method is provided for future extensibility.
    pub fn generate_comment(&self, _text: &str) -> Result<TokenStream, String> {
        // Comments are handled at the token level
        // For now, just return empty - comments will be added via token manipulation
        Ok(quote! {})
    }

    /// Generate a documentation comment
    ///
    /// Note: Doc comments are handled at the token manipulation level, not in token streams.
    /// This method is provided for future extensibility.
    pub fn generate_doc_comment(&self, _text: &str) -> Result<TokenStream, String> {
        // Doc comments are handled at the token level
        // For now, just return empty - doc comments will be added via token manipulation
        Ok(quote! {})
    }

    /// Generate a complete validator function from an ELO expression
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the validator function
    /// * `elo_expr` - The ELO validation expression
    /// * `input_type` - The type being validated
    ///
    /// # Returns
    ///
    /// A `TokenStream` representing the complete validator function
    pub fn generate_validator(
        &self,
        name: &str,
        elo_expr: &str,
        input_type: &str,
    ) -> Result<TokenStream, String> {
        let fn_name = quote::format_ident!("{}", name);
        let input_ident = quote::format_ident!("{}", input_type);

        // For now, generate a basic validator structure
        // In a full implementation, this would parse the ELO expression
        // and generate appropriate validation code
        Ok(quote! {
            pub fn #fn_name(input: &#input_ident) -> Result<(), Vec<String>> {
                // Validation logic generated from: #elo_expr
                Ok(())
            }
        })
    }

    /// Generate validator implementation for a type
    ///
    /// # Arguments
    ///
    /// * `struct_name` - The name of the struct implementing the validator
    /// * `validator_fn_name` - The name of the validation function
    /// * `input_type` - The type being validated
    ///
    /// # Returns
    ///
    /// A `TokenStream` representing the impl block
    pub fn generate_validator_impl(
        &self,
        struct_name: &str,
        validator_fn_name: &str,
        input_type: &str,
    ) -> Result<TokenStream, String> {
        let struct_ident = quote::format_ident!("{}", struct_name);
        let fn_ident = quote::format_ident!("{}", validator_fn_name);
        let input_ident = quote::format_ident!("{}", input_type);

        Ok(quote! {
            impl #struct_ident {
                pub fn #fn_ident(input: &#input_ident) -> Result<(), Vec<String>> {
                    Ok(())
                }
            }
        })
    }
}

impl Default for RustCodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}
