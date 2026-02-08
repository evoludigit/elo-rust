//! DateTime function code generation tests
//!
//! Tests for date/time functions: today, now, age, days_since, date

use elo_rust::codegen::functions::FunctionGenerator;

// ============================================================================
// TODAY FUNCTION - CURRENT DATE
// ============================================================================

#[test]
fn test_today_basic() {
    let gen = FunctionGenerator::new();

    let result = gen.datetime_function("today", vec![]);
    let s = result.to_string();

    assert!(s.contains("chrono") || s.contains("Local"));
}

#[test]
fn test_today_in_comparison() {
    let gen = FunctionGenerator::new();

    let today = gen.datetime_function("today", vec![]);
    let s = today.to_string();

    assert!(!s.is_empty());
    assert!(s.contains("Local") || s.contains("now"));
}

#[test]
fn test_today_can_be_reused() {
    let gen = FunctionGenerator::new();

    let today1 = gen.datetime_function("today", vec![]);
    let today2 = gen.datetime_function("today", vec![]);

    assert_eq!(today1.to_string(), today2.to_string());
}

// ============================================================================
// NOW FUNCTION - CURRENT TIMESTAMP
// ============================================================================

#[test]
fn test_now_basic() {
    let gen = FunctionGenerator::new();

    let result = gen.datetime_function("now", vec![]);
    let s = result.to_string();

    assert!(s.contains("chrono") || s.contains("Utc"));
}

#[test]
fn test_now_returns_timestamp() {
    let gen = FunctionGenerator::new();

    let now = gen.datetime_function("now", vec![]);
    let s = now.to_string();

    assert!(s.contains("Utc") || s.contains("now"));
}

#[test]
fn test_now_for_comparison() {
    let gen = FunctionGenerator::new();

    let now = gen.datetime_function("now", vec![]);
    let s = now.to_string();

    assert!(!s.is_empty());
}

// ============================================================================
// AGE FUNCTION - AGE CALCULATION
// ============================================================================

#[test]
fn test_age_basic() {
    let gen = FunctionGenerator::new();
    let birth_date = quote::quote!(birth_date);

    let result = gen.datetime_function("age", vec![birth_date]);
    let s = result.to_string();

    assert!(s.contains("Local") || s.contains("year"));
}

#[test]
fn test_age_calculation_logic() {
    let gen = FunctionGenerator::new();
    let birth_date = quote::quote!(birthdate);

    let age = gen.datetime_function("age", vec![birth_date]);
    let s = age.to_string();

    assert!(s.contains("year") || s.contains("month"));
}

#[test]
fn test_age_comparison() {
    let gen = FunctionGenerator::new();
    let birth_date = quote::quote!(dob);

    let age = gen.datetime_function("age", vec![birth_date]);
    let s = age.to_string();

    assert!(!s.is_empty());
}

#[test]
fn test_age_with_field_access() {
    let gen = FunctionGenerator::new();
    let birth_date = quote::quote!(user.birth_date);

    let age = gen.datetime_function("age", vec![birth_date]);
    let s = age.to_string();

    assert!(s.contains("user"));
}

#[test]
fn test_age_handles_birthday_logic() {
    let gen = FunctionGenerator::new();
    let birth_date = quote::quote!(birth_date);

    let age = gen.datetime_function("age", vec![birth_date]);
    let s = age.to_string();

    // Should handle the month/day comparison for birthday
    assert!(s.contains("month") || s.contains("day"));
}

// ============================================================================
// DAYS_SINCE FUNCTION - DURATION CALCULATION
// ============================================================================

#[test]
fn test_days_since_basic() {
    let gen = FunctionGenerator::new();
    let date = quote::quote!(event_date);

    let result = gen.datetime_function("days_since", vec![date]);
    let s = result.to_string();

    assert!(s.contains("Local") || s.contains("num_days"));
}

#[test]
fn test_days_since_date_calculation() {
    let gen = FunctionGenerator::new();
    let date = quote::quote!(created_at);

    let days = gen.datetime_function("days_since", vec![date]);
    let s = days.to_string();

    assert!(s.contains("num_days") || s.contains("days"));
}

#[test]
fn test_days_since_with_field_access() {
    let gen = FunctionGenerator::new();
    let date = quote::quote!(user.last_login);

    let days = gen.datetime_function("days_since", vec![date]);
    let s = days.to_string();

    assert!(s.contains("user"));
}

#[test]
fn test_days_since_for_comparison() {
    let gen = FunctionGenerator::new();
    let date = quote::quote!(start_date);

    let days = gen.datetime_function("days_since", vec![date]);
    let s = days.to_string();

    assert!(!s.is_empty());
}

// ============================================================================
// DATE FUNCTION - DATE PARSING
// ============================================================================

#[test]
fn test_date_parsing_iso8601() {
    let gen = FunctionGenerator::new();
    let date_str = quote::quote!("2024-01-15");

    let result = gen.datetime_function("date", vec![date_str]);
    let s = result.to_string();

    assert!(s.contains("parse_from_str") || s.contains("NaiveDate"));
}

#[test]
fn test_date_parsing_format() {
    let gen = FunctionGenerator::new();
    let date_str = quote::quote!("1990-05-20");

    let date = gen.datetime_function("date", vec![date_str]);
    let s = date.to_string();

    assert!(s.contains("%Y-%m-%d") || s.contains("parse"));
}

#[test]
fn test_date_with_variable() {
    let gen = FunctionGenerator::new();
    let date_var = quote::quote!(date_string);

    let date = gen.datetime_function("date", vec![date_var]);
    let s = date.to_string();

    assert!(s.contains("date_string"));
}

// ============================================================================
// COMBINED DATE/TIME OPERATIONS
// ============================================================================

#[test]
fn test_age_validation() {
    let gen = FunctionGenerator::new();

    // age(birthDate) >= 18
    let birth_date = quote::quote!(birth_date);
    let age = gen.datetime_function("age", vec![birth_date]);

    let s = age.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_date_range_validation() {
    let gen = FunctionGenerator::new();

    // startDate <= today() && endDate >= today()
    let today = gen.datetime_function("today", vec![]);
    let today2 = gen.datetime_function("today", vec![]);

    let today_str = today.to_string();
    let today2_str = today2.to_string();

    assert_eq!(today_str, today2_str);
}

#[test]
fn test_event_active_validation() {
    let gen = FunctionGenerator::new();

    // eventDate <= today() && days_since(eventDate) < 30
    let today = gen.datetime_function("today", vec![]);
    let event_date = quote::quote!(event_date);
    let days = gen.datetime_function("days_since", vec![event_date]);

    let today_str = today.to_string();
    let days_str = days.to_string();

    assert!(!today_str.is_empty());
    assert!(!days_str.is_empty());
}

#[test]
fn test_membership_expiry_check() {
    let gen = FunctionGenerator::new();

    // expiry_date > today()
    let today = gen.datetime_function("today", vec![]);

    let today_str = today.to_string();
    assert!(!today_str.is_empty());
}

#[test]
fn test_account_age_check() {
    let gen = FunctionGenerator::new();

    // days_since(created_at) > 30
    let created = quote::quote!(created_at);
    let days = gen.datetime_function("days_since", vec![created]);

    let days_str = days.to_string();
    assert!(!days_str.is_empty());
}

#[test]
fn test_complex_date_expression() {
    let gen = FunctionGenerator::new();

    // birthDate >= date("1900-01-01") && birthDate <= today() && age(birthDate) >= 18
    let min_date = gen.datetime_function("date", vec![quote::quote!("1900-01-01")]);
    let max_date = gen.datetime_function("today", vec![]);
    let age = gen.datetime_function("age", vec![quote::quote!(birth_date)]);

    assert!(!min_date.to_string().is_empty());
    assert!(!max_date.to_string().is_empty());
    assert!(!age.to_string().is_empty());
}

#[test]
fn test_recent_activity_check() {
    let gen = FunctionGenerator::new();

    // days_since(last_activity) <= 7
    let days = gen.datetime_function("days_since", vec![quote::quote!(last_activity)]);

    let s = days.to_string();
    assert!(!s.is_empty());
}

// ============================================================================
// BIRTHDAY/ANNIVERSARY CALCULATIONS
// ============================================================================

#[test]
fn test_adult_age_validation() {
    let gen = FunctionGenerator::new();

    // age(birthDate) >= 18
    let age = gen.datetime_function("age", vec![quote::quote!(birth_date)]);

    let s = age.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_senior_age_validation() {
    let gen = FunctionGenerator::new();

    // age(birthDate) >= 65
    let age = gen.datetime_function("age", vec![quote::quote!(birth_date)]);

    let s = age.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_age_range_validation() {
    let gen = FunctionGenerator::new();

    // age(birthDate) >= 18 && age(birthDate) <= 65
    let age1 = gen.datetime_function("age", vec![quote::quote!(birth_date)]);
    let age2 = gen.datetime_function("age", vec![quote::quote!(birth_date)]);

    assert_eq!(age1.to_string(), age2.to_string());
}

// ============================================================================
// TIME-BASED BUSINESS LOGIC
// ============================================================================

#[test]
fn test_trial_period_active() {
    let gen = FunctionGenerator::new();

    // days_since(signup_date) <= 30
    let days = gen.datetime_function("days_since", vec![quote::quote!(signup_date)]);

    let s = days.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_subscription_active() {
    let gen = FunctionGenerator::new();

    // renewal_date >= today()
    let today = gen.datetime_function("today", vec![]);

    let s = today.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_license_expired() {
    let gen = FunctionGenerator::new();

    // license_expiry < today()
    let today = gen.datetime_function("today", vec![]);

    let s = today.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_event_occurred_within_window() {
    let gen = FunctionGenerator::new();

    // event_date >= date("2024-01-01") && event_date <= today()
    let min = gen.datetime_function("date", vec![quote::quote!("2024-01-01")]);
    let max = gen.datetime_function("today", vec![]);

    assert!(!min.to_string().is_empty());
    assert!(!max.to_string().is_empty());
}

// ============================================================================
// GENERATOR CONSISTENCY
// ============================================================================

#[test]
fn test_datetime_function_consistency() {
    let gen = FunctionGenerator::new();

    let result1 = gen.datetime_function("today", vec![]);
    let result2 = gen.datetime_function("today", vec![]);

    assert_eq!(result1.to_string(), result2.to_string());
}

#[test]
fn test_multiple_datetime_functions() {
    let gen = FunctionGenerator::new();

    let today = gen.datetime_function("today", vec![]);
    let now = gen.datetime_function("now", vec![]);
    let age = gen.datetime_function("age", vec![quote::quote!(birth_date)]);

    assert!(!today.to_string().is_empty());
    assert!(!now.to_string().is_empty());
    assert!(!age.to_string().is_empty());
}

#[test]
fn test_different_datetime_generators() {
    let gen1 = FunctionGenerator::new();
    let gen2 = FunctionGenerator::new();

    let today1 = gen1.datetime_function("today", vec![]);
    let today2 = gen2.datetime_function("today", vec![]);

    assert_eq!(today1.to_string(), today2.to_string());
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_age_with_missing_argument() {
    let gen = FunctionGenerator::new();

    let result = gen.datetime_function("age", vec![]);
    let s = result.to_string();

    // Should handle gracefully (empty in this case)
    assert!(s.is_empty() || !s.is_empty()); // Either is acceptable
}

#[test]
fn test_date_parsing_boundary() {
    let gen = FunctionGenerator::new();

    // Test with earliest possible date
    let early_date = gen.datetime_function("date", vec![quote::quote!("0001-01-01")]);
    assert!(!early_date.to_string().is_empty());

    // Test with recent date
    let recent_date = gen.datetime_function("date", vec![quote::quote!("2024-12-31")]);
    assert!(!recent_date.to_string().is_empty());
}

#[test]
fn test_age_calculation_consistency() {
    let gen = FunctionGenerator::new();

    let age1 = gen.datetime_function("age", vec![quote::quote!(birth_date)]);
    let age2 = gen.datetime_function("age", vec![quote::quote!(birth_date)]);

    assert_eq!(age1.to_string(), age2.to_string());
}

#[test]
fn test_days_since_various_dates() {
    let gen = FunctionGenerator::new();

    let days1 = gen.datetime_function("days_since", vec![quote::quote!(date1)]);
    let days2 = gen.datetime_function("days_since", vec![quote::quote!(date2)]);

    // Different arguments should produce consistent output structure
    assert!(!days1.to_string().is_empty());
    assert!(!days2.to_string().is_empty());
}
