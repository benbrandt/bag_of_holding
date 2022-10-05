use axum::http::Method;
use hyper::Body;

use crate::TestServer;

#[tokio::test]
async fn generate_alignment() {
    let server = TestServer::new();

    let alignment = server
        .request(Method::POST, "/alignments", Body::empty())
        .await
        .unwrap();

    assert!(!alignment.as_str().unwrap().is_empty());
}
