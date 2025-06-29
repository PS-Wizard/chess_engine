#![allow(dead_code)]
use super::{
    board::{Bitboard, Board},
    piece::{Color, PieceType},
};

pub fn notation_to_idx(notation: &str) -> Option<usize> {
    let mut chars = notation.chars();
    let file_char = chars.next()?; // 'e'
    let rank_char = chars.next()?; // '2'

    let file = (file_char as u8).checked_sub(b'a')?;
    let rank = (rank_char as u8).checked_sub(b'1')?;

    if file > 7 || rank > 7 {
        return None;
    }

    Some((rank as usize) * 8 + (file as usize))
}

impl Board {
    pub fn get_bitboard_mut(&mut self, color: Color, piece_type: PieceType) -> &mut Bitboard {
        match (color, piece_type) {
            (Color::White, PieceType::Pawn) => &mut self.white_pawns,
            (Color::White, PieceType::Rook) => &mut self.white_rooks,
            (Color::White, PieceType::Knight) => &mut self.white_knights,
            (Color::White, PieceType::Bishop) => &mut self.white_bishops,
            (Color::White, PieceType::Queen) => &mut self.white_queens,
            (Color::White, PieceType::King) => &mut self.white_king,
            (Color::Black, PieceType::Pawn) => &mut self.black_pawns,
            (Color::Black, PieceType::Rook) => &mut self.black_rooks,
            (Color::Black, PieceType::Knight) => &mut self.black_knights,
            (Color::Black, PieceType::Bishop) => &mut self.black_bishops,
            (Color::Black, PieceType::Queen) => &mut self.black_queens,
            (Color::Black, PieceType::King) => &mut self.black_king,
        }
    }
}

#[cfg(test)]
mod board_utils_test {
    use super::*;

    #[test]
    fn test_notation_to_idx_valid() {
        assert_eq!(notation_to_idx("a1"), Some(0));
        assert_eq!(notation_to_idx("h1"), Some(7));
        assert_eq!(notation_to_idx("a8"), Some(56));
        assert_eq!(notation_to_idx("e4"), Some(28));
        assert_eq!(notation_to_idx("h8"), Some(63));
    }

    #[test]
    fn test_notation_to_idx_invalid() {
        assert_eq!(notation_to_idx("z9"), None);
        assert_eq!(notation_to_idx("a9"), None);
        assert_eq!(notation_to_idx("i1"), None);
        assert_eq!(notation_to_idx(""), None);
        assert_eq!(notation_to_idx("e"), None);
    }

    #[test]
    fn test_get_bitboard_mut() {
        let mut board = Board::new();

        let knights = board.get_bitboard_mut(Color::White, PieceType::Knight);
        assert_eq!(*knights, 0x0000_0000_0000_0042);

        // modify it
        *knights = 0x1234;
        assert_eq!(board.white_knights, 0x1234);
    }
}
