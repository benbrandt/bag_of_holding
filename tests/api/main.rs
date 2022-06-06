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
    error::Error,
    net::{SocketAddr, TcpListener},
};

use axum::{
    http::{header, Method, Request, StatusCode},
    Server,
};
use bag_of_holding::app;
use hyper::{client::HttpConnector, Body, Client};
use serde_json::Value;
use tokio::task::JoinHandle;

mod abilities;
mod dice;
mod names;

/// Use the entire server for tests
struct TestServer {
    addr: SocketAddr,
    client: Client<HttpConnector>,
    _handle: JoinHandle<()>,
}

impl TestServer {
    async fn new() -> Self {
        // Grab a random port
        let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], 0))).unwrap();
        let addr = listener.local_addr().unwrap();

        let handle = tokio::spawn(async move {
            Server::from_tcp(listener)
                .expect("failed on tcp listener")
                .serve(app().into_make_service())
                .await
                .expect("server error");
        });

        let client = Client::new();

        Self {
            addr,
            client,
            _handle: handle,
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
