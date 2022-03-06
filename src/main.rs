use std::{env, net::SocketAddr};

use axum::Server;
use bag_of_holding::app;
use clap::Parser;
use metrics_exporter_prometheus::PrometheusBuilder;
use tracing::info;
use tracing_subscriber::{
    fmt, prelude::__tracing_subscriber_SubscriberExt, registry, util::SubscriberInitExt, EnvFilter,
};

/// Command line arguments
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Config {
    /// The port to listen on
    #[clap(long, short, default_value = "5000")]
    port: u16,
}

/// Basic wrapper around `app()` to configure running in a server environment
#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "debug");
    }

    // Sentry
    let _guard = sentry::init((
        "https://c21aaae10ee74c71aa81a04f03203f59@o251876.ingest.sentry.io/6243981",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            traces_sample_rate: 0.1,
            ..Default::default()
        },
    ));

    // Setup tracing
    registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer())
        .with(sentry::integrations::tracing::layer())
        .init();

    // Parse command line arguments
    let config = Config::parse();

    // Metrics setup. Listening on port 9000
    PrometheusBuilder::new()
        .install()
        .expect("failed to install metrics recorder");

    // Run our service
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("Listening on {}", addr);
    Server::bind(&addr)
        .serve(app().into_make_service())
        .await
        .expect("server error");
}
