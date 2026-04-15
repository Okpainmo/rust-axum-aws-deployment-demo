// use axum::extract::State;
use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Serialize};

// utils import
// use crate::AppState;

#[derive(Debug, Serialize)]
pub struct Response {
    name: String,
    email: String,
    github: String,
}

pub async fn me(
    // State(_state): State<AppState>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(Response {
            name: "Andrew James Okpainmo".to_string(),
            email: "okpainmoandrew@gmail.com".to_string(),
            github: "https://github.com/Okpainmo".to_string()
        })              
    )
}