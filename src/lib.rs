use anyhow::Result;
use axum::routing::post;
use axum::{
    body::{boxed, Full},
    extract::Query,
    http::{HeaderValue, StatusCode},
};
use axum::{
    http::header,
    response::{IntoResponse, Response},
};
use axum::{routing::get, Router};
use icalendar::*;
use std::collections::HashMap;

#[cfg(feature = "shuttle")]
use sync_wrapper::SyncWrapper;

mod time;

use time::{parse_duration, parse_time};

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

/// Helper function to return a `BAD_REQUEST` status code with a custom message
fn bad_request(body: String) -> Response {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(boxed(Full::from(body)))
        .unwrap()
}

pub async fn calendar(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
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
            let start = match parse_time(start) {
                Ok(start) => start,
                Err(e) => {
                    return bad_request(format!("Invalid start time: {}", e));
                }
            };
            event.starts(start);
            if let Some(duration) = params.get("duration") {
                let duration = match parse_duration(duration) {
                    Ok(duration) => duration,
                    Err(e) => {
                        return bad_request(format!("Invalid duration: {}", e));
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
                    return bad_request(format!("Invalid end time: {}", e));
                }
            };
            event.ends(end);
            if let Some(duration) = params.get("duration") {
                if params.get("start").is_none() {
                    // If only duration is given but no start, set start to end - duration
                    let duration = match parse_duration(duration) {
                        Ok(duration) => duration,
                        Err(e) => {
                            return bad_request(format!("Invalid duration: {}", e));
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

    let ical = Calendar::new().push(event.done()).done();

    CalendarResponse(ical).into_response()
}

// cfg if shuttle feature is enabled
#[cfg(feature = "shuttle")]
#[shuttle_service::main]
async fn axum() -> shuttle_service::ShuttleAxum {
    let router = Router::new()
        .route("/", get(calendar))
        .route("/", post(calendar));
    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}
