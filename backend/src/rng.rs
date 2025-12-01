const DEFAULT_SEED: u32 = 1_234_567_890;

/// 乱数生成器のインターフェース
///
/// 乱数の生成、シャッフル、状態のリセットなどの機能を提供します。
/// スレッドセーフ（Send + Sync）である必要があります。
pub trait IRng: Send + Sync {
    /// 次の32ビット乱数を生成します
    fn next_u32(&mut self) -> u32;
    /// バイト列をシャッフルします
    fn shuffle(&mut self, slice: &mut [u8]);
    /// 乱数生成器の状態を指定回数分進めます
    fn shift(&mut self, shift: usize);
    /// 乱数生成器の状態を初期化します
    fn reset(&mut self);
}

/// XorShiftアルゴリズムによる擬似乱数生成器
///
/// 軽量で高速な乱数生成器です。暗号学的な安全性はありませんが、
/// ゲームのランダム要素としては十分な性能を持ちます。
#[derive(Clone)]
pub struct XorShift {
    initial_state: u32,
    state: u32,
}

impl XorShift {
    /// 指定されたシード値で新しいインスタンスを作成します
    pub fn new(seed: u32) -> Self {
        // seedが0だとXorShiftは動かないので、0の場合は適当な値にする
        let mut rng = Self {
            initial_state: if seed != 0 { seed } else { DEFAULT_SEED },
            state: 0,
        };
        rng.reset();
        rng
    }

    /// シード値を指定し、さらに指定回数分状態を進めた状態でインスタンスを作成します
    ///
    /// 複数の乱数生成器を使用する場合に、生成される乱数列が重複しないようにするために使用します。
    pub fn shift_new(seed: u32, shift: usize) -> Self {
        let mut rng = Self::new(seed);

        rng.shift(shift);
        rng
    }
}

impl IRng for XorShift {
    fn next_u32(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }

    fn shuffle(&mut self, slice: &mut [u8]) {
        // Fisher-Yates shuffle
        let len = slice.len();
        if len < 2 {
            return;
        }

        for i in (1..len).rev() {
            // 0..=i の範囲のランダムなインデックスを取得
            let j = (self.next_u32() as usize) % (i + 1);
            slice.swap(i, j);
        }
    }

    fn shift(&mut self, shift: usize) {
        // 指定された回数だけnext_u32を呼び出すことで状態を進める
        for _ in 0..shift {
            self.next_u32();
        }
    }
    fn reset(&mut self) {
        self.state = self.initial_state.clone();
    }
}
