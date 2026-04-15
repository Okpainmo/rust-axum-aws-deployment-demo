use crate::AppState;
use crate::core::controllers::base::base;
use crate::core::controllers::health::health;
use crate::core::controllers::me::me;
use axum::{Router, routing::{get}};

pub fn routes(_state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(base))
        .route("/health", get(health))
        .route("/me", get(me))
}