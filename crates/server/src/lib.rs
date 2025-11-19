use axum::{
    Json, Router,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

use secret_santa_lib::generate_pairings;

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
async fn generate(Json(payload): Json<GenerateRequest>) -> Json<GenerateResponse> {
    let users = payload.users;
    let pairs = generate_pairings(&users);
    Json(GenerateResponse { pairs })
}

pub fn app() -> Router {
    Router::new()
        .route("/", get(health))
        .route("/health", get(health))
        .route("/generate", post(generate))
}
