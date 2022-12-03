use axum::{extract::Query, response::IntoResponse, Json, Router};
use axum_extra::routing::Resource;
use deities::{Deity, Domain};
use rand::Rng;
use serde::Deserialize;
use strum::IntoEnumIterator;

/// Routes related to deities
pub fn routes() -> Router {
    Router::from(Resource::named("deities").create(create_deity)).merge(Router::from(
        Resource::named("deities/domains")
            .index(index_domain)
            .create(create_domain),
    ))
}

#[derive(Debug, Deserialize)]
struct DeityFilters {
    domain: Option<Domain>,
}

/// Choose a random deity
#[tracing::instrument]
async fn create_deity(Query(query): Query<DeityFilters>) -> impl IntoResponse {
    let mut rng = rand_utils::rng_from_entropy();
    Json(Deity::gen(&mut rng, query.domain, &[], &[], &[], true))
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
