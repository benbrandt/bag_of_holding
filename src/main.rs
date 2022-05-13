//! Instantiates and runs the `bag_of_holding` crate as a binary.

#![warn(
    clippy::pedantic,
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms
)]

use std::{env, net::SocketAddr};

use axum_server::tls_rustls::RustlsConfig;
use bag_of_holding::app;
use clap::Parser;
use sentry::{release_name, ClientOptions};

/// Command line arguments
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    /// The port to listen on for the app
    #[clap(long, short, default_value = "5000")]
    port: u16,
    /// SSL Certificate value
    #[clap(env, long)]
    ssl_cert: Option<String>,
    /// SSL Key value
    #[clap(env, long)]
    ssl_key: Option<String>,
}

/// Basic wrapper around `start_app()` to configure running in a server environment
#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "debug");
    }

    // Start Sentry
    let _guard = sentry::init((
        "https://c21aaae10ee74c71aa81a04f03203f59@o251876.ingest.sentry.io/6243981",
        ClientOptions {
            release: release_name!(),
            traces_sample_rate: 0.1,
            ..ClientOptions::default()
        },
    ));

    // Parse command line arguments and start app
    let config = Config::parse();

    // Get TLS config if available
    let tls_config = if let (Some(cert), Some(key)) = (config.ssl_cert, config.ssl_key) {
        Some(
            RustlsConfig::from_pem(cert.as_bytes().to_vec(), key.as_bytes().to_vec())
                .await
                .expect("Failed to load TLS certs"),
        )
    } else {
        None
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    if let Some(config) = tls_config {
        axum_server::bind_rustls(addr, config)
            .serve(app().into_make_service())
            .await
            .expect("server error");
    } else {
        axum_server::bind(addr)
            .serve(app().into_make_service())
            .await
            .expect("server error");
    }
}
