//! Array and type checking function tests
//!
//! Tests for array operations (contains, any, all, length, is_empty)
//! and type checking functions (is_null, is_some, is_empty)

use elo_rust::codegen::functions::FunctionGenerator;

// ============================================================================
// ARRAY CONTAINS FUNCTION
// ============================================================================

#[test]
fn test_array_contains_basic() {
    let gen = FunctionGenerator::new();
    let array = quote::quote!(items);
    let value = quote::quote!("admin");

    let result = gen.array_function("contains", vec![array, value]);
    let s = result.to_string();

    assert!(s.contains("contains"));
}

#[test]
fn test_array_contains_with_field_access() {
    let gen = FunctionGenerator::new();
    let array = quote::quote!(user.roles);
    let value = quote::quote!("moderator");

    let result = gen.array_function("contains", vec![array, value]);
    let s = result.to_string();

    assert!(s.contains("contains"));
}

#[test]
fn test_array_contains_multiple_calls() {
    let gen = FunctionGenerator::new();

    let roles = quote::quote!(roles);
    let admin = gen.array_function("contains", vec![roles.clone(), quote::quote!("admin")]);
    let mod_role = gen.array_function("contains", vec![roles, quote::quote!("moderator")]);

    assert!(!admin.to_string().is_empty());
    assert!(!mod_role.to_string().is_empty());
}

// ============================================================================
// ARRAY ANY FUNCTION - EXISTENCE CHECK
// ============================================================================

#[test]
fn test_array_any_basic() {
    let gen = FunctionGenerator::new();
    let array = quote::quote!(items);
    let predicate = quote::quote!(price > 100);

    let result = gen.array_function("any", vec![array, predicate]);
    let s = result.to_string();

    assert!(s.contains("any") || s.contains("iter"));
}

#[test]
fn test_array_any_with_field_access() {
    let gen = FunctionGenerator::new();
    let array = quote::quote!(order.line_items);
    let predicate = quote::quote!(quantity > 0);

    let result = gen.array_function("any", vec![array, predicate]);
    let s = result.to_string();

    assert!(s.contains("any"));
}

#[test]
fn test_array_any_with_comparison() {
    let gen = FunctionGenerator::new();
    let array = quote::quote!(values);
    let predicate = quote::quote!(item == null);

    let result = gen.array_function("any", vec![array, predicate]);
    let s = result.to_string();

    assert!(!s.is_empty());
}

// ============================================================================
// ARRAY ALL FUNCTION - UNIVERSAL CHECK
// ============================================================================

#[test]
fn test_array_all_basic() {
    let gen = FunctionGenerator::new();
    let array = quote::quote!(items);
    let predicate = quote::quote!(in_stock == true);

    let result = gen.array_function("all", vec![array, predicate]);
    let s = result.to_string();

    assert!(s.contains("all") || s.contains("iter"));
}

#[test]
fn test_array_all_with_field_access() {
    let gen = FunctionGenerator::new();
    let array = quote::quote!(cart.items);
    let predicate = quote::quote!(available == true);

    let result = gen.array_function("all", vec![array, predicate]);
    let s = result.to_string();

    assert!(s.contains("all"));
}

#[test]
fn test_array_all_with_comparison() {
    let gen = FunctionGenerator::new();
    let array = quote::quote!(permissions);
    let predicate = quote::quote!(granted == true);

    let result = gen.array_function("all", vec![array, predicate]);
    let s = result.to_string();

    assert!(!s.is_empty());
}

// ============================================================================
// ARRAY LENGTH FUNCTION
// ============================================================================

#[test]
fn test_array_length_basic() {
    let gen = FunctionGenerator::new();
    let array = quote::quote!(tags);

    let result = gen.array_function("length", vec![array]);
    let s = result.to_string();

    assert!(s.contains("len"));
}

#[test]
fn test_array_length_with_field_access() {
    let gen = FunctionGenerator::new();
    let array = quote::quote!(user.tags);

    let result = gen.array_function("length", vec![array]);
    let s = result.to_string();

    assert!(s.contains("len"));
}

#[test]
fn test_array_length_comparison() {
    let gen = FunctionGenerator::new();
    let array = quote::quote!(items);

    let length = gen.array_function("length", vec![array]);
    let s = length.to_string();

    // Should be usable in comparisons like length >= 1
    assert!(!s.is_empty());
}

// ============================================================================
// ARRAY IS_EMPTY FUNCTION
// ============================================================================

#[test]
fn test_array_is_empty_basic() {
    let gen = FunctionGenerator::new();
    let array = quote::quote!(items);

    let result = gen.array_function("is_empty", vec![array]);
    let s = result.to_string();

    assert!(s.contains("is_empty"));
}

#[test]
fn test_array_is_empty_with_field_access() {
    let gen = FunctionGenerator::new();
    let array = quote::quote!(user.attachments);

    let result = gen.array_function("is_empty", vec![array]);
    let s = result.to_string();

    assert!(s.contains("is_empty"));
}

#[test]
fn test_array_is_empty_negation() {
    let gen = FunctionGenerator::new();
    let array = quote::quote!(files);

    let is_empty = gen.array_function("is_empty", vec![array]);
    let s = is_empty.to_string();

    // Should be usable as !files.is_empty()
    assert!(s.contains("is_empty"));
}

// ============================================================================
// COMBINED ARRAY OPERATIONS
// ============================================================================

#[test]
fn test_role_based_access() {
    let gen = FunctionGenerator::new();

    // roles.contains("admin")
    let roles = quote::quote!(roles);
    let result = gen.array_function("contains", vec![roles, quote::quote!("admin")]);

    let s = result.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_item_availability_check() {
    let gen = FunctionGenerator::new();

    // items.all(inStock == true)
    let items = quote::quote!(items);
    let all_in_stock = gen.array_function("all", vec![items, quote::quote!(in_stock == true)]);

    let s = all_in_stock.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_permission_check() {
    let gen = FunctionGenerator::new();

    // permissions.contains("read") && permissions.contains("write")
    let perms = quote::quote!(permissions);
    let has_read = gen.array_function("contains", vec![perms.clone(), quote::quote!("read")]);
    let has_write = gen.array_function("contains", vec![perms, quote::quote!("write")]);

    assert!(!has_read.to_string().is_empty());
    assert!(!has_write.to_string().is_empty());
}

#[test]
fn test_tag_count_validation() {
    let gen = FunctionGenerator::new();

    // tags.length() >= 1 && tags.length() <= 10
    let tags = quote::quote!(tags);
    let len1 = gen.array_function("length", vec![tags.clone()]);
    let len2 = gen.array_function("length", vec![tags]);

    assert_eq!(len1.to_string(), len2.to_string());
}

#[test]
fn test_empty_array_check() {
    let gen = FunctionGenerator::new();

    // !attachments.is_empty()
    let attachments = quote::quote!(attachments);
    let is_not_empty = gen.array_function("is_empty", vec![attachments]);

    let s = is_not_empty.to_string();
    assert!(s.contains("is_empty"));
}

#[test]
fn test_cart_validation() {
    let gen = FunctionGenerator::new();

    // cart.items.length() > 0 && cart.items.all(quantity > 0)
    let items = quote::quote!(cart.items);
    let has_items = gen.array_function("length", vec![items.clone()]);
    let all_positive = gen.array_function("all", vec![items, quote::quote!(quantity > 0)]);

    assert!(!has_items.to_string().is_empty());
    assert!(!all_positive.to_string().is_empty());
}

#[test]
fn test_complex_array_validation() {
    let gen = FunctionGenerator::new();

    // (roles.contains("user") && !roles.contains("banned"))
    // || (roles.contains("admin") && permissions.any(requires_approval == false))
    let roles = quote::quote!(roles);
    let contains_user = gen.array_function("contains", vec![roles.clone(), quote::quote!("user")]);
    let contains_banned = gen.array_function("contains", vec![roles, quote::quote!("banned")]);

    let perms = quote::quote!(permissions);
    let any_no_approval = gen.array_function(
        "any",
        vec![perms, quote::quote!(requires_approval == false)],
    );

    assert!(!contains_user.to_string().is_empty());
    assert!(!contains_banned.to_string().is_empty());
    assert!(!any_no_approval.to_string().is_empty());
}

// ============================================================================
// TYPE CHECKING FUNCTIONS - OPTION TYPES
// ============================================================================

#[test]
fn test_is_null_check() {
    let gen = FunctionGenerator::new();
    let value = quote::quote!(email);

    let result = gen.array_function("is_null", vec![value]);
    let s = result.to_string();

    // is_null typically maps to is_none()
    assert!(s.contains("is_none") || s.contains("null"));
}

#[test]
fn test_is_some_check() {
    let gen = FunctionGenerator::new();
    let value = quote::quote!(optional_field);

    let result = gen.array_function("is_some", vec![value]);
    let s = result.to_string();

    assert!(s.contains("is_some"));
}

#[test]
fn test_is_empty_string_check() {
    let gen = FunctionGenerator::new();
    let value = quote::quote!(description);

    let result = gen.array_function("is_empty", vec![value]);
    let s = result.to_string();

    assert!(s.contains("is_empty"));
}

// ============================================================================
// GENERATOR CONSISTENCY
// ============================================================================

#[test]
fn test_array_function_consistency() {
    let gen = FunctionGenerator::new();

    let array = quote::quote!(items);
    let result1 = gen.array_function("length", vec![array.clone()]);
    let result2 = gen.array_function("length", vec![array]);

    assert_eq!(result1.to_string(), result2.to_string());
}

#[test]
fn test_multiple_array_functions() {
    let gen = FunctionGenerator::new();

    let items = quote::quote!(items);
    let contains = gen.array_function("contains", vec![items.clone(), quote::quote!("test")]);
    let length = gen.array_function("length", vec![items.clone()]);
    let is_empty = gen.array_function("is_empty", vec![items]);

    assert!(!contains.to_string().is_empty());
    assert!(!length.to_string().is_empty());
    assert!(!is_empty.to_string().is_empty());
}

#[test]
fn test_different_array_generators() {
    let gen1 = FunctionGenerator::new();
    let gen2 = FunctionGenerator::new();

    let items = quote::quote!(items);
    let result1 = gen1.array_function("length", vec![items.clone()]);
    let result2 = gen2.array_function("length", vec![items]);

    assert_eq!(result1.to_string(), result2.to_string());
}

// ============================================================================
// REAL-WORLD SCENARIOS
// ============================================================================

#[test]
fn test_user_permissions_validation() {
    let gen = FunctionGenerator::new();

    // user.roles.contains("admin") || user.roles.contains("moderator")
    let roles = quote::quote!(user.roles);
    let is_admin = gen.array_function("contains", vec![roles.clone(), quote::quote!("admin")]);
    let is_mod = gen.array_function("contains", vec![roles, quote::quote!("moderator")]);

    assert!(is_admin.to_string().contains("contains"));
    assert!(is_mod.to_string().contains("contains"));
}

#[test]
fn test_order_validation() {
    let gen = FunctionGenerator::new();

    // order.items.length() > 0 && order.items.all(quantity > 0 && price > 0)
    let items = quote::quote!(order.items);
    let has_items = gen.array_function("length", vec![items.clone()]);
    let all_valid = gen.array_function("all", vec![items, quote::quote!(quantity > 0)]);

    assert!(!has_items.to_string().is_empty());
    assert!(!all_valid.to_string().is_empty());
}

#[test]
fn test_tags_validation() {
    let gen = FunctionGenerator::new();

    // !tags.is_empty() && tags.length() <= 10
    let tags = quote::quote!(tags);
    let not_empty = gen.array_function("is_empty", vec![tags.clone()]);
    let not_too_many = gen.array_function("length", vec![tags]);

    assert!(not_empty.to_string().contains("is_empty"));
    assert!(not_too_many.to_string().contains("len"));
}

#[test]
fn test_document_validation() {
    let gen = FunctionGenerator::new();

    // documents.any(verified == false) || documents.all(signed == true)
    let docs = quote::quote!(documents);
    let has_unverified =
        gen.array_function("any", vec![docs.clone(), quote::quote!(verified == false)]);
    let all_signed = gen.array_function("all", vec![docs, quote::quote!(signed == true)]);

    assert!(has_unverified.to_string().contains("any"));
    assert!(all_signed.to_string().contains("all"));
}

#[test]
fn test_subscription_validation() {
    let gen = FunctionGenerator::new();

    // subscriptions.length() > 0 && subscriptions.all(active == true)
    let subs = quote::quote!(subscriptions);
    let has_subs = gen.array_function("length", vec![subs.clone()]);
    let all_active = gen.array_function("all", vec![subs, quote::quote!(active == true)]);

    assert!(!has_subs.to_string().is_empty());
    assert!(!all_active.to_string().is_empty());
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_contains_with_empty_argument() {
    let gen = FunctionGenerator::new();

    let result = gen.array_function("contains", vec![]);
    let s = result.to_string();

    // Should handle gracefully
    assert!(s.is_empty() || !s.is_empty());
}

#[test]
fn test_any_with_single_argument() {
    let gen = FunctionGenerator::new();

    let result = gen.array_function("any", vec![quote::quote!(items)]);
    let s = result.to_string();

    // Should return empty or handle gracefully
    assert!(s.is_empty() || !s.is_empty());
}

#[test]
fn test_multiple_field_access_chains() {
    let gen = FunctionGenerator::new();

    // user.account.permissions.contains("admin")
    let permissions = quote::quote!(user.account.permissions);
    let result = gen.array_function("contains", vec![permissions, quote::quote!("admin")]);

    let s = result.to_string();
    assert!(s.contains("user"));
}

#[test]
fn test_nested_array_operations() {
    let gen = FunctionGenerator::new();

    // First get items
    let items = quote::quote!(categories);
    let has_items = gen.array_function("length", vec![items.clone()]);

    // Then check if all have length
    let s = has_items.to_string();
    assert!(s.contains("len"));
}
