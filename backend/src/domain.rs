use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone, Debug)]
pub struct BingoGame {
    pub remaining_numbers: Vec<u8>,
    pub history: Vec<u8>,
}

impl BingoGame {
    pub fn new() -> Self {
        let mut game = Self {
            remaining_numbers: (1..=75).collect(),
            history: Vec::new(),
        };
        game.shuffle();
        game
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.remaining_numbers.shuffle(&mut rng);
    }

    pub fn draw_number(&mut self) -> Option<u8> {
        if let Some(num) = self.remaining_numbers.pop() {
            self.history.push(num);
            Some(num)
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.remaining_numbers = (1..=75).collect();
        self.history.clear();
        self.shuffle();
    }
}
