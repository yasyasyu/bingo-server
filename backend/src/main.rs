mod domain;
mod handlers;
mod rng;
mod state;

use axum::{
    Router,
    http::Method,
    routing::{get, post},
};
use handlers::{get_amida, get_amida_result, get_next_number, reset_game, set_amida};
use state::AppState;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use tower_http::cors::{Any, CorsLayer};

/// シードファイルを読み込んでシード値を計算する
///
/// `seeds.txt` ファイルから数値を読み込み、それらを掛け合わせてシード値を生成します。
/// ファイルが存在しない場合やエラーが発生した場合は 0 を返します。
///
/// # Arguments
/// * `path` - シードファイルのパス
///
/// # Returns
/// * `u32` - 計算されたシード値
fn calculate_seed_from_file(path: &str) -> u32 {
    let path = Path::new(path);
    if !path.exists() {
        println!("Warning: seeds.txt not found. Using default seed.");
        return 0;
    }

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening seeds.txt: {}", e);
            return 0;
        }
    };

    let mut seed: u32 = 0;
    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            if let Ok(num) = line.trim().parse::<u32>() {
                // オーバーフローしてもラップアラウンドするように wrapping_mul を使用
                seed = seed.wrapping_mul(num);
            }
        }
    }

    println!("Calculated seed: {}", seed);
    seed
}

#[tokio::main]
async fn main() {
    // シードの計算
    // seeds.txt から数値を読み込み、それらを掛け合わせてシード値を生成します。
    // これにより、外部からシード値を制御し、再現性を確保します。
    let seed = calculate_seed_from_file("seeds.txt");

    // 初期状態の作成
    // アプリケーション全体で共有される状態（ビンゴ、あみだくじ）を初期化します。
    let state = AppState::new(seed);

    // CORS設定
    // 開発環境向けに、全てのオリジンからのリクエストを許可しています。
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    // ルーティング設定
    // /next_number: ビンゴの次の数字を引く
    // /reset: ゲームをリセットする
    // /amida: あみだくじの設定（GET: 取得, POST: 更新）
    // /amida/result: あみだくじの結果（誰がどの番号か）を取得
    let app = Router::new()
        .route("/next_number", get(get_next_number))
        .route("/reset", post(reset_game))
        .route("/amida", post(set_amida).get(get_amida))
        .route("/amida/result", get(get_amida_result))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
