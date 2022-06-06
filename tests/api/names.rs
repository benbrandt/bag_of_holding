use axum::http::Method;
use hyper::Body;

use crate::TestServer;

#[tokio::test]
async fn generate_names() {
    let server = TestServer::new().await;

    let options = server
        .request(Method::GET, "/names", Body::empty())
        .await
        .unwrap()
        .as_array()
        .unwrap()
        .clone();

    for generator in options {
        let name = server
            .request(
                Method::POST,
                &format!("/names/{}", generator.as_str().unwrap()),
                Body::empty(),
            )
            .await
            .unwrap();

        assert!(!name.as_str().unwrap().is_empty());
    }
}
