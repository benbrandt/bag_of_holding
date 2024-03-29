use alignments::Alignment;
use axum::{response::IntoResponse, Json, Router};
use axum_extra::routing::Resource;

/// Routes related to alignments
pub fn routes() -> Router {
    Router::from(Resource::named("alignments").create(create))
}

/// Create new alignment
#[tracing::instrument]
async fn create() -> impl IntoResponse {
    let alignment = Alignment::gen(&mut rand_utils::rng_from_entropy(), &[], &[]);
    Json(alignment.to_string())
}
