use axum::http::Method;
use hyper::Body;
use itertools::Itertools;

use crate::TestServer;

#[tokio::test]
async fn generate_characters() {
    let server = TestServer::new().await;

    let character = server
        .request(Method::POST, "/characters", Body::empty())
        .await
        .unwrap();

    // Assert ability scores are present
    assert_eq!(
        ["CHA", "CON", "DEX", "INT", "STR", "WIS"],
        character["ability_scores"]
            .as_object()
            .unwrap()
            .keys()
            .collect_vec()
            .as_slice(),
    );

    // Race is generated
    assert_eq!(character["race"].as_str().unwrap().split('(').count(), 2);
}
