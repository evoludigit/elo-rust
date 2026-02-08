//! Example: User validation with Axum
//!
//! This example demonstrates how to use ELO validators with Axum,
//! a modern async web framework.
//!
//! The validator checks:
//! - Email format matches pattern
//! - Age is at least 18
//! - Username is between 3 and 20 characters

use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::fmt;

// ============================================================================
// VALIDATION ERRORS
// ============================================================================

/// Custom validation error type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

/// Collection of validation errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationErrors {
    pub errors: Vec<ValidationError>,
}

impl ValidationErrors {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn add_error(&mut self, field: &str, message: &str) {
        self.errors.push(ValidationError {
            field: field.to_string(),
            message: message.to_string(),
        });
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
}

impl fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.errors
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl Default for ValidationErrors {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoResponse for ValidationErrors {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, Json(self)).into_response()
    }
}

// ============================================================================
// REQUEST/RESPONSE TYPES
// ============================================================================

/// Create user request body
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub age: i32,
}

/// Create user response
#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    pub success: bool,
    pub message: String,
}

// ============================================================================
// VALIDATORS
// ============================================================================

/// Validator for user creation requests
pub struct UserValidator;

impl UserValidator {
    /// Validate a user creation request
    pub fn validate(input: &CreateUserRequest) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        // Validate username: 3-20 characters
        if input.username.len() < 3 {
            errors.add_error("username", "Username must be at least 3 characters");
        }
        if input.username.len() > 20 {
            errors.add_error("username", "Username must be at most 20 characters");
        }

        // Validate email: basic format check
        if !input.email.contains('@') {
            errors.add_error("email", "Email must contain @");
        }
        if !input.email.contains('.') {
            errors.add_error("email", "Email must contain domain");
        }

        // Validate age: must be at least 18
        if input.age < 18 {
            errors.add_error("age", "User must be at least 18 years old");
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

// ============================================================================
// HTTP HANDLERS
// ============================================================================

/// Create user endpoint
///
/// POST /users
/// Content-Type: application/json
///
/// Request body:
/// ```json
/// {
///   "username": "john_doe",
///   "email": "john@example.com",
///   "age": 25
/// }
/// ```
async fn create_user(
    Json(req): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<CreateUserResponse>), ValidationErrors> {
    // Validate the request
    UserValidator::validate(&req)?;

    // Validation passed - user would be created here
    Ok((
        StatusCode::CREATED,
        Json(CreateUserResponse {
            success: true,
            message: format!("User '{}' created successfully", req.username),
        }),
    ))
}

/// Health check endpoint
async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status": "ok"}))
}

// ============================================================================
// SERVER SETUP
// ============================================================================

#[tokio::main]
async fn main() {
    println!("Starting Axum server on http://127.0.0.1:8081");
    println!("Try: curl -X POST http://127.0.0.1:8081/users -H 'Content-Type: application/json' -d '{{\"username\":\"alice\",\"email\":\"alice@example.com\",\"age\":25}}'");

    let app = Router::new()
        .route("/health", axum::routing::get(health))
        .route("/users", post(create_user));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8081")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_user() {
        let user = CreateUserRequest {
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
            age: 25,
        };
        assert!(UserValidator::validate(&user).is_ok());
    }

    #[test]
    fn test_username_too_short() {
        let user = CreateUserRequest {
            username: "ab".to_string(),
            email: "ab@example.com".to_string(),
            age: 25,
        };
        let result = UserValidator::validate(&user);
        assert!(result.is_err());
    }

    #[test]
    fn test_age_too_young() {
        let user = CreateUserRequest {
            username: "bob".to_string(),
            email: "bob@example.com".to_string(),
            age: 16,
        };
        let result = UserValidator::validate(&user);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_email() {
        let user = CreateUserRequest {
            username: "charlie".to_string(),
            email: "invalid-email".to_string(),
            age: 30,
        };
        let result = UserValidator::validate(&user);
        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_validation_errors() {
        let user = CreateUserRequest {
            username: "x".to_string(),
            email: "invalid".to_string(),
            age: 10,
        };
        let result = UserValidator::validate(&user);
        let errors = result.unwrap_err();
        assert!(!errors.errors.is_empty());
    }
}
