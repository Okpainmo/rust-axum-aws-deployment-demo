// use axum::extract::State;
use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Serialize};

// utils import
// use crate::AppState;

#[derive(Debug, Serialize)]
pub struct Response {
    message: String,
}

pub async fn health(
    // State(_state): State<AppState>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(Response {
            message: "healthy".to_string(),
        })              
    )
}