use std::sync::Arc;

use axum::{extract::Extension, routing::post, Json, Router};
use dice::Die;
use serde::Deserialize;

use crate::State;

/// Routes related to dice
pub(crate) fn dice_routes() -> Router {
    Router::new().route("/roll", post(roll))
}

/// Payload for requesting a roll
#[derive(Deserialize)]
struct RollRequest {
    die: Die,
    amount: usize,
}

/// Roll a given type and amount of dice
async fn roll(
    Extension(state): Extension<Arc<State>>,
    Json(payload): Json<RollRequest>,
) -> Json<Vec<u32>> {
    let mut rng = state.rng.lock();
    Json(payload.die.roll_multiple(&mut *rng, payload.amount))
}
