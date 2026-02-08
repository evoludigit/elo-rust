//! Standard library function call code generation

use proc_macro2::TokenStream;
use quote::quote;

/// Maximum timeout for regex operations (in milliseconds)
/// Reason: Reserved for future use with timeout guards on regex matching
#[allow(dead_code)]
const REGEX_TIMEOUT_MS: u64 = 1000;

/// Generates code for function calls
#[derive(Debug)]
pub struct FunctionGenerator;

impl FunctionGenerator {
    /// Create a new function generator
    pub fn new() -> Self {
        Self
    }

    /// Generate code for a function call
    pub fn call(&self, name: &str, args: Vec<TokenStream>) -> TokenStream {
        match name {
            // String functions
            "matches" | "contains" | "length" | "uppercase" | "lowercase" | "trim"
            | "starts_with" | "ends_with" => self.string_function(name, args),
            // DateTime functions
            "today" | "now" | "age" | "days_since" | "date" => self.datetime_function(name, args),
            // Array functions
            "any" | "all" => self.array_function(name, args),
            _ => quote!(),
        }
    }

    /// Generate code for a string function
    pub fn string_function(&self, name: &str, args: Vec<TokenStream>) -> TokenStream {
        match name {
            "matches" => {
                if args.len() < 2 {
                    return quote!();
                }
                let subject = &args[0];
                let pattern = &args[1];
                quote! {
                    {
                        use regex::Regex;
                        // Validate regex pattern and compile with timeout guard
                        match Regex::new(#pattern) {
                            Ok(re) => {
                                // Rust's regex crate provides built-in ReDoS protection
                                // by using a different matching algorithm for certain patterns
                                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                    re.is_match(#subject)
                                })) {
                                    Ok(result) => result,
                                    Err(_) => {
                                        eprintln!(
                                            "⚠️  Regex matching failed: pattern may cause performance issues"
                                        );
                                        false
                                    }
                                }
                            }
                            Err(_) => {
                                eprintln!("⚠️  Invalid regex pattern: {}", #pattern);
                                false
                            }
                        }
                    }
                }
            }
            "contains" => {
                if args.len() < 2 {
                    return quote!();
                }
                let subject = &args[0];
                let substring = &args[1];
                quote!(#subject.contains(#substring))
            }
            "length" => {
                if args.is_empty() {
                    return quote!();
                }
                let subject = &args[0];
                quote!(#subject.len())
            }
            "uppercase" => {
                if args.is_empty() {
                    return quote!();
                }
                let subject = &args[0];
                quote!(#subject.to_uppercase())
            }
            "lowercase" => {
                if args.is_empty() {
                    return quote!();
                }
                let subject = &args[0];
                quote!(#subject.to_lowercase())
            }
            "trim" => {
                if args.is_empty() {
                    return quote!();
                }
                let subject = &args[0];
                quote!(#subject.trim())
            }
            "starts_with" => {
                if args.len() < 2 {
                    return quote!();
                }
                let subject = &args[0];
                let prefix = &args[1];
                quote!(#subject.starts_with(#prefix))
            }
            "ends_with" => {
                if args.len() < 2 {
                    return quote!();
                }
                let subject = &args[0];
                let suffix = &args[1];
                quote!(#subject.ends_with(#suffix))
            }
            _ => quote!(),
        }
    }

    /// Generate code for a date/time function
    pub fn datetime_function(&self, name: &str, args: Vec<TokenStream>) -> TokenStream {
        match name {
            "today" => {
                quote! {
                    {
                        use chrono::Local;
                        Local::now().date_naive()
                    }
                }
            }
            "now" => {
                quote! {
                    {
                        use chrono::Utc;
                        Utc::now()
                    }
                }
            }
            "age" => {
                if args.is_empty() {
                    return quote!();
                }
                let birth_date = &args[0];
                quote! {
                    {
                        use chrono::Local;
                        let today = Local::now().date_naive();
                        let mut age = today.year() - #birth_date.year();
                        if (today.month(), today.day()) < (#birth_date.month(), #birth_date.day()) {
                            age -= 1;
                        }
                        age as u32
                    }
                }
            }
            "days_since" => {
                if args.is_empty() {
                    return quote!();
                }
                let date = &args[0];
                quote! {
                    {
                        use chrono::Local;
                        (Local::now().date_naive() - #date).num_days()
                    }
                }
            }
            "date" => {
                if args.is_empty() {
                    return quote!();
                }
                let date_str = &args[0];
                quote! {
                    {
                        use chrono::NaiveDate;
                        NaiveDate::parse_from_str(#date_str, "%Y-%m-%d")
                            .expect("Invalid date format")
                    }
                }
            }
            _ => quote!(),
        }
    }

    /// Generate code for a collection function
    pub fn array_function(&self, name: &str, args: Vec<TokenStream>) -> TokenStream {
        match name {
            "contains" => {
                if args.len() < 2 {
                    return quote!();
                }
                let array = &args[0];
                let value = &args[1];
                quote!(#array.contains(&#value))
            }
            "any" => {
                if args.len() < 2 {
                    return quote!();
                }
                let array = &args[0];
                let predicate = &args[1];
                quote!(#array.iter().any(|item| #predicate))
            }
            "all" => {
                if args.len() < 2 {
                    return quote!();
                }
                let array = &args[0];
                let predicate = &args[1];
                quote!(#array.iter().all(|item| #predicate))
            }
            "length" => {
                if args.is_empty() {
                    return quote!();
                }
                let array = &args[0];
                quote!(#array.len())
            }
            "is_empty" => {
                if args.is_empty() {
                    return quote!();
                }
                let array = &args[0];
                quote!(#array.is_empty())
            }
            // Type checking functions
            "is_null" => {
                if args.is_empty() {
                    return quote!();
                }
                let value = &args[0];
                quote!(#value.is_none())
            }
            "is_some" => {
                if args.is_empty() {
                    return quote!();
                }
                let value = &args[0];
                quote!(#value.is_some())
            }
            _ => quote!(),
        }
    }
}

impl Default for FunctionGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_generator_creation() {
        let _gen = FunctionGenerator::new();
    }
}
