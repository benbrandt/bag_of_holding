use axum::http::Method;
use hyper::Body;

use crate::TestServer;

#[tokio::test]
async fn generate_characters() {
    let server = TestServer::new();

    let character = server
        .request(Method::POST, "/characters", Body::empty())
        .await
        .unwrap();

    // There is a name
    assert!(!character["name"].as_str().unwrap().is_empty());

    // Ability scores are present
    assert_eq!(
        ["CHA", "CON", "DEX", "INT", "STR", "WIS"],
        character["ability_scores"]
            .as_object()
            .unwrap()
            .keys()
            .collect::<Vec<_>>()
            .as_slice(),
    );

    // Race is generated
    assert_eq!(character["race"].as_str().unwrap().split('(').count(), 2);

    // There is an age
    assert!(character["age"].as_i64().unwrap() > 0);
    // There is size info
    assert!(character["height"].as_i64().unwrap() > 0);
    assert!(character["weight"].as_i64().unwrap() > 0);
    assert!(!character["size"].as_str().unwrap().is_empty());
}
