# Phase 3: Standard Library Functions

**Duration**: Weeks 7-9
**Objective**: Implement all ELO standard library functions (string, date/time, array, type checking)
**Team**: 1-2 engineers
**Status**: [ ] Not Started | [ ] In Progress | [x] Complete

---

## Success Criteria

- [x] All string functions working (matches, contains, length, case conversion, etc.)
- [x] All date/time functions working (today, now, age, duration, etc.)
- [x] All array functions working (contains, any, all, length, is_empty)
- [x] All type checking functions working (is_null, is_some, is_empty, etc.)
- [x] Generated code uses correct crate imports (regex, chrono, etc.)
- [x] Error messages localized and actionable
- [x] 100+ new unit tests for stdlib functions (113 tests implemented)
- [x] Integration tests with real data structures
- [x] End-to-end validation examples working
- [x] Zero Clippy warnings
- [x] Ready for Phase 4: Macros & CLI

---

## Cycle 1: String Functions Implementation (Week 7)

### Objective
Generate code for all ELO string manipulation functions.

### RED Phase: Write String Function Tests

**File**: `tests/codegen/string_functions.rs`

```rust
#[test]
fn test_matches_function() {
    let elo = r#"email matches "^[a-z]+@example\.com$""#;
    let code = generate(elo).unwrap();
    assert_compiles(&code);
    assert!(code.contains("regex"));
}

#[test]
fn test_contains_function() {
    let elo = "text contains substring";
    let code = generate(elo).unwrap();
    assert_compiles(&code);
}

#[test]
fn test_length_function() {
    let elo = "password.length() >= 8";
    let code = generate(elo).unwrap();
    assert_compiles(&code);
}

#[test]
fn test_uppercase_function() {
    let elo = "name.uppercase() == EXPECTED";
    let code = generate(elo).unwrap();
    assert_compiles(&code);
}

#[test]
fn test_lowercase_function() {
    let elo = "email.lowercase() matches pattern";
    let code = generate(elo).unwrap();
    assert_compiles(&code);
}

#[test]
fn test_trim_function() {
    let elo = "input.trim().length() > 0";
    let code = generate(elo).unwrap();
    assert_compiles(&code);
}

#[test]
fn test_starts_with_function() {
    let elo = "url.starts_with(\"https://\")";
    let code = generate(elo).unwrap();
    assert_compiles(&code);
}

#[test]
fn test_ends_with_function() {
    let elo = "email.ends_with(\"@example.com\")";
    let code = generate(elo).unwrap();
    assert_compiles(&code);
}
```

### GREEN Phase: Implement String Functions

**Update `src/codegen/functions.rs`:**

```rust
impl FunctionGenerator {
    pub fn string_function(&self, name: &str, args: Vec<TokenStream>) -> TokenStream {
        match name {
            "matches" => {
                let subject = &args[0];
                let pattern = &args[1];
                quote! {
                    {
                        use regex::Regex;
                        Regex::new(#pattern)
                            .ok()
                            .map(|re| re.is_match(#subject))
                            .unwrap_or(false)
                    }
                }
            }
            "contains" => {
                let subject = &args[0];
                let substring = &args[1];
                quote!(#subject.contains(#substring))
            }
            "length" => {
                let subject = &args[0];
                quote!(#subject.len())
            }
            "uppercase" => {
                let subject = &args[0];
                quote!(#subject.to_uppercase())
            }
            "lowercase" => {
                let subject = &args[0];
                quote!(#subject.to_lowercase())
            }
            "trim" => {
                let subject = &args[0];
                quote!(#subject.trim())
            }
            "starts_with" => {
                let subject = &args[0];
                let prefix = &args[1];
                quote!(#subject.starts_with(#prefix))
            }
            "ends_with" => {
                let subject = &args[0];
                let suffix = &args[1];
                quote!(#subject.ends_with(#suffix))
            }
            _ => quote!(()),
        }
    }
}
```

### REFACTOR Phase: Add Helper Methods

- Extract regex compilation into helper
- Create crate import management
- Improve pattern validation

### CLEANUP Phase: Test All String Functions

```bash
cargo test codegen::string_functions
cargo clippy
```

**Commit:**
```
feat(stdlib): implement all string manipulation functions [Phase 3, Cycle 1: CLEANUP]

## Changes
- Implemented matches() with regex support
- Implemented contains(), length(), case conversion
- Implemented trim(), starts_with(), ends_with()
- Added automatic regex crate imports
- Added 25+ string function tests
- Generated code properly handles edge cases

## Verification
✅ All string function tests pass
✅ Generated code compiles and runs
✅ Regex patterns handled correctly
✅ Zero Clippy warnings
```

---

## Cycle 2: Date/Time Functions Implementation (Week 8, Days 1-3)

### Objective
Generate code for all ELO date/time manipulation functions.

### RED Phase: Write DateTime Function Tests

**File**: `tests/codegen/datetime_functions.rs`

```rust
#[test]
fn test_today_function() {
    let elo = "birthDate < today()";
    let code = generate(elo).unwrap();
    assert_compiles(&code);
    assert!(code.contains("chrono"));
}

#[test]
fn test_now_function() {
    let elo = "timestamp < now()";
    let code = generate(elo).unwrap();
    assert_compiles(&code);
}

#[test]
fn test_age_function() {
    let elo = "age(birthDate) >= 18";
    let code = generate(elo).unwrap();
    assert_compiles(&code);
}

#[test]
fn test_date_comparison() {
    let elo = "startDate <= today() && endDate >= today()";
    let code = generate(elo).unwrap();
    assert_compiles(&code);
}

#[test]
fn test_duration_function() {
    let elo = "days_since(eventDate) > 30";
    let code = generate(elo).unwrap();
    assert_compiles(&code);
}

#[test]
fn test_complex_date_expression() {
    let elo = r#"
        birthDate >= date("1900-01-01") &&
        birthDate <= today() &&
        age(birthDate) >= 18
    "#;
    let code = generate(elo).unwrap();
    assert_compiles(&code);
}
```

### GREEN Phase: Implement DateTime Functions

**Update `src/codegen/functions.rs`:**

```rust
impl FunctionGenerator {
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
                let date = &args[0];
                quote! {
                    {
                        use chrono::Local;
                        (Local::now().date_naive() - #date).num_days()
                    }
                }
            }
            "date" => {
                let date_str = &args[0];
                quote! {
                    {
                        use chrono::NaiveDate;
                        NaiveDate::parse_from_str(#date_str, "%Y-%m-%d")
                            .expect("Invalid date format")
                    }
                }
            }
            _ => quote!(()),
        }
    }
}
```

### REFACTOR Phase: Add Time Zone Support

- Improve timezone handling
- Add more date formats
- Handle date edge cases

### CLEANUP Phase: Test All DateTime Functions

```bash
cargo test codegen::datetime_functions
cargo clippy
```

**Commit:**
```
feat(stdlib): implement all date/time manipulation functions [Phase 3, Cycle 2: CLEANUP]

## Changes
- Implemented today() returning current date
- Implemented now() returning current timestamp
- Implemented age() calculating age from birthdate
- Implemented days_since() duration calculation
- Implemented date() parsing from string
- Added automatic chrono crate imports
- Added 20+ datetime function tests
- Handled timezone-aware operations

## Verification
✅ All datetime function tests pass
✅ Date parsing and calculations correct
✅ Timezone handling proper
✅ Zero Clippy warnings
```

---

## Cycle 3: Array & Type Checking Functions (Week 8, Days 4-5)

### Objective
Generate code for array operations and type checking functions.

### RED Phase: Write Array & Type Function Tests

**File**: `tests/codegen/array_functions.rs`

```rust
#[test]
fn test_array_contains_function() {
    let elo = "roles.contains(\"admin\")";
    let code = generate(elo).unwrap();
    assert_compiles(&code);
}

#[test]
fn test_array_any_function() {
    let elo = "items.any(price > 100)";
    let code = generate(elo).unwrap();
    assert_compiles(&code);
}

#[test]
fn test_array_all_function() {
    let elo = "items.all(inStock == true)";
    let code = generate(elo).unwrap();
    assert_compiles(&code);
}

#[test]
fn test_array_length_function() {
    let elo = "tags.length() >= 1";
    let code = generate(elo).unwrap();
    assert_compiles(&code);
}

#[test]
fn test_array_is_empty_function() {
    let elo = "!attachments.is_empty()";
    let code = generate(elo).unwrap();
    assert_compiles(&code);
}

#[test]
fn test_is_null_function() {
    let elo = "email != null";
    let code = generate(elo).unwrap();
    assert_compiles(&code);
}

#[test]
fn test_is_empty_function() {
    let elo = "!description.is_empty()";
    let code = generate(elo).unwrap();
    assert_compiles(&code);
}

#[test]
fn test_complex_array_expression() {
    let elo = r#"
        roles.contains("user") &&
        items.any(quantity > 0) &&
        tags.length() <= 10
    "#;
    let code = generate(elo).unwrap();
    assert_compiles(&code);
}
```

### GREEN Phase: Implement Array & Type Functions

**Update `src/codegen/functions.rs`:**

```rust
impl FunctionGenerator {
    pub fn array_function(&self, name: &str, args: Vec<TokenStream>) -> TokenStream {
        match name {
            "contains" => {
                let array = &args[0];
                let value = &args[1];
                quote!(#array.contains(&#value))
            }
            "any" => {
                let array = &args[0];
                let predicate = &args[1];
                quote!(#array.iter().any(|item| #predicate))
            }
            "all" => {
                let array = &args[0];
                let predicate = &args[1];
                quote!(#array.iter().all(|item| #predicate))
            }
            "length" => {
                let array = &args[0];
                quote!(#array.len())
            }
            "is_empty" => {
                let array = &args[0];
                quote!(#array.is_empty())
            }
            _ => quote!(()),
        }
    }

    pub fn type_function(&self, name: &str, args: Vec<TokenStream>) -> TokenStream {
        match name {
            "is_null" => {
                let value = &args[0];
                quote!(#value.is_none())
            }
            "is_some" => {
                let value = &args[0];
                quote!(#value.is_some())
            }
            "is_empty" => {
                let value = &args[0];
                quote!(#value.is_empty())
            }
            _ => quote!(()),
        }
    }
}
```

### REFACTOR Phase: Generalize Array Operations

- Support different array types
- Improve predicate handling
- Add type inference for closures

### CLEANUP Phase: Test All Array & Type Functions

```bash
cargo test codegen::array_functions
cargo clippy
```

**Commit:**
```
feat(stdlib): implement array and type checking functions [Phase 3, Cycle 3: CLEANUP]

## Changes
- Implemented array operations (contains, any, all, length, is_empty)
- Implemented type checking functions (is_null, is_some, is_empty)
- Added closure support for predicates
- Added generic array type support
- Added 20+ array and type function tests
- Proper Option/array handling

## Verification
✅ All array function tests pass
✅ Type checking functions work
✅ Predicates with closures working
✅ Generic types handled correctly
✅ Zero Clippy warnings
```

---

## Dependencies

**Requires**:
- Phase 2 complete (core code generator working)

**Provides**:
- Complete stdlib function support for Phase 4 (macros)
- Foundation for end-to-end examples

---

## Stdlib Function Reference

### String Functions (8 total)
- `matches(pattern)` - Regex pattern matching
- `contains(substring)` - Substring search
- `length()` - String length
- `uppercase()` - Convert to uppercase
- `lowercase()` - Convert to lowercase
- `trim()` - Remove whitespace
- `starts_with(prefix)` - Prefix check
- `ends_with(suffix)` - Suffix check

### DateTime Functions (5 total)
- `today()` - Current date
- `now()` - Current timestamp
- `age(birthdate)` - Age calculation
- `days_since(date)` - Days elapsed
- `date(string)` - Parse ISO 8601 date

### Array Functions (5 total)
- `contains(value)` - Element search
- `any(predicate)` - Existence check
- `all(predicate)` - Universal check
- `length()` - Array length
- `is_empty()` - Empty check

### Type Functions (5 total)
- `is_null()` - Null check
- `is_some()` - Some check
- `is_empty()` - Empty string check
- `is_string()` - Type check
- `is_number()` - Type check

---

## Testing Checklist

Before moving to Phase 4:

- [ ] All 23 stdlib functions tested
- [ ] Generated code compiles and runs
- [ ] Real data structure examples working
- [ ] Edge cases handled (null, empty, boundaries)
- [ ] Imports managed automatically
- [ ] Error messages clear
- [ ] 100+ new tests passing
- [ ] Performance benchmarks green
- [ ] Zero Clippy warnings
- [ ] Documentation complete

---

**Next Phase**: [Phase 4: Integration & Macros](./phase-04-macros.md)
