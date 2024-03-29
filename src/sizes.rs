use axum::{response::IntoResponse, Json, Router};
use axum_extra::routing::{Resource, TypedPath};
use serde::Deserialize;
use sizes::HeightAndWeightTable;
use strum::IntoEnumIterator;

/// Routes related to height and weight tables
pub fn routes() -> Router {
    Router::from(Resource::named("height-and-weight").index(index)).merge(Router::from(
        Resource::named("height-and-weight/:table").create(create),
    ))
}

/// List height and weight table generator options
#[tracing::instrument]
async fn index() -> impl IntoResponse {
    Json(HeightAndWeightTable::iter().collect::<Vec<_>>())
}

#[derive(Debug, Deserialize, TypedPath)]
#[typed_path("/:table")]
struct Table {
    table: HeightAndWeightTable,
}

/// Create a new height and weight for the given generator type
#[tracing::instrument]
async fn create(path: Table) -> impl IntoResponse {
    let mut rng = rand_utils::rng_from_entropy();
    Json(path.table.gen(&mut rng))
}
