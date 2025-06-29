#![allow(dead_code)]

use crate::types::{Bitboard, Board, Color, Piece, PieceType};

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

    pub fn black_pieces(&self) -> Bitboard {
        self.black_pawns
            | self.black_rooks
            | self.black_knights
            | self.black_bishops
            | self.black_queens
            | self.black_king
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

    pub fn draw_board(&self) {
        println!("===================================");
        let files = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        for rank in (0..8).rev() {
            print!("{} ", rank + 1);
            for file in 0..8 {
                let idx = rank * 8 + file;
                let mask = 1u64 << idx;
                let piece = self.get_piece_by_mask(mask);
                let symbol = match piece {
                    Some(Piece {
                        color: Color::White,
                        piece_type,
                    }) => match piece_type {
                        PieceType::Pawn => 'P',
                        PieceType::Rook => 'R',
                        PieceType::Knight => 'N',
                        PieceType::Bishop => 'B',
                        PieceType::Queen => 'Q',
                        PieceType::King => 'K',
                    },
                    Some(Piece {
                        color: Color::Black,
                        piece_type,
                    }) => match piece_type {
                        PieceType::Pawn => 'p',
                        PieceType::Rook => 'r',
                        PieceType::Knight => 'n',
                        PieceType::Bishop => 'b',
                        PieceType::Queen => 'q',
                        PieceType::King => 'k',
                    },
                    None => '.',
                };
                print!("{} ", symbol);
            }
            println!();
        }
        print!("  ");
        for file in files {
            print!("{} ", file);
        }
        println!();
        println!("===================================");
    }
}

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

#[cfg(test)]
mod utils_tests {
    use super::*;

    #[test]
    fn test_notation_to_idx() {
        assert_eq!(notation_to_idx("a1"), Some(0));
        assert_eq!(notation_to_idx("h8"), Some(63));
        assert_eq!(notation_to_idx("e2"), Some(12));
    }
}
