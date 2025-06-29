#![allow(dead_code)]

use crate::{moves::get_white_pawn_move, utils};

pub type Bitboard = u64;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
}

impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Self {
        Self { color, piece_type }
    }
}

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

    fn move_piece(&mut self, from: &str, to: &str) -> Result<(), String> {
        let from_idx = utils::notation_to_idx(from).unwrap();
        let to_idx = utils::notation_to_idx(to).unwrap();
        let from_mask = 1u64 << from_idx;
        let to_mask = 1u64 << to_idx;

        let piece = self
            .get_piece_by_mask(from_mask)
            .ok_or(format!("No piece at {}", from))?;

        if match piece.color {
            Color::White => self.white_pieces(),
            Color::Black => self.black_pieces(),
        } & to_mask
            != 0
        {
            return Err("Cannot capture your own piece".to_string());
        }

        match piece {
            Piece {
                color: Color::White,
                piece_type: PieceType::Pawn,
            } => {
                let (one_step, two_steps) = get_white_pawn_move(from_idx);

                if let Some(two_step) = two_steps {
                    println!("Can Move To {:?} or {:?}", one_step, two_step);
                } else {
                    println!("Can Move To {:?}", one_step);
                }
            }
            _ => println!("Some other piece"),
        }

        // Remove source
        *self.get_bitboard_mut(piece.color, piece.piece_type) &= !from_mask;

        // Remove captured
        for pt in [
            PieceType::Pawn,
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
        ] {
            *self.get_bitboard_mut(Color::White, pt) &= !to_mask;
            *self.get_bitboard_mut(Color::Black, pt) &= !to_mask;
        }

        // Place piece at destination
        *self.get_bitboard_mut(piece.color, piece.piece_type) |= to_mask;

        println!(
            "Moved {:?} {:?} from {} to {}",
            piece.color, piece.piece_type, from, to
        );

        Ok(())
    }
}

#[cfg(test)]
mod types_tests {
    use super::*;

    #[test]
    fn test_draw_board() {
        let board = Board::new();
        board.draw_board();
    }

    #[test]
    fn test_get_piece() {
        let board = Board::new();
        assert_eq!(
            board.get_piece_at("a2"),
            Some(Piece::new(Color::White, PieceType::Pawn))
        );
        assert_eq!(
            board.get_piece_at("a8"),
            Some(Piece::new(Color::Black, PieceType::Rook))
        );
        assert_eq!(board.get_piece_at("a4"), None);
    }

    #[test]
    fn test_move_piece() {
        let mut board = Board::new();

        // Move white pawn from a2 to a4
        assert!(board.move_piece("a2", "a4").is_ok());
        board.draw_board();
        assert_eq!(board.get_piece_at("a2"), None);
        assert_eq!(
            board.get_piece_at("a4"),
            Some(Piece::new(Color::White, PieceType::Pawn))
        );

        // // Move black pawn from a7 to a5
        // assert!(board.move_piece("a7", "a5").is_ok());
        // board.draw_board();
        // assert_eq!(board.get_piece_at("a7"), None);
        // assert_eq!(
        //     board.get_piece_at("a5"),
        //     Some(Piece::new(Color::Black, PieceType::Pawn))
        // );
        //
        // // Move white knight from b1 to c3
        // assert!(board.move_piece("b1", "c3").is_ok());
        // board.draw_board();
        // assert_eq!(board.get_piece_at("b1"), None);
        // assert_eq!(
        //     board.get_piece_at("c3"),
        //     Some(Piece::new(Color::White, PieceType::Knight))
        // );
        //
        // // Move black knight from b8 to c6
        // assert!(board.move_piece("b8", "c6").is_ok());
        // board.draw_board();
        // assert_eq!(board.get_piece_at("b8"), None);
        // assert_eq!(
        //     board.get_piece_at("c6"),
        //     Some(Piece::new(Color::Black, PieceType::Knight))
        // );
        //
        // // Invalid move: no piece on e3
        // assert!(board.move_piece("e3", "e4").is_err());
        // board.draw_board();
        //
        // // Valid capture: white pawn captures black pawn at a5
        // assert!(board.move_piece("a4", "a5").is_ok());
        // board.draw_board();
        // assert_eq!(
        //     board.get_piece_at("a5"),
        //     Some(Piece::new(Color::White, PieceType::Pawn))
        // );
        // assert_eq!(board.get_piece_at("a4"), None);
        //
        // // Invalid move: try to capture own knight
        // assert!(board.move_piece("c3", "a5").is_err()); // can't take friendly pawn
        // board.draw_board();
    }
}
