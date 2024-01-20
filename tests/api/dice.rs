use std::collections::HashMap;

use axum::{body::Body, http::Method};
use dice::Die;
use serde_json::json;
use statrs::{
    distribution::Uniform,
    statistics::{Distribution, Statistics},
};
use strum::IntoEnumIterator;

use crate::TestServer;

#[tokio::test]
async fn die_roll() {
    let mut server = TestServer::new();

    for sides in [4u32, 6, 8, 10, 12, 20, 100] {
        let mut rolls = vec![];
        for _ in 0..sides * 10 {
            rolls.push(
                server
                    .request(Method::POST, &format!("/dice/d{sides}/roll"), Body::empty())
                    .await
                    .unwrap(),
            );
        }

        let dist = Uniform::new(1.0, f64::from(sides)).unwrap();
        assert!(rolls
            .iter()
            .all(|roll| (1..=u64::from(sides)).contains(&(roll.as_u64().unwrap()))));
        let mean = rolls.into_iter().map(|r| r.as_f64().unwrap()).mean();
        assert!((mean - dist.mean().unwrap()).abs() < dist.std_dev().unwrap());
    }
}

#[tokio::test]
async fn roll_multiple_die_rolls() {
    let mut server = TestServer::new();

    let items = Die::iter().enumerate().map(|(i, d)| (d, i));
    let body: HashMap<Die, usize> = items.clone().collect();

    let resp = server
        .request(
            Method::POST,
            "/dice/roll",
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
            assert!((1..=u64::from(die)).contains(&(roll.as_u64().unwrap())));
        }
    }
}
