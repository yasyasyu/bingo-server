use crate::domain::BingoGame;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub game: Arc<Mutex<BingoGame>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            game: Arc::new(Mutex::new(BingoGame::new())),
        }
    }
}
