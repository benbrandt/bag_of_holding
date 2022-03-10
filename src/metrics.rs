//! # Metrics
//!
//! Metrics-related setup and initialization code

use std::net::SocketAddr;

use axum::{extract::MatchedPath, http::Request, middleware::Next, response::IntoResponse};
use metrics::KeyName;
use metrics_exporter_prometheus::PrometheusBuilder;
use strum::IntoStaticStr;
use tracing_subscriber::{
    fmt, prelude::__tracing_subscriber_SubscriberExt, registry, util::SubscriberInitExt, EnvFilter,
};

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
pub(crate) async fn track_metrics<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };
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

/// Initialize all metrics configuration and subscribers for the app
pub(crate) fn init_tracing_and_metrics(metrics_port: u16) -> anyhow::Result<()> {
    // Setup tracing
    registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer())
        .with(sentry::integrations::tracing::layer())
        .init();

    // Metrics setup. Listening on separate port than the app
    PrometheusBuilder::new()
        .with_http_listener(SocketAddr::from(([0, 0, 0, 0], metrics_port)))
        .install()?;

    Ok(())
}
