#![allow(dead_code)]
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
        let idx = notation_to_idx(pos)?;
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

#[cfg(test)]
mod piece_utils_test {
    use super::*;

    #[test]
    fn test_get_piece_at_starting_position() {
        let board = Board::new();

        assert_eq!(
            board.get_piece_at("e2"),
            Some(Piece::new(Color::White, PieceType::Pawn))
        );
        assert_eq!(
            board.get_piece_at("a1"),
            Some(Piece::new(Color::White, PieceType::Rook))
        );
        assert_eq!(
            board.get_piece_at("b8"),
            Some(Piece::new(Color::Black, PieceType::Knight))
        );
        assert_eq!(
            board.get_piece_at("e8"),
            Some(Piece::new(Color::Black, PieceType::King))
        );
        assert_eq!(
            board.get_piece_at("d8"),
            Some(Piece::new(Color::Black, PieceType::Queen))
        );

        assert_eq!(board.get_piece_at("e4"), None); // empty square
        assert_eq!(board.get_piece_at("z9"), None); // invalid notation
    }

    #[test]
    fn test_get_piece_by_mask() {
        let board = Board::new();
        let idx = notation_to_idx("g1").unwrap();
        let mask = 1u64 << idx;

        let piece = board.get_piece_by_mask(mask);
        assert_eq!(piece, Some(Piece::new(Color::White, PieceType::Knight)));

        let empty_mask = 1u64 << notation_to_idx("e4").unwrap();
        assert_eq!(board.get_piece_by_mask(empty_mask), None);
    }
}
