use axum::{response::IntoResponse, Json, Router};
use axum_extra::routing::{Resource, TypedPath};
use names::Name;
use serde::Deserialize;
use strum::IntoEnumIterator;

/// Routes related to names
pub fn routes() -> Router {
    Router::from(Resource::named("names").index(index))
        .merge(Router::from(Resource::named("names/:name").create(create)))
}

/// List name generator options
#[tracing::instrument]
async fn index() -> impl IntoResponse {
    Json(Name::iter().collect::<Vec<_>>())
}

#[derive(Debug, Deserialize, TypedPath)]
#[typed_path("/:name")]
struct NameGenerator {
    name: Name,
}

/// Create a new name for the given generator type
#[tracing::instrument]
async fn create(path: NameGenerator) -> impl IntoResponse {
    let mut rng = rand_utils::rng_from_entropy();
    Json(path.name.gen(&mut rng))
}
