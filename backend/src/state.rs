use crate::domain::{AmidaGame, BingoGame};
use crate::rng::XorShift;
use std::sync::{Arc, Mutex};

const BINGO_MAX_NUMBER: usize = 75;
const AMIDA_PRIZES_COUNT: usize = 10;

#[derive(Clone)]
pub struct AppState {
    pub game: Arc<Mutex<BingoGame>>,
    pub amida: Arc<Mutex<AmidaGame>>,
    pub seed: u32,
}

impl AppState {
    pub fn new(seed: u32) -> Self {
        Self {
            game: Arc::new(Mutex::new(BingoGame::new(
                BINGO_MAX_NUMBER,
                Box::new(XorShift::new(seed)),
            ))),
            amida: Arc::new(Mutex::new(AmidaGame::new(
                AMIDA_PRIZES_COUNT,
                Box::new(XorShift::shift_new(seed, BINGO_MAX_NUMBER)),
            ))),
            seed,
        }
    }
}
