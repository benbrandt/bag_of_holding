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
    body::Body,
    http::{header, Method, Request, StatusCode},
    routing::RouterIntoService,
};
use bag_of_holding::{app, start_server, Config};
use http_body_util::BodyExt;
use serde_json::Value;
use tokio::task::JoinHandle;
use tower::{Service, ServiceExt};

mod abilities;
mod alignments;
mod characters;
mod deities;
mod dice;
mod names;
mod sizes;

/// Use the entire server for tests
struct TestServer {
    addr: SocketAddr,
    app: RouterIntoService<Body>,
    _handle: JoinHandle<()>,
}

impl TestServer {
    fn new() -> Self {
        // Grab a random port
        let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], 0))).unwrap();
        let addr = listener.local_addr().unwrap();

        Self {
            addr,
            app: app().into_service(),
            _handle: tokio::spawn(start_server(Config::new(listener, None))),
        }
    }

    async fn request(
        &mut self,
        method: Method,
        endpoint: &str,
        body: Body,
    ) -> Result<Value, Box<dyn Error>> {
        let request = Request::builder()
            .method(method)
            .uri(format!("http://{}{endpoint}", self.addr))
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(body)?;
        let response = ServiceExt::<Request<Body>>::ready(&mut self.app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        Ok(serde_json::from_slice(&body[..])?)
    }
}
