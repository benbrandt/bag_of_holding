use std::sync::Arc;

use axum::{
    extract::{Extension, Path},
    routing::post,
    Json, Router,
};
use dice::Die;

use crate::State;

/// Routes related to dice
pub(crate) fn dice_routes() -> Router {
    Router::new().route("/:die/roll/", post(roll))
}

/// Roll a given type and amount of dice
async fn roll(Extension(state): Extension<Arc<State>>, Path(die): Path<Die>) -> Json<u32> {
    let mut rng = state.rng.lock();
    Json(die.roll(&mut *rng))
}
