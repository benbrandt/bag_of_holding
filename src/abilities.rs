use abilities::AbilityScores;
use axum::{response::IntoResponse, Json, Router};
use axum_extra::routing::Resource;

/// Routes related to abilities
pub fn routes() -> Router {
    let abilities = Resource::named("abilities").create(create);
    Router::new().merge(abilities)
}

/// Create new set of ability scores
#[tracing::instrument]
async fn create() -> impl IntoResponse {
    let mut rng = rand_utils::rng_from_entropy();
    Json(AbilityScores::gen(&mut rng))
}
