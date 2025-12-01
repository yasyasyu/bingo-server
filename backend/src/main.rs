mod domain;
mod handlers;
mod rng;
mod state;

use axum::{
    Router,
    http::Method,
    routing::{get, post},
};
use handlers::{get_amida, next_number, reset_game, setup_amida};
use state::AppState;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use tower_http::cors::{Any, CorsLayer};

fn calculate_seed_from_file(path: &str) -> u32 {
    let path = Path::new(path);
    if !path.exists() {
        println!("Warning: seeds.txt not found. Using default seed.");
        return 123456789;
    }

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening seeds.txt: {}", e);
            return 123456789;
        }
    };

    let mut seed: u32 = 1;
    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            if let Ok(num) = line.trim().parse::<u32>() {
                // オーバーフローしてもラップアラウンドするように wrapping_mul を使用
                seed = seed.wrapping_mul(num);
            }
        }
    }

    // 0になってしまった場合は補正
    if seed == 0 {
        seed = 987654321;
    }

    println!("Calculated seed: {}", seed);
    seed
}

#[tokio::main]
async fn main() {
    // シードの計算
    let seed = calculate_seed_from_file("seeds.txt");

    // 初期状態の作成
    let state = AppState::new(seed);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let app = Router::new()
        .route("/next", get(next_number))
        .route("/reset", post(reset_game))
        .route("/amida", get(get_amida))
        .route("/amida/setup", post(setup_amida))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
