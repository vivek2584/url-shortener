use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let shared_state = SharedUrlMap {
        map: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/shorten", post(shorten_url))
        .route("/goto/{short_path}", get(serve_original_url))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn shorten_url(
    Json(payload): Json<UrlPayload>,
    State(shared_map): State<SharedUrlMap>,
) -> impl IntoResponse {
    let payload = payload.original_url;

    todo!(); // shorten_url and store it in hash_map, return shortened url as json
}

async fn serve_original_url(
    Path(short_url): Path<String>,
    State(shared_map): State<SharedUrlMap>,
) -> impl IntoResponse {
    todo!();
}

#[derive(Clone)]
struct SharedUrlMap {
    map: Arc<Mutex<HashMap<String, String>>>,
}

#[derive(Deserialize)]
struct UrlPayload {
    original_url: String,
}

#[derive(Serialize)]
struct ShortenResponse {
    shorten_url: String,
}
