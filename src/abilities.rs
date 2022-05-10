use abilities::AbilityScores;
use axum::{response::IntoResponse, Json, Router};
use axum_extra::routing::Resource;
use rand::SeedableRng;
use rand_pcg::Pcg64;

/// Routes related to abilities
pub fn routes() -> Router {
    let abilities = Resource::named("abilities").create(create);
    Router::new().merge(abilities)
}

/// Create new set of ability scores
#[tracing::instrument]
async fn create() -> impl IntoResponse {
    let mut rng = Pcg64::from_entropy();
    Json(AbilityScores::gen(&mut rng))
}
