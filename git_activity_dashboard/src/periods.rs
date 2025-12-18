//! Time period strategies for activity aggregation
//!
//! Implements the PeriodStrategy trait for daily, weekly, and monthly periods.

use crate::traits::PeriodStrategy;
use chrono::{DateTime, Datelike, Duration, Utc};

/// Daily period strategy
#[derive(Debug, Clone, Default)]
pub struct DailyPeriod;

impl PeriodStrategy for DailyPeriod {
    fn boundaries(&self, index: u32) -> (DateTime<Utc>, DateTime<Utc>) {
        let now = Utc::now();
        let day = now - Duration::days(index as i64);
        let start = day.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();
        let end = day.date_naive().and_hms_opt(23, 59, 59).unwrap().and_utc();
        (start, end)
    }

    fn label(&self, index: u32) -> String {
        let now = Utc::now();
        let day = now - Duration::days(index as i64);
        day.format("%A, %b %d").to_string()
    }
}

/// Weekly period strategy (weeks start on Monday)
#[derive(Debug, Clone, Default)]
pub struct WeeklyPeriod;

impl PeriodStrategy for WeeklyPeriod {
    fn boundaries(&self, index: u32) -> (DateTime<Utc>, DateTime<Utc>) {
        let now = Utc::now();
        let days_since_monday = now.weekday().num_days_from_monday();
        let week_start = now - Duration::days((days_since_monday + index * 7) as i64);
        let start = week_start.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();
        let end = start + Duration::days(7) - Duration::seconds(1);
        (start, end)
    }

    fn label(&self, index: u32) -> String {
        let (start, _) = self.boundaries(index);
        format!("Week of {}", start.format("%b %d"))
    }
}

/// Monthly period strategy
#[derive(Debug, Clone, Default)]
pub struct MonthlyPeriod;

impl PeriodStrategy for MonthlyPeriod {
    fn boundaries(&self, index: u32) -> (DateTime<Utc>, DateTime<Utc>) {
        let now = Utc::now();
        let mut year = now.year();
        let mut month = now.month() as i32 - index as i32;

        while month <= 0 {
            month += 12;
            year -= 1;
        }

        let month = month as u32;
        let start = chrono::NaiveDate::from_ymd_opt(year, month, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc();

        let next_month = if month == 12 { 1 } else { month + 1 };
        let next_year = if month == 12 { year + 1 } else { year };
        let end = chrono::NaiveDate::from_ymd_opt(next_year, next_month, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            - Duration::seconds(1);

        (start, end)
    }

    fn label(&self, index: u32) -> String {
        let (start, _) = self.boundaries(index);
        start.format("%B %Y").to_string()
    }
}

/// Quarterly period strategy
#[derive(Debug, Clone, Default)]
pub struct QuarterlyPeriod;

impl PeriodStrategy for QuarterlyPeriod {
    fn boundaries(&self, index: u32) -> (DateTime<Utc>, DateTime<Utc>) {
        let now = Utc::now();
        let current_quarter = (now.month() - 1) / 3;
        let target_quarter = (current_quarter as i32 - index as i32).rem_euclid(4) as u32;
        let years_back = (index as i32 + (3 - current_quarter as i32)) / 4;
        let year = now.year() - years_back;

        let start_month = target_quarter * 3 + 1;
        let end_month = start_month + 3;

        let start = chrono::NaiveDate::from_ymd_opt(year, start_month, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc();

        let (end_year, end_month) = if end_month > 12 {
            (year + 1, end_month - 12)
        } else {
            (year, end_month)
        };

        let end = chrono::NaiveDate::from_ymd_opt(end_year, end_month, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            - Duration::seconds(1);

        (start, end)
    }

    fn label(&self, index: u32) -> String {
        let (start, _) = self.boundaries(index);
        let quarter = (start.month() - 1) / 3 + 1;
        format!("Q{} {}", quarter, start.year())
    }
}

/// Custom date range period
#[derive(Debug, Clone)]
pub struct CustomPeriod {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub label: String,
}

impl PeriodStrategy for CustomPeriod {
    fn boundaries(&self, _index: u32) -> (DateTime<Utc>, DateTime<Utc>) {
        (self.start, self.end)
    }

    fn label(&self, _index: u32) -> String {
        self.label.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Timelike, Weekday};

    #[test]
    fn test_daily_period_boundaries() {
        let period = DailyPeriod;
        let (start, end) = period.boundaries(0);

        // Start should be midnight today
        assert_eq!(start.hour(), 0);
        assert_eq!(start.minute(), 0);

        // End should be 23:59:59 today
        assert_eq!(end.hour(), 23);
        assert_eq!(end.minute(), 59);
    }

    #[test]
    fn test_weekly_period_boundaries() {
        let period = WeeklyPeriod;
        let (start, end) = period.boundaries(0);

        // Start should be Monday
        assert_eq!(start.weekday(), Weekday::Mon);

        // End should be Sunday
        assert_eq!(end.weekday(), Weekday::Sun);
    }

    #[test]
    fn test_monthly_period_boundaries() {
        let period = MonthlyPeriod;
        let (start, end) = period.boundaries(0);

        // Start should be 1st of month
        assert_eq!(start.day(), 1);

        // End should be last day of month
        let next_month_start = end + Duration::seconds(1);
        assert_eq!(next_month_start.day(), 1);
    }

    #[test]
    fn test_period_labels() {
        let daily = DailyPeriod;
        let weekly = WeeklyPeriod;
        let monthly = MonthlyPeriod;

        // Labels should not be empty
        assert!(!daily.label(0).is_empty());
        assert!(weekly.label(0).starts_with("Week of"));
        assert!(!monthly.label(0).is_empty());
    }
}
