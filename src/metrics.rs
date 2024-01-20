//! # Metrics
//!
//! Metrics-related tracking code

use axum::{
    body::Body, extract::MatchedPath, http::Request, middleware::Next, response::IntoResponse,
};

/// Track path-related metrics
pub async fn track_requests(req: Request<Body>, next: Next) -> impl IntoResponse {
    let path = req.extensions().get::<MatchedPath>().map_or_else(
        || req.uri().path().to_owned(),
        |matched_path| matched_path.as_str().to_owned(),
    );
    let method = req.method().clone();

    let response = next.run(req).await;
    let status = response.status().as_u16().to_string();

    let labels = [
        ("method", method.to_string()),
        ("path", path),
        ("status", status),
    ];

    metrics::counter!("app_http_requests_total", &labels).increment(1);

    response
}
