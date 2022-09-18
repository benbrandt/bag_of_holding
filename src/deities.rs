use axum::{response::IntoResponse, Json, Router};
use axum_extra::routing::Resource;
use deities::Domain;
use strum::IntoEnumIterator;

/// Routes related to deities
pub fn routes() -> Router {
    let names = Resource::named("deities").nest_collection(
        Router::new().merge(Resource::named("domains").index(index).create(create)),
    );
    Router::new().merge(names)
}

/// List domain options
#[tracing::instrument]
async fn index() -> impl IntoResponse {
    Json(Domain::iter().collect::<Vec<_>>())
}

/// Choose a random domain
#[tracing::instrument]
async fn create() -> impl IntoResponse {
    let mut rng = rand_utils::rng_from_entropy();
    Json(Domain::gen(&mut rng, &[]))
}
