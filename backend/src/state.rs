use crate::domain::{AmidaGame, BingoGame};
use crate::rng::XorShift;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub game: Arc<Mutex<BingoGame>>,
    pub amida: Arc<Mutex<AmidaGame>>,
    pub seed: u32,
}

impl AppState {
    pub fn new(seed: u32) -> Self {
        let rng = Box::new(XorShift::new(seed));
        Self {
            game: Arc::new(Mutex::new(BingoGame::new(rng))),
            amida: Arc::new(Mutex::new(AmidaGame::new())),
            seed,
        }
    }
}
