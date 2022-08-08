//! Instantiates and runs the `bag_of_holding` crate as a binary.

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

use std::env;

use bag_of_holding::{start_server, Config, ServerConfig};
use clap::Parser;
use sentry::{release_name, ClientOptions};

/// Basic wrapper around `start_app()` to configure running in a server environment
#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "debug");
    }

    // Start Sentry
    let _guard = sentry::init(ClientOptions {
        release: release_name!(),
        traces_sample_rate: 0.1,
        ..ClientOptions::default()
    });

    // Parse command line arguments and start app
    let server_config = ServerConfig::from_config(Config::parse())
        .await
        .expect("Failed to start app");
    start_server(server_config).await;
}
