use axum::Router;

use self::dice::dice_routes;

mod dice;

/// Top-level app. To be consumed by main.rs and tests
pub fn app() -> Router {
    Router::new().nest("/dice", dice_routes())
}
