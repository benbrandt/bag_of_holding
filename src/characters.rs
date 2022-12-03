use axum::{response::IntoResponse, Json, Router};
use axum_extra::routing::Resource;
use characters::Character;
use rand::Rng;

/// Routes related to characters
pub fn routes() -> Router {
    Router::from(Resource::named("characters").create(create))
}

/// Create new set of ability scores
#[tracing::instrument]
async fn create() -> impl IntoResponse {
    let character: Character = rand_utils::rng_from_entropy().gen();
    Json(character)
}
