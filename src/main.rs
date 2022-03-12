#![warn(clippy::pedantic)]

use std::env;

use bag_of_holding::start_app;
use clap::Parser;
use sentry::{release_name, ClientOptions};

/// Command line arguments
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    /// The port to listen on for the app
    #[clap(long, short, default_value = "5000")]
    port: u16,
    /// The port to listen on for metrics
    #[clap(long, short, default_value = "9000")]
    metrics_port: u16,
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
    start_app(config.port, config.metrics_port).await;
}
