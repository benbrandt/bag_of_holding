use axum::http::Method;
use deities::Domain;
use hyper::Body;
use itertools::Itertools;
use serde_json::json;
use strum::IntoEnumIterator;

use crate::TestServer;

#[tokio::test]
async fn get_domain_options() {
    let server = TestServer::new().await;

    let options = server
        .request(Method::GET, "/deities/domains", Body::empty())
        .await
        .unwrap();

    assert_eq!(options, json!(Domain::iter().collect::<Vec<_>>()));
}

#[tokio::test]
async fn generate_domain() {
    let server = TestServer::new().await;

    let domain = server
        .request(Method::POST, "/deities/domains", Body::empty())
        .await
        .unwrap();

    assert!(Domain::iter().map(|d| json!(d)).contains(&domain));
}