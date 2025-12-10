use crate::domain::{AmidaGame, BingoGame};
use crate::rng::{MersenneTwister, XorShift};
use std::sync::{Arc, Mutex};

const BINGO_MAX_NUMBER: usize = 75;
const AMIDA_PRIZES_COUNT: usize = 8;

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
        Self {
            game: Arc::new(Mutex::new(BingoGame::new(
                BINGO_MAX_NUMBER,
                Box::new(XorShift::new(seed)),
            ))),
            amida: Arc::new(Mutex::new(AmidaGame::new(
                AMIDA_PRIZES_COUNT,
                Box::new(MersenneTwister::new(seed)),
            ))),
            seed,
        }
    }
}
