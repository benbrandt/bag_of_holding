//! # Metrics
//!
//! Metrics-related setup and initialization code

use std::net::SocketAddr;

use axum::{extract::MatchedPath, http::Request, middleware::Next, response::IntoResponse};
use metrics_exporter_prometheus::PrometheusBuilder;

/// Track path-related metrics
#[tracing::instrument(skip_all)]
pub async fn track_requests<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
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

    metrics::increment_counter!("app_http_requests_total", &labels);

    response
}

#[tracing::instrument]
pub fn init(metrics_port: u16) {
    // Metrics setup. Listening on separate port than the app
    PrometheusBuilder::new()
        .with_http_listener(SocketAddr::from(([0, 0, 0, 0], metrics_port)))
        .install()
        .expect("failed to start metrics endpoint");
}
