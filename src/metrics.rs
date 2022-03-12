//! # Metrics
//!
//! Metrics-related setup and initialization code

use axum::{extract::MatchedPath, http::Request, middleware::Next, response::IntoResponse};
use metrics::KeyName;
use strum::IntoStaticStr;

/// Metric name constants
#[derive(IntoStaticStr)]
pub enum MetricName {
    #[strum(serialize = "app_http_requests_total")]
    HttpRequestsTotal,
}

impl From<MetricName> for KeyName {
    fn from(name: MetricName) -> Self {
        name.into()
    }
}

/// Track path-related metrics
#[tracing::instrument(skip_all)]
pub async fn track_metrics<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
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

    metrics::increment_counter!(MetricName::HttpRequestsTotal, &labels);

    response
}
