use anyhow::Result;
use icalendar::*;
use std::collections::HashMap;

use crate::time::{parse_duration, parse_time};
use chrono::{DateTime, Duration, Utc};

const DEFAULT_EVENT_TITLE: &str = "New Calendar Event";
const DEFAULT_DESCRIPTION: &str = "Powered by zerocal.shuttleapp.rs";

pub(crate) fn parse_opt_time(time: Option<&String>) -> Result<Option<DateTime<Utc>>> {
    // Turn time into Option<Result>
    time.filter(|s| !s.is_empty())
        .map(|s| parse_time(s))
        // Turn Option<Result> into Result<Option>
        .map_or(Ok(None), |r| r.map(Some))
}

pub fn create_calendar(params: HashMap<String, String>) -> Result<Calendar> {
    #[allow(non_snake_case)]
    let DEFAULT_EVENT_DURATION: Duration = Duration::hours(1);
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

    let start = parse_opt_time(params.get("start")).map_err(|e| e.context("Invalid start time"))?;
    let end = parse_opt_time(params.get("end")).map_err(|e| e.context("Invalid end time"))?;
    let duration = params
        .get("duration")
        .map(|d| parse_duration(d))
        // go from Option<Result> to Result<Option>
        .map_or(Ok(None), |r| r.map(Some))
        .map_err(|e| e.context("Invalid duration"))?;

    let (start, end) = match (start, end, duration) {
        // If start + end + duration given, ignore duration (or assume it's correct).
        (Some(start), Some(end), _) => (start, end),

        // If given either start or end, use that plus duration (or default duration).
        (Some(start), None, dur) => (start, start + dur.unwrap_or(DEFAULT_EVENT_DURATION)),
        (None, Some(end), dur) => (end - dur.unwrap_or(DEFAULT_EVENT_DURATION), end),

        // If not given a start or an end, use now() and duration (or default duration).
        (None, None, dur) => (
            chrono::Utc::now(),
            chrono::Utc::now() + dur.unwrap_or(DEFAULT_EVENT_DURATION),
        ),
    };
    event.starts(start);
    event.ends(end);

    if let Some(location) = params.get("location") {
        event.location(location);
    }

    Ok(Calendar::new().push(event.done()).done())
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    fn unix_to_datetime(unix: i64) -> DatePerhapsTime {
        DatePerhapsTime::DateTime(CalendarDateTime::Utc(Utc.timestamp(unix, 0)))
    }

    #[test]
    fn basic_defaults() {
        let cal = create_calendar(HashMap::from([
            ("start".into(), "1511648546".into()),
            ("end".into(), "1511648547".into()),
        ]))
        .unwrap();
        let event = cal.get(0).unwrap().as_event().unwrap();

        assert_eq!(event.get_start().unwrap(), unix_to_datetime(1511648546));
        assert_eq!(event.get_end().unwrap(), unix_to_datetime(1511648547));
        assert_eq!(event.get_description().unwrap(), DEFAULT_DESCRIPTION);
        assert_eq!(event.get_summary().unwrap(), DEFAULT_EVENT_TITLE);
        assert!(event.get_location().is_none());
    }

    #[test]
    fn default_duration_with_start() {
        let cal = create_calendar(HashMap::from([("start".into(), "1511648546".into())])).unwrap();
        let event = cal.get(0).unwrap().as_event().unwrap();

        assert_eq!(
            event.get_end().unwrap(),
            unix_to_datetime(1511648546 + 3600)
        );
    }

    #[test]
    fn default_duration_with_end() {
        let cal = create_calendar(HashMap::from([("end".into(), "1511648546".into())])).unwrap();
        let event = cal.get(0).unwrap().as_event().unwrap();

        assert_eq!(
            event.get_start().unwrap(),
            unix_to_datetime(1511648546 - 3600)
        );
    }

    #[test]
    fn description_summary_location() {
        let cal = create_calendar(HashMap::from([
            ("desc".into(), "description".into()),
            ("title".into(), "summary".into()),
            ("location".into(), "location".into()),
        ]))
        .unwrap();
        let event = cal.get(0).unwrap().as_event().unwrap();

        assert_eq!(event.get_description().unwrap(), "description");
        assert_eq!(event.get_location().unwrap(), "location");
        assert_eq!(event.get_summary().unwrap(), "summary");
    }
}
