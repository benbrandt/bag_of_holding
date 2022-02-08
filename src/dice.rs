use axum::{extract::Path, routing::post, Json, Router};
use dice::Die;
use rand::SeedableRng;
use rand_pcg::Pcg64;

/// Routes related to dice
pub(crate) fn dice_routes() -> Router {
    Router::new().route("/:die/roll/", post(roll))
}

/// Roll a given type and amount of dice
async fn roll(Path(die): Path<Die>) -> Json<u32> {
    let mut rng = Pcg64::from_entropy();
    Json(die.roll(&mut rng))
}
