use axum::http;
use bag_of_holding::app;
use hyper::{Body, Request, StatusCode};
use serde_json::Value;
use tower::ServiceExt;

#[tokio::test]
async fn die_roll() {
    for sides in [4, 6, 8, 10, 12, 20, 100] {
        let app = app();
        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri(format!("/dice/d{sides}/roll/"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert!((1..=sides).contains(&body.as_u64().unwrap()));
    }
}
