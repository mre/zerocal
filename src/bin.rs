#[cfg(feature = "local")]
use axum::{
    routing::{get, post},
    Router,
};

#[cfg(feature = "local")]
use zerocal::calendar;

#[cfg(feature = "local")]
use std::net::SocketAddr;

#[cfg(feature = "local")]
#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(calendar))
        .route("/", post(calendar));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "local"))]
fn main() {
    // dummy main function if shuttle is used
}
