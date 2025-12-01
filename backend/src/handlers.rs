use crate::state::AppState;
use axum::{extract::State, Json};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Serialize;

#[derive(Serialize)]
pub struct NumberResponse {
    pub number: Option<u8>,
    pub history: Vec<u8>,
    pub message: String,
}

pub async fn next_number(State(state): State<AppState>) -> Json<NumberResponse> {
    let mut remaining = state.remaining_numbers.lock().unwrap();
    let mut history = state.history.lock().unwrap();

    if let Some(num) = remaining.pop() {
        history.push(num);
        Json(NumberResponse {
            number: Some(num),
            history: history.clone(),
            message: "Success".to_string(),
        })
    } else {
        Json(NumberResponse {
            number: None,
            history: history.clone(),
            message: "Game Over".to_string(),
        })
    }
}

pub async fn reset_game(State(state): State<AppState>) -> Json<NumberResponse> {
    let mut remaining = state.remaining_numbers.lock().unwrap();
    let mut history = state.history.lock().unwrap();

    *remaining = (1..=75).collect();
    let mut rng = thread_rng();
    remaining.shuffle(&mut rng);

    history.clear();

    Json(NumberResponse {
        number: None,
        history: Vec::new(),
        message: "Game Reset".to_string(),
    })
}
