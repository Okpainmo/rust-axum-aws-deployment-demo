// use axum::extract::State;
use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Serialize};

// utils import
// use crate::AppState;

#[derive(Debug, Serialize)]
pub struct Response {
    message: String,
}

pub async fn base(
    // State(_state): State<AppState>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(Response {
            message: "API is running".to_string(),
        })              
    )
}