use axum::{response::IntoResponse, Json, Router};
use axum_extra::routing::Resource;
use deities::{Deity, Domain};
use rand::Rng;
use strum::IntoEnumIterator;

/// Routes related to deities
pub fn routes() -> Router {
    let names = Resource::named("deities")
        .create(create_deity)
        .nest_collection(
            Router::new().merge(
                Resource::named("domains")
                    .index(index_domain)
                    .create(create_domain),
            ),
        );
    Router::new().merge(names)
}

/// Choose a random deity
#[tracing::instrument]
async fn create_deity() -> impl IntoResponse {
    let mut rng = rand_utils::rng_from_entropy();
    Json(Deity::gen(&mut rng, &[], &[], &[], true))
}

/// List domain options
#[tracing::instrument]
async fn index_domain() -> impl IntoResponse {
    Json(Domain::iter().collect::<Vec<_>>())
}

/// Choose a random domain
#[tracing::instrument]
async fn create_domain() -> impl IntoResponse {
    let mut rng = rand_utils::rng_from_entropy();
    Json(rng.gen::<Domain>())
}
