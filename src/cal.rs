use anyhow::Result;
use icalendar::*;
use std::collections::HashMap;

use crate::time::{parse_duration, parse_time};

const DEFAULT_EVENT_TITLE: &str = "New Calendar Event";
const DEFAULT_DESCRIPTION: &str = "Powered by zerocal.shuttleapp.rs";

pub fn create_calendar(params: HashMap<String, String>) -> Result<Calendar> {
    let mut event = Event::new();

    event.summary(
        params
            .get("title")
            .map(String::as_str)
            .unwrap_or(DEFAULT_EVENT_TITLE),
    );
    event.description(
        params
            .get("desc")
            .map(String::as_str)
            .unwrap_or(DEFAULT_DESCRIPTION),
    );

    match params.get("start") {
        Some(start) if !start.is_empty() => {
            let start = match parse_time(start) {
                Ok(start) => start,
                Err(e) => {
                    return Err(e.context("Invalid start time"));
                }
            };
            event.starts(start);
            if let Some(duration) = params.get("duration") {
                let duration = match parse_duration(duration) {
                    Ok(duration) => duration,
                    Err(e) => {
                        return Err(e.context("Invalid duration"));
                    }
                };
                event.ends(start + duration);
            }
        }
        _ => {
            // start is not set or empty; default to 1 hour event
            event.starts(chrono::Utc::now());
            event.ends(chrono::Utc::now() + chrono::Duration::hours(1));
        }
    }

    match params.get("end") {
        Some(end) if !end.is_empty() => {
            let end = match parse_time(end) {
                Ok(end) => end,
                Err(e) => {
                    return Err(e.context("Invalid end time"));
                }
            };
            event.ends(end);
            if let Some(duration) = params.get("duration") {
                if params.get("start").is_none() {
                    // If only duration is given but no start, set start to end - duration
                    let duration = match parse_duration(duration) {
                        Ok(duration) => duration,
                        Err(e) => {
                            return Err(e.context("Invalid duration"));
                        }
                    };
                    event.starts(end - duration);
                }
            }
        }
        _ => {
            // end is not set or empty; default to 1 hour event
            // TODO: handle case where start is set
            event.starts(chrono::Utc::now());
            event.ends(chrono::Utc::now() + chrono::Duration::hours(1));
        }
    }

    if let Some(location) = params.get("location") {
        event.location(location);
    }

    Ok(Calendar::new().push(event.done()).done())
}
