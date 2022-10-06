use axum::{
    body::{boxed, Full},
    extract::Query,
    http::HeaderValue,
    routing::post,
};
use axum::{
    http::header,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use icalendar::*;
use std::collections::HashMap;
use sync_wrapper::SyncWrapper;

const DEFAULT_EVENT_TITLE: &str = "New Calendar Event";

/// Newtype wrapper around Calendar for `IntoResponse` impl
#[derive(Debug)]
pub struct CalendarResponse(pub Calendar);

impl IntoResponse for CalendarResponse {
    fn into_response(self) -> Response {
        let mut res = Response::new(boxed(Full::from(self.0.to_string())));
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("text/calendar"),
        );
        res
    }
}

async fn calendar(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    // if query is empty, show form
    if params.is_empty() || params.values().all(|s| s.is_empty()) {
        return Response::builder()
            .status(200)
            .body(boxed(Full::from(include_str!("../static/index.html"))))
            .unwrap();
    }

    let mut event = Event::new();

    if let Some(title) = params.get("title") {
        event.summary(title);
    } else {
        event.summary(DEFAULT_EVENT_TITLE);
    }
    if let Some(desc) = params.get("desc") {
        event.description(desc);
    } else {
        event.description("Powered by zerocal.shuttleapp.rs");
    }

    match params.get("start") {
        Some(start) if !start.is_empty() => {
            let start = dateparser::parse(start).unwrap_or_else(|_| {
                let time = chrono::NaiveDateTime::parse_from_str(start, "%Y-%m-%dT%H:%M").unwrap();
                chrono::DateTime::<chrono::Utc>::from_utc(time, chrono::Utc)
            });
            event.starts(start);
            if let Some(duration) = params.get("duration") {
                let duration = humantime::parse_duration(duration).unwrap();
                let duration = chrono::Duration::from_std(duration).unwrap();
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
            let end = dateparser::parse(end).unwrap_or_else(|_| {
                let time = chrono::NaiveDateTime::parse_from_str(end, "%Y-%m-%dT%H:%M").unwrap();
                chrono::DateTime::<chrono::Utc>::from_utc(time, chrono::Utc)
            });

            event.ends(end);
            if let Some(duration) = params.get("duration") {
                if params.get("start").is_none() {
                    // If only duration is given but no start, set start to end - duration
                    let duration = humantime::parse_duration(duration).unwrap();
                    let duration = chrono::Duration::from_std(duration).unwrap();
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

    let ical = Calendar::new().push(event.done()).done();

    CalendarResponse(ical).into_response()
}

#[shuttle_service::main]
async fn axum() -> shuttle_service::ShuttleAxum {
    let router = Router::new()
        .route("/", get(calendar))
        .route("/", post(calendar));
    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}
