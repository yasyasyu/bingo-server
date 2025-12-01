pub trait BingoRng: Send + Sync {
    fn next_u32(&mut self) -> u32;
    fn shuffle(&mut self, slice: &mut [u8]);
}

pub struct XorShift {
    state: u32,
}

impl XorShift {
    pub fn new(seed: u32) -> Self {
        // seedが0だとXorShiftは動かないので、0の場合は適当な値にする
        let state = if seed == 0 { 123456789 } else { seed };
        Self { state }
    }
}

impl BingoRng for XorShift {
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
}
