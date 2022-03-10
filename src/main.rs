use std::env;

use bag_of_holding::{start_app, Config};
use clap::StructOpt;

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
        sentry::ClientOptions {
            release: sentry::release_name!(),
            traces_sample_rate: 0.1,
            ..Default::default()
        },
    ));

    // Parse command line arguments and start app
    start_app(Config::parse()).await;
}
