#[cfg(feature = "local")]
use std::net::SocketAddr;

#[cfg(feature = "local")]
use anyhow::Result;

#[cfg(feature = "local")]
#[tokio::main]
async fn main() -> Result<()> {
    let router = zerocal::get_router();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;
    Ok(())
}

#[cfg(not(feature = "local"))]
fn main() {
    // dummy main function if shuttle is used
}
