use axum::http::Method;
use hyper::Body;

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
        rolls
            .as_object()
            .unwrap()
            .keys()
            .collect::<Vec<_>>()
            .as_slice(),
    );

    // Assert all values are valid
    assert!(rolls
        .as_object()
        .unwrap()
        .values()
        .all(|v| (3..=18).contains(&v["score"].as_u64().unwrap())));

    assert!(rolls
        .as_object()
        .unwrap()
        .values()
        .all(|v| (-5..=5).contains(&v["modifier"].as_i64().unwrap())));
}
