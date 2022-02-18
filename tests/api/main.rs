use std::error::Error;

use axum::http::{header, Method, Request, StatusCode};
use bag_of_holding::app;
use hyper::Body;
use serde_json::Value;
use tower::ServiceExt;

mod dice;

/// Helper to fire one request at the app
async fn oneshot(method: Method, uri: &str, body: Body) -> Result<Value, Box<dyn Error>> {
    let app = app();
    let response = app
        .oneshot(
            Request::builder()
                .method(method)
                .uri(uri)
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(body)?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await?;
    Ok(serde_json::from_slice(&body)?)
}
