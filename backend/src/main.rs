mod handlers;
mod state;

use axum::{
    Router,
    http::Method,
    routing::{get, post},
};
use handlers::{next_number, reset_game};
use state::AppState;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    // 初期状態の作成
    let state = AppState::new();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST]);

    let app = Router::new()
        .route("/next", get(next_number))
        .route("/reset", post(reset_game))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
