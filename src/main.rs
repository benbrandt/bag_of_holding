use std::{env, net::SocketAddr, os::unix::prelude::OsStrExt};

use axum::Server;
use axum_server::tls_rustls::RustlsConfig;
use bag_of_holding::app;
use clap::Parser;

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
        env::set_var("RUST_LOG", "bag_of_holding=debug,tower_http=debug")
    }

    // Setup tracing
    tracing_subscriber::fmt::init();

    // Parse command line arguments
    let config = Config::parse();
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    let tls_config = RustlsConfig::from_pem(
        env::var_os("SSL_CERT").unwrap().as_bytes().to_vec(),
        env::var_os("SSL_KEY").unwrap().as_bytes().to_vec(),
    )
    .await
    .unwrap();

    // Run our service
    tracing::info!("Listening on {}", addr);
    axum_server::bind_rustls(addr, tls_config)
        .serve(app().into_make_service())
        .await
        .expect("server error");
}
