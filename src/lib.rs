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

use std::{
    net::{SocketAddr, TcpListener},
    sync::Arc,
    time::Duration,
};

use axum::{http::header, middleware, Router};
use axum_server::tls_rustls::RustlsConfig;
use clap::Parser;
use metrics_exporter_prometheus::PrometheusBuilder;
use once_cell::sync::Lazy;
use sentry::integrations::tower::{NewSentryLayer, SentryHttpLayer};
use tower::ServiceBuilder;
use tower_http::{timeout::TimeoutLayer, ServiceBuilderExt};
use tracing_subscriber::{
    fmt, prelude::__tracing_subscriber_SubscriberExt, registry, util::SubscriberInitExt, EnvFilter,
};

mod abilities;
mod alignments;
mod characters;
mod deities;
mod dice;
mod metrics;
mod names;
mod sizes;

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
fn app() -> Router {
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
        .merge(alignments::routes())
        .merge(characters::routes())
        .merge(deities::routes())
        .merge(names::routes())
        .merge(sizes::routes())
        .layer(middleware)
        .route_layer(route_middleware)
}

/// Command line arguments
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct CliConfig {
    /// The port to listen on for the app
    #[clap(value_parser, long, short, default_value = "5000")]
    port: u16,
    /// SSL Certificate value
    #[clap(value_parser, env, long)]
    ssl_cert: Option<String>,
    /// SSL Key value
    #[clap(value_parser, env, long)]
    ssl_key: Option<String>,
}

/// Derived server config from `Config` options
#[derive(Debug)]
pub struct Config {
    /// Bound TCP Listener for the designated port
    listener: TcpListener,
    /// Config for serving over HTTPS
    tls: Option<RustlsConfig>,
}

impl Config {
    /// Generate a new server config
    #[must_use]
    pub fn new(listener: TcpListener, tls: Option<RustlsConfig>) -> Self {
        Self { listener, tls }
    }

    /// Parse config from command line
    ///
    /// # Errors
    /// Errors if can't bind to port or read from cert files
    pub async fn parse() -> std::io::Result<Self> {
        let config = CliConfig::parse();

        let tls = if let (Some(cert), Some(key)) = (config.ssl_cert, config.ssl_key) {
            Some(RustlsConfig::from_pem(cert.as_bytes().to_vec(), key.as_bytes().to_vec()).await?)
        } else {
            None
        };

        Ok(Self {
            listener: TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], config.port)))?,
            tls,
        })
    }
}

/// Start the server with a given `TcpListener` and TLS Config
///
/// # Panics
/// Will panic if the server can't start
pub async fn start_server(Config { listener, tls }: Config) {
    if let Some(tls_config) = tls {
        axum_server::from_tcp_rustls(listener, tls_config)
            .serve(app().into_make_service())
            .await
            .expect("server error");
    } else {
        axum_server::from_tcp(listener)
            .serve(app().into_make_service())
            .await
            .expect("server error");
    }
}
