use std::{sync::Arc, time::Duration};

use axum::{
    error_handling::HandleErrorLayer,
    http::{header, StatusCode},
    middleware::{self},
    response::IntoResponse,
    BoxError, Router,
};
use sentry_tower::{NewSentryLayer, SentryHttpLayer};
use tower::ServiceBuilder;
use tower_http::{catch_panic::CatchPanicLayer, trace::TraceLayer, ServiceBuilderExt};

use self::{dice::dice_routes, metrics::track_metrics};

mod dice;
pub mod metrics;

/// Top-level app. To be consumed by main.rs and
#[tracing::instrument]
pub fn app() -> Router {
    // Mark the `Authorization` and `Cookie` headers as sensitive so it doesn't show in logs
    let sensitive_headers: Arc<[_]> = vec![header::AUTHORIZATION, header::COOKIE].into();

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
        .layer(SentryHttpLayer::with_transaction())
        // Recall after tracing
        .sensitive_response_headers(sensitive_headers)
        // Set a timeout
        .timeout(Duration::from_secs(10))
        // Compress responses
        .compression();

    Router::new()
        .nest("/dice", dice_routes())
        .layer(middleware)
        .route_layer(middleware::from_fn(track_metrics))
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
