use anyhow::{Context, Result};
use chrono::Duration;

/// Helper function to parse all supported duration formats
pub(crate) fn parse_duration(duration: &str) -> Result<Duration> {
    let duration = humantime::parse_duration(duration)?;
    Ok(Duration::from_std(duration)?)
}

/// Helper function to parse all supported time formats
pub(crate) fn parse_time(time: &str) -> Result<chrono::DateTime<chrono::Utc>> {
    match dateparser::parse(time) {
        Ok(date) => Ok(date),
        Err(_) => {
            let time = chrono::NaiveDateTime::parse_from_str(time, "%Y-%m-%dT%H:%M")
                .context("Invalid time format. Please use ISO 8601 or RFC 3339 format.")?;
            Ok(chrono::DateTime::<chrono::Utc>::from_utc(time, chrono::Utc))
        }
    }
}
