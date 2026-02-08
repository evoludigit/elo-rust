//! Temporal type operations for ELO runtime
//!
//! Provides date, datetime, and duration handling with comprehensive operations
//! for temporal arithmetic, comparisons, and calculations.

use chrono::{DateTime, Duration, Local, NaiveDate, TimeZone, Utc};
use std::fmt;

/// Represents a temporal value (Date, DateTime, or Duration)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemporalValue {
    /// A date without time (ISO8601: YYYY-MM-DD)
    Date(NaiveDate),

    /// A datetime with timezone
    DateTime(DateTime<Utc>),

    /// A duration/interval
    Duration(Duration),
}

impl TemporalValue {
    /// Parse an ISO8601 date string (YYYY-MM-DD)
    pub fn parse_date(date_str: &str) -> Result<Self, String> {
        NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .map(TemporalValue::Date)
            .map_err(|e| format!("Invalid date format: {} (expected YYYY-MM-DD)", e))
    }

    /// Parse an ISO8601 datetime string
    pub fn parse_datetime(datetime_str: &str) -> Result<Self, String> {
        DateTime::parse_from_rfc3339(datetime_str)
            .map(|dt| TemporalValue::DateTime(dt.with_timezone(&Utc)))
            .map_err(|e| format!("Invalid datetime format: {}", e))
    }

    /// Parse an ISO8601 duration string
    pub fn parse_duration(duration_str: &str) -> Result<Self, String> {
        // Simple ISO8601 duration parsing
        // Format: P[n]Y[n]M[n]DT[n]H[n]M[n]S or P[n]W
        if let Some(weeks_part) = duration_str.strip_prefix('P') {
            if let Some(weeks) = weeks_part.strip_suffix('W') {
                let weeks: i64 = weeks
                    .parse()
                    .map_err(|_| format!("Invalid duration: {}", duration_str))?;
                return Ok(TemporalValue::Duration(Duration::weeks(weeks)));
            }
        }

        // Basic day parsing (P1D, P2D, etc.)
        if duration_str.starts_with('P') && duration_str.ends_with('D') {
            let days_str = &duration_str[1..duration_str.len() - 1];
            let days: i64 = days_str
                .parse()
                .map_err(|_| format!("Invalid duration: {}", duration_str))?;
            return Ok(TemporalValue::Duration(Duration::days(days)));
        }

        // PT parsing for time durations (PT1H, PT30M, PT1H30M)
        if let Some(time_part) = duration_str.strip_prefix("PT") {
            let mut total_secs = 0i64;

            // Simple parser for PTnHnMnS format
            let mut current = String::new();
            for ch in time_part.chars() {
                match ch {
                    'H' => {
                        if let Ok(hours) = current.parse::<i64>() {
                            total_secs += hours * 3600;
                        }
                        current.clear();
                    }
                    'M' => {
                        if let Ok(mins) = current.parse::<i64>() {
                            total_secs += mins * 60;
                        }
                        current.clear();
                    }
                    'S' => {
                        if let Ok(secs) = current.parse::<i64>() {
                            total_secs += secs;
                        }
                        current.clear();
                    }
                    '.' => {
                        // Handle fractional seconds (simplified - just truncate)
                        current.clear();
                    }
                    _ => current.push(ch),
                }
            }

            return Ok(TemporalValue::Duration(Duration::seconds(total_secs)));
        }

        Err(format!(
            "Invalid duration format: {} (expected ISO8601 format)",
            duration_str
        ))
    }

    /// Get the type name
    pub fn type_name(&self) -> &'static str {
        match self {
            TemporalValue::Date(_) => "date",
            TemporalValue::DateTime(_) => "datetime",
            TemporalValue::Duration(_) => "duration",
        }
    }

    /// Get today's date
    pub fn today() -> Self {
        TemporalValue::Date(Local::now().naive_local().date())
    }

    /// Get current datetime
    pub fn now() -> Self {
        TemporalValue::DateTime(Utc::now())
    }

    /// Add a duration to this temporal value
    pub fn add_duration(&self, duration: &TemporalValue) -> Result<TemporalValue, String> {
        let dur = match duration {
            TemporalValue::Duration(d) => *d,
            _ => return Err("Can only add Duration to temporal values".to_string()),
        };

        match self {
            TemporalValue::Date(date) => {
                let naive_dt = date
                    .and_hms_opt(0, 0, 0)
                    .ok_or("Invalid date time combination")?;
                let dt = Utc.from_utc_datetime(&naive_dt);
                let result_dt = dt + dur;
                Ok(TemporalValue::DateTime(result_dt))
            }
            TemporalValue::DateTime(dt) => Ok(TemporalValue::DateTime(*dt + dur)),
            TemporalValue::Duration(d) => Ok(TemporalValue::Duration(*d + dur)),
        }
    }

    /// Subtract a duration from this temporal value
    pub fn subtract_duration(&self, duration: &TemporalValue) -> Result<TemporalValue, String> {
        let dur = match duration {
            TemporalValue::Duration(d) => *d,
            _ => return Err("Can only subtract Duration from temporal values".to_string()),
        };

        match self {
            TemporalValue::Date(date) => {
                let naive_dt = date
                    .and_hms_opt(0, 0, 0)
                    .ok_or("Invalid date time combination")?;
                let dt = Utc.from_utc_datetime(&naive_dt);
                let result_dt = dt - dur;
                Ok(TemporalValue::DateTime(result_dt))
            }
            TemporalValue::DateTime(dt) => Ok(TemporalValue::DateTime(*dt - dur)),
            TemporalValue::Duration(d) => Ok(TemporalValue::Duration(*d - dur)),
        }
    }

    /// Get the difference between two temporal values
    pub fn difference(&self, other: &TemporalValue) -> Result<TemporalValue, String> {
        match (self, other) {
            (TemporalValue::Date(d1), TemporalValue::Date(d2)) => {
                let dt1 = d1
                    .and_hms_opt(0, 0, 0)
                    .ok_or("Invalid date time combination")?;
                let dt2 = d2
                    .and_hms_opt(0, 0, 0)
                    .ok_or("Invalid date time combination")?;
                let diff = dt1.signed_duration_since(dt2);
                Ok(TemporalValue::Duration(diff))
            }
            (TemporalValue::DateTime(dt1), TemporalValue::DateTime(dt2)) => {
                let diff = dt1.signed_duration_since(*dt2);
                Ok(TemporalValue::Duration(diff))
            }
            _ => Err(format!(
                "Cannot compute difference between {} and {}",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    /// Compare two temporal values
    pub fn compare(&self, other: &TemporalValue) -> Result<std::cmp::Ordering, String> {
        match (self, other) {
            (TemporalValue::Date(d1), TemporalValue::Date(d2)) => Ok(d1.cmp(d2)),
            (TemporalValue::DateTime(dt1), TemporalValue::DateTime(dt2)) => Ok(dt1.cmp(dt2)),
            (TemporalValue::Duration(d1), TemporalValue::Duration(d2)) => Ok(d1.cmp(d2)),
            _ => Err(format!(
                "Cannot compare {} with {}",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    /// Check if this is before another temporal value
    pub fn is_before(&self, other: &TemporalValue) -> Result<bool, String> {
        Ok(self.compare(other)? == std::cmp::Ordering::Less)
    }

    /// Check if this is after another temporal value
    pub fn is_after(&self, other: &TemporalValue) -> Result<bool, String> {
        Ok(self.compare(other)? == std::cmp::Ordering::Greater)
    }

    /// Get the number of days in a duration
    pub fn days(&self) -> Result<i64, String> {
        match self {
            TemporalValue::Duration(d) => Ok(d.num_days()),
            _ => Err(format!("Cannot get days from {}", self.type_name())),
        }
    }

    /// Get the number of seconds in a duration
    pub fn seconds(&self) -> Result<i64, String> {
        match self {
            TemporalValue::Duration(d) => Ok(d.num_seconds()),
            _ => Err(format!("Cannot get seconds from {}", self.type_name())),
        }
    }

    /// Get start of day for a date
    pub fn start_of_day(&self) -> Result<TemporalValue, String> {
        match self {
            TemporalValue::Date(date) => {
                let dt = date
                    .and_hms_opt(0, 0, 0)
                    .ok_or("Invalid date time combination")?;
                Ok(TemporalValue::DateTime(Utc.from_utc_datetime(&dt)))
            }
            TemporalValue::DateTime(dt) => {
                let date = dt.date_naive();
                let start_dt = date
                    .and_hms_opt(0, 0, 0)
                    .ok_or("Invalid date time combination")?;
                Ok(TemporalValue::DateTime(Utc.from_utc_datetime(&start_dt)))
            }
            _ => Err(format!("Cannot get start of day from {}", self.type_name())),
        }
    }

    /// Get end of day for a date
    pub fn end_of_day(&self) -> Result<TemporalValue, String> {
        match self {
            TemporalValue::Date(date) => {
                let dt = date
                    .and_hms_opt(23, 59, 59)
                    .ok_or("Invalid date time combination")?;
                Ok(TemporalValue::DateTime(Utc.from_utc_datetime(&dt)))
            }
            TemporalValue::DateTime(dt) => {
                let date = dt.date_naive();
                let end_dt = date
                    .and_hms_opt(23, 59, 59)
                    .ok_or("Invalid date time combination")?;
                Ok(TemporalValue::DateTime(Utc.from_utc_datetime(&end_dt)))
            }
            _ => Err(format!("Cannot get end of day from {}", self.type_name())),
        }
    }

    /// Format as ISO8601 string
    pub fn to_iso8601(&self) -> String {
        match self {
            TemporalValue::Date(date) => date.to_string(),
            TemporalValue::DateTime(dt) => dt.to_rfc3339(),
            TemporalValue::Duration(d) => {
                let days = d.num_days();
                let secs = d.num_seconds() % 86400;
                format!("P{}DT{}S", days, secs)
            }
        }
    }
}

impl fmt::Display for TemporalValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_iso8601())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_parse_date() {
        let date = TemporalValue::parse_date("2024-01-15").unwrap();
        match date {
            TemporalValue::Date(d) => {
                assert_eq!(d.year(), 2024);
                assert_eq!(d.month(), 1);
                assert_eq!(d.day(), 15);
            }
            _ => panic!("Expected Date"),
        }
    }

    #[test]
    fn test_parse_duration_days() {
        let duration = TemporalValue::parse_duration("P5D").unwrap();
        match duration {
            TemporalValue::Duration(d) => {
                assert_eq!(d.num_days(), 5);
            }
            _ => panic!("Expected Duration"),
        }
    }

    #[test]
    fn test_parse_duration_hours() {
        let duration = TemporalValue::parse_duration("PT2H").unwrap();
        match duration {
            TemporalValue::Duration(d) => {
                assert_eq!(d.num_hours(), 2);
            }
            _ => panic!("Expected Duration"),
        }
    }

    #[test]
    fn test_add_duration_to_date() {
        let date = TemporalValue::parse_date("2024-01-15").unwrap();
        let duration = TemporalValue::parse_duration("P5D").unwrap();
        let result = date.add_duration(&duration).unwrap();

        match result {
            TemporalValue::DateTime(dt) => {
                assert_eq!(dt.date_naive().day(), 20);
            }
            _ => panic!("Expected DateTime"),
        }
    }

    #[test]
    fn test_subtract_duration_from_date() {
        let date = TemporalValue::parse_date("2024-01-15").unwrap();
        let duration = TemporalValue::parse_duration("P10D").unwrap();
        let result = date.subtract_duration(&duration).unwrap();

        match result {
            TemporalValue::DateTime(dt) => {
                assert_eq!(dt.date_naive().day(), 5);
            }
            _ => panic!("Expected DateTime"),
        }
    }

    #[test]
    fn test_date_comparison() {
        let date1 = TemporalValue::parse_date("2024-01-15").unwrap();
        let date2 = TemporalValue::parse_date("2024-01-20").unwrap();

        assert!(date1.is_before(&date2).unwrap());
        assert!(date2.is_after(&date1).unwrap());
    }

    #[test]
    fn test_difference_between_dates() {
        let date1 = TemporalValue::parse_date("2024-01-15").unwrap();
        let date2 = TemporalValue::parse_date("2024-01-20").unwrap();

        let diff = date2.difference(&date1).unwrap();
        match diff {
            TemporalValue::Duration(d) => {
                assert_eq!(d.num_days(), 5);
            }
            _ => panic!("Expected Duration"),
        }
    }

    #[test]
    fn test_duration_arithmetic() {
        let d1 = TemporalValue::parse_duration("P2D").unwrap();
        let d2 = TemporalValue::parse_duration("P3D").unwrap();

        let sum = d1.add_duration(&d2).unwrap();
        match sum {
            TemporalValue::Duration(d) => {
                assert_eq!(d.num_days(), 5);
            }
            _ => panic!("Expected Duration"),
        }
    }

    #[test]
    fn test_start_end_of_day() {
        let date = TemporalValue::parse_date("2024-01-15").unwrap();

        let start = date.start_of_day().unwrap();
        let end = date.end_of_day().unwrap();

        match (&start, &end) {
            (TemporalValue::DateTime(dt1), TemporalValue::DateTime(dt2)) => {
                assert_eq!(dt1.date_naive().day(), dt2.date_naive().day());
                assert_eq!(dt1.format("%H:%M:%S").to_string(), "00:00:00");
                assert_eq!(dt2.format("%H:%M:%S").to_string(), "23:59:59");
            }
            _ => panic!("Expected DateTimes"),
        }
    }

    #[test]
    fn test_invalid_date_parsing() {
        let result = TemporalValue::parse_date("2024-13-45");
        assert!(result.is_err());
    }

    #[test]
    fn test_today() {
        let today = TemporalValue::today();
        match today {
            TemporalValue::Date(_) => {}
            _ => panic!("Expected Date"),
        }
    }

    #[test]
    fn test_type_name() {
        assert_eq!(
            TemporalValue::parse_date("2024-01-15").unwrap().type_name(),
            "date"
        );
        assert_eq!(
            TemporalValue::parse_duration("P1D").unwrap().type_name(),
            "duration"
        );
    }
}
