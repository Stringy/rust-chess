pub struct Bitboard(u64);

impl Bitboard {

    pub fn add(&self, other: Bitboard) -> Bitboard {
        (self.0 + other.0).into()
    }

    pub fn add_eq(&mut self, other: Bitboard) {
        self.0 ^= other.0;
    }

    pub fn xor(&self, other: Bitboard) -> Bitboard {
        (self.0 ^ other.0).into()
    }

    pub fn xor_eq(&mut self, other: Bitboard) {
        self.0 ^= other.0;
    }

    pub fn lsb(&self) -> usize {
        const debruijn: u64 = 0x07edd5e59a4e28c2_u64;

        const indexes: [usize; 64] = [
            63, 0, 58, 1, 59, 47, 53, 2,
            60, 39, 48, 27, 54, 33, 42, 3,
            61, 51, 37, 40, 49, 18, 28, 20,
            55, 30, 34, 11, 43, 14, 22, 4,
            62, 57, 46, 52, 38, 26, 32, 41,
            50, 36, 17, 19, 29, 10, 13, 21,
            56, 45, 25, 31, 35, 16, 9, 12,
            44, 24, 15, 8, 23, 7, 6, 5,
        ];

        let n = self.0;
        indexes[(((n & (!n+1))*debruijn)>>58) as usize]
    }

    pub fn msb(&self) -> usize {
        0
    }
}

impl Into<Bitboard> for u64 {
    fn into(self) -> Bitboard {
        Bitboard(self)
    }
}