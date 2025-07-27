use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    extract::State,
    http::StatusCode,
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
        .route("/shorten", get(shorten_url))
        .route("/redirect", get(redirect))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn shorten_url(
    State(shared_map): State<SharedUrlMap>,
    Json(payload): Json<UrlPayload>,
) -> impl IntoResponse {
    let payload = payload.url;
    let digest: [u8; 16] = md5::compute(&payload).into();
    let u128_md5 = u128::from_be_bytes(digest);
    let b62_encoded = base62::encode(u128_md5);
    let trim_hash = String::from(&b62_encoded[0..4]);
    let short_url = format!("short.url/{trim_hash}");

    shared_map
        .map
        .lock()
        .unwrap()
        .insert(short_url.clone(), payload);

    (
        StatusCode::OK,
        Json(ShortenResponse {
            shorten_url: short_url,
        }),
    )
}

async fn redirect(
    State(shared_map): State<SharedUrlMap>,
    Json(short_path): Json<UrlPayload>,
) -> impl IntoResponse {
    if let Some(url) = shared_map.map.lock().unwrap().get(&short_path.url) {
        Json(UrlPayload {
            url: url.to_owned(),
        })
        .into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            "No redirects found for this shortened URL",
        )
            .into_response()
    }
}

#[derive(Clone)]
struct SharedUrlMap {
    map: Arc<Mutex<HashMap<String, String>>>,
}

#[derive(Deserialize, Serialize)]
struct UrlPayload {
    url: String,
}

#[derive(Serialize)]
struct ShortenResponse {
    shorten_url: String,
}
