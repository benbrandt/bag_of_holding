use abilities::AbilityScores;
use axum::{response::IntoResponse, Json, Router};
use axum_extra::routing::Resource;
use rand::Rng;

/// Routes related to abilities
pub fn routes() -> Router {
    let abilities = Resource::named("abilities").create(create);
    Router::new().merge(abilities)
}

/// Create new set of ability scores
#[tracing::instrument]
async fn create() -> impl IntoResponse {
    let scores: AbilityScores = rand_utils::rng_from_entropy().gen();
    Json(scores)
}
