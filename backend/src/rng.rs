const DEFAULT_SEED: u32 = 1_234_567_890;

/// 乱数生成器のインターフェース
///
/// 乱数の生成、シャッフル、状態のリセットなどの機能を提供します。
/// スレッドセーフ（Send + Sync）である必要があります。
pub trait IRng: Send + Sync {
    /// 次の32ビット乱数を生成します
    fn next(&mut self) -> u32;
    /// バイト列をシャッフルします
    fn shuffle(&mut self, slice: &mut [u8]) {
        // Fisher-Yates shuffle
        let len = slice.len();
        if len < 2 {
            return;
        }

        for i in (1..len).rev() {
            // 0..=i の範囲のランダムなインデックスを取得
            let j = (self.next() as usize) % (i + 1);
            slice.swap(i, j);
        }
    }
    
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
    pub fn new(seed: u32) -> Self {
        // seedが0だとXorShiftは動かないので、0の場合は適当な値にする
        let mut rng = Self {
            initial_state: if seed != 0 { seed } else { DEFAULT_SEED },
            state: 0,
        };
        rng.reset();
        rng
    }
}

impl IRng for XorShift {
    fn next(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }

    fn reset(&mut self) {
        self.state = self.initial_state.clone();
    }
}

/// Mersenne Twister (MT19937) 擬似乱数生成器
///
/// 周期が2^19937-1と非常に長く、統計的品質の高い乱数生成器です。
/// XorShiftよりも高品質ですが、やや重い処理となります。
#[derive(Clone)]
pub struct MersenneTwister {
    initial_seed: u32,
    mt: [u32; 624],
    index: usize,
}

impl MersenneTwister {
    const N: usize = 624;
    const M: usize = 397;
    const MATRIX_A: u32 = 0x9908b0df;
    const UPPER_MASK: u32 = 0x80000000;
    const LOWER_MASK: u32 = 0x7fffffff;

    pub fn new(seed: u32) -> Self {
        let seed = if seed != 0 { seed } else { DEFAULT_SEED };
        let mut rng = Self {
            initial_seed: seed,
            mt: [0; 624],
            index: 624,
        };
        rng.initialize(seed);
        rng
    }

    fn initialize(&mut self, seed: u32) {
        self.mt[0] = seed;
        for i in 1..Self::N {
            self.mt[i] = 1812433253u32
                .wrapping_mul(self.mt[i - 1] ^ (self.mt[i - 1] >> 30))
                .wrapping_add(i as u32);
        }
        self.index = Self::N;
    }

    fn twist(&mut self) {
        for i in 0..Self::N {
            let x = (self.mt[i] & Self::UPPER_MASK) | (self.mt[(i + 1) % Self::N] & Self::LOWER_MASK);
            let mut x_a = x >> 1;
            if x & 1 != 0 {
                x_a ^= Self::MATRIX_A;
            }
            self.mt[i] = self.mt[(i + Self::M) % Self::N] ^ x_a;
        }
        self.index = 0;
    }
}

impl IRng for MersenneTwister {

    fn next(&mut self) -> u32 {
        if self.index >= Self::N {
            self.twist();
        }

        let mut y = self.mt[self.index];
        self.index += 1;

        // Tempering
        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c5680;
        y ^= (y << 15) & 0xefc60000;
        y ^= y >> 18;

        y
    }

    fn reset(&mut self) {
        self.initialize(self.initial_seed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mersenne_twister_correctness() {
        // Test with default seed 5489
        // Expected values from standard MT19937 implementation (mt19937ar.c)
        let mut rng = MersenneTwister::new(5489);
        
        assert_eq!(rng.next(), 3499211612);
        assert_eq!(rng.next(), 581869302);
        assert_eq!(rng.next(), 3890346734);
        assert_eq!(rng.next(), 3586334585);
        assert_eq!(rng.next(), 545404204);
    }
}