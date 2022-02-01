use std::sync::Arc;

use axum::{AddExtensionLayer, Router};
use parking_lot::Mutex;
use rand::SeedableRng;
use rand_pcg::Pcg64;

use self::dice::dice_routes;

mod dice;

/// Shared app state between handlers
struct State {
    /// Shared random number generator
    rng: Mutex<Pcg64>,
}

impl State {
    fn new() -> Self {
        Self {
            rng: Mutex::new(Pcg64::from_entropy()),
        }
    }
}

/// Top-level app. To be consumed by main.rs and tests
pub fn app() -> Router {
    Router::new()
        .nest("/dice", dice_routes())
        .layer(AddExtensionLayer::new(Arc::new(State::new())))
}
