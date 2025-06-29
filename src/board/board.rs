pub type Bitboard = u64;

pub struct Board {
    pub white_pawns: Bitboard,
    pub white_rooks: Bitboard,
    pub white_knights: Bitboard,
    pub white_bishops: Bitboard,
    pub white_queens: Bitboard,
    pub white_king: Bitboard,
    pub black_pawns: Bitboard,
    pub black_rooks: Bitboard,
    pub black_knights: Bitboard,
    pub black_bishops: Bitboard,
    pub black_queens: Bitboard,
    pub black_king: Bitboard,
}

impl Board {
    fn new() -> Self {
        Self {
            white_pawns: 0x0000_0000_0000_FF00,
            white_rooks: 0x0000_0000_0000_0081,
            white_knights: 0x0000_0000_0000_0042,
            white_bishops: 0x0000_0000_0000_0024,
            white_queens: 0x0000_0000_0000_0008,
            white_king: 0x0000_0000_0000_0010,
            black_pawns: 0x00FF_0000_0000_0000,
            black_rooks: 0x8100_0000_0000_0000,
            black_knights: 0x4200_0000_0000_0000,
            black_bishops: 0x2400_0000_0000_0000,
            black_queens: 0x0800_0000_0000_0000,
            black_king: 0x1000_0000_0000_0000,
        }
    }
}
