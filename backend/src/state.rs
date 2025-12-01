use rand::seq::SliceRandom;
use rand::thread_rng;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    // 残りの数字のリスト
    pub remaining_numbers: Arc<Mutex<Vec<u8>>>,
    // 出た数字の履歴
    pub history: Arc<Mutex<Vec<u8>>>,
}

impl AppState {
    pub fn new() -> Self {
        let mut numbers: Vec<u8> = (1..=75).collect();
        let mut rng = thread_rng();
        numbers.shuffle(&mut rng);

        Self {
            remaining_numbers: Arc::new(Mutex::new(numbers)),
            history: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
