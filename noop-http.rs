use axum::{routing::any, Router};
use axum::http::StatusCode;
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() {
    let port: u16 = env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let stall = env::var("STALL")
        .map(|v| matches!(v.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"))
        .unwrap_or(false);

    let app = if stall {
        // Accept requests but never respond (connection just hangs).
        Router::new().fallback(|| async {
            // Await forever — the client is connected, but we do "nothing".
            futures::future::pending::<()>().await;
            // Unreachable, but gives the compiler a concrete type:
            StatusCode::NO_CONTENT
        })
    } else {
        // Fast no-op: answer anything with 204 No Content.
        Router::new()
            .route("/", any(|| async { StatusCode::NO_CONTENT }))
            .fallback(|| async { StatusCode::NO_CONTENT })
    };

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("noop http server listening on http://{addr} (STALL={stall})");

    axum::serve(listener, app).await.unwrap();
}
