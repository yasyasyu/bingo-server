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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_new_game_initialization() {
        let game = BingoGame::new();
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
        let mut game = BingoGame::new();
        let initial_len = game.remaining_numbers.len();

        // 1回引く
        let num = game.draw_number();
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
        let mut game = BingoGame::new();

        // 75回引く
        for _ in 0..75 {
            assert!(game.draw_number().is_some());
        }

        assert_eq!(game.remaining_numbers.len(), 0);
        assert_eq!(game.history.len(), 75);

        // 76回目はNoneになるはず
        assert!(game.draw_number().is_none());
    }

    #[test]
    fn test_reset() {
        let mut game = BingoGame::new();
        game.draw_number();
        game.draw_number();

        assert_ne!(game.remaining_numbers.len(), 75);
        assert_ne!(game.history.len(), 0);

        game.reset();

        assert_eq!(game.remaining_numbers.len(), 75);
        assert_eq!(game.history.len(), 0);
    }
}
