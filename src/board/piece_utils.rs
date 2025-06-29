use super::{
    board::{Bitboard, Board},
    board_utils::notation_to_idx,
    piece::{Color, Piece, PieceType},
};

impl Board {
    pub fn get_piece_by_mask(&self, mask: u64) -> Option<Piece> {
        for &(color, piece_type, bb) in &[
            (Color::White, PieceType::Pawn, self.white_pawns),
            (Color::White, PieceType::Rook, self.white_rooks),
            (Color::White, PieceType::Knight, self.white_knights),
            (Color::White, PieceType::Bishop, self.white_bishops),
            (Color::White, PieceType::Queen, self.white_queens),
            (Color::White, PieceType::King, self.white_king),
            (Color::Black, PieceType::Pawn, self.black_pawns),
            (Color::Black, PieceType::Rook, self.black_rooks),
            (Color::Black, PieceType::Knight, self.black_knights),
            (Color::Black, PieceType::Bishop, self.black_bishops),
            (Color::Black, PieceType::Queen, self.black_queens),
            (Color::Black, PieceType::King, self.black_king),
        ] {
            if bb & mask != 0 {
                return Some(Piece::new(color, piece_type));
            }
        }
        None
    }

    pub fn get_piece_at(&self, pos: &str) -> Option<Piece> {
        let idx = notation_to_idx(pos).unwrap();
        let mask = 1u64 << idx;
        self.get_piece_by_mask(mask)
    }

    pub fn white_pieces(&self) -> Bitboard {
        self.white_pawns
            | self.white_rooks
            | self.white_knights
            | self.white_bishops
            | self.white_queens
            | self.white_king
    }

    pub fn black_pieces(&self) -> Bitboard {
        self.black_pawns
            | self.black_rooks
            | self.black_knights
            | self.black_bishops
            | self.black_queens
            | self.black_king
    }
}
