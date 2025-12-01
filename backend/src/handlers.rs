use crate::state::AppState;
use axum::{Json, extract::State};
use serde::Serialize;

#[derive(Serialize)]
pub struct NumberResponse {
    pub number: Option<u8>,
    pub history: Vec<u8>,
    pub message: String,
    pub seed: u32,
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
