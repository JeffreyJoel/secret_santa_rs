use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

use secret_santa_lib::{generate_pairings, shuffle};

#[derive(Deserialize)]
pub struct GenerateRequest {
    users: Vec<String>,
}

#[derive(Serialize)]
pub struct GenerateResponse {
    pairs: Vec<(String, String)>,
}

async fn health() -> &'static str {
    "Working"
}
async fn generate(Json(payload): Json<GenerateRequest>) -> impl IntoResponse {
    let mut users = payload.users;
    shuffle(&mut users);
    match generate_pairings(&users) {
        Ok(pairs) => (StatusCode::OK, Json(GenerateResponse { pairs })).into_response(),
        Err(error) => (StatusCode::BAD_REQUEST, error).into_response(),
    }
}

pub fn app() -> Router {
    Router::new()
        .route("/", get(health))
        .route("/health", get(health))
        .route("/generate", post(generate))
}
