//! # `bag_of_holding`
//!
//! Top-level app for running the bag of holding server.

#![warn(
    clippy::pedantic,
    future_incompatible,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    unused
)]

use std::{sync::Arc, time::Duration};

use axum::{http::header, middleware, Router};
use metrics_exporter_prometheus::PrometheusBuilder;
use once_cell::sync::Lazy;
use sentry::integrations::tower::{NewSentryLayer, SentryHttpLayer};
use tower::ServiceBuilder;
use tower_http::{timeout::TimeoutLayer, ServiceBuilderExt};
use tracing_subscriber::{
    fmt, prelude::__tracing_subscriber_SubscriberExt, registry, util::SubscriberInitExt, EnvFilter,
};

mod abilities;
mod characters;
mod dice;
mod metrics;
mod names;

// Setup tracing
static TRACING: Lazy<()> = Lazy::new(|| {
    registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer())
        .with(sentry::integrations::tracing::layer())
        .init();
});

// Metrics setup. Listening on separate port than the app
static METRICS: Lazy<()> = Lazy::new(|| {
    PrometheusBuilder::new()
        .install()
        .expect("failed to start metrics endpoint");
});

/// Top-level app. To be consumed by main.rs and
#[must_use]
pub fn app() -> Router {
    // In once_cells so they work in test threads
    Lazy::force(&TRACING);
    Lazy::force(&METRICS);

    // Mark the `Authorization` and `Cookie` headers as sensitive so it doesn't show in logs
    let sensitive_headers: Arc<[_]> = vec![header::AUTHORIZATION, header::COOKIE].into();

    // Middleware for entire service
    let middleware = ServiceBuilder::new()
        // Strip sensitive request headers
        .sensitive_request_headers(sensitive_headers.clone())
        // `TraceLayer` adds high level tracing and logging
        .trace_for_http()
        // Sentry setup
        .layer(NewSentryLayer::new_from_top())
        // Set a timeout
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        // Turn panics into a 500
        .catch_panic()
        // Compress responses
        .compression()
        // Strip sensitive response headers
        .sensitive_response_headers(sensitive_headers);

    // Middleware that should only run if the request matches a route
    let route_middleware = ServiceBuilder::new()
        // Add metrics tracking to endpoints
        .layer(middleware::from_fn(metrics::track_requests))
        // Start performance transactions for matched requests
        .layer(SentryHttpLayer::with_transaction());

    Router::new()
        .nest("/dice", dice::routes())
        .merge(abilities::routes())
        .merge(characters::routes())
        .merge(names::routes())
        .layer(middleware)
        .route_layer(route_middleware)
}
