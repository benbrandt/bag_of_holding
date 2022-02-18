use std::collections::HashMap;

use axum::http::Method;
use dice::Die;
use hyper::Body;
use serde_json::json;
use strum::IntoEnumIterator;

use super::oneshot;

#[tokio::test]
async fn die_roll() {
    for sides in [4, 6, 8, 10, 12, 20, 100] {
        let resp = oneshot(
            Method::POST,
            &format!("/dice/d{sides}/roll/"),
            Body::empty(),
        )
        .await
        .unwrap();

        assert!((1..=sides).contains(&resp.as_u64().unwrap()));
    }
}

#[tokio::test]
async fn roll_multiple_die_rolls() {
    let items = Die::iter().enumerate().map(|(i, d)| (d, i));

    let body: HashMap<Die, usize> = HashMap::from_iter(items.clone());

    let resp = oneshot(
        Method::POST,
        "/dice/roll/",
        Body::from(serde_json::to_vec(&json!(&body)).unwrap()),
    )
    .await
    .unwrap();

    // Make sure we got the requested number of dice
    for (die, num) in items {
        let rolls = resp[die.to_string()].as_array().unwrap();
        assert_eq!(rolls.len(), num);

        // And they are within the bounds
        for roll in rolls {
            assert!((1..=die.sides() as u64).contains(&roll.as_u64().unwrap()));
        }
    }
}
