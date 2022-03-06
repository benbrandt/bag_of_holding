use std::collections::HashMap;

use axum::{extract::Path, routing::post, Json, Router};
use dice::Die;
use rand::SeedableRng;
use rand_pcg::Pcg64;

/// Routes related to dice
#[tracing::instrument]
pub(crate) fn dice_routes() -> Router {
    Router::new()
        .route("/:die/roll/", post(roll))
        .route("/roll/", post(roll_multiple))
}

/// Roll a given type and amount of dice
#[tracing::instrument]
async fn roll(Path(die): Path<Die>) -> Json<u32> {
    let mut rng = Pcg64::from_entropy();
    Json(die.roll(&mut rng))
}

/// Roll multiple dice at once. Can specify a number of dice for each type of die
#[tracing::instrument]
async fn roll_multiple(Json(payload): Json<HashMap<Die, usize>>) -> Json<HashMap<Die, Vec<u32>>> {
    let mut rng = Pcg64::from_entropy();
    Json(HashMap::from_iter(
        payload
            .into_iter()
            .map(|(die, num)| (die, die.roll_multiple(&mut rng, num))),
    ))
}
