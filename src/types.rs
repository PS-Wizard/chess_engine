type Bitboard = u64;

struct Pieces {
    white_bishops: Bitboard,
    black_bishops: Bitboard,
    white_knights: Bitboard,
    black_knights: Bitboard,
    white_rooks: Bitboard,
    black_rooks: Bitboard,
    white_queens: Bitboard,
    black_queens: Bitboard,
    white_kings: Bitboard,
    black_kings: Bitboard,
    white_pawns: Bitboard,
    black_pawns: Bitboard,
}

impl Pieces {
    fn new() -> Self {
        Self {
            white_pawns: 0x00_FF_00_00_00_00_00_00,
            white_rooks: 0x81_00_00_00_00_00_00_00,
            white_knights: 0x42_00_00_00_00_00_00_00,
            white_bishops: 0x24_00_00_00_00_00_00_00,
            white_queens: 0x10_00_00_00_00_00_00_00,
            white_kings: 0x08_00_00_00_00_00_00_00,

            black_pawns: 0x00_00_00_00_00_00_FF_00,
            black_rooks: 0x00_00_00_00_00_00_00_81,
            black_knights: 0x00_00_00_00_00_00_00_42,
            black_bishops: 0x00_00_00_00_00_00_00_24,
            black_queens: 0x00_00_00_00_00_00_00_10,
            black_kings: 0x00_00_00_00_00_00_00_08,
        }
    }
    // You can add helper functions here like:
    fn white_pieces(&self) -> Bitboard {
        self.white_bishops
            | self.white_knights
            | self.white_rooks
            | self.white_queens
            | self.white_kings
            | self.white_pawns
    }

    fn black_pieces(&self) -> Bitboard {
        self.black_bishops
            | self.black_knights
            | self.black_rooks
            | self.black_queens
            | self.black_kings
            | self.black_pawns
    }

    fn get_board(&self) -> Bitboard {
        self.black_pieces() | self.white_pieces()
    }

    fn draw_board(&self) {
        
    }
}

