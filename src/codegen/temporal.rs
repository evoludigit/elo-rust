//! Temporal value code generation for dates, times, and durations

use proc_macro2::TokenStream;
use quote::quote;

/// Generates code for temporal operations
#[derive(Debug)]
pub struct TemporalGenerator;

impl TemporalGenerator {
    /// Create a new temporal generator
    pub fn new() -> Self {
        Self
    }

    /// Generate code for a date literal (ISO8601)
    pub fn date(&self, date_str: &str) -> TokenStream {
        quote! {
            {
                use chrono::NaiveDate;
                NaiveDate::parse_from_str(#date_str, "%Y-%m-%d")
                    .expect("Invalid date format")
            }
        }
    }

    /// Generate code for a datetime literal (RFC3339)
    pub fn datetime(&self, datetime_str: &str) -> TokenStream {
        quote! {
            {
                use chrono::DateTime;
                DateTime::parse_from_rfc3339(#datetime_str)
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .expect("Invalid datetime format")
            }
        }
    }

    /// Generate code for a duration literal (ISO8601)
    pub fn duration(&self, duration_str: &str) -> TokenStream {
        quote! {
            {
                use chrono::Duration;
                use elo_rust::runtime::TemporalValue;
                TemporalValue::parse_duration(#duration_str)
                    .map(|tv| match tv {
                        TemporalValue::Duration(d) => d,
                        _ => Duration::zero(),
                    })
                    .unwrap_or_else(|_| Duration::zero())
            }
        }
    }

    /// Generate code for a temporal keyword
    pub fn keyword(&self, keyword: &str) -> TokenStream {
        match keyword {
            "NOW" => quote! {
                {
                    use chrono::Utc;
                    Utc::now()
                }
            },
            "TODAY" => quote! {
                {
                    use chrono::Local;
                    Local::now().naive_local().date()
                }
            },
            "TOMORROW" => quote! {
                {
                    use chrono::{Local, Duration};
                    (Local::now().naive_local().date() + Duration::days(1))
                }
            },
            "YESTERDAY" => quote! {
                {
                    use chrono::{Local, Duration};
                    (Local::now().naive_local().date() - Duration::days(1))
                }
            },
            "START_OF_DAY" => quote! {
                {
                    use chrono::Local;
                    let today = Local::now().naive_local().date();
                    today.and_hms_opt(0, 0, 0).unwrap()
                }
            },
            "END_OF_DAY" => quote! {
                {
                    use chrono::Local;
                    let today = Local::now().naive_local().date();
                    today.and_hms_opt(23, 59, 59).unwrap()
                }
            },
            "START_OF_WEEK" => quote! {
                {
                    use chrono::{Local, Datelike};
                    let today = Local::now().naive_local().date();
                    let days_since_monday = today.weekday().number_from_monday() - 1;
                    today - chrono::Duration::days(days_since_monday as i64)
                }
            },
            "END_OF_WEEK" => quote! {
                {
                    use chrono::{Local, Datelike};
                    let today = Local::now().naive_local().date();
                    let days_until_sunday = 7 - today.weekday().number_from_monday();
                    today + chrono::Duration::days(days_until_sunday as i64)
                }
            },
            "START_OF_MONTH" => quote! {
                {
                    use chrono::Local;
                    let today = Local::now().naive_local().date();
                    today.with_day(1).unwrap()
                }
            },
            "END_OF_MONTH" => quote! {
                {
                    use chrono::{Local, NaiveDate};
                    let today = Local::now().naive_local().date();
                    let last_day = if today.month() == 12 {
                        NaiveDate::from_ymd_opt(today.year() + 1, 1, 1)
                            .unwrap()
                            - chrono::Duration::days(1)
                    } else {
                        NaiveDate::from_ymd_opt(today.year(), today.month() + 1, 1)
                            .unwrap()
                            - chrono::Duration::days(1)
                    };
                    last_day
                }
            },
            "START_OF_QUARTER" => quote! {
                {
                    use chrono::Local;
                    let today = Local::now().naive_local().date();
                    let quarter = (today.month() - 1) / 3;
                    let month = quarter * 3 + 1;
                    today.with_month(month).unwrap().with_day(1).unwrap()
                }
            },
            "END_OF_QUARTER" => quote! {
                {
                    use chrono::{Local, NaiveDate};
                    let today = Local::now().naive_local().date();
                    let quarter = (today.month() - 1) / 3;
                    let next_quarter_month = (quarter + 1) * 3 + 1;
                    let year = if next_quarter_month > 12 {
                        today.year() + 1
                    } else {
                        today.year()
                    };
                    let month = if next_quarter_month > 12 {
                        next_quarter_month - 12
                    } else {
                        next_quarter_month
                    };
                    NaiveDate::from_ymd_opt(year, month, 1)
                        .unwrap()
                        - chrono::Duration::days(1)
                }
            },
            "START_OF_YEAR" => quote! {
                {
                    use chrono::Local;
                    let today = Local::now().naive_local().date();
                    today.with_month(1).unwrap().with_day(1).unwrap()
                }
            },
            "END_OF_YEAR" => quote! {
                {
                    use chrono::Local;
                    let today = Local::now().naive_local().date();
                    today.with_month(12).unwrap().with_day(31).unwrap()
                }
            },
            "BEGINNING_OF_TIME" => quote! {
                {
                    use chrono::NaiveDate;
                    NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()
                }
            },
            "END_OF_TIME" => quote! {
                {
                    use chrono::NaiveDate;
                    NaiveDate::from_ymd_opt(9999, 12, 31).unwrap()
                }
            },
            _ => quote!(),
        }
    }

    /// Generate code for temporal comparison (dates)
    pub fn date_compare(&self, left: TokenStream, op: &str, right: TokenStream) -> TokenStream {
        match op {
            "<" => quote!(#left < #right),
            "<=" => quote!(#left <= #right),
            ">" => quote!(#left > #right),
            ">=" => quote!(#left >= #right),
            "==" => quote!(#left == #right),
            "!=" => quote!(#left != #right),
            _ => quote!(),
        }
    }

    /// Generate code for temporal arithmetic (dates + durations)
    pub fn temporal_add(&self, left: TokenStream, right: TokenStream) -> TokenStream {
        quote! {
            (#left + #right)
        }
    }

    /// Generate code for temporal subtraction
    pub fn temporal_subtract(&self, left: TokenStream, right: TokenStream) -> TokenStream {
        quote! {
            (#left - #right)
        }
    }
}

impl Default for TemporalGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temporal_generator_creation() {
        let _gen = TemporalGenerator::new();
    }

    #[test]
    fn test_date_literal_generation() {
        let gen = TemporalGenerator::new();
        let token = gen.date("2024-01-15");
        let token_str = token.to_string();
        assert!(token_str.contains("parse_from_str"));
        assert!(token_str.contains("2024-01-15"));
    }

    #[test]
    fn test_datetime_literal_generation() {
        let gen = TemporalGenerator::new();
        let token = gen.datetime("2024-01-15T10:30:00Z");
        let token_str = token.to_string();
        assert!(token_str.contains("parse_from_rfc3339"));
    }

    #[test]
    fn test_now_keyword() {
        let gen = TemporalGenerator::new();
        let token = gen.keyword("NOW");
        let token_str = token.to_string();
        assert!(token_str.contains("Utc"));
    }

    #[test]
    fn test_today_keyword() {
        let gen = TemporalGenerator::new();
        let token = gen.keyword("TODAY");
        let token_str = token.to_string();
        assert!(token_str.contains("Local"));
    }

    #[test]
    fn test_tomorrow_keyword() {
        let gen = TemporalGenerator::new();
        let token = gen.keyword("TOMORROW");
        let token_str = token.to_string();
        assert!(token_str.contains("days"));
    }

    #[test]
    fn test_start_of_day_keyword() {
        let gen = TemporalGenerator::new();
        let token = gen.keyword("START_OF_DAY");
        let token_str = token.to_string();
        assert!(token_str.contains("0"));
    }

    #[test]
    fn test_date_comparison() {
        let gen = TemporalGenerator::new();
        let left = quote!(date1);
        let right = quote!(date2);
        let token = gen.date_compare(left, "<", right);
        let token_str = token.to_string();
        assert!(token_str.contains("<"));
    }
}
