# ELO Rust Target: Examples & Use Cases

**Document Version**: 1.0
**Date**: February 8, 2026
**Purpose**: Demonstrate practical applications of ELO Rust code generation

---

## 1. GraphQL Input Validation (FraiseQL)

### Use Case
Validate mutation inputs at framework level before database execution.

### ELO Rule

```elo
# File: validation/user_validation.elo

# Create user validation
create_user_input:
  email matches "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"
  && birthDate >= date("1900-01-01")
  && birthDate < today()
  && age(birthDate) >= 18
  && (role == "user" || (role == "admin" && has_admin_permission))

# Update user validation
update_user_input:
  email matches email_pattern
  && (startDate == null || endDate == null || startDate <= endDate)
```

### Generated Rust Code

```rust
// Generated file: validation.rs
use chrono::{Local, NaiveDate};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub path: String,
    pub message: String,
    pub rule: String,
    pub value: Option<String>,
}

pub fn validate_create_user_input(
    input: &CreateUserInput,
) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    // Email validation
    let email_pattern = Regex::new(
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    ).unwrap();

    if !email_pattern.is_match(&input.email) {
        errors.push(ValidationError {
            path: "email".into(),
            message: "Email must match pattern".into(),
            rule: "email_pattern".into(),
            value: Some(input.email.clone()),
        });
    }

    // Birth date validation
    if input.birth_date < NaiveDate::from_ymd_opt(1900, 1, 1).unwrap() {
        errors.push(ValidationError {
            path: "birthDate".into(),
            message: "Birth date must be >= 1900-01-01".into(),
            rule: "birth_date_min".into(),
            value: None,
        });
    }

    let today = Local::now().date_naive();
    if input.birth_date >= today {
        errors.push(ValidationError {
            path: "birthDate".into(),
            message: "Birth date must be in the past".into(),
            rule: "birth_date_max".into(),
            value: None,
        });
    }

    // Age validation
    let mut age = today.year() - input.birth_date.year();
    if (today.month(), today.day()) < (input.birth_date.month(), input.birth_date.day()) {
        age -= 1;
    }

    if age < 18 {
        errors.push(ValidationError {
            path: "birthDate".into(),
            message: "User must be at least 18 years old".into(),
            rule: "min_age".into(),
            value: None,
        });
    }

    // Role validation
    let role_valid = input.role == "user" ||
        (input.role == "admin" && has_admin_permission());

    if !role_valid {
        errors.push(ValidationError {
            path: "role".into(),
            message: "Invalid role for current user".into(),
            rule: "role_permission".into(),
            value: Some(input.role.clone()),
        });
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub fn validate_update_user_input(
    input: &UpdateUserInput,
) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    // Email validation (if provided)
    if let Some(email) = &input.email {
        let email_pattern = Regex::new(EMAIL_PATTERN).unwrap();
        if !email_pattern.is_match(email) {
            errors.push(ValidationError {
                path: "email".into(),
                message: "Email must be valid".into(),
                rule: "email_pattern".into(),
                value: Some(email.clone()),
            });
        }
    }

    // Date range validation
    if let (Some(start), Some(end)) = (&input.start_date, &input.end_date) {
        if start > end {
            errors.push(ValidationError {
                path: "dateRange".into(),
                message: "Start date must be <= end date".into(),
                rule: "date_range".into(),
                value: None,
            });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
```

### GraphQL Resolver Integration

```rust
// In fraiseql-server/src/resolvers/user.rs
use crate::validation::*;

#[async_trait]
impl UserResolver for GraphQLContext {
    async fn create_user(&self, input: CreateUserInput) -> Result<User, GraphQLError> {
        // LAYER 1: Framework-level validation
        validate_create_user_input(&input)
            .map_err(|errors| GraphQLError::ValidationError(errors))?;

        // LAYER 2: Database execution
        // (Database constraints provide defense-in-depth)
        let user = self.db.create_user(input).await?;

        Ok(user)
    }
}
```

---

## 2. REST API Validation (Actix-web)

### Use Case
Validate request payloads in a REST API endpoint.

### ELO Rules

```elo
# File: validation/order_validation.elo

order_create:
  order.items.length > 0
  && order.items.all(item => item.quantity > 0 && item.price > 0)
  && order.shippingAddress.zipCode matches zip_pattern
  && order.billingAddress.zipCode matches zip_pattern
  && (order.total > 0)

order_update:
  (order.status == "draft" || order.status == "pending")
  && order.status != "cancelled"
```

### Generated Validator

```rust
// Generated file: validators/order_validator.rs

pub struct OrderValidator;

impl OrderValidator {
    pub fn validate_create(order: &CreateOrderRequest) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        // Items length check
        if order.items.is_empty() {
            errors.push(ValidationError {
                path: "items".into(),
                message: "Order must have at least one item".into(),
                rule: "items_required".into(),
                value: None,
            });
        } else {
            // Validate each item
            for (idx, item) in order.items.iter().enumerate() {
                if item.quantity <= 0 {
                    errors.push(ValidationError {
                        path: format!("items[{}].quantity", idx),
                        message: "Quantity must be > 0".into(),
                        rule: "quantity_positive".into(),
                        value: Some(item.quantity.to_string()),
                    });
                }

                if item.price <= 0.0 {
                    errors.push(ValidationError {
                        path: format!("items[{}].price", idx),
                        message: "Price must be > 0".into(),
                        rule: "price_positive".into(),
                        value: Some(item.price.to_string()),
                    });
                }
            }
        }

        // Shipping address zip code
        let zip_pattern = Regex::new(ZIP_PATTERN).unwrap();
        if !zip_pattern.is_match(&order.shipping_address.zip_code) {
            errors.push(ValidationError {
                path: "shippingAddress.zipCode".into(),
                message: "Invalid zip code format".into(),
                rule: "zip_pattern".into(),
                value: Some(order.shipping_address.zip_code.clone()),
            });
        }

        // Billing address zip code
        if !zip_pattern.is_match(&order.billing_address.zip_code) {
            errors.push(ValidationError {
                path: "billingAddress.zipCode".into(),
                message: "Invalid zip code format".into(),
                rule: "zip_pattern".into(),
                value: Some(order.billing_address.zip_code.clone()),
            });
        }

        // Total check
        if order.total <= 0.0 {
            errors.push(ValidationError {
                path: "total".into(),
                message: "Total must be > 0".into(),
                rule: "total_positive".into(),
                value: Some(order.total.to_string()),
            });
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn validate_update(order: &UpdateOrderRequest) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        // Status must be draft or pending
        let valid_statuses = vec!["draft", "pending"];
        if !valid_statuses.contains(&order.status.as_str()) {
            errors.push(ValidationError {
                path: "status".into(),
                message: "Status must be 'draft' or 'pending'".into(),
                rule: "status_value".into(),
                value: Some(order.status.clone()),
            });
        }

        // Cannot change to cancelled
        if order.status == "cancelled" {
            errors.push(ValidationError {
                path: "status".into(),
                message: "Cannot change status to 'cancelled'".into(),
                rule: "status_not_cancelled".into(),
                value: None,
            });
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
```

### Actix Handler

```rust
// In src/handlers/orders.rs
use actix_web::{post, put, web, HttpResponse, Result};
use crate::validators::OrderValidator;

#[post("/orders")]
async fn create_order(
    payload: web::Json<CreateOrderRequest>,
    state: web::Data<AppState>,
) -> Result<HttpResponse> {
    // Validate input
    match OrderValidator::validate_create(&payload) {
        Ok(()) => {
            // Process the order
            let order = state.db.create_order(payload.into_inner()).await?;
            Ok(HttpResponse::Created().json(order))
        }
        Err(errors) => {
            let error_response = serde_json::json!({
                "errors": errors.into_iter().map(|e| serde_json::json!({
                    "path": e.path,
                    "message": e.message,
                })).collect::<Vec<_>>()
            });
            Ok(HttpResponse::BadRequest().json(error_response))
        }
    }
}

#[put("/orders/{id}")]
async fn update_order(
    path: web::Path<u64>,
    payload: web::Json<UpdateOrderRequest>,
    state: web::Data<AppState>,
) -> Result<HttpResponse> {
    let order_id = path.into_inner();

    // Validate input
    match OrderValidator::validate_update(&payload) {
        Ok(()) => {
            let order = state.db.update_order(order_id, payload.into_inner()).await?;
            Ok(HttpResponse::Ok().json(order))
        }
        Err(errors) => {
            Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "errors": errors,
            })))
        }
    }
}
```

---

## 3. Derive Macro Usage

### Simple Example

```rust
// Before: Hand-written validator
pub fn validate_user(user: &User) -> Result<(), Vec<ValidationError>> {
    // ... lots of code ...
}

// After: Using ELO derive macro
#[elo_validator(
    elo = r#"
        user.email matches email_pattern &&
        user.age >= 18
    "#
)]
pub struct UserValidator;

// Usage:
match UserValidator::validate(user) {
    Ok(()) => println!("Valid user"),
    Err(e) => println!("Validation errors: {:?}", e),
}
```

### Complex Example with Custom Types

```rust
#[elo_validator(
    elo = r#"
        payment.amount > 0 &&
        payment.currency in ["USD", "EUR", "GBP"] &&
        payment.method in ["credit_card", "bank_transfer", "paypal"] &&
        (payment.expiryDate == null || payment.expiryDate > today())
    "#,
    error_type = "PaymentError"
)]
pub struct PaymentValidator;

// The macro generates:
impl PaymentValidator {
    pub fn validate(payment: &Payment) -> Result<(), PaymentError> {
        // Generated validation code
    }
}
```

---

## 4. Two-Layer Validation (FraiseQL)

### ELO Source

```elo
# File: validation/user.elo

user_creation:
  email matches email_pattern &&
  age(birthDate) >= 18 &&
  age(birthDate) <= 150 &&
  (role == "user" || (role == "admin" && has_permission("admin"))) &&
  password.length >= 12
```

### Generated Rust (Layer 1: Framework)

```rust
// Runs before database
pub fn validate_user_creation(input: &CreateUserInput) -> Result<()> {
    let email_regex = Regex::new(EMAIL_PATTERN)?;
    if !email_regex.is_match(&input.email) {
        return Err(ValidationError::InvalidEmail.into());
    }

    let age = calculate_age(input.birth_date);
    if age < 18 {
        return Err(ValidationError::UnderageUser.into());
    }

    if age > 150 {
        return Err(ValidationError::InvalidAge.into());
    }

    let role_valid = input.role == "user" ||
        (input.role == "admin" && has_admin_permission());
    if !role_valid {
        return Err(ValidationError::InvalidRole.into());
    }

    if input.password.len() < 12 {
        return Err(ValidationError::WeakPassword.into());
    }

    Ok(())
}
```

### Generated SQL (Layer 2: Database)

```sql
-- Same rules, database level
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email TEXT NOT NULL,
    birth_date DATE NOT NULL,
    role TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    CONSTRAINT valid_email CHECK (email ~ '^[a-zA-Z0-9._%+-]+@...'),
    CONSTRAINT valid_age CHECK (
        EXTRACT(YEAR FROM AGE(birth_date)) >= 18 AND
        EXTRACT(YEAR FROM AGE(birth_date)) <= 150
    ),
    CONSTRAINT valid_role CHECK (role IN ('user', 'admin')),
    CONSTRAINT valid_password CHECK (char_length(password_hash) >= 32)
);
```

### Usage in Resolver

```rust
#[async_trait]
impl Mutation {
    async fn create_user(&self, input: CreateUserInput) -> Result<User> {
        // LAYER 1: Framework validation (fast path)
        validate_user_creation(&input)?;  // <1µs

        // LAYER 2: Database execution
        // If somehow invalid data gets here, database constraints catch it
        let user = self.db.create_user(input).await?;

        Ok(user)
    }
}
```

---

## 5. Batch Processing Validation

### Use Case
Validate large data imports with aggregated error reporting.

### ELO Rules

```elo
# File: validation/import.elo

import_record:
  record.id.length > 0 &&
  record.email matches email_pattern &&
  record.amount > 0 &&
  record.date >= date("2020-01-01")
```

### Generated Validator

```rust
pub fn validate_import_batch(
    records: &[ImportRecord]
) -> Result<Vec<ImportRecord>, Vec<BatchValidationError>> {
    let mut valid_records = Vec::new();
    let mut errors = Vec::new();

    for (idx, record) in records.iter().enumerate() {
        match validate_import_record(record) {
            Ok(()) => valid_records.push(record.clone()),
            Err(field_errors) => {
                for error in field_errors {
                    errors.push(BatchValidationError {
                        record_index: idx,
                        path: error.path,
                        message: error.message,
                    });
                }
            }
        }
    }

    if errors.is_empty() {
        Ok(valid_records)
    } else {
        Err(errors)  // Return all errors at once
    }
}
```

### Tokio Processing

```rust
async fn process_import_file(
    file_path: &str,
) -> Result<usize> {
    let records = read_csv_file(file_path).await?;

    // Validate all records, collect errors
    match validate_import_batch(&records) {
        Ok(valid) => {
            // Insert valid records in parallel
            let handles: Vec<_> = valid.iter()
                .map(|record| tokio::spawn(db.insert_record(record.clone())))
                .collect();

            futures::future::try_join_all(handles).await?;
            Ok(valid.len())
        }
        Err(errors) => {
            // Report errors to user
            let summary = format!(
                "Import failed: {} errors in {} records",
                errors.len(),
                records.len()
            );
            Err(ImportError::ValidationFailed(errors).into())
        }
    }
}
```

---

## 6. Type Guard Macros (Future)

### Compile-Time Guarantees

```rust
#[elo_guard(input: "input.quantity > 0")]
pub async fn place_order(input: OrderInput) -> Result<Order> {
    // Compiler guarantees: input.quantity > 0
    // This assertion is redundant and can be removed:
    // assert!(input.quantity > 0);

    db.create_order(input).await
}
```

### Generated With Guard

```rust
pub async fn place_order(input: OrderInput) -> Result<Order> {
    // Guard check inserted automatically
    if input.quantity <= 0 {
        return Err(GuardViolation::QuantityMustBePositive.into());
    }

    // Guarantee: input.quantity > 0
    db.create_order(input).await
}
```

---

## Summary: ELO Rust Target in Action

| Use Case | Layer 1 (Rust) | Layer 2 (SQL) | Layer 3 (ELO Source) |
|----------|---|---|---|
| **GraphQL** | Validates mutations | CHECK constraints | Single .elo file |
| **REST API** | Actix handlers | Database constraints | Single .elo file |
| **Batch** | Tokio processing | Bulk insert constraints | Single .elo file |
| **Real-time** | gRPC validators | Stream validators | Single .elo file |

**Key benefit**: One source of truth, multiple targets, zero duplication.

---

## Next Steps for Implementation

1. Create `examples/` directory with working projects
2. Each example should be independently runnable
3. Include benchmarks for performance verification
4. Document common patterns and best practices

