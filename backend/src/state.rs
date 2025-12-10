use crate::domain::{AmidaGame, BingoGame};
use crate::rng::{MersenneTwister, XorShift};
use std::fs;
use std::sync::{Arc, Mutex};

const BINGO_MAX_NUMBER: usize = 75;

/// アプリケーション全体の状態を管理する構造体
///
/// ビンゴゲームとあみだくじの状態をスレッドセーフに保持します。
/// Axumの状態共有機能を通じて、各ハンドラからアクセスされます。
#[derive(Clone)]
pub struct AppState {
    /// ビンゴゲームの状態（排他制御あり）
    pub game: Arc<Mutex<BingoGame>>,
    /// あみだくじの状態（排他制御あり）
    pub amida: Arc<Mutex<AmidaGame>>,
    /// 初期シード値（参照用）
    pub seed: u32,
}

impl AppState {
    /// 新しいアプリケーション状態を作成します
    ///
    /// 指定されたシード値を使用して、ビンゴとあみだくじのゲーム状態を初期化します。
    /// あみだくじの乱数生成器は、ビンゴの乱数生成器と状態が重ならないように
    /// シード値をシフトして初期化されます。
    pub fn new(seed: u32) -> Self {
        let prize_count = fs::read_to_string("prize.txt")
            .or_else(|_| fs::read_to_string("../prize.txt"))
            .ok()
            .and_then(|s| {
                eprintln!("prize count loaded: [{}]", &s.trim().to_string());
                Some(s.trim().parse().ok()?)
            })
            .unwrap_or(8);

        Self {
            game: Arc::new(Mutex::new(BingoGame::new(
                BINGO_MAX_NUMBER,
                Box::new(XorShift::new(seed)),
            ))),
            amida: Arc::new(Mutex::new(AmidaGame::new(
                prize_count,
                Box::new(MersenneTwister::new(seed)),
            ))),
            seed,
        }
    }
}
