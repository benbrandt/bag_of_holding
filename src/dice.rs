use std::collections::HashMap;

use axum::{response::IntoResponse, routing::post, Json, Router};
use axum_extra::routing::{RouterExt, TypedPath};
use dice::Die;
use serde::Deserialize;

/// Routes related to dice
pub fn routes() -> Router {
    Router::new()
        .typed_post(roll)
        .route("/roll", post(roll_multiple))
}

#[derive(Debug, Deserialize, TypedPath)]
#[typed_path("/:die/roll")]
struct DieRoll {
    die: Die,
}

/// Roll a given type and amount of dice
#[tracing::instrument]
async fn roll(path: DieRoll) -> impl IntoResponse {
    let mut rng = rand_utils::rng_from_entropy();
    Json(path.die.roll(&mut rng))
}

/// Roll multiple dice at once. Can specify a number of dice for each type of die
#[tracing::instrument]
async fn roll_multiple(Json(payload): Json<HashMap<Die, usize>>) -> impl IntoResponse {
    let mut rng = rand_utils::rng_from_entropy();
    Json(
        payload
            .into_iter()
            .map(|(die, num)| (die, die.roll_multiple(&mut rng, num).collect::<Vec<_>>()))
            .collect::<HashMap<_, _>>(),
    )
}
