use axum::{http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};

pub(crate) mod error_handler;
pub(crate) mod user;

#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    status: String,
}

pub async fn health_check() -> (StatusCode, Json<HealthResponse>) {
    let response = HealthResponse {
        status: "healthy".to_string(),
    };

    (StatusCode::OK, Json(response))
}
