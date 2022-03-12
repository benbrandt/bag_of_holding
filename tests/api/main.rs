use std::{
    error::Error,
    net::{SocketAddr, TcpListener},
};

use axum::http::{header, Method, Request, StatusCode};
use bag_of_holding::start_app;
use hyper::{client::HttpConnector, Body, Client};
use serde_json::Value;
use tokio::task::JoinHandle;
use tracing_subscriber::{
    prelude::__tracing_subscriber_SubscriberExt, registry, util::SubscriberInitExt, EnvFilter,
};

mod dice;

/// Use the entire server for tests
struct TestServer {
    addr: SocketAddr,
    client: Client<HttpConnector>,
    _handle: JoinHandle<()>,
}

impl TestServer {
    async fn new() -> Self {
        // Setup tracing
        registry().with(EnvFilter::from_default_env()).init();

        // Grab a random port
        let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], 0))).unwrap();
        let addr = listener.local_addr().unwrap();

        let _handle = tokio::spawn(async move {
            start_app(listener, 0).await;
        });

        let client = Client::new();

        Self {
            addr,
            client,
            _handle,
        }
    }

    async fn request(
        &self,
        method: Method,
        endpoint: &str,
        body: Body,
    ) -> Result<Value, Box<dyn Error>> {
        let response = self
            .client
            .request(
                Request::builder()
                    .method(method)
                    .uri(format!("http://{}{endpoint}", self.addr))
                    .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await?;
        Ok(serde_json::from_slice(&body)?)
    }
}
