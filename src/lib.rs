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

use axum::{
    error_handling::HandleErrorLayer,
    http::{header, StatusCode},
    middleware,
    response::IntoResponse,
    BoxError, Router,
};
use metrics_exporter_prometheus::PrometheusBuilder;
use once_cell::sync::Lazy;
use sentry::integrations::tower::{NewSentryLayer, SentryHttpLayer};
use tower::ServiceBuilder;
use tower_http::{catch_panic::CatchPanicLayer, trace::TraceLayer, ServiceBuilderExt};
use tracing_subscriber::{
    fmt, prelude::__tracing_subscriber_SubscriberExt, registry, util::SubscriberInitExt, EnvFilter,
};

mod abilities;
mod dice;
mod metrics;

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
        // Turn panics into a 500
        .layer(CatchPanicLayer::new())
        // Handle errors from middleware
        //
        // This middleware most be added above any fallible
        // ones if you're using `ServiceBuilder`, due to how ordering works
        .layer(HandleErrorLayer::new(handle_errors))
        // Call before tracing
        .sensitive_request_headers(sensitive_headers.clone())
        // `TraceLayer` adds high level tracing and logging
        .layer(TraceLayer::new_for_http())
        // Sentry setup
        .layer(NewSentryLayer::new_from_top())
        // Recall after tracing
        .sensitive_response_headers(sensitive_headers)
        // Set a timeout
        .timeout(Duration::from_secs(10))
        // Compress responses
        .compression();

    // Middleware that should only run if the request matches a route
    let route_middleware = ServiceBuilder::new()
        // Add metrics tracking to endpoints
        .layer(middleware::from_fn(metrics::track_requests))
        // Start performance transactions for matched requests
        .layer(SentryHttpLayer::with_transaction());

    Router::new()
        .nest("/dice", dice::routes())
        .merge(abilities::routes())
        .layer(middleware)
        .route_layer(route_middleware)
}

/// Handle errors propagated from middleware
#[tracing::instrument]
async fn handle_errors(err: BoxError) -> impl IntoResponse {
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            "Request took too long".to_string(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", err),
        )
    }
}
