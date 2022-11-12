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

mod cal;
mod time;

/// Newtype wrapper around Calendar for `IntoResponse` impl
#[derive(Debug)]
pub struct CalendarResponse(pub Calendar);
pub struct BytesResponse {
    pub bytes: Vec<u8>,
    pub content_type: &'static str,
}

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

impl IntoResponse for BytesResponse {
    fn into_response(self) -> Response {
        let mut res = Response::new(boxed(Full::from(self.bytes)));
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static(self.content_type),
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

pub async fn qr(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    // Get calendar
    let cal = match cal::create_calendar(params) {
        Ok(cal) => cal,
        Err(e) => {
            return bad_request(e.err);
        }
    };

    let qr = match qrcode_generator::to_png_to_vec(
        cal.to_string(),
        qrcode_generator::QrCodeEcc::Medium,
        256,
    ) {
        Ok(qr) => qr,
        Err(e) => {
            return bad_request(format!("Failed to turn calendar into qr code: {}", e));
        }
    };

    BytesResponse {
        bytes: qr,
        content_type: "image/png",
    }
    .into_response()
}

pub async fn calendar(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    // if query is empty, show form
    if params.is_empty() || params.values().all(String::is_empty) {
        return Response::builder()
            .status(200)
            .body(boxed(Full::from(include_str!("../static/index.html"))))
            .unwrap();
    }

    let ical = match cal::create_calendar(params) {
        Ok(cal) => cal,
        Err(e) => {
            return bad_request(e.err);
        }
    };

    CalendarResponse(ical).into_response()
}

pub fn get_router() -> Router {
    Router::new()
        .route("/qr", get(qr))
        .route("/", get(calendar))
        .route("/", post(calendar))
}

// cfg if shuttle feature is enabled
#[cfg(feature = "shuttle")]
#[shuttle_service::main]
async fn axum() -> shuttle_service::ShuttleAxum {
    let router = get_router();
    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}
