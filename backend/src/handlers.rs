use crate::state::AppState;
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct NumberResponse {
    pub number: Option<u8>,
    pub history: Vec<u8>,
    pub message: String,
    pub seed: u32,
}

#[derive(Serialize, Deserialize)]
pub struct AmidaRequest {
    pub items: Vec<String>,
}

#[derive(Serialize)]
pub struct AmidaResponse {
    pub items: Vec<String>,
    pub message: String,
}

pub async fn next_number(State(state): State<AppState>) -> Json<NumberResponse> {
    let mut game = state.game.lock().unwrap();

    if let Some(num) = game.draw_number() {
        Json(NumberResponse {
            number: Some(num),
            history: game.history.clone(),
            message: "Success".to_string(),
            seed: state.seed,
        })
    } else {
        Json(NumberResponse {
            number: None,
            history: game.history.clone(),
            message: "Game Over".to_string(),
            seed: state.seed,
        })
    }
}

pub async fn reset_game(State(state): State<AppState>) -> Json<NumberResponse> {
    let mut game = state.game.lock().unwrap();
    game.reset();

    Json(NumberResponse {
        number: None,
        history: Vec::new(),
        message: "Game Reset".to_string(),
        seed: state.seed,
    })
}

pub async fn get_amida(State(state): State<AppState>) -> Json<AmidaResponse> {
    let amida = state.amida.lock().unwrap();
    Json(AmidaResponse {
        items: amida.items.clone(),
        message: "Success".to_string(),
    })
}

pub async fn setup_amida(
    State(state): State<AppState>,
    Json(payload): Json<AmidaRequest>,
) -> Json<AmidaResponse> {
    let mut amida = state.amida.lock().unwrap();
    amida.update(payload.items);

    Json(AmidaResponse {
        items: amida.items.clone(),
        message: "Updated".to_string(),
    })
}
