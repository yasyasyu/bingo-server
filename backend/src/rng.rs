const DEFAULT_SEED: u32 = 1_234_567_890;

pub trait IRng: Send + Sync {
    fn next_u32(&mut self) -> u32;
    fn shuffle(&mut self, slice: &mut [u8]);
    fn shift(&mut self, shift: usize);
}

#[derive(Clone)]
pub struct XorShift {
    state: u32,
}

impl XorShift {
    pub fn new(seed: u32) -> Self {
        // seedが0だとXorShiftは動かないので、0の場合は適当な値にする
        Self {
            state: if seed != 0 { seed } else { DEFAULT_SEED },
        }
    }
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
}
