use axum::http::Method;
use hyper::Body;
use itertools::Itertools;

use crate::TestServer;

#[tokio::test]
async fn generate_ability_scores() {
    let server = TestServer::new().await;

    let rolls = server
        .request(Method::POST, "/abilities", Body::empty())
        .await
        .unwrap();

    // Assert all keys are present
    assert_eq!(
        ["CHA", "CON", "DEX", "INT", "STR", "WIS"],
        rolls.as_object().unwrap().keys().collect_vec().as_slice(),
    );

    // Assert all values are valid
    assert!(rolls
        .as_object()
        .unwrap()
        .values()
        .all(|v| (3..=18).contains(&v.as_u64().unwrap())));
}
