use crate::rng::IRng;
use std::{fmt, usize};

pub struct AmidaGame {
    count: usize,
    pub gests: Vec<String>,
    pub prizes: Vec<u8>,
    rng: Box<dyn IRng>,
}

impl fmt::Debug for AmidaGame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AmidaGame")
            .field("items", &self.gests)
            .finish()
    }
}

impl AmidaGame {
    pub fn new(count: usize, rng: Box<dyn IRng>) -> Self {
        let mut game = Self {
            count,
            gests: Vec::new(),
            prizes: (1..=count as u8).collect(),
            rng,
        };
        game.shuffle();
        game
    }
    fn shuffle(&mut self) {
        self.rng.shuffle(&mut self.prizes);
    }

    pub fn update(&mut self, gests: Vec<String>) {
        self.gests = gests;
    }
    pub fn get_result(&self) -> Option<Vec<(String, String)>> {
        if self.gests.len() != self.count {
            return None;
        }

        let mut result = Vec::new();
        for (gest, prize) in self.gests.iter().zip(self.prizes.iter()) {
            result.push((gest.clone(), prize.to_string()));
        }

        Some(result)
    }
}

pub struct BingoGame {
    count: usize,
    pub remaining_numbers: Vec<u8>,
    pub history: Vec<u8>,
    rng: Box<dyn IRng>,
}

impl fmt::Debug for BingoGame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BingoGame")
            .field("remaining_numbers", &self.remaining_numbers)
            .field("history", &self.history)
            .finish()
    }
}

impl BingoGame {
    pub fn new(count: usize, rng: Box<dyn IRng>) -> Self {
        let mut game = Self {
            count,
            remaining_numbers: (1..=count as u8).collect(),
            history: Vec::new(),
            rng,
        };
        game.shuffle();
        game
    }

    fn shuffle(&mut self) {
        self.rng.shuffle(&mut self.remaining_numbers);
    }

    pub fn get_next_number(&mut self) -> Option<u8> {
        if let Some(num) = self.remaining_numbers.pop() {
            self.history.push(num);
            Some(num)
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.remaining_numbers = (1..=self.count as u8).collect();
        self.history.clear();
        self.shuffle();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::XorShift;
    use std::collections::HashSet;

    #[test]
    fn test_new_game_initialization() {
        let rng = Box::new(XorShift::new(123));
        let game = BingoGame::new(75, rng);
        assert_eq!(game.remaining_numbers.len(), 75);
        assert_eq!(game.history.len(), 0);

        // 1から75までの数字が全て含まれているか確認
        let set: HashSet<u8> = game.remaining_numbers.iter().cloned().collect();
        assert_eq!(set.len(), 75);
        assert!(set.contains(&1));
        assert!(set.contains(&75));
    }

    #[test]
    fn test_draw_number() {
        let rng = Box::new(XorShift::new(123));
        let mut game = BingoGame::new(75, rng);
        let initial_len = game.remaining_numbers.len();

        // 1回引く
        let num = game.get_next_number();
        assert!(num.is_some());
        assert_eq!(game.remaining_numbers.len(), initial_len - 1);
        assert_eq!(game.history.len(), 1);
        assert_eq!(game.history[0], num.unwrap());

        // 引いた数字が履歴に含まれているか
        assert!(game.history.contains(&num.unwrap()));
        // 引いた数字が残りの数字に含まれていないか
        assert!(!game.remaining_numbers.contains(&num.unwrap()));
    }

    #[test]
    fn test_draw_all_numbers() {
        let rng = Box::new(XorShift::new(123));
        let mut game = BingoGame::new(75, rng);

        // 75回引く
        for _ in 0..75 {
            assert!(game.get_next_number().is_some());
        }

        assert_eq!(game.remaining_numbers.len(), 0);
        assert_eq!(game.history.len(), 75);

        // 76回目はNoneになるはず
        assert!(game.get_next_number().is_none());
    }

    #[test]
    fn test_reset() {
        let rng = Box::new(XorShift::new(123));
        let mut game = BingoGame::new(75, rng);
        game.get_next_number();
        game.get_next_number();

        assert_ne!(game.remaining_numbers.len(), 75);
        assert_ne!(game.history.len(), 0);

        game.reset();

        assert_eq!(game.remaining_numbers.len(), 75);
        assert_eq!(game.history.len(), 0);
    }
}
