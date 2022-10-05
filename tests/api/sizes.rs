use axum::http::Method;
use hyper::Body;

use crate::TestServer;

#[tokio::test]
async fn generate_height_and_weight() {
    let server = TestServer::new();

    let options = server
        .request(Method::GET, "/height-and-weight", Body::empty())
        .await
        .unwrap()
        .as_array()
        .unwrap()
        .clone();

    for generator in options {
        let result = server
            .request(
                Method::POST,
                &format!("/height-and-weight/{}", generator.as_str().unwrap()),
                Body::empty(),
            )
            .await
            .unwrap();

        assert!(result["height"].as_i64().unwrap() > 0);
        assert!(result["weight"].as_i64().unwrap() > 0);
    }
}
